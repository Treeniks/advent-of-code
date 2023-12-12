use std::io::Read;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
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

fn parse_line(line: &str) -> Line {
    let mut it = line.split_whitespace();

    let springs = it.next().unwrap();
    let records = it.next().unwrap();

    let springs: Vec<Spring> = springs.chars().map(|c| c.try_into().unwrap()).collect();
    let records: Vec<usize> = records.split(',').map(|s| s.parse().unwrap()).collect();

    Line { springs, records }
}

fn is_correct(springs: &[Spring], records: &[usize]) -> bool {
    let mut groups: Vec<usize> = vec![];

    let mut b = false;
    for s in springs {
        match (s, b) {
            (Spring::Broken, true) => {
                let i = groups.len() - 1;
                groups[i] += 1;
            }
            (Spring::Broken, false) => {
                groups.push(1);
                b = true;
            }
            _ => b = false,
        }
    }

    groups == records
}

fn combinations(springs: &[Spring], records: &[usize]) -> usize {
    let first_unknown = match springs
        .iter()
        .enumerate()
        .find(|&(_, &spring)| spring == Spring::Unknown)
    {
        Some((i, _)) => i,
        None => {
            if is_correct(springs, records) {
                return 1;
            } else {
                return 0;
            }
        }
    };

    let mut tmp = Vec::with_capacity(springs.len());
    tmp.extend_from_slice(springs);

    let mut tmp2 = Vec::with_capacity(springs.len());
    tmp2.extend_from_slice(springs);

    tmp[first_unknown] = Spring::Broken;
    tmp2[first_unknown] = Spring::Operational;

    return combinations(&tmp, records) + combinations(&tmp2, records);
}

fn extend_part2(springs: &mut Vec<Spring>, records: &mut Vec<usize>) {
    let clone = springs.clone();
    springs.extend(clone.clone());
    springs.extend(clone.clone());
    springs.extend(clone.clone());
    springs.extend(clone);

    let clone = records.clone();
    records.extend(clone.clone());
    records.extend(clone.clone());
    records.extend(clone.clone());
    records.extend(clone);
}

fn part1(input: &str) -> usize {
    let lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    return lines.iter().fold(0, |acc, line| {
        acc + combinations(&line.springs, &line.records)
    });
}

fn part2(input: &str) -> usize {
    let mut lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    lines
        .iter_mut()
        .for_each(|l| extend_part2(&mut l.springs, &mut l.records));
    return lines.iter().fold(0, |acc, line| {
        acc + combinations(&line.springs, &line.records)
    });
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

    //     #[test]
    //     fn test_part2() {
    //         let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1
    // ";
    //
    //         let expected = 525152;
    //         let result = part2(input);
    //
    //         assert_eq!(expected, result);
    //     }
}
