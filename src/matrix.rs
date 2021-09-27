use std::{cmp::min, ops::{Index, IndexMut}};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub row: usize,
    pub column: usize,
    _buffer: Vec<f32>,
}

impl Matrix {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            _buffer: vec![0.0; row*column],
        }
    }


    pub fn from_buffer(row: usize, column: usize, buffer: &[f32]) -> Result<Self, String> {
        if row * column == buffer.len() {
            Ok(Self { row, column, _buffer: buffer.into()})
        } else {
            Err("Matrix - Error creating matrix; buffer size mistmatchd ".into())
        }
    }   
    
    pub fn size(&self) -> usize {
        self.row * self.column
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for i in 0..self._buffer.len() {
            if self._buffer[i] != other._buffer[i] {
                return false;
            }
        }
        true
    }
}


impl Index<[usize; 2]> for  Matrix {
    type Output = f32;
    fn index(&self, idx:[usize; 2]) -> &Self::Output {
        let col = min(self.column - 1, idx[0]);
        let row = min(self.row - 1, idx[1]);
        &self._buffer[row * self.column + col]
    }    
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, idx:[usize; 2]) -> &mut Self::Output {
        &mut self._buffer[(min(self.row - 1, idx[1]) * self.column) + min(self.column - 1, idx[0])]
    }    
}


