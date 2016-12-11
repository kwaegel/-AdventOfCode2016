
use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Eq,PartialEq)]
pub struct Matrix2D<T: Eq> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Copy + Eq> Matrix2D<T> {
    pub fn new_with_default(rows: usize, cols: usize, default: T) -> Matrix2D<T> {
        let size = rows * cols;
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(default);
        }
        Matrix2D {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    #[allow(dead_code)] // Used in test harness
    pub fn from_rowmajor_vec(rows: usize, cols: usize, data: Vec<T>) -> Matrix2D<T> {
        Matrix2D {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let tmp = self[b];
        self[b] = self[a];
        self[a] = tmp;
    }

    pub fn set(&mut self, pos: (usize, usize), num_rows: usize, num_cols: usize, value: T) {
        for r in pos.0..num_rows {
            for c in pos.1..num_cols {
                self[(r, c)] = value;
            }
        }
    }

    pub fn rotate_col_down(&mut self, col: usize, distance: usize) {
        // Adapted from http://www.cplusplus.com/reference/algorithm/rotate/
        let mut end = (self.rows - 1) as i32;
        let mut middle = (end - distance as i32) as i32;
        let mut next = middle;
        let first = -1i32;
        while end != next {
            if next == first {
                next = middle;
            } else if end == middle {
                middle = next;
            }
            self.swap((end as usize, col), (next as usize, col));
            end -= 1;
            next -= 1;
        }
    }

    pub fn rotate_row_right(&mut self, row: usize, distance: usize) {
        // Adapted from http://www.cplusplus.com/reference/algorithm/rotate/
        let mut end = (self.cols - 1) as i32;
        let mut middle = (end - distance as i32) as i32;
        let mut next = middle;
        let first = -1i32;
        while end != next {
            if next == first {
                next = middle;
            } else if end == middle {
                middle = next;
            }
            self.swap((row, end as usize), (row, next as usize));
            end -= 1;
            next -= 1;
        }
    }
}

// (row, col) indexing
impl<T: Eq> Index<(usize, usize)> for Matrix2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        let row = index.0;
        let col = index.1;
        let pos = row * self.cols + col;
        &self.data[pos]
    }
}

// (row, col) indexing
impl<T: Sized + Eq> IndexMut<(usize, usize)> for Matrix2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let row = index.0;
        let col = index.1;
        let pos = row * self.cols + col;
        &mut self.data[pos]
    }
}

// #![feature(specialization)]
// impl <T: fmt::Display> fmt::Display for Matrix2D<T> {
//    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        for r in 0usize..self.rows {
//            for c in 0usize..self.cols {
//                write!(f, "{} ", self[(r, c)])?;
//            }
//            write!(f, "\n")?;
//        }
//        write!(f, "")
//    }
// }

impl fmt::Display for Matrix2D<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0usize..self.rows() {
            for c in 0usize..self.cols() {
                let val = if self[(r, c)] { '#' } else { ' ' };
                write!(f, "{} ", val)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
