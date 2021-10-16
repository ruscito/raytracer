//use crate::material::Material;

use crate::tuple::{Point, Vector};
use crate::{intersection::Intersections, ray::Ray};
use std::fmt;
// `Any` allows us to do dynamic typecasting.
use std::any::Any;


pub trait Shape : fmt::Debug {
    // intesect return None or two values that represent 
    // respectively number of units away from the ray's origin
    fn intersect(&self, ray: Ray) -> Intersections;

    fn id(&self) -> usize;

    // I need this because can't implement a clone for a trait object
    fn clone_box(&self) -> Box<dyn Shape>;

    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test.
    fn eq_box(&self, other: &dyn Any) -> bool;

    fn normal_at(&self, p:&Point) -> Vector;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
        self.eq_box(other.as_any())
    }
}


