use core::panic;
use std::ops::{Index, IndexMut, Mul};

const SIZE: usize = 3;

#[derive(Debug, Clone)]
pub struct Mat3 {
    buffer: [f32; SIZE*SIZE],
}

impl Mat3 {
    pub fn new() -> Self {
        Self {
            buffer:[0.0; SIZE*SIZE],
        }
    }


    pub fn from_buffer(buffer: &[f32;SIZE*SIZE]) -> Self {
        Self { buffer: *buffer}
    }   
    
    pub fn size(&self) -> usize {
        SIZE * SIZE
    }

    pub fn identiy()-> Self {
        Self{
            buffer: [ 1.0, 0.0, 0.0,
                      0.0, 1.0, 0.0,  
                      0.0, 0.0, 1.0]
        }
    }

    pub fn transpose(&self) -> Self {
        // transpose a matrix turn its rows into columns
        let mut out= Mat3::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(col, row)]
            }
        }
        out
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

impl Index<(usize, usize)> for  Mat3 {
    type Output = f32;
    fn index(&self, (row, col):(usize, usize)) -> &Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &self.buffer[row*SIZE + col]
    }    
}

impl IndexMut<(usize, usize)> for Mat3 {
    fn index_mut(&mut self, (row, col):(usize, usize)) -> &mut Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &mut self.buffer[row*SIZE +col]
    }    
}

impl  Mul<Mat3> for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut out = Mat3::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(row, 0)] * rhs[(0, col)] +
                                  self[(row, 1)] * rhs[(1, col)] +
                                  self[(row, 2)] * rhs[(2, col)];
            }
        }
        out
    }
}


