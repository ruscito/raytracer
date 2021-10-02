use std::f32::consts::PI;

use raytracer::{matrix::Mat4, tuple::Tuple};

#[test]
fn translation_fetures() {
    let transform = Mat4::translate(5.0, -3.0, 2.0);
    assert_eq!(transform * Tuple::point(-3.0, 4.0, 5.0),  Tuple::point(2.0, 1.0, 7.0));
    
    // muliplying by the inverse translation matrix
    let transform = Mat4::translate(5.0, -3.0, 2.0);
    assert_eq!(transform.inv() * Tuple::point(-3.0, 4.0, 5.0),  Tuple::point(-8.0, 7.0, 3.0));

    let transform = Mat4::translate(5.0, -3.0, 2.0);
    assert_eq!(transform * Tuple::vector(-3.0, 4.0, 5.0),  Tuple::vector(-3.0, 4.0, 5.0));
}

#[test]
fn scaling_fetures() {
    // scale applied to a point
    let scale = Mat4::scale(2.0, 3.0, 4.0);
    assert_eq!(scale * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(-8.0, 18.0, 32.0));
    
    // scale applied to a vector
    let scale = Mat4::scale(2.0, 3.0, 4.0);
    assert_eq!(scale * Tuple::vector(-4.0, 6.0, 8.0),  Tuple::vector(-8.0, 18.0, 32.0));

    // muliplying by the inverse translation matrix
    let scale = Mat4::scale(2.0, 3.0, 4.0);
    assert_eq!(scale.inv() * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(-2.0, 2.0, 2.0));

    // reflection or mirror
    let scale = Mat4::scale(-1.0, 1.0, 1.0);
    assert_eq!(scale * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(4.0, 6.0, 8.0));
}

#[test]
fn rotation_x() {
    // rotation around x axis
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::rotate_x(PI/4.0);
    let full_quarter = Mat4::rotate_x(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(0.0, 2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0));
    assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    // inverse of x rotation rotates in opposite dir
    assert_eq!(half_quarter.inv() * p, Tuple::point(0.0, 2.0f32.sqrt()/2.0, -2.0f32.sqrt()/2.0));
}

#[test]
fn rotation_y() {
    // rotation around y axis
    let p = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = Mat4::rotate_y(PI/4.0);
    let full_quarter = Mat4::rotate_y(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(2.0f32.sqrt()/2.0, 0.0, 2.0f32.sqrt()/2.0));
    assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
}

#[test]
fn rotation_z() {
    // rotation around z axis
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::rotate_z(PI/4.0);
    let full_quarter = Mat4::rotate_z(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(-2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0, 0.0));
    assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
}

#[test]
fn shearing() {
    let t = Mat4::skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(5.0, 3.0, 4.0));

    let t = Mat4::skew(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(6.0, 3.0, 4.0));

    let t = Mat4::skew(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 5.0, 4.0));

    let t = Mat4::skew(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 7.0, 4.0));

    let t = Mat4::skew(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 3.0, 6.0));

    let t = Mat4::skew(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 3.0, 7.0))
}