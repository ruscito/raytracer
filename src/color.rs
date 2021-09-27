use std::ops::{Add, Mul, Sub};
use std::convert::From;
use crate::f32_to_u8;

pub const BLACK: Color = Color{r:0.0, g:0.0, b:0.0};
pub const WHITE: Color = Color{r:1.0, g:1.0, b:1.0};
pub const RED: Color = Color{r:1.0, g:0.0, b:0.0};
pub const GREEN: Color = Color{r:0.0, g:1.0, b:0.0};
pub const BLUE: Color = Color{r:0.0, g:0.0, b:1.0};
pub const YELLOW: Color = Color{r:1.0, g:1.0, b:0.0};
pub const MAGENTA: Color = Color{r:1.0, g:0.0, b:1.0};
pub const CYAN: Color = Color{r:0.0, g:1.0, b:1.0};

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

    pub fn black() -> Self {
        Self { r:0.0, g:0.0, b:0.0}
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