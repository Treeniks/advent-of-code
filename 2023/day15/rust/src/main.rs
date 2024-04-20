use std::io::Read;

struct BoxEntry<'a> {
    label: &'a str,
    focal_length: usize,
}

fn hash(s: &str) -> u8 {
    let mut result: u8 = 0;
    for c in s.chars() {
        result = result.wrapping_add(u8::try_from(c).unwrap());
        result = result.wrapping_mul(17);
    }
    result
}

fn part1(input: &str) -> usize {
    let sequence = input.trim().split(',');

    sequence.map(hash).map(usize::from).sum()
}

fn part2(input: &str) -> usize {
    let sequence = input.trim().split(',');

    // TODO replace with `[const { vec![] }; 256]` once inline const is stable
    const V: Vec<BoxEntry> = vec![];
    let mut map = [V; 256];

    for s in sequence {
        if let Some(i) = s.find('-') {
            let label = &s[..i];
            let index = usize::from(hash(label));

            if let Some(j) = map[index].iter().position(|e| e.label == label) {
                map[index].remove(j);
            }
        } else if let Some(i) = s.find('=') {
            let label = &s[..i];
            let val = s[i + 1..].parse::<usize>().unwrap();
            let index = usize::from(hash(label));

            if let Some(j) = map[index].iter().position(|e| e.label == label) {
                map[index][j].focal_length = val;
            } else {
                map[index].push(BoxEntry {
                    label,
                    focal_length: val,
                })
            }
        }
    }

    map.iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, e)| (i + 1) * (j + 1) * e.focal_length)
                .sum::<usize>()
        })
        .sum()
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

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        let expected = 52;
        let actual = hash("HASH");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let expected = 1320;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 145;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
