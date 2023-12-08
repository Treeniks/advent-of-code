use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io::Read;

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

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars().map(|c| c.try_into().unwrap()).collect()
}

fn parse_paths<'a>(paths: &[&'a str]) -> HashMap<&'a str, (&'a str, &'a str)> {
    let mut result = HashMap::new();

    let re = Regex::new(r#"(?<from>.{3}) = \((?<left>.{3}), (?<right>.{3})\)"#).unwrap();

    for line in paths {
        let caps = re.captures(line).unwrap();

        // we cannot use indexing here because of lifetime stuff
        let from = caps.name("from").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();

        _ = result.insert(from, (left, right));
    }

    result
}

fn get_start_nodes_part2<'a>(paths: &[&'a str]) -> Vec<&'a str> {
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
        .collect()
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.trim().lines().collect();
    let instructions = parse_instructions(lines[0]);
    let paths = parse_paths(&lines[2..]);

    let mut current = "AAA";
    for (i, instr) in instructions.iter().cycle().enumerate() {
        current = match instr {
            Instruction::Left => paths[current].0,
            Instruction::Right => paths[current].1,
        };

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

    // naïve solution that takes way too long to run
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

    start_nodes
        .into_iter()
        .map(|start| {
            instructions
                .iter()
                .cycle()
                .scan(
                    (start, 0, None),
                    |(current, steps, first_iteration), instr| {
                        *current = match instr {
                            Instruction::Left => paths[current].0,
                            Instruction::Right => paths[current].1,
                        };
                        *steps += 1;

                        if current.ends_with('Z') {
                            match first_iteration {
                                Some(first_iteration) => Some(Some(lcm(*first_iteration, *steps))),
                                None => {
                                    *first_iteration = Some(*steps);
                                    *steps = 0;
                                    Some(None)
                                }
                            }
                        } else {
                            Some(None)
                        }
                    },
                )
                .find_map(|x| x)
                .unwrap()
        })
        .reduce(lcm)
        .unwrap()
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
