use std::error::Error;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

// I'm not actually sure if I could add the `Eq` trait here...
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    grid: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: TryFrom<char>> TryFrom<&str> for Grid<T>
where
    <T as TryFrom<char>>::Error: Into<Box<dyn Error>>,
{
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let trimmed = input.trim();

        let rows = trimmed.lines().count();

        let first_line = trimmed
            .lines()
            .next()
            .ok_or::<Box<dyn Error>>("empty input".into())?;
        let columns = first_line.len();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in trimmed.lines() {
            if line.len() != columns {
                return Err("not a grid".into());
            }

            let line: Result<Vec<_>, _> = line.chars().map(|c| T::try_from(c)).collect();

            grid.extend(line.map_err(|e| e.into())?);
        }

        Ok(Grid {
            grid,
            rows,
            columns,
        })
    }
}

impl Grid<usize> {
    pub fn try_from_usize(input: &str) -> Result<Self, Box<dyn Error>> {
        struct UsizeTile(usize);

        impl TryFrom<char> for UsizeTile {
            type Error = Box<dyn Error>;

            fn try_from(value: char) -> Result<Self, Self::Error> {
                let digit = value
                    .to_digit(10)
                    .ok_or::<Box<dyn Error>>("not a digit".into())?;
                let val = usize::try_from(digit)?;
                Ok(UsizeTile(val))
            }
        }

        Grid::<UsizeTile>::try_from(input).map(|grid| Self {
            grid: grid.grid.iter().map(|t| t.0).collect(),
            columns: grid.columns,
            rows: grid.rows,
        })
    }
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<T>, rows: usize, columns: usize) -> Self {
        assert!(rows * columns == grid.len());

        Self {
            grid,
            rows,
            columns,
        }
    }

    pub fn lines(&self) -> LinesIterator<T> {
        LinesIterator {
            grid: self,
            current_row: 0,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn grid(&self) -> &Vec<T> {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Vec<T> {
        &mut self.grid
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row < self.rows {
            Some(&self.grid[row * self.columns..row * self.columns + self.columns])
        } else {
            None
        }
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row < self.rows {
            Some(&mut self.grid[row * self.columns..row * self.columns + self.columns])
        } else {
            None
        }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if x < self.columns && y < self.rows {
            Some(&self.grid[y * self.columns + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        if x < self.columns && y < self.rows {
            Some(&mut self.grid[y * self.columns + x])
        } else {
            None
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        self.get_row(row).unwrap()
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        self.get_row_mut(row).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct LinesIterator<'a, T> {
    grid: &'a Grid<T>,
    current_row: usize,
}

impl<'a, T> Iterator for LinesIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.grid.rows {
            let r = &self.grid[self.current_row];
            self.current_row += 1;
            return Some(r);
        } else {
            None
        }
    }
}
