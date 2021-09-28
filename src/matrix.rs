use core::panic;
use std::ops::{Index, IndexMut};

const SIZE: usize = 4;

#[derive(Debug, Clone)]
pub struct Mat4 {
    buffer: Vec<f32>,
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            buffer: vec![0.0; SIZE*SIZE],
        }
    }


    pub fn from_buffer(buffer: &[f32]) -> Result<Self, String> {
        if SIZE * SIZE == buffer.len() {
            Ok(Self { buffer: buffer.into()})
        } else {
            Err("Matrix - Error creating matrix: buffer size mistmatchd ".into())
        }
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


