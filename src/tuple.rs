use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {x, y, z, w}
    }

    pub fn is_point(self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(self) -> bool {
        self.w == 0.0
    }

    pub fn point(x: f32, y: f32, z: f32,) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32,) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn zero_vector() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Add <Tuple> for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub <Tuple> for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for  Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Div <f32> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }        
    }
}

impl Mul <f32> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }        
    }
}
