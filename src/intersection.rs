use std::ops::Index;

use crate::comps::Comps;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::EPSILON;

/// The type contains the value [t: f64] of the intersection
/// and the [object: <dyn shape>] that was intersect by a ray. The value
/// of the intersection represent the distance in unit from the origin of the ray
/// to the point of intersection with the shape
#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Shape>,
}

impl Intersection {
    pub fn new(t: f64, object: Box<dyn Shape>) -> Self {
        Self { t, object }
    }

    pub fn prepare_computation(&self, r: Ray) -> Comps {
        let point = r.position(self.t);
        let mut normalv = self.object.normal_at(point);
        let eyev = -r.direction;
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        Comps {
            t: self.t,
            object: self.object.clone_box(),
            point: point,
            eyev: eyev,
            normalv: normalv,
            inside: inside,
            over_point: point + normalv * EPSILON,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && &self.object == &other.object
    }
}

/// The type contains a list [xs: vec<Intersection>] of intersections.
#[derive(Debug, Clone)]
pub struct Intersections {
    pub xs: Vec<Intersection>,
    pub hit: Option<Intersection>
}

impl Intersections {
    pub fn new(xs: Vec<Intersection>) -> Self {
        let mut out = Intersections {xs, hit: None};
        out.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        for i in out.xs.iter() {
            if i.t > 0.0 {
                out.hit = Some(i.clone());
                return out;
            }
        }
        out
    }

    /// It returns the number of intersections
    pub fn len(&self) -> usize {
        self.xs.len()
    }
    /// it return the visible intersection from the ray origin. The [hit] will
    /// always be the intersection with the lowest non negative [t: f64] value.
    /// The method return [None] if not intersections
    pub fn hit(&self) -> Option<Intersection> {
        match &self.hit {
            Some(hit) => Some(hit.clone()),
            None => None,
        }
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.len() - 1 {
            panic!(
                "Error trying to access Intersections[{}] with length: {}",
                idx,
                self.len()
            );
        }
        &self.xs[idx]
    }
}
