use raytracer::color::{BLACK, Color, WHITE};
use raytracer::intersection::Intersection;
use raytracer::light::Light;
use raytracer::material::Material;
use raytracer::matrix::mat4::scale;
use raytracer::ray::Ray;
use raytracer::world::World;
use raytracer::tuple::*;

#[test]
fn creating_world() {
    let w = World::new(None, None);
    assert!(w.light.is_none());
    assert_eq!(w.objects, vec![])
}

#[test]
fn default_world() {
    let w = World::default();
    let ligth = Light::new(Point::new(-10., 10., -10.), WHITE);
    assert_eq!(w.light, Some(ligth));
    assert_eq!(w.objects[0].material().color, Color::new(0.8, 1.0, 0.6));
    assert_eq!(w.objects[0].material().diffuse, 0.7);
    assert_eq!(w.objects[0].material().specular, 0.2);
    assert_eq!(w.objects[1].transform(), scale(0.5, 0.5, 0.5));
}

#[test]
fn intersect_a_world() {
    let w = World::default();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.0));
    let xs = w.intersect(ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
    let i = Intersection::new(4.0, w.objects[0].clone_box());
    let c = w.shade_it(i.prepare_computation(r));
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
}

#[test]
fn shading_an_intersection_from_inside() {
    let mut w = World::default();
    w.light = Some(Light::new(
        Point::new(0.0, 0.25, 0.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
    let i = Intersection::new(0.5, w.objects[1].clone_box());
    let c = w.shade_it(i.prepare_computation(r));
    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498))
}

#[test]
fn color_when_ray_miss() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.), Vector::new(0., 1., 0.));
    assert_eq!(w.color_at(r), BLACK);
}

#[test]
fn color_when_ray_hit() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.), Vector::new(0., 0., 1.));
    assert_eq!(w.color_at(r), Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_with_intersection_behind_ray() {
    let w = World::default();
    let mut outher = w.objects[0].clone_box();
    outher.set_material(Material::new(None, Some(1.0), None, None, None));
    println!("Material outer {:?}", outher.material());
    let mut inner = w.objects[1].clone_box();
    inner.set_material(Material::new(None, Some(1.0), None, None, None));
    println!("Material inner {:?}", inner.material());
    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0., 0., -1.));
    assert_eq!(w.color_at(r), Color::new(0.1, 0.1, 0.1));
}