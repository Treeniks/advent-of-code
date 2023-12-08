use regex::Regex;
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
            return i + 1
        }
    }

    unreachable!()
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));

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
}
