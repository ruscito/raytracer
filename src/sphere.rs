use crate::{get_id, ray::Ray};


#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: get_id()
        }
    }

    pub fn intersect(self, ray: Ray) -> Vec<f32>{
        vec![]
    }
}