use std::ops::Index;

use crate::comps::Comps;
use crate::ray::Ray; 
use crate::shape::Shape;

/// The type contains the value [t: f32] of the intersection
/// and the [object: <dyn shape>] that was intersect by a ray. The value
/// of the intersection represent the distance in unit from the origin of the ray
/// to the point of intersection with the shape
#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object: Box<dyn Shape>,
}

impl Intersection {
    pub fn new(t: f32, object: Box<dyn Shape>) -> Self {
        Self {
            t,
            object,
        }
    }

    /// It return a [Comps] structure containing precomputed
    /// information relating to the intersection 
    pub fn prepare_computations(&self, ray: Ray) -> Comps  {
        let position = ray.position(self.t);
        let normalv = self.object.normal_at(position);
        let inside = normalv.dot(&-ray.direction);
        Comps { 
            t:self.t, 
            object:self.object.clone_box(), 
            point: position, 
            eyev: - ray.direction, 
            normalv: if inside < 0.0 {-normalv} else {normalv},
            inside: if inside < 0.0 {true} else {false},
        }
    } 
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t &&
        &self.object == &other.object
    }
}




/// The type contains a list [xs: vec<Intersection>] of intersections. 
#[derive(Debug, Clone)]
pub struct Intersections {
    pub xs: Vec<Intersection>,
}

impl Intersections {
    pub fn new(xs: Vec<Intersection>) -> Self {
        let mut out = Self { xs };
        out.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        out
    }
    
    /// It returns the number of intersections 
    pub fn len(&self) -> usize {
        self.xs.len()
    }
    /// it return the visible intersection from the ray origin. The [hit] will 
    /// always be the intersection with the lowest non negative [t: f32] value.
    /// The method return [None] if not intersections
    pub fn hit(&self) -> Option<Intersection> {
        for i in self.xs.iter() {
            if i.t > 0.0 {
                return Some(i.clone());
            }
        }
        None   
    }
}

impl Index<usize> for  Intersections {
    type Output = Intersection;
    fn index(&self, idx:usize) -> &Self::Output {
        if idx > self.len() - 1 {
            panic!("Error trying to access Intersections[{}] with length: {}", idx, self.len());
        }
        &self.xs[idx]
    }    
}