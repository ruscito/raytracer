use std::f32::consts::PI;

use raytracer::{camera::Camera, matrix::mat4::identity};

#[test]
fn constructing_a_camera() {
    let hsize = 160usize;
    let vsize = 120usize;
    let fov =  PI/2.0;
    let c = Camera::new(hsize, vsize, fov);
    assert_eq!(c.hsize, hsize);
    assert_eq!(c.vsize, vsize);
    assert_eq!(c.fov, fov);
    assert_eq!(c.transform, identity());
}

#[test]
fn pixel_size_horizontal_canvas() {
    let c = Camera::new(200, 125, PI/2.0);
    assert_eq!(c.pixel_size, 0.01);
}

#[test]
fn pixel_size_vertical_canvas() {
    let c = Camera::new(125, 200, PI/2.0);
    assert_eq!(c.pixel_size, 0.01);
}