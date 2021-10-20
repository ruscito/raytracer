use crate::color::{Color, WHITE};
use crate::comps::Comps;
use crate::intersection::{Intersection, Intersections};
use crate::light::Light; 
use crate::material::Material;
use crate::matrix::Mat4;
use crate::ray::Ray;
use crate::shape::{Shape, Sphere};
use crate::tuple::Point;

/// A struct containig a list od shapes [shapes: Vec<Box<dyn Shape>>] and
/// a light [light: Option<Light>]
#[derive(Debug, Clone)]
pub struct World {
    pub light: Option<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn new(light: Option<Light>, shapes: Vec<Box<dyn Shape>>) -> Self {
        World {light, shapes}
    }

    /// This function return all the intersections between
    /// the given Ray and the world. The instersections are sorted
    pub fn intersect(&self, ray:Ray) -> Intersections {
        let out: Vec<Intersections> = self.shapes.iter().map(|s| s.intersect(ray)).collect();
        let mut i: Vec<Intersection> = vec![];
        for intersections in out{
            for intersection in intersections.xs {
                i.push(intersection);
            }
        }
        Intersections::new(i)
    }

    pub fn shade_hit(&self, comps: Comps) -> Color {
        comps.object.material().lighting(self.light.unwrap(), comps.point, comps.eyev, comps.normalv)
    }
}


impl Default for World {
    fn default() -> Self {
        let mut m = Material::new();
        m.color =  Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        let mut s1= Sphere::new();
        s1.set_material(m.clone());
        
        let mut s2= Sphere::new();
        s2.set_transform(Mat4::identity().scale(0.5, 0.5, 0.5));
        
        Self {
            light:Some(Light::new(Point::new(-10.0, 10.0, -10.0), WHITE)),
            shapes: vec![Box::new(s1), Box::new(s2)],
        }
    }
}