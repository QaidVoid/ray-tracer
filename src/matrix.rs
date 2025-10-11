use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = value;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut data = vec![0.; rows * cols];
        for row in 0..rows {
            for col in 0..cols {
                if row == col {
                    data[row * cols + col] = 1.;
                }
            }
        }

        Self {
            data,
            rows,
            cols,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![0.; self.cols * self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                data[col * self.rows + row] = self[row][col];
            }
        }
        Self {
            data,
            rows: self.cols,
            cols: self.rows,
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, I> FromIterator<I> for Matrix
where
    I: IntoIterator<Item = T>,
    T: Into<f32>,
{
    fn from_iter<U: IntoIterator<Item = I>>(iter: U) -> Self {
        let iter = iter.into_iter();

        let rows = iter.size_hint().0;
        let mut data = Vec::new();
        let mut cols = None;
        let mut actual_rows = 0;

        iter.for_each(|row| {
            let row_vec = row.into_iter().map(Into::into).collect::<Vec<_>>();

            if let Some(cols) = cols {
                assert_eq!(cols, row_vec.len(), "All rows must have same length");
            }

            cols = Some(row_vec.len());
            data.reserve(row_vec.len() * rows);
            data.extend(row_vec);
            actual_rows += 1;
        });

        Self {
            data,
            rows,
            cols: cols.unwrap_or(0),
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows, "Matrix dimensions must match");

        let rows = self.rows;
        let cols = rhs.cols;
        let shared = self.cols;

        let rhs_t = rhs.transpose().data;

        let mut result = Matrix::new(rows, cols);
        for row in 0..rows {
            let self_row = &self[row];
            for col in 0..cols {
                let rhs_col = &rhs_t[col * shared..(col + 1) * shared];
                let mut sum = 0.;

                for i in 0..shared {
                    sum += self_row[i] * rhs_col[i];
                }

                result[row][col] = sum;
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        assert_eq!(self.cols, 4, "Matrix must be 4x4");
        let (x, y, z, w) = (rhs.x, rhs.y, rhs.z, rhs.w);
        let r0 = self.data[0] * x + self.data[1] * y + self.data[2] * z + self.data[3] * w;
        let r1 = self.data[4] * x + self.data[5] * y + self.data[6] * z + self.data[7] * w;
        let r2 = self.data[8] * x + self.data[9] * y + self.data[10] * z + self.data[11] * w;
        let r3 = self.data[12] * x + self.data[13] * y + self.data[14] * z + self.data[15] * w;

        Tuple::new(r0, r1, r2, r3)
    }
}
