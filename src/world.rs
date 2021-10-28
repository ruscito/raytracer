use crate::comps::Comps;
use crate::intersection::Intersections;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::mat4::scale;
use crate::ray::Ray;
use crate::shape::{Shape, Sphere};
use crate::color::{BLACK, Color};
use crate::tuple::*;

#[derive(Debug, Clone)]
pub struct World {
    pub light: Option<Light>,
    pub objects: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn new(light: Option<Light>, objects: Option< Vec<Box<dyn Shape>>>) -> Self {
    
        Self {
            light,
            objects: objects.unwrap_or(vec![]),
        }
    }

    /// Function iterate over all of the objects that have been added to the world, 
    /// intersecting each of them with the given ray, and returnig the collection
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let vec_intersections = self.objects.iter().map(|obj| obj.intersect(ray)).collect::<Vec<Intersections>>();
        let mut xs = vec![];
        for intersections in vec_intersections {
            for intersection in intersections.xs {
                xs.push(intersection)
            }
        }
        Intersections::new(xs)
    }

    /// This function return the color at the intersection encapsulated
    /// by the given [c: Comps] with the world
    pub fn shade_it(&self, c: Comps) -> Color {
        c.object.material().lighting(self.light.unwrap(), c.point, c.eyev, c.normalv)
    }

    /// This function intersect the world with the given ray 
    /// and then return the color at the resulting intersection.
    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);        
        if let Some(hit) = xs.hit() {
            self.shade_it(hit.prepare_computation(r))
        } else {
            BLACK
        }
    }

}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Box::new(Sphere::new());
        let mut s2 = Box::new(Sphere::new());
        let mtrl = Material::new(
            Some(Color::new(0.8, 1.0, 0.6)), 
            None, 
            Some(0.7), 
            Some(0.2), 
            None);
        s1.set_material(mtrl);
        s2.set_transform(scale(0.5, 0.5, 0.5));
        World { 
            light: Some(Light::new(
                Point::new(-10., 10., -10.), 
                Color::new(1., 1., 1.))
            ), 
            objects: vec![s1, s2],
        }
    }
}
