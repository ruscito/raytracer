use raytracer::{color::{Color, WHITE}, light::Light, material::Material, matrix::mat4::scale, ray::Ray, shape::{Shape, Sphere}, tuple::{Point, Vector}, world::World};

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
    let mt = Material::new(Some(Color::new(0.8, 1.0, 0.6)), None, Some(0.7), Some(0.2), None);
    let s1 = Sphere::new().set_material(mt);
    let s2 = Sphere::new().set_transform(scale(0.5, 0.5, 0.5));
    assert_eq!(w.light, Some(ligth));
    assert_eq!(w.objects[0].material().color, Color::new(0.8, 1.0, 0.6));
    assert_eq!(w.objects[0].material().diffuse, 0.7);
    assert_eq!(w.objects[0].material().specular, 0.2);
}

#[test]
fn intersect_a_world(){
    let w = World::default();
    let ray = Ray::new(Point::new(0.,0.,-5.), Vector::new(0.,0.,1.0));
    let xs = w.intersect(ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);

}