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
        Self::try_from_with(input, |c| T::try_from(c).map_err(|e| e.into()))
    }
}

impl Grid<usize> {
    pub fn try_from_usize(input: &str) -> Result<Self, Box<dyn Error>> {
        Self::try_from_with(input, |c| {
            let digit = c.to_digit(10).ok_or("not a digit")?;
            let val = usize::try_from(digit)?;
            Ok(val)
        })
    }
}

impl<T> Grid<T> {
    pub fn try_from_with<F>(input: &str, try_from: F) -> Result<Self, Box<dyn Error>>
    where
        F: Fn(char) -> Result<T, Box<dyn Error>>,
    {
        let trimmed = input.trim();

        let rows = trimmed.lines().count();

        let first_line = trimmed.lines().next().ok_or("empty input")?;
        let columns = first_line.len();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in trimmed.lines() {
            if line.len() != columns {
                return Err("not a grid".into());
            }

            let line = line
                .chars()
                .map(|c| try_from(c))
                .collect::<Result<Vec<_>, _>>()?;

            grid.extend(line);
        }

        Ok(Grid {
            grid,
            rows,
            columns,
        })
    }

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
        self.grid
            .get(row * self.columns..row * self.columns + self.columns)
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        self.grid
            .get_mut(row * self.columns..row * self.columns + self.columns)
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.grid.get(y * self.columns + x)
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        self.grid.get_mut(y * self.columns + x)
    }

    pub fn get_row_isize(&self, row: isize) -> Option<&[T]> {
        match usize::try_from(row) {
            Ok(row) => self
                .grid
                .get(row * self.columns..row * self.columns + self.columns),
            _ => None,
        }
    }

    pub fn get_row_mut_isize(&mut self, row: isize) -> Option<&mut [T]> {
        match usize::try_from(row) {
            Ok(row) => self
                .grid
                .get_mut(row * self.columns..row * self.columns + self.columns),
            _ => None,
        }
    }

    pub fn get_isize(&self, (x, y): (isize, isize)) -> Option<&T> {
        match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => self.grid.get(y * self.columns + x),
            _ => None,
        }
    }

    pub fn get_mut_isize(&mut self, (x, y): (isize, isize)) -> Option<&mut T> {
        match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => self.grid.get_mut(y * self.columns + x),
            _ => None,
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

impl<T> Index<isize> for Grid<T> {
    type Output = [T];

    fn index(&self, row: isize) -> &Self::Output {
        self.get_row_isize(row).unwrap()
    }
}

impl<T> IndexMut<isize> for Grid<T> {
    fn index_mut(&mut self, row: isize) -> &mut Self::Output {
        self.get_row_mut_isize(row).unwrap()
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        self.get_isize(index).unwrap()
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        self.get_mut_isize(index).unwrap()
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
