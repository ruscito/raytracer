use raytracer::color::{Color, WHITE};
use raytracer::light::Light;
use raytracer::matrix::Mat4;

use raytracer::tuple::Point; 
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