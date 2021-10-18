use crate::color::Color;
use crate::tuple::Point;

/// A point light: a light source with no size existing at single 
/// [position: Point] in space. It is also defined by its [intensity: Color] 
/// which describes how bright is the color of the light source
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
   