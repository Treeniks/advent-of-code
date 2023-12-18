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

type Grid = Vec<Vec<Tile>>;

fn generate_grid(input: &str) -> Grid {
    let mut grid: Grid = vec![vec![Tile::Digged]];

    let (mut x, mut y): (usize, usize) = (0, 0);

    for line in input.lines() {
        let mut it = line.split_whitespace();
        let direction: Direction = it.next().unwrap().try_into().unwrap();
        let length: usize = it.next().unwrap().parse().unwrap();

        match direction {
            Direction::Left => {
                for i in 1..=length {
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
                x -= length;
            }
            Direction::Right => {
                for i in 1..=length {
                    if x + i < grid[y].len() {
                        grid[y][x + i] = Tile::Digged;
                    } else {
                        grid[y].push(Tile::Digged);
                    }
                }
                x += length;
            }
            Direction::Up => {
                for i in 1..=length {
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
                y -= length;
            }
            Direction::Down => {
                for i in 1..=length {
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
                y += length;
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

fn part1(input: &str) -> usize {
    let mut grid = generate_grid(input.trim());

    flood_fill_inside(&mut grid);

    grid.iter()
        .map(|line| {
            line.iter()
                .filter(|&&t| t == Tile::Inside || t == Tile::Digged)
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    0
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

        let actual = generate_grid(input.trim());
        assert_eq!(expected, actual);
    }
}
