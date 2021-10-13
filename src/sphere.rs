use crate::{get_id, intersection::{Intersection, Intersections}, matrix::Mat4, ray::Ray, shape::Shape, tuple::point};
// `Any` allows us to do dynamic typecasting.
use std::any::Any;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    id: usize,
    transform: Mat4,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: get_id(),
            transform: Mat4::identity(),
        }
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

        let ray = ray.transform(&self.transform.inv());

        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);

        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0 ;
        
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        } 

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            return Intersections::new(vec![Intersection { t: t2, object: Box::new(*self)} , Intersection { t: t1, object: Box::new(*self)}]);
        }  
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

    fn get_transform(&self) -> Mat4 {
        self.transform.clone()
    }

    fn set_transform(&mut self, m: Mat4) {
        self.transform = m;
    }
}