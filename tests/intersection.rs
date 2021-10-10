use raytracer::{intersection::{Intersection, intersections}, shape::Shape, sphere::Sphere};

#[test]
fn create_intersection() {
    let s = Box::new(Sphere::new());
    let i = Intersection::new(3.5, s.clone());
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object.id(), s.id());
}

#[test]
fn aggregating_intrsections() {
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(1.0, s.clone());
    let i2 = Intersection::new(2.0, s.clone());
    let xs = intersections(i1, i2);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[1].t, 2.0);
}