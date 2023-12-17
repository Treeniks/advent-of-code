use std::collections::BinaryHeap;
use std::io::Read;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<usize>,
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

            grid.extend(
                line.chars()
                    .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap()),
            );
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
    fn get(&self, (x, y): (usize, usize)) -> Option<usize> {
        if x < self.columns && y < self.rows {
            Some(self.grid[y * self.columns + x])
        } else {
            None
        }
    }

    #[allow(unused)]
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut usize> {
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
    type Item = &'a [usize];

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
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index * self.columns..index * self.columns + self.columns]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index * self.columns..index * self.columns + self.columns]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    cost: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We flip the ordering on costs to always get the State with the lowest cost.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        // See also https://doc.rust-lang.org/std/collections/binary_heap/index.html.
        other.cost.cmp(&self.cost).then_with(|| {
            self.x
                .cmp(&other.x)
                .then_with(|| self.y.cmp(&other.y))
                .then_with(|| self.dir.cmp(&other.dir))
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn neighbours(
    x: usize,
    y: usize,
    dir: Direction,
    min: usize,
    max: usize,
    grid: &Grid,
) -> Vec<(usize, usize, Direction, usize)> {
    let mut result = vec![];

    match dir {
        Direction::VERTICAL => {
            for i in min..=max {
                if x >= i {
                    let mut cost = 0;
                    for j in 1..=i {
                        cost += grid[y][x - j];
                    }
                    result.push((x - i, y, Direction::HORIZONTAL, cost));
                }
                if x + i < grid.columns {
                    let mut cost = 0;
                    for j in 1..=i {
                        cost += grid[y][x + j];
                    }
                    result.push((x + i, y, Direction::HORIZONTAL, cost));
                }
            }
        }
        Direction::HORIZONTAL => {
            for i in min..=max {
                if y >= i {
                    let mut cost = 0;
                    for j in 1..=i {
                        cost += grid[y - j][x];
                    }
                    result.push((x, y - i, Direction::VERTICAL, cost));
                }
                if y + i < grid.rows {
                    let mut cost = 0;
                    for j in 1..=i {
                        cost += grid[y + j][x];
                    }
                    result.push((x, y + i, Direction::VERTICAL, cost));
                }
            }
        }
    }

    result
}

fn djikstra(grid: &Grid, min: usize, max: usize) -> usize {
    // basically copied from https://doc.rust-lang.org/std/collections/binary_heap/index.html
    // with some modifications to fit the context
    let mut heap = BinaryHeap::new();
    let mut dist_horizontal = Grid {
        grid: vec![usize::MAX; grid.grid.len()],
        rows: grid.rows,
        columns: grid.columns,
    };

    let mut dist_vertical = Grid {
        grid: vec![usize::MAX; grid.grid.len()],
        rows: grid.rows,
        columns: grid.columns,
    };

    let neighbours_local =
        |x: usize, y: usize, dir: Direction| -> Vec<(usize, usize, Direction, usize)> {
            neighbours(x, y, dir, min, max, &grid)
        };

    for (x, y, dir, cost) in neighbours_local(0, 0, Direction::VERTICAL) {
        dist_horizontal[y][x] = cost;
        heap.push(State { x, y, cost, dir })
    }

    for (x, y, dir, cost) in neighbours_local(0, 0, Direction::HORIZONTAL) {
        dist_vertical[y][x] = cost;
        heap.push(State { x, y, cost, dir })
    }

    while let Some(State { x, y, cost, dir }) = heap.pop() {
        if (x, y) == (grid.columns - 1, grid.rows - 1) {
            return cost;
        }

        for (xn, yn, dirn, costn) in neighbours_local(x, y, dir) {
            let costn = cost + costn;
            let dist = match dir {
                Direction::VERTICAL => &mut dist_horizontal,
                Direction::HORIZONTAL => &mut dist_vertical,
            };
            if costn < dist[yn][xn] {
                dist[yn][xn] = costn;
                heap.push(State {
                    x: xn,
                    y: yn,
                    cost: costn,
                    dir: dirn,
                })
            }
        }
    }

    unreachable!();
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_input(input);
    djikstra(&grid, 1, 3)
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_input(input);
    djikstra(&grid, 4, 10)
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

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const EXAMPLE2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_simple() {
        let input = "2413
3215
3255
3446
";

        let expected = 21;
        let actual = part1(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let expected = 102;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 94;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_2() {
        let expected = 71;
        let actual = part2(EXAMPLE2);

        assert_eq!(expected, actual);
    }
}
