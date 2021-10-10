use crate::shape::{Shape};

pub struct Intersection {
    pub t: f32,
    pub object:  Box<dyn Shape>,
}

impl Intersection {
    pub fn new(t: f32, object: Box<dyn Shape>) -> Self {
        Self {
            t,
            object,
        }
    }
}

pub fn intersections(i1:Intersection, i2:Intersection) -> Vec<Intersection> {
    if i1.t > i2.t {
        return vec![i2, i1];
    } 
    vec![i1, i2]
}

