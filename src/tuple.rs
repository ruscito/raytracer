use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
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

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn distance(&self, other:&Tuple) -> f32 {
        ((other.x -self.x).powf(2.0) 
            + (other.y -self.y).powf(2.0)
            + (other.z -self.z).powf(2.0) 
        ).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    pub fn mut_normalize(&mut self){
            let magnitude = self.magnitude();
            self.x = self.x / magnitude;
            self.y = self.y / magnitude;
            self.z = self.z / magnitude;
            self.w = self.w / magnitude;
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w 
    }

    pub fn cross(&self, other: &Self) -> Tuple {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
            w: 0.0,
        }
    }
}

impl PartialEq for Tuple {
    fn eq (&self, other: &Tuple) -> bool {
        super::f32eq(self.x, other.x) &&
        super::f32eq(self.y, other.y) &&
        super::f32eq(self.z, other.z) &&
        super::f32eq(self.w, other.w)
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


pub fn point(x: f32, y: f32, z: f32,) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32,) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}
