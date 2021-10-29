use crate::shape::Shape;
use crate::tuple::*;

pub struct Comps {
    pub t: f64,
    pub object: Box<dyn Shape>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
}
