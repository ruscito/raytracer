use core::panic;
use std::ops::{Index, IndexMut, Mul};

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
    fn index(&self, (x, y):(usize, usize)) -> &Self::Output {
        if x >= SIZE || y >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &self.buffer[y*SIZE + x]
    }    
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (x, y):(usize, usize)) -> &mut Self::Output {
        if x >= SIZE || y >= SIZE {
            panic!("Matrix - Error indexing matrix: out of bound");
        } 
        &mut self.buffer[y*SIZE +x]
    }    
}

impl  Mul<Mat4> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut out = Mat4::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                out[(row, col)] = self[(row, 0 as usize)] * rhs[(0 as usize, col)] +
                                  self[(row, 1 as usize)] * rhs[(1 as usize, col)] +
                                  self[(row, 2 as usize)] * rhs[(2 as usize, col)] +
                                  self[(row, 3 as usize)] * rhs[(3 as usize, col)];
            }
        }
        out
    }
}


