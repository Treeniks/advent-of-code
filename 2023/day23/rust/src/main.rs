use std::io::Read;
use std::ops::{Index, IndexMut};

const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
    Walked,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::Up)),
            'v' => Ok(Self::Slope(Direction::Down)),
            '<' => Ok(Self::Slope(Direction::Left)),
            '>' => Ok(Self::Slope(Direction::Right)),
            _ => Err(()),
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
    fn from_input(input: &str) -> Self {
        let trimmed = input.trim();

        let rows = trimmed.lines().count();

        let first_line = trimmed.lines().next().unwrap();
        let columns = first_line.len();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in trimmed.lines() {
            if line.len() != columns {
                panic!("not a grid")
            }

            grid.extend(line.chars().map(|c| Tile::try_from(c).unwrap()));
        }

        Grid {
            grid,
            rows,
            columns,
        }
    }

    #[allow(unused)]
    fn lines(&self) -> GridIterator {
        GridIterator {
            grid: self,
            current_row: 0,
        }
    }

    #[allow(unused)]
    fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
        if x < self.columns && y < self.rows {
            Some(self.grid[y * self.columns + x])
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

fn walk_path_part1(grid: Grid, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == grid.rows - 1 {
        paths.push(path);
    }

    // check up
    match grid[y - 1][x] {
        Tile::Path | Tile::Slope(Direction::Up) => {
            let mut clone = grid.clone();
            clone[y - 1][x] = Tile::Walked;
            walk_path_part1(clone, (x, y - 1), path + 1, paths);
        }
        _ => {}
    }

    // check down
    // I have no idea why I need this guard
    if y + 1 < grid.rows {
        match grid[y + 1][x] {
            Tile::Path | Tile::Slope(Direction::Down) => {
                let mut clone = grid.clone();
                clone[y + 1][x] = Tile::Walked;
                walk_path_part1(clone, (x, y + 1), path + 1, paths);
            }
            _ => {}
        }
    }

    // check left
    match grid[y][x - 1] {
        Tile::Path | Tile::Slope(Direction::Left) => {
            let mut clone = grid.clone();
            clone[y][x - 1] = Tile::Walked;
            walk_path_part1(clone, (x - 1, y), path + 1, paths);
        }
        _ => {}
    }

    // check right
    match grid[y][x + 1] {
        Tile::Path | Tile::Slope(Direction::Right) => {
            let mut clone = grid.clone();
            clone[y][x + 1] = Tile::Walked;
            walk_path_part1(clone, (x + 1, y), path + 1, paths);
        }
        _ => {}
    }
}

fn walk_path_part2(grid: &mut Grid, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == grid.rows - 1 {
        paths.push(path);
    }

    // check up
    match grid[y - 1][x] {
        Tile::Path | Tile::Slope(_) => {
            let tile_before = grid[y - 1][x];
            grid[y - 1][x] = Tile::Walked;
            walk_path_part2(grid, (x, y - 1), path + 1, paths);
            grid[y - 1][x] = tile_before;
        }
        _ => {}
    }

    // check down
    // I have no idea why I need this guard
    if y + 1 < grid.rows {
        match grid[y + 1][x] {
            Tile::Path | Tile::Slope(_) => {
                let tile_before = grid[y + 1][x];
                grid[y + 1][x] = Tile::Walked;
                walk_path_part2(grid, (x, y + 1), path + 1, paths);
                grid[y + 1][x] = tile_before;
            }
            _ => {}
        }
    }

    // check left
    match grid[y][x - 1] {
        Tile::Path | Tile::Slope(_) => {
            let tile_before = grid[y][x - 1];
            grid[y][x - 1] = Tile::Walked;
            walk_path_part2(grid, (x - 1, y), path + 1, paths);
            grid[y][x - 1] = tile_before;
        }
        _ => {}
    }

    // check right
    match grid[y][x + 1] {
        Tile::Path | Tile::Slope(_) => {
            let tile_before = grid[y][x + 1];
            grid[y][x + 1] = Tile::Walked;
            walk_path_part2(grid, (x + 1, y), path + 1, paths);
            grid[y][x + 1] = tile_before;
        }
        _ => {}
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_input(input);

    let start_index = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &t)| t == Tile::Path)
        .unwrap()
        .0;

    grid[0][start_index] = Tile::Walked;

    let (x, y) = (start_index, 1usize);

    let mut result = Vec::new();
    walk_path_part1(grid, (x, y), 1, &mut result);

    *result.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_input(input);

    let start_index = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &t)| t == Tile::Path)
        .unwrap()
        .0;

    grid[0][start_index] = Tile::Walked;

    let (x, y) = (start_index, 1usize);

    let mut result = Vec::new();
    walk_path_part2(&mut grid, (x, y), 1, &mut result);

    *result.iter().max().unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;
    // let input = EXAMPLE;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
