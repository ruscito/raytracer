use std::f64::consts::PI;

use raytracer::matrix::mat4::*;
use raytracer::tuple::*;

#[test]
fn translation_fetures() {
    let transform = translate(5.0, -3.0, 2.0);
    assert_eq!(
        transform * Point::new(-3.0, 4.0, 5.0),
        Point::new(2.0, 1.0, 7.0)
    );

    // muliplying by the inverse translation matrix
    let transform = translate(5.0, -3.0, 2.0);
    assert_eq!(
        transform.inv() * Point::new(-3.0, 4.0, 5.0),
        Point::new(-8.0, 7.0, 3.0)
    );

    let transform = translate(5.0, -3.0, 2.0);
    assert_eq!(
        transform * Vector::new(-3.0, 4.0, 5.0),
        Vector::new(-3.0, 4.0, 5.0)
    );
}

#[test]
fn scaling_fetures() {
    // scale applied to a point
    let s = scale(2.0, 3.0, 4.0);
    assert_eq!(s * Point::new(-4.0, 6.0, 8.0), Point::new(-8.0, 18.0, 32.0));

    // scale applied to a vector
    let s = scale(2.0, 3.0, 4.0);
    assert_eq!(
        s * Vector::new(-4.0, 6.0, 8.0),
        Vector::new(-8.0, 18.0, 32.0)
    );

    // muliplying by the inverse translation matrix
    let s = scale(2.0, 3.0, 4.0);
    assert_eq!(
        s.inv() * Point::new(-4.0, 6.0, 8.0),
        Point::new(-2.0, 2.0, 2.0)
    );

    // reflection or mirror
    let s = scale(-1.0, 1.0, 1.0);
    assert_eq!(s * Point::new(-4.0, 6.0, 8.0), Point::new(4.0, 6.0, 8.0));
}

#[test]
fn rotation_x() {
    // rotation around x axis
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotate_x(PI / 4.0);
    let full_quarter = rotate_x(PI / 2.0);
    assert_eq!(
        half_quarter * p,
        Point::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    // inverse of x rotation rotates in opposite dir
    assert_eq!(
        half_quarter.inv() * p,
        Point::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0)
    );
}

#[test]
fn rotation_y() {
    // rotation around y axis
    let p = Point::new(0.0, 0.0, 1.0);
    let half_quarter = rotate_y(PI / 4.0);
    let full_quarter = rotate_y(PI / 2.0);
    assert_eq!(
        half_quarter * p,
        Point::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
}

#[test]
fn rotation_z() {
    // rotation around z axis
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = rotate_z(PI / 4.0);
    let full_quarter = rotate_z(PI / 2.0);
    assert_eq!(
        half_quarter * p,
        Point::new(-2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0)
    );
    assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
}

#[test]
fn shearing() {
    let t = skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(5.0, 3.0, 4.0));

    let t = skew(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(6.0, 3.0, 4.0));

    let t = skew(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 5.0, 4.0));

    let t = skew(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 7.0, 4.0));

    let t = skew(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 3.0, 6.0));

    let t = skew(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(t * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 3.0, 7.0))
}
