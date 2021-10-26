use raytracer::color::Color;
use raytracer::light::Light;
use raytracer::material::Material;
use raytracer::tuple::*;

#[test]
fn default_material() {
    let m = Material::default();
    assert_eq!(m.color, Color::new(1., 1., 1.));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}

#[test]
fn lighting_eye_between_light_and_surface() {
    let m = Material::default();
    let position = Point::new(0., 0., 0.);
    let eyev = Vector::new(0., 0., -1.);
    let normalv = Vector::new(0., 0., -1.);
    let light = Light::new(Point::new(0., 0., -10.), Color::new(1., 1.,1.));
    let result = m.lighting(light, position, eyev, normalv);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_eye_between_light_and_surface_eye_45ofset() {
    let m = Material::default();
    let position = Point::new(0., 0., 0.);
    let eyev = Vector::new(0., 2f32.sqrt()/2.0, 2f32.sqrt()/2.0);
    let normalv = Vector::new(0., 0., -1.);
    let light = Light::new(Point::new(0., 0., -10.), Color::new(1., 1.,1.));
    let result = m.lighting(light, position, eyev, normalv);
    assert_eq!(result, Color::new(1., 1., 1.));
}

#[test]
fn lighting_eye_opposite_surface_light_45ofset() {
    let m = Material::default();
    let position = Point::new(0., 0., 0.);
    let eyev = Vector::new(0., 0., -1.);
    let normalv = Vector::new(0., 0., -1.);
    let light = Light::new(Point::new(0., 10., -10.), Color::new(1., 1.,1.));
    let result = m.lighting(light, position, eyev, normalv);
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_eye_in_path_reflectio_vector() {
    let m = Material::default();
    let position = Point::new(0., 0., 0.);
    let eyev = Vector::new(0., -2f32.sqrt()/2.0, -2f32.sqrt()/2.0);
    let normalv = Vector::new(0., 0., -1.);
    let light = Light::new(Point::new(0., 10., -10.), Color::new(1., 1.,1.));
    let result = m.lighting(light, position, eyev, normalv);
    assert_eq!(result, Color::new(1.6363853, 1.6363853, 1.6363853));
}

#[test]
fn lighting_light_behind_surface() {
    let m = Material::default();
    let position = Point::new(0., 0., 0.);
    let eyev = Vector::new(0., 0., -1.);
    let normalv = Vector::new(0., 0., -1.);
    let light = Light::new(Point::new(0., 0., 10.), Color::new(1., 1.,1.));
    let result = m.lighting(light, position, eyev, normalv);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}