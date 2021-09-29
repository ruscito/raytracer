use core::panic;
use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

const SIZE: usize = 4;

#[derive(Debug, Clone)]
pub struct Mat4 {
    buffer: [f32; SIZE*SIZE],
}

impl Mat4 {
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
            buffer: [ 1.0, 0.0, 0.0, 0.0,
                      0.0, 1.0, 0.0, 0.0,  
                      0.0, 0.0, 1.0, 0.0,  
                      0.0, 0.0, 0.0, 1.0 ]
        }
    }

    pub fn transpose(&self) -> Self {
        // transpose a matrix turn its rows into columns
        let mut out= Mat4::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(col, row)]
            }
        }
        out
    }
}

impl PartialEq for Mat4 {
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

impl Index<(usize, usize)> for  Mat4 {
    type Output = f32;
    fn index(&self, (row, col):(usize, usize)) -> &Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &self.buffer[row*SIZE + col]
    }    
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (row, col):(usize, usize)) -> &mut Self::Output {
        if row >= SIZE || col >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &mut self.buffer[row*SIZE +col]
    }    
}

impl  Mul<Mat4> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut out = Mat4::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(row, 0)] * rhs[(0, col)] +
                                  self[(row, 1)] * rhs[(1, col)] +
                                  self[(row, 2)] * rhs[(2, col)] +
                                  self[(row, 3)] * rhs[(3, col)];
            }
        }
        out
    }
}

impl Mul<Tuple> for Mat4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        // the tuple is one column 4 row 1 col
        Tuple::new(
            self[(0, 0)] * rhs.x + 
                self[(0, 1)] * rhs.y +
                self[(0, 2)] * rhs.z +
                self[(0, 3)] * rhs.w,
            self[(1, 0)] * rhs.x + 
                self[(1, 1)] * rhs.y +
                self[(1, 2)] * rhs.z +
                self[(1, 3)] * rhs.w,
            self[(2, 0)] * rhs.x + 
                self[(2, 1)] * rhs.y +
                self[(2, 2)] * rhs.z +
                self[(2, 3)] * rhs.w,
            self[(3, 0)] * rhs.x + 
                self[(3, 1)] * rhs.y +
                self[(3, 2)] * rhs.z +
                self[(3, 3)] * rhs.w)
    }
}


