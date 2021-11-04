//use std::time;

use crate::get_id; 
use crate::intersection::{Intersection, Intersections}; 
use crate::material::Material; 
use crate::matrix::Mat4; 
use crate::ray::Ray; 
use crate::tuple::Point;
use crate::tuple::Vector;

// `Any` allows us to do dynamic typecasting.
use std::any::Any;
use super::shape::Shape;


/// A shape that contain an [id: usize] a defaul idenityt [transform: Mat4] and
/// a default [material: Material]
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub id: usize,
    transform: Mat4,
    inverse_transform: Mat4,
    transpose_inverse_transform: Mat4,
    material: Material,
}

impl Plane {
    pub fn new(transform: Option<Mat4>, material: Option<Material>) -> Self {
        let t = transform.unwrap_or(Mat4::identity());
        Self {
            id: get_id(),
            transform: t, 
            inverse_transform: t.inv(),
            transpose_inverse_transform: t.inv().transpose(),
            material: material.unwrap_or( Material::default()),
        }
    }
}

impl PartialEq for Plane {

    fn eq(&self, other: &Self) -> bool {
        //std::ptr::eq(self, other)
        self.id() == other.id()
    }
}

impl Shape for Plane {
    
    fn intersect(&self, ray: Ray) -> Intersections {
        if crate::f64eq(0.0, ray.direction.y) {
            return Intersections::new(vec![]);
        }
        let t = -ray.origin.y / ray.direction.y;

        Intersections::new(vec![Intersection::new(t, Box::new(self.clone()))])
    }   

    fn id(&self) -> usize {
        self.id
    }

    fn clone_box(&self) -> Box<dyn Shape> { 
        Box::new(self.clone())
    }

    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any{
        self 
    }

    // Perform the test.
    fn eq_box(&self, other: &dyn Any) -> bool {
        //other.downcast_ref::<Self>().map_or(false, |a| self == a)
        match other.downcast_ref::<Self>() {
            Some(s) => self.id == s.id,
            _ => false
        } 
    }
    
    fn normal_at(&self, _pnt:Point) -> Vector {
        Vector::new(0.0, 0.1, 0.0)
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn material(&self) -> Material {
        self.material.clone()
    }
    
    fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
        self.inverse_transform = transform.inv();
        self.transpose_inverse_transform = transform.inv().transpose();
    }

    fn transform(&self) -> Mat4 {
        self.transform
    }
}
