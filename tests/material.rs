use raytracer::{color::Color, material::Material};

#[test]
fn default_material() {
    let m = Material::new();
    assert_eq!(m.color, Color::new(1., 1., 1.));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}