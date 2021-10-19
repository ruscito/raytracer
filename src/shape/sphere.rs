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
pub struct Sphere {
    id: usize,
    transform: Mat4,
    inverse_transform: Mat4,
    transpose_inverse_transform: Mat4,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: get_id(),
            transform: Mat4::identity(), 
            inverse_transform: Mat4::identity(),
            transpose_inverse_transform: Mat4::identity().transpose(),
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
        self.inverse_transform = transform.inv();
        self.transpose_inverse_transform = transform.inv().transpose();
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    } 
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        //std::ptr::eq(self, other)
        self.id() == other.id()
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Intersections {
        //let start = time::Instant::now();
        let ray = ray.transform(&self.inverse_transform);
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        //println!("{} elpased.", start.elapsed().as_micros());

        let a = ray.direction.dot(&ray.direction);
        
        let b = 2.0 * ray.direction.dot(&sphere_to_ray.into());

        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0 ;
        
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        } 

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
  
        Intersections::new(vec![Intersection { t: t1, object: Box::new(*self)} , Intersection { t: t2, object: Box::new(*self)}])
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
    
    fn normal_at(&self, pnt:Point) -> Vector {
        // convert the point from world space to object space
        let object_point =  self.inverse_transform * pnt;
        // Now I can calculate the object normal 
        let object_normal = object_point - Point::new(0., 0., 0.);
        // Now transform the object_normal in world space
        let mut world_normal =  self.transpose_inverse_transform * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Mat4 {
        self.transform
    }
}
