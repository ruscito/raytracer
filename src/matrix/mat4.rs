use core::panic;
use std::{convert::TryInto, ops::{Index, IndexMut, Mul}};
use crate::tuple::Tuple;
use super::Mat3;

const SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    buffer: [f32; SIZE*SIZE],
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            buffer:[0.0; SIZE*SIZE],
        }
    }


    pub fn from_buffer(buffer: [f32;SIZE*SIZE]) -> Self {
        Self { buffer: buffer}
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

    pub fn submatrix(&self, row:usize, col: usize) ->Mat3 {
        let mut tmp = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                if i != row && j != col {
                    tmp.push(self[(i, j)]); 
                }  
            }
        }
        // let arr: [_; N] = slice.try_into().unwrap() (replace N)
        // (&tmp[0..9]).try_into().unwrap() or
        // let arr:[f32;9] = tmp[0..9].try_into().unwrap(); 
        Mat3::from_buffer(tmp[0..9].try_into().unwrap())
    }

    pub fn minor(&self, row:usize, col:usize) -> f32 {
        // the submatrix return a mat3
        // so the determinat called is the one from mat3
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row:usize, col:usize) -> f32 {
        if (row + col) % 2 != 0 {
            return -self.minor(row, col);
        } 
        self.minor(row, col)
    }

    pub fn det(&self) -> f32{
        let mut det = 0.0f32;
        for col in 0..SIZE {
            det = det + self[(0, col)] * self.cofactor(0, col);
        }
        det
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.0 
    }

    pub fn inv(&self) -> Mat4 {
        if !self.is_invertible() {
            panic!("Non invertible matrix :{:?}", self);
        }
        let mut out = Mat4::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                let cofactor = self.cofactor(row, col);
                // note that "col, row" here in stead of "row, col",
                // accomplishes the transpose operation
                out[(col, row)] = cofactor / self.det();
            }
        }
        out
    }

    pub fn translate(&self, x:f32, y:f32, z:f32) -> Mat4 {
        *self * Self{
            buffer: [ 1.0, 0.0, 0.0,  x,
                      0.0, 1.0, 0.0,  y,  
                      0.0, 0.0, 1.0,  z,  
                      0.0, 0.0, 0.0, 1.0 ]
        }
    }

    pub fn scale(&self, x:f32, y:f32, z:f32) -> Mat4 {
        *self * Self{
            buffer: [  x,  0.0, 0.0, 0.0,
                      0.0,  y,  0.0, 0.0,  
                      0.0, 0.0,  z,  0.0,  
                      0.0, 0.0, 0.0, 1.0 ]
        }
    }

    pub fn rotate_x(&self, r: f32) -> Mat4 {
        *self * Self{
            buffer: [ 1.0,    0.0,      0.0, 0.0,
                      0.0, r.cos(),-r.sin(), 0.0,  
                      0.0, r.sin(), r.cos(), 0.0,  
                      0.0,    0.0,      0.0, 1.0 ]
        }
    }
    
    pub fn rotate_y(&self, r: f32) -> Mat4 {
        *self * Self{
            buffer: [r.cos(), 0.0, r.sin(), 0.0,
                         0.0, 1.0,     0.0, 0.0,  
                    -r.sin(), 0.0, r.cos(), 0.0,  
                         0.0, 0.0,     0.0, 1.0 ]
        }
    }
    
    pub fn rotate_z(&self, r: f32) -> Mat4 {
        *self * Self{
            buffer: [ r.cos(), -r.sin(), 0.0, 0.0,
                      r.sin(),  r.cos(), 0.0, 0.0,  
                          0.0,      0.0, 1.0, 0.0,  
                          0.0,      0.0, 0.0, 1.0 ]
        }
    }

    pub fn skew(&self, xy:f32, xz:f32, yx:f32, yz:f32, zx:f32, zy:f32) -> Mat4 {
        *self * Self{
            buffer: [ 1.0,  xy,  xz, 0.0,
                       yx, 1.0,  yz, 0.0,  
                       zx,  zy, 1.0, 0.0,  
                      0.0, 0.0, 0.0, 1.0 ]
        }        
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

//--------------------------------------------------------------------

pub fn identity() -> Mat4 {
    Mat4::from_buffer([ 
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,  
        0.0, 0.0, 1.0, 0.0,  
        0.0, 0.0, 0.0, 1.0]
    )
}

pub fn translate(x:f32, y:f32, z:f32) -> Mat4 {
    Mat4::from_buffer([ 
        1.0, 0.0, 0.0,  x,
        0.0, 1.0, 0.0,  y,  
        0.0, 0.0, 1.0,  z,  
        0.0, 0.0, 0.0, 1.0]
    )
}

pub fn scale(x:f32, y:f32, z:f32) -> Mat4 {
    Mat4::from_buffer([ 
        x,  0.0, 0.0, 0.0,
        0.0,  y,  0.0, 0.0,  
        0.0, 0.0,  z,  0.0,  
        0.0, 0.0, 0.0, 1.0 ]
    )
}

pub fn rotate_x(r: f32) -> Mat4 {
    Mat4::from_buffer([ 
        1.0,    0.0,      0.0, 0.0,
        0.0, r.cos(),-r.sin(), 0.0,  
        0.0, r.sin(), r.cos(), 0.0,  
        0.0,    0.0,      0.0, 1.0 ]
    )
}

pub fn rotate_y(r: f32) -> Mat4 {
    Mat4::from_buffer([
        r.cos(), 0.0, r.sin(), 0.0,
            0.0, 1.0,     0.0, 0.0,  
       -r.sin(), 0.0, r.cos(), 0.0,  
            0.0, 0.0,     0.0, 1.0 ]
    )
}

pub fn rotate_z(r: f32) -> Mat4 {
    Mat4::from_buffer([ 
        r.cos(), -r.sin(), 0.0, 0.0,
        r.sin(),  r.cos(), 0.0, 0.0,  
            0.0,      0.0, 1.0, 0.0,  
            0.0,      0.0, 0.0, 1.0 ]
        )
}

pub fn skew(xy:f32, xz:f32, yx:f32, yz:f32, zx:f32, zy:f32) -> Mat4 {
    Mat4::from_buffer([
        1.0,  xy,  xz, 0.0,
         yx, 1.0,  yz, 0.0,  
         zx,  zy, 1.0, 0.0,  
         0.0, 0.0, 0.0, 1.0 ]
        )
}