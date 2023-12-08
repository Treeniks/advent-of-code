use num::integer::lcm;
use regex::Regex;
use std::thread;
use std::{collections::HashMap, io::Read};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(()),
        }
    }
}

fn parse_instructions(line: &str) -> Box<[Instruction]> {
    line.chars()
        .map(|c| c.try_into().unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn parse_paths<'a>(paths: &[&'a str]) -> HashMap<&'a str, (&'a str, &'a str)> {
    let mut result = HashMap::new();

    let to_re = Regex::new(r#"\((?<left>.{3}), (?<right>.{3})\)"#).unwrap();

    for line in paths {
        let mut it = line.split('=');
        let from = it.next().unwrap().trim();
        let to = it.next().unwrap().trim();

        let caps = to_re.captures(to).unwrap();

        // we cannot use indexing here because of lifetime stuff
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();

        _ = result.insert(from, (left, right));
    }

    result
}

fn get_start_nodes_part2<'a>(paths: &[&'a str]) -> Box<[&'a str]> {
    paths
        .iter()
        .filter_map(|l| {
            let m = l.split('=').next().unwrap().trim();
            if m.ends_with('A') {
                Some(m)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.trim().lines().collect();
    let instructions = parse_instructions(lines[0]);
    let paths = parse_paths(&lines[2..]);

    let mut current = "AAA";
    for (i, instr) in instructions.iter().cycle().enumerate() {
        match instr {
            Instruction::Left => current = paths[current].0,
            Instruction::Right => current = paths[current].1,
        }

        if current == "ZZZ" {
            return i + 1;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.trim().lines().collect();
    let instructions = parse_instructions(lines[0]);
    let paths = parse_paths(&lines[2..]);

    // single threaded solution that takes way too long to run
    //
    // let mut nodes = get_start_nodes_part2(&lines[2..]);
    // for (i, instr) in instructions.iter().cycle().enumerate() {
    //     nodes.iter_mut().for_each(|n| match instr {
    //         Instruction::Left => *n = paths[n].0,
    //         Instruction::Right => *n = paths[n].1,
    //     });
    //
    //     if nodes.iter().all(|n| n.ends_with('Z')) {
    //         return i + 1;
    //     }
    // }
    // unreachable!()

    let start_nodes = get_start_nodes_part2(&lines[2..]);

    let iterations = thread::scope(|s| {
        let mut threads = Vec::new();

        for start in start_nodes.iter() {
            threads.push(s.spawn(|| {
                let mut current = *start;
                let mut first_iteration = 0;

                for (i, instr) in instructions.iter().cycle().enumerate() {
                    match instr {
                        Instruction::Left => current = paths[current].0,
                        Instruction::Right => current = paths[current].1,
                    }

                    if current.ends_with('Z') {
                        if first_iteration == 0 {
                            first_iteration = i + 1;
                        } else {
                            return (first_iteration, i + 1 - first_iteration);
                        }
                    }
                }

                unreachable!()
            }));
        }

        threads
            .into_iter()
            .map(|t| {
                let val = t.join().unwrap();
                lcm(val.0, val.1)
            })
            .collect::<Vec<_>>()
    });

    iterations.into_iter().reduce(lcm).unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let expected = 2;
        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let expected = 6;
        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let expected = 6;
        let result = part2(input);

        assert_eq!(expected, result);
    }
}
