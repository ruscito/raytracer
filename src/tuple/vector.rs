use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Tuple, f32eq};
use crate::tuple::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple for Vector{}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z, w:0.0}
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn distance(&self, other:&Vector) -> f32 {
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

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w 
    }

    pub fn cross(&self, other: &Self) -> Vector {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
            w: 0.0,
        }
    }

    pub fn reflect(&self, normal:Vector) -> Vector {
        *self - normal * 2.0 * self.dot(&normal)
    }
}

impl PartialEq for Vector {
    fn eq (&self, other: &Vector) -> bool {
        f32eq(self.x, other.x) &&
        f32eq(self.y, other.y) &&
        f32eq(self.z, other.z) &&
        f32eq(self.w, other.w)
    }
}

impl Add <Vector> for Vector {
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

impl Add <Point> for Vector {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub <Vector> for Vector {
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

impl Neg for  Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 0.0,
        }
    }
}

impl Div <f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 0.0,
        }        
    }
}

impl Mul <f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 0.0,
        }        
    }
}

impl From <Point> for Vector {
    fn from(p:Point) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 0.0
        }
    }
}