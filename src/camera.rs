use crate::canvas::Canvas;
use crate::matrix::{mat4::identity, Mat4};
use crate::ray::Ray;
use crate::tuple::Point;
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = (fov / 2.).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect_ratio >= 1. {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2. / hsize as f64;
        Self {
            hsize,
            vsize,
            fov,
            half_width,
            half_height,
            pixel_size,
            transform: identity(),
            inverse_transform: identity().inv(),
        }
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    pub fn set_transform(&mut self, m: Mat4) {
        self.transform = m;
        self.inverse_transform = m.inv();
    }

    /// This function return a Ray that start at the camera and
    /// pass to the given [x, y] pixel on the canvas
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let x_world = self.half_width - x_offset;
        let y_world = self.half_height - y_offset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = self.inverse_transform * Point::new(x_world, y_world, -1.0);
        let origin = self.inverse_transform * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    /// This function cast a ray through each of world canvas pixels
    pub fn render(&self, w: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                image[(x, y)] = w.color_at(r);
            }
        }
        image
    }
}
