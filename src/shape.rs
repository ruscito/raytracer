use crate::matrix::Mat4;
use crate::tuple::Tuple;
use crate::{intersection::Intersections, ray::Ray};
use std::fmt;
// `Any` allows us to do dynamic typecasting.
use std::any::Any;

pub type ShapeObject = Box<dyn Shape>;

pub trait Shape : fmt::Debug {
    // intesect return None or two values that represent 
    // respectively number of units away from the ray's origin
    fn intersect(&self, ray: Ray) -> Intersections;

    fn id(&self) -> usize;

    // I need this because can't implement a clone for a trait object
    fn clone_box(&self) -> ShapeObject;

    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test.
    fn eq_box(&self, other: &dyn Any) -> bool;

    fn get_transform(&self) -> Mat4;

    fn set_transform(&mut self, m: Mat4);

    fn normal_at(&self, p:&Tuple) -> Tuple;
}

impl Clone for ShapeObject {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for ShapeObject {
    fn eq(&self, other: &ShapeObject) -> bool {
        self.eq_box(other.as_any())
    }
}


