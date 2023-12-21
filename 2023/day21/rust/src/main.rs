use std::{
    fmt::Debug,
    io::{Error, Read},
    ops::{Index, IndexMut},
};

const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Plot,
    Rock,
    O,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "S"),
            Tile::Plot => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::O => write!(f, "O"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Plot),
            '#' => Ok(Self::Rock),
            'O' => Ok(Self::O),
            _ => Err(format!("unknown tile: {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Tile>,
    rows: usize,
    columns: usize,
}

impl Grid {
    #[allow(unused)]
    fn lines(&self) -> GridIterator {
        GridIterator {
            grid: self,
            current_row: 0,
        }
    }

    #[allow(unused)]
    fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x < self.columns && y < self.rows {
            Some(&self.grid[y * self.columns + x])
        } else {
            None
        }
    }

    #[allow(unused)]
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Tile> {
        if x < self.columns && y < self.rows {
            Some(&mut self.grid[y * self.columns + x])
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Grid {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let trimmed = input.trim();

        let rows = trimmed.lines().count();

        let first_line = trimmed.lines().next().ok_or("input is empty")?;
        let columns = first_line.len();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in trimmed.lines() {
            if line.len() != columns {
                return Err("not a grid".into());
            }

            let tiles = line
                .chars()
                .map(|c| Tile::try_from(c))
                .collect::<Result<Vec<Tile>, _>>()?;
            grid.extend(tiles);
        }

        Ok(Grid {
            grid,
            rows,
            columns,
        })
    }
}

struct GridIterator<'a> {
    grid: &'a Grid,
    current_row: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.grid.rows {
            let r = Some(&self.grid[self.current_row]);
            self.current_row += 1;
            return r;
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Index<usize> for Grid {
    type Output = [Tile];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index * self.columns..index * self.columns + self.columns]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index * self.columns..index * self.columns + self.columns]
    }
}

fn take_step(grid: &mut Grid) {
    let mut starts = Vec::new();
    grid.lines().enumerate().for_each(|(row, tiles)| {
        starts.extend(
            tiles
                .iter()
                .enumerate()
                .filter_map(|(column, tile)| match tile {
                    Tile::Start | Tile::O => Some((column, row)),
                    Tile::Plot | Tile::Rock => None,
                }),
        );
    });

    // first mark all starts as plots
    for &start in &starts {
        grid[start] = Tile::Plot;
    }

    // then mark all the possible steps
    for &start in &starts {
        // west
        if start.0 > 0 {
            if let Some(tile) = grid.get_mut((start.0 - 1, start.1)) {
                match tile {
                    Tile::Plot => *tile = Tile::O,
                    Tile::Start => unreachable!(),
                    _ => {}
                }
            }
        }

        // east
        if let Some(tile) = grid.get_mut((start.0 + 1, start.1)) {
            match tile {
                Tile::Plot => *tile = Tile::O,
                Tile::Start => unreachable!(),
                _ => {}
            }
        }

        // north
        if start.1 > 0 {
            if let Some(tile) = grid.get_mut((start.0, start.1 - 1)) {
                match tile {
                    Tile::Plot => *tile = Tile::O,
                    Tile::Start => unreachable!(),
                    _ => {}
                }
            }
        }

        // south
        if let Some(tile) = grid.get_mut((start.0, start.1 + 1)) {
            match tile {
                Tile::Plot => *tile = Tile::O,
                Tile::Start => unreachable!(),
                _ => {}
            }
        }
    }
}

fn part1(input: &str, steps: usize) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

    for _ in 0..steps {
        take_step(&mut grid);
    }

    grid.lines()
        .map(|tiles| {
            tiles
                .iter()
                .filter_map(|tile| match tile {
                    Tile::Start | Tile::O => Some(1),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input, 64));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 16;
        let actual = part1(EXAMPLE, 6);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
