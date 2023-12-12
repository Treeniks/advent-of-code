use std::collections::HashMap;
use std::{fmt::Display, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug, Clone)]
struct Line {
    springs: Vec<Spring>,
    records: Vec<usize>,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Broken),
            '?' => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in &self.springs {
            match spring {
                Spring::Operational => write!(f, ".")?,
                Spring::Broken => write!(f, "#")?,
                Spring::Unknown => write!(f, "?")?,
            }
        }

        write!(f, " ")?;

        for (i, record) in self.records.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", record)?;
            } else {
                write!(f, ",{}", record)?;
            }
        }

        Ok(())
    }
}

fn parse_line(line: &str) -> Line {
    let mut it = line.split_whitespace();

    let springs = it.next().unwrap();
    let records = it.next().unwrap();

    let springs: Vec<Spring> = springs.chars().map(|c| c.try_into().unwrap()).collect();
    let records: Vec<usize> = records.split(',').map(|s| s.parse().unwrap()).collect();

    Line { springs, records }
}

fn check(springs: &[Spring], records: &[usize]) -> bool {
    let spring_groups: Vec<usize> = springs
        .split(|spring| *spring == Spring::Operational)
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect();

    spring_groups == records
}

fn check_weak(springs: &[Spring], records: &[usize]) -> Option<usize> {
    let spring_groups: Vec<usize> = springs
        .split(|spring| *spring == Spring::Operational)
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect();

    if spring_groups.len() <= records.len() && spring_groups == records[..spring_groups.len()] {
        Some(spring_groups.len())
    } else {
        None
    }
}

fn combinations(
    springs: &mut [Spring],
    records: &[usize],
    cache: &mut HashMap<(Vec<Spring>, Vec<usize>), usize>,
) -> usize {
    if let Some(r) = cache.get(&(springs.to_vec(), records.to_vec())) {
        return *r;
    }

    if records.is_empty() {
        return springs.iter().all(|&spring| spring != Spring::Broken) as usize;
    }

    let first_unknown = match springs
        .iter()
        .enumerate()
        .find(|&(_, &spring)| spring == Spring::Unknown)
    {
        Some((i, _)) => i,
        None => return check(springs, records) as usize,
    };

    let r1 = if !(springs[..first_unknown]
        .iter()
        .all(|spring| *spring == Spring::Broken)
        && records[0] < first_unknown)
    {
        springs[first_unknown] = Spring::Broken;
        let r1 = combinations(springs, records, cache);

        springs[first_unknown] = Spring::Unknown;

        cache.insert((springs.to_vec(), records.to_vec()), r1);
        r1
    } else {
        0
    };

    match check_weak(&springs[..first_unknown], records) {
        Some(i) => {
            let r2 = combinations(&mut springs[first_unknown + 1..], &records[i..], cache);
            cache.insert((springs.to_vec(), records.to_vec()), r1 + r2);
            r1 + r2
        }
        None => {
            cache.insert((springs.to_vec(), records.to_vec()), r1);
            r1
        }
    }
}

fn extend(springs: &mut Vec<Spring>, records: &mut Vec<usize>, n: usize) {
    let clone_springs = springs.clone();
    let clone_records = records.clone();
    for _ in 0..n - 1 {
        springs.push(Spring::Unknown);
        springs.extend(clone_springs.clone());
        records.extend(clone_records.clone());
    }
}

fn part1(input: &str) -> usize {
    let mut lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    return lines.iter_mut().fold(0, |acc, line| {
        let mut cache = HashMap::new();
        acc + combinations(&mut line.springs, &line.records, &mut cache)
    });
}

fn part2(input: &str) -> usize {
    let mut lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    lines
        .iter_mut()
        .for_each(|l| extend(&mut l.springs, &mut l.records, 5));

    let mut cache = HashMap::new();
    return lines.iter_mut().fold(0, |acc, line| {
        acc + combinations(&mut line.springs, &line.records, &mut cache)
    });
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
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

        let expected = 21;
        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
    ";

        let expected = 525152;
        let result = part2(input);

        assert_eq!(expected, result);
    }
}
