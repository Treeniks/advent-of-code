use itertools::Itertools;
use std::{fmt::Display, io::Read, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Vec3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(", ");
        let x = it.next().ok_or("no x".to_string())?.trim();
        let y = it.next().ok_or("no y".to_string())?.trim();
        let z = it.next().ok_or("no z".to_string())?.trim();

        let x = x
            .parse::<f64>()
            .map_err(|_| format!("could not parse x: {}", x))?;
        let y = y
            .parse::<f64>()
            .map_err(|_| "could not parse y".to_string())?;
        let z = z
            .parse::<f64>()
            .map_err(|_| "could not parse y".to_string())?;

        Ok(Self { x, y, z })
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Path {
    start: Vec3,
    velocity: Vec3,
}

impl FromStr for Path {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut it = line.trim().split('@');
        let start = it.next().ok_or("no start".to_string())?;
        let velocity = it.next().ok_or("no velocity".to_string())?;

        let start = start.parse::<Vec3>()?;
        let velocity = velocity.parse::<Vec3>()?;

        Ok(Self { start, velocity })
    }
}

impl Path {
    fn intersect_xy(&self, other: &Self) -> (f64, f64, (f64, f64)) {
        let Path {
            start:
                Vec3 {
                    x: p1x,
                    y: p1y,
                    z: _,
                },
            velocity:
                Vec3 {
                    x: v1x,
                    y: v1y,
                    z: _,
                },
        } = *self;

        let Path {
            start:
                Vec3 {
                    x: p2x,
                    y: p2y,
                    z: _,
                },
            velocity:
                Vec3 {
                    x: v2x,
                    y: v2y,
                    z: _,
                },
        } = *other;

        // solving the equation system
        let t2 = ((p2y - p1y) * v1x - (p2x - p1x) * v1y) / (v2x * v1y - v2y * v1x);
        let t1 = (p2x - p1x + t2 * v2x) / v1x;

        let x = p1x + t1 * v1x;
        let y = p1y + t1 * v1y;

        (t1, t2, (x, y))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.start, self.velocity)
    }
}

fn parse_input(input: &str) -> Vec<Path> {
    let line_count = input.trim().lines().count();
    let mut result = Vec::with_capacity(line_count);

    for line in input.trim().lines() {
        result.push(line.parse::<Path>().unwrap())
    }

    result
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let paths = parse_input(input);

    let mut result = 0;

    for (a, b) in paths.iter().tuple_combinations() {
        let (t1, t2, (x, y)) = a.intersect_xy(b);
        if t1.is_sign_positive()
            && t2.is_sign_positive()
            && x >= min
            && y >= min
            && x <= max
            && y <= max
        {
            result += 1;
            // println!(
            //     "A: {}\nB: {}\nintersect inside test area at x: {}, y: {} at time t1: {}, t2: {}",
            //     a, b, x, y, t1, t2
            // );
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let _paths = parse_input(input);

    0
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!(
        "Part 1: {}",
        part1(&input, 200000000000000., 400000000000000.)
    );
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_part1() {
        let expected = 2;
        let actual = part1(EXAMPLE, 7., 27.);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
