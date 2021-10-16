use crate::{color::Color, tuple::Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self { position, intensity}
    }
}