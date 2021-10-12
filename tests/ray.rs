use raytracer::{matrix::Mat4, ray::Ray, tuple::{point, vector}};


#[test]
fn creating_ray(){
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn calculate_position() {
    let  r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn translating_a_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = Mat4::identity().translate(3.0, 4.0, 5.0);
    let r2 = r.transform(&m);
    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray() {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = Mat4::identity().scale(2.0, 3.0, 4.0);
    let r2 = r.transform(&m);
    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}