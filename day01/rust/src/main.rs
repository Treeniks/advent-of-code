use std::cmp::Ordering;

#[allow(unused)]
fn calibration_sum_part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut it = line.chars().filter_map(|c| c.to_digit(10));
        if let Some(first) = it.next() {
            let last = it.last().unwrap_or(first);

            sum += first * 10 + last;
        }
        // else continue to next line, as the line did not contain any digits (e.g. empty line)
    }
    sum
}

#[allow(unused)]
fn calibration_sum_part2(input: &str) -> u32 {
    let spelled_to_digit = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;
    for line in input.lines() {
        let mut it = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)));

        let first_char = it.next();
        let last_char = it.last().or(first_char);

        let first_spelled = spelled_to_digit
            .iter()
            .filter_map(|(s, d)| line.find(s).map(|i| (i, *d)))
            .min();
        let last_spelled = spelled_to_digit
            .iter()
            .filter_map(|(s, d)| line.rfind(s).map(|i| (i, *d)))
            .max();

        let first_digit = match (first_char, first_spelled) {
            (None, None) => continue,
            (None, Some((_, d))) => d,
            (Some((_, d)), None) => d,
            (Some((i1, d1)), Some((i2, d2))) => match i1.cmp(&i2) {
                Ordering::Less => d1,
                Ordering::Equal => panic!(),
                Ordering::Greater => d2,
            },
        };

        let last_digit = match (last_char, last_spelled) {
            (None, None) => continue,
            (None, Some((_, d))) => d,
            (Some((_, d)), None) => d,
            (Some((i1, d1)), Some((i2, d2))) => match i1.cmp(&i2) {
                Ordering::Less => d2,
                Ordering::Equal => panic!(),
                Ordering::Greater => d1,
            },
        };

        sum += first_digit * 10 + last_digit;
    }
    sum
}

fn main() -> Result<(), std::io::Error> {
    let path = "input.txt";

    let input = std::fs::read_to_string(path)?;

    println!("Part 1 Solution: {}", calibration_sum_part1(&input));
    println!("Part 2 Solution: {}", calibration_sum_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let expected = 142;
        let result = calibration_sum_part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let expected = 281;
        let result = calibration_sum_part2(input);

        assert_eq!(expected, result);
    }
}
