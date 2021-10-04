use raytracer::{ray::Ray, sphere::Sphere, tuple::{point, vector}};


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
fn intersect_a_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 0.1));
    let s = Sphere::new();
    let points = s.intersect(r);
    assert_eq!(points[0], 4.6); 
    assert_eq!(points[1], 6.0);
}

#[test]
fn intersect_a_sphere_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 0.1));
    let s = Sphere::new();
    let points = s.intersect(r);
    assert_eq!(points[0], 5.0); 
    assert_eq!(points[1], 5.0);
}


#[test]
fn miss_a_the_sphere() {
    let r = Ray::new(point(0.0,2.0, -5.0), vector(0.0, 0.0, 0.1));
    let s = Sphere::new();
    let points = s.intersect(r);
    assert_eq!(points.len(), 0);
}

#[test]
fn origin_inside_sphere() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 0.1));
    let s = Sphere::new();
    let points = s.intersect(r);
    assert_eq!(points[0], -1.0); 
    assert_eq!(points[1], 1.0);
}
#[test]

fn behind_sphere() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 0.1));
    let s = Sphere::new();
    let points = s.intersect(r);
    assert_eq!(points[0], -6.0); 
    assert_eq!(points[1], -4.0);
}