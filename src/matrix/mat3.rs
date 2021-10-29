use core::panic;
use std::{
    convert::TryInto,
    ops::{Index, IndexMut, Mul},
};

use super::Mat2;

const SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    buffer: [f64; SIZE * SIZE],
}

impl Mat3 {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; SIZE * SIZE],
        }
    }

    pub fn from_buffer(buffer: [f64; SIZE * SIZE]) -> Self {
        Self { buffer: buffer }
    }

    pub fn size(&self) -> usize {
        SIZE * SIZE
    }

    pub fn identiy() -> Self {
        Self {
            buffer: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn transpose(&self) -> Self {
        // transpose a matrix turn its rows into columns
        let mut out = Mat3::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(col, row)]
            }
        }
        out
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Mat2 {
        let mut tmp = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                if i != row && j != col {
                    tmp.push(self[(i, j)]);
                }
            }
        }
        Mat2::from_buffer(tmp[0..4].try_into().unwrap())
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        // the submatrix return a mat2
        // so the determinat called is the one from mat2
        // with a mat2 we know how to calculate the
        // determinant
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 != 0 {
            return -self.minor(row, col);
        }
        self.minor(row, col)
    }

    pub fn det(&self) -> f64 {
        let mut det = 0.0f64;
        for col in 0..SIZE {
            det = det + self[(0, col)] * self.cofactor(0, col);
        }
        det
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.0
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for i in 0..self.buffer.len() {
            if self.buffer[i] != other.buffer[i] {
                return false;
            }
        }
        true
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        }
        &self.buffer[row * SIZE + col]
    }
}

impl IndexMut<(usize, usize)> for Mat3 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        }
        &mut self.buffer[row * SIZE + col]
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut out = Mat3::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)];
            }
        }
        out
    }
}
