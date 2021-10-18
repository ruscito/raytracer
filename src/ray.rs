use crate::{matrix::Mat4, tuple::{Point, Vector}};

/// A ray has a starting point called [origin: Point] and a [direction: Vector] 
/// which it says where the ray point to.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction,
        }
    }

    /// This method return a [Point] at the given distance [distance: f32]
    /// along the ray. It multiplays the ray's direction by the distance to 
    /// find the total distance travled, and add that to the ray's origin
    pub fn position(&self, distance: f32) -> Point {
        // calculate the position of the ray at a distance
        // of t along the line 
        self.origin + self.direction * distance
    }

    /// This function applies the given [tm: Mat4] transformation matrix to 
    /// to the ray. It return a new [Ray]
    pub fn transform(&self, tm: &Mat4) -> Ray {
        Self {
            origin: *tm * self.origin,
            direction: *tm * self.direction,
        }
    }
}