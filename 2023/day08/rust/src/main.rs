use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io::Read;
use std::ops::ControlFlow;

#[derive(Debug, Clone, Copy)]
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
    let re = Regex::new(r#"(?<from>.{3}) = \((?<left>.{3}), (?<right>.{3})\)"#).unwrap();

    paths
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            // we cannot use indexing here because of lifetime stuff
            let from = caps.name("from").unwrap().as_str();
            let left = caps.name("left").unwrap().as_str();
            let right = caps.name("right").unwrap().as_str();

            (from, (left, right))
        })
        .collect()
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

fn next_node<'a>(
    paths: &'a HashMap<&'a str, (&'a str, &'a str)>,
    node: &'a str,
    instruction: Instruction,
) -> &'a str {
    match instruction {
        Instruction::Left => paths[node].0,
        Instruction::Right => paths[node].1,
    }
}

// I am aware this is ridiculous code
// but I wanted to see how far I can push iterators here
//
// a more reasonable solution can be found in commits

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.trim().lines().collect();
    let instructions = parse_instructions(lines[0]);
    let paths = parse_paths(&lines[2..]);

    match instructions
        .into_iter()
        .cycle()
        .try_fold(("AAA", 0), |(current, steps), instr| {
            if current == "ZZZ" {
                ControlFlow::Break(steps)
            } else {
                ControlFlow::Continue((next_node(&paths, current, instr), steps + 1))
            }
        }) {
        // TODO replace with `.break_value().unwrap()` once `break_value()` is stable
        ControlFlow::Continue(_) => unreachable!(),
        ControlFlow::Break(result) => result,
    }
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.trim().lines().collect();
    let instructions = parse_instructions(lines[0]);
    let paths = parse_paths(&lines[2..]);

    let start_nodes = get_start_nodes_part2(&lines[2..]);

    start_nodes
        .into_iter()
        .map(|start| {
            match instructions.iter().cycle().try_fold(
                (start, 0, None),
                |(current, steps, first_iteration), instr| {
                    if current.ends_with('Z') {
                        match first_iteration {
                            Some(first_iteration) => {
                                ControlFlow::Break(lcm(first_iteration, steps))
                            }
                            None => ControlFlow::Continue((
                                next_node(&paths, current, *instr),
                                1,
                                Some(steps),
                            )),
                        }
                    } else {
                        ControlFlow::Continue((
                            next_node(&paths, current, *instr),
                            steps + 1,
                            first_iteration,
                        ))
                    }
                },
            ) {
                // TODO replace with `.break_value().unwrap()` once `break_value()` is stable
                ControlFlow::Continue(_) => unreachable!(),
                ControlFlow::Break(res) => res,
            }
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
