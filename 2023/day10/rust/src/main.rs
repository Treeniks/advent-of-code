use std::io::Read;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe(Direction, Direction),
    Inner,
    Outer,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::Pipe(Direction::Up, Direction::Down)),
            '-' => Ok(Self::Pipe(Direction::Left, Direction::Right)),
            '7' => Ok(Self::Pipe(Direction::Left, Direction::Down)),
            'J' => Ok(Self::Pipe(Direction::Left, Direction::Up)),
            'F' => Ok(Self::Pipe(Direction::Right, Direction::Down)),
            'L' => Ok(Self::Pipe(Direction::Right, Direction::Up)),
            _ => Err("unknown char".to_string()),
        }
    }
}

type Grid = grid::Grid<Tile>;

fn find_and_replace_s(grid: &mut Grid) -> (usize, usize) {
    // find S
    let find_s = || {
        for y in 0..grid.rows() {
            for x in 0..grid.columns() {
                if grid[(x, y)] == Tile::Start {
                    return Some((x, y));
                }
            }
        }
        None
    };

    let (x, y) = find_s().expect("found no S");

    // find neighbours
    let (mut d1, mut d2) = (None, None);

    let mut set_neighbour = |i| match d1 {
        Some(_) => d2 = Some(i),
        None => d1 = Some(i),
    };

    // left
    if let Some(x) = x.checked_sub(1) {
        match grid.get((x, y)) {
            Some(Tile::Pipe(Direction::Right, _)) | Some(Tile::Pipe(_, Direction::Right)) => {
                set_neighbour(Direction::Left);
            }
            _ => {}
        }
    }

    // right
    if let Some(x) = x.checked_add(1) {
        match grid.get((x, y)) {
            Some(Tile::Pipe(Direction::Left, _)) | Some(Tile::Pipe(_, Direction::Left)) => {
                set_neighbour(Direction::Right);
            }
            _ => {}
        }
    }

    // up
    if let Some(y) = y.checked_sub(1) {
        match grid.get((x, y)) {
            Some(Tile::Pipe(Direction::Down, _)) | Some(Tile::Pipe(_, Direction::Down)) => {
                set_neighbour(Direction::Up);
            }
            _ => {}
        }
    }

    // down
    if let Some(y) = y.checked_add(1) {
        match grid.get((x, y)) {
            Some(Tile::Pipe(Direction::Up, _)) | Some(Tile::Pipe(_, Direction::Up)) => {
                set_neighbour(Direction::Down);
            }
            _ => {}
        }
    }

    grid[(x, y)] = Tile::Pipe(d1.unwrap(), d2.unwrap());
    (x, y)
}

fn find_loop(grid: &Grid, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![start];

    let neighbour = |dir, (x, y)| match dir {
        // -1 cannot underflow here
        // because we are necessarily within the grid
        // assuming the loop is well formed
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
    };

    let (mut px, mut py) = start;
    let (mut x, mut y) = match grid[start] {
        Tile::Pipe(d1, _) => neighbour(d1, start),
        _ => panic!(),
    };

    while (x, y) != start {
        result.push((x, y));
        let (n1, n2) = match grid[(x, y)] {
            Tile::Pipe(d1, d2) => {
                let n1 = neighbour(d1, (x, y));
                let n2 = neighbour(d2, (x, y));

                (n1, n2)
            }
            _ => panic!(),
        };

        if n1 == (px, py) {
            (px, py) = (x, y);
            (x, y) = n2;
        } else {
            (px, py) = (x, y);
            (x, y) = n1;
        }
    }

    result
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

    let start = find_and_replace_s(&mut grid);

    let l = find_loop(&grid, start);

    l.len() / 2
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

    let start = find_and_replace_s(&mut grid);

    let l = find_loop(&grid, start);

    // convert all non-loop pipes into ground tiles
    for y in 0..grid.rows() {
        for x in 0..grid.columns() {
            if !l.contains(&(x, y)) {
                grid[(x, y)] = Tile::Ground;
            }
        }
    }

    // convert all inner tiles to Tile::Inner
    for y in 0..grid.rows() {
        let mut inside = false;
        let mut prev_dir = Direction::Up;
        for x in 0..grid.columns() {
            match grid[(x, y)] {
                Tile::Ground => {
                    if inside {
                        grid[(x, y)] = Tile::Inner;
                    } else {
                        grid[(x, y)] = Tile::Outer;
                    }
                }
                Tile::Pipe(Direction::Up, Direction::Down)
                | Tile::Pipe(Direction::Down, Direction::Up) => {
                    inside = !inside;
                }
                Tile::Pipe(Direction::Up, Direction::Right)
                | Tile::Pipe(Direction::Right, Direction::Up) => {
                    prev_dir = Direction::Up;
                }
                Tile::Pipe(Direction::Down, Direction::Right)
                | Tile::Pipe(Direction::Right, Direction::Down) => {
                    prev_dir = Direction::Down;
                }
                Tile::Pipe(Direction::Up, Direction::Left)
                | Tile::Pipe(Direction::Left, Direction::Up) => match prev_dir {
                    Direction::Down => inside = !inside,
                    Direction::Up => {}
                    _ => unreachable!(),
                },
                Tile::Pipe(Direction::Down, Direction::Left)
                | Tile::Pipe(Direction::Left, Direction::Down) => match prev_dir {
                    Direction::Up => inside = !inside,
                    Direction::Down => {}
                    _ => unreachable!(),
                },
                Tile::Pipe(Direction::Left, Direction::Right)
                | Tile::Pipe(Direction::Right, Direction::Left) => {}
                _ => panic!(),
            }
        }
    }

    // count how many Tile::Inner are in the grid
    grid.grid()
        .iter()
        .filter(|&&tile| tile == Tile::Inner)
        .count()
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

    const EXAMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part1() {
        let expected = 4;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_2() {
        let expected = 8;
        let actual = part1(EXAMPLE2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 1;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_2() {
        let expected = 1;
        let actual = part2(EXAMPLE2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_3() {
        let expected = 4;
        let actual = part2(EXAMPLE3);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_4() {
        let expected = 8;
        let actual = part2(EXAMPLE4);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_5() {
        let expected = 10;
        let actual = part2(EXAMPLE5);

        assert_eq!(expected, actual);
    }
}
