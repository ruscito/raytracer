use crate::matrix::{Mat4, mat4::identity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
    pub transform: Mat4,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
        let half_view = (fov / 2.).tan();
        let aspect_ratio = hsize as f32 / vsize as f32;
        let (half_width, half_height) = if aspect_ratio >= 1. {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2. / hsize as f32;
        Self { 
            hsize, 
            vsize, 
            fov,
            half_width,
            half_height,
            pixel_size,
            transform: identity()
        } 
    }
}

