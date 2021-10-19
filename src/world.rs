use crate::color::{Color, WHITE};
use crate::light::Light; 
use crate::material::Material;
use crate::matrix::Mat4;
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