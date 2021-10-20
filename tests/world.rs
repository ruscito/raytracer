

use raytracer::color::{Color, WHITE};
use raytracer::intersection::Intersection;
use raytracer::light::Light;
use raytracer::matrix::Mat4;

use raytracer::ray::Ray;
use raytracer::tuple::*; 
use raytracer::world::World;

#[test]
fn creating_a_world() {
    let w = World::new(None, vec![]);
   assert!(w.light.is_none());
   assert_eq!(w.shapes.len(), 0);
}

#[test]
fn default_world() {
    let w = World::default();


    assert_eq!(w.light, Some(Light::new(Point::new(-10.0, 10.0, -10.0), WHITE)));

    assert_eq!(w.shapes[0].material().color, Color::new(0.8, 1.0, 0.6));
    assert_eq!(w.shapes[0].material().diffuse, 0.7);
    assert_eq!(w.shapes[0].material().specular, 0.2);
    assert_eq!(w.shapes[1].transform(), Mat4::identity().scale(0.5, 0.5, 0.5));
}

#[test]
fn interesct_a_world() {
    let w = World::default();
    let r = Ray::new(Point::new(0., 0., -5.0), Vector::new(0., 0., 1.0));
    let xs = w.intersect(r);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let ray = Ray::new(Point::new(0., 0., -5.0), Vector::new(0., 0., 1.0));
    let shape = w.shapes[0].clone_box();
    let i = Intersection::new(4.0, shape);
    let comps = i.prepare_computations(ray);
    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn shading_an_intersection_from_inside() {
    let mut w = World::default();
    w.light = Some(Light::new(Point::new(0., 0.25, 0.0), WHITE));
    let ray = Ray::new(Point::new(0., 0.0, 0.0), Vector::new(0., 0., 1.0));
    let shape = w.shapes[1].clone_box();
    let i = Intersection::new(0.5, shape);
    let comps = i.prepare_computations(ray);
    let c = w.shade_hit(comps);
    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}