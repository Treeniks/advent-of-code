use std::{
    fmt::Debug,
    io::{Error, Read},
    ops::{Index, IndexMut},
};

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
    grid: Vec<Vec<Tile>>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn lines(&self) -> GridIterator {
        GridIterator {
            grid: self,
            current_row: 0,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x < self.columns && y < self.rows {
            Some(&self.grid[y][x])
        } else {
            None
        }
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Tile> {
        if x < self.columns && y < self.rows {
            Some(&mut self.grid[y][x])
        } else {
            None
        }
    }

    // naming things is hard
    // (see part 2 comments for what this does)
    fn make_big(&mut self) {
        // find s
        let mut s_pos = (0, 0);
        for (j, line) in self.grid.iter().enumerate() {
            for (i, tile) in line.iter().enumerate() {
                if *tile == Tile::Start {
                    s_pos = (i, j);
                    break;
                }
            }
        }

        // remove s
        self[s_pos] = Tile::Plot;

        // extend horizontally
        for line in self.grid.iter_mut() {
            let clone = line.clone();

            line.extend(clone.clone());
            line.extend(clone.clone());
            line.extend(clone.clone());
            line.extend(clone);
        }

        // extend vertically
        let clone = self.grid.clone();
        self.grid.extend(clone.clone());
        self.grid.extend(clone.clone());
        self.grid.extend(clone.clone());
        self.grid.extend(clone);

        // add s back in
        self.grid[self.rows * 2 + s_pos.1][self.columns * 2 + s_pos.0] = Tile::Start;

        self.columns *= 5;
        self.rows *= 5;
    }
}

impl TryFrom<&str> for Grid {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let trimmed = input.trim();

        let rows = trimmed.lines().count();

        let first_line = trimmed.lines().next().ok_or("input is empty")?;
        let columns = first_line.len();

        let mut grid = Vec::with_capacity(rows);

        for line in trimmed.lines() {
            if line.len() != columns {
                return Err("not a grid".into());
            }

            let tiles = line
                .chars()
                .map(|c| Tile::try_from(c))
                .collect::<Result<Vec<Tile>, _>>()?;
            grid.push(tiles);
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
        &self.grid[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
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

fn count_os(grid: &Grid) -> usize {
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

fn part1(input: &str, steps: usize) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

    for _ in 0..steps {
        take_step(&mut grid);
    }

    count_os(&grid)
}

fn aitken_neville(v0: usize, v1: usize, v2: usize, x: usize) -> usize {
    let mut p = [v0, v1, v2];
    for i in 1..3 {
        for j in 0..3 - i {
            p[j] = p[j] + (x - j) / ((i + j) - j) * (p[j + 1] - p[j]);
        }
    }
    p[0]
}

// I honestly still don't understand this one.
// Mostly solved with the help of reddit comments.
// Supposedly by calculating the reached tiles for 65, 65 + 131 and 65 + 131 * 2 steps,
// one can use the resulting values to extrapolate.
//
// It has something to do with how the input is well formed again.
// For one, the starting point has no obstacles to all the edges,
// then the edges themselves also have no rocks,
// and lastly there is this big diamond of plot in the input that goes from edge to edge (most
// easily seen with those code-minimaps from vscode or sublime).
// Also, the starting point is right in the middle of the grid,
// the grid is 131 wide and high (making it a square), and that's where
// the 65 (= floor(131/2)) and 131 constants come from.
// Lastly, the number of steps in the puzzle question is
// 26501365, while 26501365 mod 131 = 65.
//
// Because of that, we want a function of the form f(x) = reached tiles in 65 + 131 * x steps.
// And because of the observations above, that function happens to be quadratic (no idea why).
// So, all we have to do is get the first 3 values (i.e. f(0), f(1) and f(2)), then we can uniquely
// calculate the actual quadratic function, and then just evaluate f((26501365 - 65) / 131)
// or more specifically f(202300).
//
// In this case, because we need only the value of a single argument,
// the Aitken Neville scheme fit well.
// The code of Aitken Neville above is copied from lecture slides of mine.
fn part2(input: &str) -> usize {
    let mut grid = Grid::try_from(input).unwrap();

    // To find the values of the first 3 xs, we first need to make the grid sufficiently large.
    // `make_big` just extends the grid by 5 in each direction.
    // 5 is just a random value that turned out to be enough.
    grid.make_big();

    for _ in 0..65 {
        take_step(&mut grid);
    }

    let v0 = count_os(&grid);
    // println!("0: {}", count_os(&grid));

    for _ in 0..131 {
        take_step(&mut grid);
    }

    let v1 = count_os(&grid);
    // println!("1: {}", count_os(&grid));

    for _ in 0..131 {
        take_step(&mut grid);
    }

    let v2 = count_os(&grid);
    // println!("2: {}", count_os(&grid));

    aitken_neville(v0, v1, v2, (26501365 - 65) / 131)
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

    #[test]
    fn test_part1() {
        let expected = 16;
        let actual = part1(EXAMPLE, 6);

        assert_eq!(expected, actual);
    }
}
