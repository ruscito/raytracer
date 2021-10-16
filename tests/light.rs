use raytracer::{color::Color, light::Light, tuple::point};

#[test]
fn point_light() {
    let l = Light::new(point(0., 0., 0.), Color::new(1., 1., 1.));
    assert_eq!(l.position, point(0., 0., 0.));
    assert_eq!(l.intensity, Color::new(1., 1., 1.));
}