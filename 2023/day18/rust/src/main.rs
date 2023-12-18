use std::fmt::Debug;
use std::io::{Error, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Digged,
    Inside,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Digged => write!(f, "#"),
            Tile::Inside => write!(f, "$"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Digged),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    length: usize,
}

type Grid = Vec<Vec<Tile>>;

#[allow(unused)]
fn generate_grid(instructions: &[Instruction]) -> Grid {
    let mut grid: Grid = vec![vec![Tile::Digged]];

    let (mut x, mut y): (usize, usize) = (0, 0);

    for instruction in instructions {
        match instruction.direction {
            Direction::Left => {
                for i in 1..=instruction.length {
                    if let Some(nx) = x.checked_sub(i) {
                        grid[y][nx] = Tile::Digged;
                    } else {
                        for line in grid.iter_mut() {
                            line.insert(0, Tile::Empty);
                        }
                        x += 1;
                        grid[y][x - i] = Tile::Digged;
                    }
                }
                x -= instruction.length;
            }
            Direction::Right => {
                for i in 1..=instruction.length {
                    if x + i < grid[y].len() {
                        grid[y][x + i] = Tile::Digged;
                    } else {
                        grid[y].push(Tile::Digged);
                    }
                }
                x += instruction.length;
            }
            Direction::Up => {
                for i in 1..=instruction.length {
                    if let Some(ny) = y.checked_sub(i) {
                        match grid[ny].get_mut(x) {
                            Some(tile) => *tile = Tile::Digged,
                            None => {
                                let extend = vec![Tile::Empty; x - grid[ny].len() + 1];
                                grid[ny].extend(extend);
                                grid[ny][x] = Tile::Digged;
                            }
                        }
                    } else {
                        let mut new_vec = vec![Tile::Empty; x + 1];
                        new_vec[x] = Tile::Digged;
                        grid.insert(0, new_vec);
                        y += 1;
                    }
                }
                y -= instruction.length;
            }
            Direction::Down => {
                for i in 1..=instruction.length {
                    if y + i < grid.len() {
                        match grid[y + i].get_mut(x) {
                            Some(tile) => *tile = Tile::Digged,
                            None => {
                                let extend = vec![Tile::Empty; x - grid[y + i].len() + 1];
                                grid[y + i].extend(extend);
                                grid[y + i][x] = Tile::Digged;
                            }
                        }
                    } else {
                        let mut new_vec = vec![Tile::Empty; x + 1];
                        new_vec[x] = Tile::Digged;
                        grid.push(new_vec);
                    }
                }
                y += instruction.length;
            }
        }
    }

    grid
}

// this one assumes that the top left of the grid will have something like this:
// ....#####
// ....#....
// ....#....
//
// in particular, it assumes that the grid does **not** have something like this:
//
// ....##..
// ....##..
// ..#####.
// ..#...#.
#[allow(unused)]
fn flood_fill_inside(grid: &mut Grid) {
    let first_x = grid[0]
        .iter()
        .enumerate()
        .find_map(|(i, t)| match t {
            Tile::Digged => Some(i),
            _ => None,
        })
        .unwrap();

    let mut work_set = vec![(first_x + 1, 1)];

    while let Some((x, y)) = work_set.pop() {
        grid[y][x] = Tile::Inside;
        if grid[y - 1][x] == Tile::Empty {
            work_set.push((x, y - 1));
        }
        if grid[y][x - 1] == Tile::Empty {
            work_set.push((x - 1, y));
        }
        if grid[y + 1][x] == Tile::Empty {
            work_set.push((x, y + 1));
        }
        if grid[y][x + 1] == Tile::Empty {
            work_set.push((x + 1, y));
        }
    }
}

fn parse_instructions_part1(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let direction = it.next().unwrap().try_into().unwrap();
            let length = it.next().unwrap().parse().unwrap();
            Instruction { direction, length }
        })
        .collect()
}

fn parse_instructions_part2(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let _ = it.next();
            let _ = it.next();

            let tmp = it
                .next()
                .unwrap()
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(")")
                .unwrap();

            let length = usize::from_str_radix(&tmp[..5], 16).unwrap();
            let direction = match tmp.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };

            Instruction { direction, length }
        })
        .collect()
}

#[allow(unused)]
fn count_inside_digged(grid: &Grid) -> usize {
    grid.iter()
        .map(|line| {
            line.iter()
                .filter(|&&t| t == Tile::Inside || t == Tile::Digged)
                .count()
        })
        .sum()
}

fn generate_points(instructions: &[Instruction]) -> Vec<(isize, isize)> {
    let mut result = Vec::new();

    let (mut x, mut y) = (0isize, 0isize);
    result.push((x, y));
    let mut it = instructions.windows(2);
    while let Some([instruction, next_instruction]) = it.next() {
        match instruction.direction {
            Direction::Left => {
                x -= instruction.length as isize;
                match next_instruction.direction {
                    Direction::Up => result.push((x, y + 1)),
                    Direction::Down => result.push((x + 1, y + 1)),
                    _ => unreachable!(),
                }
            }
            Direction::Right => {
                x += instruction.length as isize;
                match next_instruction.direction {
                    Direction::Up => result.push((x, y)),
                    Direction::Down => result.push((x + 1, y)),
                    _ => unreachable!(),
                }
            }
            Direction::Up => {
                y -= instruction.length as isize;
                match next_instruction.direction {
                    Direction::Left => result.push((x, y + 1)),
                    Direction::Right => result.push((x, y)),
                    _ => unreachable!(),
                }
            }
            Direction::Down => {
                y += instruction.length as isize;
                match next_instruction.direction {
                    Direction::Left => result.push((x + 1, y + 1)),
                    Direction::Right => result.push((x + 1, y)),
                    _ => unreachable!(),
                }
            }
        }
    }

    return result;
}

// https://en.wikipedia.org/wiki/Shoelace_formula
// modified trapezoid formula
// simplified as we know we only ever have straight lines
fn calculate_area(points: &[(isize, isize)]) -> isize {
    let mut result = 0;

    for i in 0..points.len() - 1 {
        result += points[i].1 * (points[i].0 - points[i + 1].0)
    }
    // not needed, because the last step is always straight up or down,
    // making its area 0
    // result +=
    //     (points[points.len() - 1].1 + points[0].1) * (points[points.len() - 1].0 - points[0].0);

    result
}

fn part1(input: &str) -> isize {
    let instructions = parse_instructions_part1(input);
    let points = generate_points(&instructions);

    calculate_area(&points)
}

fn part2(input: &str) -> isize {
    let instructions = parse_instructions_part2(input);
    let points = generate_points(&instructions);

    calculate_area(&points)
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1() {
        let expected = 62;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_2() {
        let input = "R 3 (#000000)
D 3 (#000000)
L 3 (#000000)
U 3 (#000000)";

        let expected = 16;
        let actual = part1(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 952408144115;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_generate_grid() {
        let input = "
R 3 (#000000)
D 3 (#000000)
R 3 (#000000)
U 3 (#000000)
R 3 (#000000)
D 6 (#000000)
L 12 (#000000)
U 9 (#000000)
R 3 (#000000)
D 3 (#000000)
";

        let expected = "
####
#..#
#..#
#..####..####
#.....#..#..#
#.....#..#..#
#.....####..#
#...........#
#...........#
#############
";

        let expected: Vec<Vec<Tile>> = expected
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
            .collect();

        let instructions = parse_instructions_part1(input);
        let actual = generate_grid(&instructions);
        assert_eq!(expected, actual);
    }
}
