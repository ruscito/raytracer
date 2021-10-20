use crate::shape::Shape;
use crate::tuple::*;

pub struct Comps {
    pub t: f32,
    pub object: Box<dyn Shape>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl Comps {
    pub fn new(t: f32, object: Box<dyn Shape>, point: Point, eyev: Vector, normalv: Vector, inside: bool) ->Self {
        Self {t, object, point, eyev, normalv, inside}
    }
}