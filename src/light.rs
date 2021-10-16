use crate::color::Color;
use crate::tuple::Point;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self { position, intensity}
    }
}