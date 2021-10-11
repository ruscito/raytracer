

use std::ops::Index;

use crate::shape::{ShapeObject};

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object:  ShapeObject,
}

impl Intersection {
    pub fn new(t: f32, object: ShapeObject) -> Self {
        Self {
            t,
            object,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t &&
        &self.object == &other.object
    }
}

#[derive(Debug)]
pub struct Intersections {
    pub xs: Vec<Intersection>,
}

impl Intersections {
    pub fn new(xs: Vec<Intersection>) -> Self {
        let mut out = Self { xs };
        out.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        out
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        for i in self.xs.iter() {
            if i.t > 0.0 {
                return Some(i.clone());
            }
        }
        None

    }   
}

impl Index<usize> for  Intersections {
    type Output = Intersection;
    fn index(&self, idx:usize) -> &Self::Output {
        if idx > self.len() - 1 {
            panic!("Error trying to access Intersections[{}] with length: {}", idx, self.len());
        }
        &self.xs[idx]
    }    
}
