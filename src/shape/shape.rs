//use crate::material::Material;

use crate::material::Material;
use crate::tuple::{Point, Vector};
use crate::{intersection::Intersections, ray::Ray};
use std::fmt;
// `Any` allows us to do dynamic typecasting.
use std::any::Any;


pub trait Shape : fmt::Debug {
    /// It return any Intersections that occured 
    /// between the shape and the ray provided as
    /// argument
    fn intersect(&self, ray: Ray) -> Intersections;

    fn id(&self) -> usize;

    // I need this because can't implement a clone for a trait object
    fn clone_box(&self) -> Box<dyn Shape>;

    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test.
    fn eq_box(&self, other: &dyn Any) -> bool;

    /// Return the normal vector from the shape
    /// for the given point
    fn normal_at(&self, p:Point) -> Vector;

    /// Return shape's material
    fn material(&self) ->Material;
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


