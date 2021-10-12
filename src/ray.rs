use crate::{matrix::Mat4, tuple::Tuple};

// A ray is composed by an origin and
// a direction. This direction vector can be the speed

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn position(&self, t: f32) -> Tuple {
        // calculate the position of the ray at a distance
        // of t along the line 
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: &Mat4) -> Ray {
        Self {
            origin: *m * self.origin,
            direction: *m * self.direction,
        }
    }
}