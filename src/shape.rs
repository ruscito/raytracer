use crate::ray::Ray;

pub trait Shape {
    // intesect return None or two values that represent 
    // respectively number of units away from the ray's origin
    fn intersect(&self, ray: Ray) -> Vec<f32>;

    fn id(&self) -> usize;
}


//pub type ShapeObject = Box<dyn Shape>;

impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
        self.id() == other.id()
    }
}