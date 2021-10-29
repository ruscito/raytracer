use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::matrix::mat4::*;
use raytracer::{f64eq, tuple::*};
use raytracer::world::World;
use std::f64::consts::PI;

#[test]
fn constructing_a_camera() {
    let hsize = 160usize;
    let vsize = 120usize;
    let fov = PI / 2.0;
    let c = Camera::new(hsize, vsize, fov);
    assert_eq!(c.hsize, hsize);
    assert_eq!(c.vsize, vsize);
    assert_eq!(c.fov, fov);
    assert_eq!(c.transform(), identity());
}

#[test]
fn pixel_size_horizontal_canvas() {
    let c = Camera::new(200, 125, PI / 2.0);
    assert!(f64eq(c.pixel_size, 0.01));
}

#[test]
fn pixel_size_vertical_canvas() {
    let c = Camera::new(125, 200, PI / 2.0);
    assert!(f64eq(c.pixel_size, 0.01));
}

#[test]
fn ray_through_the_center_of_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, Point::new(0., 0., 0.));
    assert_eq!(r.direction, Vector::new(0., 0., -1.));
}

#[test]
fn ray_through_the_corner_of_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, Point::new(0., 0., 0.));
    assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
}

#[test]
fn ray_when_camera_is_transformed() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.set_transform(rotate_y(PI / 4.0).translate(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, Point::new(0., 2., -5.));
    assert_eq!(
        r.direction,
        Vector::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
    );
}

#[test]
fn rendering_a_world_with_camera() {
    let w = World::default();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = Point::new(0., 0., -5.);
    let to = Point::new(0., 0., 0.);
    let up = Vector::new(0., 1., 0.);
    c.set_transform(view_transform(from, to, up));
    let image = c.render(w);
    assert_eq!(image[(5, 5)], Color::new(0.38066, 0.47583, 0.2855))
}
