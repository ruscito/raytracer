use std::ops::{Add, Mul, Sub};
use std::convert::From;

use crate::f32_to_u8;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g:f32, b:f32) -> Self {
        Self {r, g, b}
    }  

    pub fn to_bytes(&self) -> (u8, u8, u8) {
        (f32_to_u8(self.r), f32_to_u8(self.g), f32_to_u8(self.b))
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        super::f32eq(self.r, rhs.r) &&
        super::f32eq(self.g, rhs.g) &&
        super::f32eq(self.b, rhs.b) 
    }
}

impl From <Color> for (u8, u8, u8) {
    fn from (c: Color) -> (u8, u8, u8) {
        (f32_to_u8(c.r), f32_to_u8(c.g), f32_to_u8(c.b))
    }
}

impl From <(u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8)) -> Self {
        Self {
            r: (color.0 as f32 / 255.0),
            g: (color.1 as f32 / 255.0),
            b: (color.2 as f32 / 255.0),
        }
    }
}


impl Add<Color> for Color   {
    type Output = Self;
    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<Color> for Color   {
    type Output = Self;
    fn sub(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}


impl Mul <f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }        
    }
}

impl Mul <Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        Self{
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }        
    }
}