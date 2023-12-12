use std::{fmt::Display, io::Read, thread};

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

fn combinations(springs: &mut [Spring], records: &[usize]) -> usize {
    match (&springs, records) {
        ([], [_]) => return 0,
        ([Spring::Unknown], []) | ([Spring::Operational], []) => return 1,
        ([Spring::Broken], []) => return 0,
        ([Spring::Broken], [1]) => return 1,
        ([Spring::Unknown], [1]) => return 1,
        ([_], [_]) => return 0,
        ([Spring::Unknown, Spring::Unknown], [1]) => return 2,
        ([Spring::Unknown, Spring::Unknown], [2]) => return 1,
        ([Spring::Unknown, Spring::Unknown], [_]) => return 0,

        ([Spring::Unknown, Spring::Broken], [1]) | ([Spring::Broken, Spring::Unknown], [1]) => {
            return 1
        }
        ([Spring::Unknown, Spring::Broken], [2]) | ([Spring::Broken, Spring::Unknown], [2]) => {
            return 1
        }
        ([Spring::Unknown, Spring::Broken], [_]) | ([Spring::Broken, Spring::Unknown], [_]) => {
            return 0
        }

        ([Spring::Broken, Spring::Broken], [2]) => return 1,
        ([Spring::Broken, Spring::Broken], [_]) => return 0,

        ([Spring::Unknown, Spring::Operational], [1])
        | ([Spring::Operational, Spring::Unknown], [1]) => return 1,
        ([Spring::Unknown, Spring::Operational], [_])
        | ([Spring::Operational, Spring::Unknown], [_]) => return 0,
        ([Spring::Unknown, Spring::Operational], [])
        | ([Spring::Operational, Spring::Unknown], []) => return 1,

        ([Spring::Operational, Spring::Operational], []) => return 1,
        ([Spring::Operational, Spring::Operational], [_]) => return 0,

        ([Spring::Broken, Spring::Operational], [1])
        | ([Spring::Operational, Spring::Broken], [1]) => return 1,
        ([Spring::Broken, Spring::Operational], [_])
        | ([Spring::Operational, Spring::Broken], [_]) => return 0,
        ([Spring::Broken, Spring::Operational], [])
        | ([Spring::Operational, Spring::Broken], []) => return 0,

        (_, []) => {
            if springs.iter().all(|spring| *spring != Spring::Broken) {
                return 1;
            } else {
                return 0;
            }
        }
        _ => {}
    }

    let first_unknown = match springs
        .iter()
        .enumerate()
        .find(|&(_, &spring)| spring == Spring::Unknown)
    {
        Some((i, _)) => i,
        None => {
            if check(springs, records) {
                return 1;
            } else {
                return 0;
            }
        }
    };

    let r1 = if !(springs[..first_unknown]
        .iter()
        .all(|spring| *spring == Spring::Broken)
        && records[0] < first_unknown)
    {
        springs[first_unknown] = Spring::Broken;
        let r1 = combinations(springs, records);

        springs[first_unknown] = Spring::Unknown;

        r1
    } else {
        0
    };

    match check_weak(&springs[..first_unknown], records) {
        Some(i) => {
            let r2 = combinations(&mut springs[first_unknown + 1..], &records[i..]);
            r1 + r2
        }
        None => r1,
    }
}

fn extend_part2(springs: &mut Vec<Spring>, records: &mut Vec<usize>) {
    let clone = springs.clone();
    springs.push(Spring::Unknown);
    springs.extend(clone.clone());
    springs.push(Spring::Unknown);
    springs.extend(clone.clone());
    springs.push(Spring::Unknown);
    springs.extend(clone.clone());
    springs.push(Spring::Unknown);
    springs.extend(clone);

    let clone = records.clone();
    records.extend(clone.clone());
    records.extend(clone.clone());
    records.extend(clone.clone());
    records.extend(clone);
}

fn part1(input: &str) -> usize {
    let mut lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    return lines.iter_mut().fold(0, |acc, line| {
        acc + combinations(&mut line.springs, &line.records)
    });
}

const THREAD_COUNT: usize = 16;

fn part2(input: &str) -> usize {
    let mut lines: Vec<Line> = input.trim().lines().map(parse_line).collect();
    lines
        .iter_mut()
        .for_each(|l| extend_part2(&mut l.springs, &mut l.records));

    let mut ls = lines.chunks_exact(lines.len() / THREAD_COUNT);
    thread::scope(move |s| {
        let mut threads = vec![];

        for _ in 0..THREAD_COUNT {
            let mut lines = Vec::from(ls.next().unwrap());
            threads.push(s.spawn(move || {
                lines.iter_mut().fold(0, |acc, line| {
                    println!("finding combinations for line {}", line);
                    acc + combinations(&mut line.springs, &line.records)
                })
            }))
        }

        let r = Vec::from(ls.remainder()).iter_mut().fold(0, |acc, line| {
            println!("finding combinations for line {}", line);
            acc + combinations(&mut line.springs, &line.records)
        });

        r + threads
            .into_iter()
            .fold(0, |acc, t| acc + t.join().unwrap())
    })
    // let lines2 = lines.next().unwrap();
    // let t2 = thread::spawn(|| {
    //     lines2.iter_mut().fold(0, |acc, line| {
    //         acc + combinations(&mut line.springs, &line.records)
    //     })
    // });
    // return lines.iter_mut().fold(0, |acc, line| {
    //     acc + combinations(&mut line.springs, &line.records)
    // });
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
