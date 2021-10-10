use crate::{get_id, ray::Ray, shape::Shape, tuple::point};


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
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);

        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0 ;
        
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return vec![];
        } 

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            return vec![t2, t1]
        }  
        vec![t1, t2]

    }   

    fn id(&self) -> usize {
        self.id
    }
}