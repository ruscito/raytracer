
use raytracer::ray::Ray;
use raytracer::shape::Shape;
use raytracer::shape::Plane;
use raytracer::tuple::{Point, Vector};

#[test]
fn intersect_ray_parallel_to_the_plane() { 
    let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Plane::new(None, None);
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_with_coplanar_ray() { 
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Plane::new(None, None);
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_intersect_plane_from_above() { 
    let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
    let s = Plane::new(None, None);
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object.id(), s.id);
}
#[test]
fn ray_intersect_plane_from_below() { 
    let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let s = Plane::new(None, None);
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[0].object.id(), s.id);
}

