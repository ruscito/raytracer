use std::f32::consts::PI;

use raytracer::material::Material;
use raytracer::matrix::{Mat4, mat4::{*, self}};
use raytracer::ray::Ray;
use raytracer::shape::Shape;
use raytracer::shape::Sphere;
use raytracer::tuple::{Point, Vector};

#[test]
fn ray_intersect() { 
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
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
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_misses_sphere() { 
    let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);   
}

#[test]
fn ray_inside_sphere() { 
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);    
}

#[test]
fn sphere_behind_ray() { 
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
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
    assert_eq!(s.transform(), Mat4::identity())
}

#[test]
fn changing_sphere_transform() {
    let mut s = Sphere::new();
    s.set_transform(translate(2.0, 3.0, 4.0));
    assert_eq!(s.transform(), translate(2.0, 3.0, 4.0))
}

#[test]
fn intersecting_scaled_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.set_transform(scale(2.0, 2.0, 2.0));
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn intersecting_traslated_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.set_transform(translate(5.0, 0.0, 0.0));
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn normal_at_point_on_x() {
    let s= Sphere::new();
    let n = s.normal_at(Point::new(1.,0.,0.));
    assert_eq!(n, Vector::new(1., 0., 0.))
}

#[test]
fn normal_at_point_on_y() {
    let s= Sphere::new();
    let n = s.normal_at(Point::new(0.,1.,0.));
    assert_eq!(n, Vector::new(0., 1., 0.))
}

#[test]
fn normal_at_point_on_z() {
    let s= Sphere::new();
    let n = s.normal_at(Point::new(0.,0.,1.));
    assert_eq!(n, Vector::new(0., 0., 1.))
}

#[test]
fn normal_at_nonaxial_point() {
    let s= Sphere::new();
    let n = s.normal_at(Point::new(3.0f32.sqrt()/3.0,3.0f32.sqrt()/3.0,3.0f32.sqrt()/3.0));
    assert_eq!(n, Vector::new(3.0f32.sqrt()/3.0, 3.0f32.sqrt()/3.0, 3.0f32.sqrt()/3.0))
}

#[test]
fn normal_is_normalized_vector() {
    let s= Sphere::new();
    let n = s.normal_at(Point::new(3.0f32.sqrt()/3.0,3.0f32.sqrt()/3.0,3.0f32.sqrt()/3.0));
    assert_eq!(n, n.normalize());
}

#[test]
fn normal_on_a_traslate_sphere() {
    let mut s = Sphere::new();
    s.set_transform(mat4::translate(0., 1., 0.));
    let n = s.normal_at(Point::new(0., 1.70711, -0.70711));
    assert_eq!(n, Vector::new(0., 0.7071068, -0.70710677))
}

#[test]
fn normal_on_a_transformed_sphere() {
    let mut s = Sphere::new();
    let m = Mat4::identity().scale(1., 0.5, 1.).rotate_z(PI/5.0);
    s.set_transform(m);
    let n = s.normal_at(Point::new(0., 2.0f32.sqrt()/2.0, -2.0f32.sqrt()/2.0));
    assert_eq!(n, Vector::new(0., 0.97014, -0.24254))
}

#[test]
fn sphere_default_material() {
    let s = Sphere::new();
    let m = Material::new();
    assert_eq!(s.material, m);
}

#[test]
fn sphere_assign_material() {
    let mut s = Sphere::new();
    let mut m = Material::new();
    m.ambient = 1.0;
    s.material = m;
    assert_eq!(s.material, m);
}
