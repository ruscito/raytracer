use raytracer::color::Color; 
use raytracer::light::Light;
use raytracer::tuple::Point;

#[test]
fn point_light() {
    let l = Light::new(Point::new(0., 0., 0.), Color::new(1., 1., 1.));
    assert_eq!(l.position, Point::new(0., 0., 0.));
    assert_eq!(l.intensity, Color::new(1., 1., 1.));
}