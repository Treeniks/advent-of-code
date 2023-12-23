use std::io::Read;

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
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::Up)),
            'v' => Ok(Self::Slope(Direction::Down)),
            '<' => Ok(Self::Slope(Direction::Left)),
            '>' => Ok(Self::Slope(Direction::Right)),
            _ => Err("unknown char".to_string()),
        }
    }
}

type Grid = utils::grid::Grid<Tile>;

fn walk_path_part1(grid: &mut Grid, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == grid.rows() - 1 {
        paths.push(path);
        return;
    }

    // check up
    match grid[y - 1][x] {
        Tile::Path | Tile::Slope(Direction::Up) => {
            let tile_before = grid[y - 1][x];
            grid[y - 1][x] = Tile::Walked;
            walk_path_part1(grid, (x, y - 1), path + 1, paths);
            grid[y - 1][x] = tile_before;
        }
        _ => {}
    }

    // check down
    match grid[y + 1][x] {
        Tile::Path | Tile::Slope(Direction::Down) => {
            let tile_before = grid[y + 1][x];
            grid[y + 1][x] = Tile::Walked;
            walk_path_part1(grid, (x, y + 1), path + 1, paths);
            grid[y + 1][x] = tile_before;
        }
        _ => {}
    }

    // check left
    match grid[y][x - 1] {
        Tile::Path | Tile::Slope(Direction::Left) => {
            let tile_before = grid[y][x - 1];
            grid[y][x - 1] = Tile::Walked;
            walk_path_part1(grid, (x - 1, y), path + 1, paths);
            grid[y][x - 1] = tile_before;
        }
        _ => {}
    }

    // check right
    match grid[y][x + 1] {
        Tile::Path | Tile::Slope(Direction::Right) => {
            let tile_before = grid[y][x + 1];
            grid[y][x + 1] = Tile::Walked;
            walk_path_part1(grid, (x + 1, y), path + 1, paths);
            grid[y][x + 1] = tile_before;
        }
        _ => {}
    }
}

fn walk_path_part2(grid: &mut Grid, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == grid.rows() - 1 {
        paths.push(path);
        return;
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
    match grid[y + 1][x] {
        Tile::Path | Tile::Slope(_) => {
            let tile_before = grid[y + 1][x];
            grid[y + 1][x] = Tile::Walked;
            walk_path_part2(grid, (x, y + 1), path + 1, paths);
            grid[y + 1][x] = tile_before;
        }
        _ => {}
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
    let mut grid = Grid::try_from(input).unwrap();

    let start_index = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &t)| t == Tile::Path)
        .unwrap()
        .0;

    grid[0][start_index] = Tile::Walked;

    let (x, y) = (start_index, 1usize);

    let mut result = Vec::new();
    walk_path_part1(&mut grid, (x, y), 1, &mut result);

    *result.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

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

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part1() {
        let expected = 94;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 154;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
