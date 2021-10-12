use raytracer::{matrix::{Mat4, mat4::*}, ray::Ray, shape::Shape, sphere::Sphere, tuple::{point, vector}};

#[test]
fn ray_intersect() { 
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
    assert_eq!(xs[0].object.id(), s.id());
    assert_eq!(xs[1].object.id(), s.id());
}

#[test]
fn ray_tangent() { 
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_misses_sphere() { 
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);   
}

#[test]
fn ray_inside_sphere() { 
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);    
}

#[test]
fn sphere_behind_ray() { 
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn equal_spheres() {
    let s1 = Sphere::new();
    let s2 = &s1;
    assert_eq!(&s1, s2);
}

#[test]
fn sphere_default_transform() {
    let s = Sphere::new();
    assert_eq!(s.get_transform(), Mat4::identity())
}

#[test]
fn changing_sphere_transform() {
    let mut s = Sphere::new();
    s.set_transform(translate(2.0, 3.0, 4.0));
    assert_eq!(s.get_transform(), translate(2.0, 3.0, 4.0))
}

#[test]
fn intersecting_scaled_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.set_transform(scale(2.0, 2.0, 2.0));
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn intersecting_traslated_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.set_transform(translate(5.0, 0.0, 0.0));
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);
}