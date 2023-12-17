use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::io::Read;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
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

    fn checked_add_vertical(&self, lhs: usize, rhs: usize) -> Option<usize> {
        if let Some(r) = lhs.checked_add(rhs) {
            if r < self.rows {
                return Some(r);
            }
        }
        None
    }

    fn checked_add_horizontal(&self, lhs: usize, rhs: usize) -> Option<usize> {
        if let Some(r) = lhs.checked_add(rhs) {
            if r < self.columns {
                return Some(r);
            }
        }
        None
    }

    fn lines(&self) -> GridIterator {
        GridIterator {
            grid: self,
            current_row: 0,
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

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = false;
        for line in self.lines() {
            if b {
                write!(f, "\n")?;
            }
            write!(f, "{:?}", line)?;
            b = true;
        }

        Ok(())
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
    heuristic: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We flip the ordering on costs to always get the State with the lowest cost.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        // See also https://doc.rust-lang.org/std/collections/binary_heap/index.html.
        (other.cost + other.heuristic)
            .cmp(&(self.cost + self.heuristic))
            .then_with(|| {
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

fn part1(input: &str) -> usize {
    let grid = Grid::from_input(input);

    // A*
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

    for i in 1..=3 {
        let mut cost = 0;
        for j in 1..=i {
            cost += grid[0][j]
        }
        heap.push(State {
            x: i,
            y: 0,
            cost,
            heuristic: grid.columns - i + grid.rows,
            dir: Direction::HORIZONTAL,
        });

        let mut cost = 0;
        for j in 1..=i {
            cost += grid[j][0]
        }
        heap.push(State {
            x: 0,
            y: i,
            cost,
            heuristic: grid.rows - i + grid.columns,
            dir: Direction::VERTICAL,
        });
    }

    while let Some(State {
        x,
        y,
        cost,
        heuristic: _,
        dir,
    }) = heap.pop()
    {
        if (x, y) == (grid.columns - 1, grid.rows - 1) {
            return cost;
        }

        match dir {
            Direction::HORIZONTAL => {
                for i in 1..=3 {
                    if let Some(new_y) = y.checked_sub(i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y - j][x]
                        }

                        if new_cost < dist_vertical[new_y][x] {
                            dist_vertical[new_y][x] = new_cost;

                            heap.push(State {
                                x,
                                y: new_y,
                                cost: new_cost,
                                heuristic: grid.rows - new_y + grid.columns,
                                dir: Direction::VERTICAL,
                            });
                        }
                    }

                    if let Some(new_y) = grid.checked_add_vertical(y, i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y + j][x]
                        }

                        if new_cost < dist_vertical[new_y][x] {
                            dist_vertical[new_y][x] = new_cost;

                            heap.push(State {
                                x,
                                y: new_y,
                                cost: new_cost,
                                heuristic: grid.rows - new_y + grid.columns,
                                dir: Direction::VERTICAL,
                            });
                        }
                    }
                }
            }
            Direction::VERTICAL => {
                for i in 1..=3 {
                    if let Some(new_x) = x.checked_sub(i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y][x - j]
                        }

                        if new_cost < dist_horizontal[y][new_x] {
                            dist_horizontal[y][new_x] = new_cost;

                            heap.push(State {
                                x: new_x,
                                y,
                                cost: new_cost,
                                heuristic: grid.columns - new_x + grid.rows,
                                dir: Direction::HORIZONTAL,
                            });
                        }
                    }

                    if let Some(new_x) = grid.checked_add_horizontal(x, i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y][x + j]
                        }

                        if new_cost < dist_horizontal[y][new_x] {
                            dist_horizontal[y][new_x] = new_cost;

                            heap.push(State {
                                x: new_x,
                                y,
                                cost: new_cost,
                                heuristic: grid.columns - new_x + grid.rows,
                                dir: Direction::HORIZONTAL,
                            });
                        }
                    }
                }
            }
        }
    }

    unreachable!();
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_input(input);

    // A*
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

    for i in 4..=10 {
        let mut cost = 0;
        for j in 1..=i {
            cost += grid[0][j]
        }
        heap.push(State {
            x: i,
            y: 0,
            cost,
            heuristic: grid.columns - i + grid.rows,
            dir: Direction::HORIZONTAL,
        });

        let mut cost = 0;
        for j in 1..=i {
            cost += grid[j][0]
        }
        heap.push(State {
            x: 0,
            y: i,
            cost,
            heuristic: grid.rows - i + grid.columns,
            dir: Direction::VERTICAL,
        });
    }

    while let Some(State {
        x,
        y,
        cost,
        heuristic: _,
        dir,
    }) = heap.pop()
    {
        if (x, y) == (grid.columns - 1, grid.rows - 1) {
            return cost;
        }

        match dir {
            Direction::HORIZONTAL => {
                for i in 4..=10 {
                    if let Some(new_y) = y.checked_sub(i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y - j][x]
                        }

                        if new_cost < dist_vertical[new_y][x] {
                            dist_vertical[new_y][x] = new_cost;

                            heap.push(State {
                                x,
                                y: new_y,
                                cost: new_cost,
                                heuristic: grid.rows - new_y + grid.columns,
                                dir: Direction::VERTICAL,
                            });
                        }
                    }

                    if let Some(new_y) = grid.checked_add_vertical(y, i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y + j][x]
                        }

                        if new_cost < dist_vertical[new_y][x] {
                            dist_vertical[new_y][x] = new_cost;

                            heap.push(State {
                                x,
                                y: new_y,
                                cost: new_cost,
                                heuristic: grid.rows - new_y + grid.columns,
                                dir: Direction::VERTICAL,
                            });
                        }
                    }
                }
            }
            Direction::VERTICAL => {
                for i in 4..=10 {
                    if let Some(new_x) = x.checked_sub(i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y][x - j]
                        }

                        if new_cost < dist_horizontal[y][new_x] {
                            dist_horizontal[y][new_x] = new_cost;

                            heap.push(State {
                                x: new_x,
                                y,
                                cost: new_cost,
                                heuristic: grid.columns - new_x + grid.rows,
                                dir: Direction::HORIZONTAL,
                            });
                        }
                    }

                    if let Some(new_x) = grid.checked_add_horizontal(x, i) {
                        let mut new_cost = cost;
                        for j in 1..=i {
                            new_cost += grid[y][x + j]
                        }

                        if new_cost < dist_horizontal[y][new_x] {
                            dist_horizontal[y][new_x] = new_cost;

                            heap.push(State {
                                x: new_x,
                                y,
                                cost: new_cost,
                                heuristic: grid.columns - new_x + grid.rows,
                                dir: Direction::HORIZONTAL,
                            });
                        }
                    }
                }
            }
        }
    }

    unreachable!();
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
