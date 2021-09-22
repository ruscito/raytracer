use raytracer::tuple::*;

#[test]
fn tuple_is_a_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    let b= a.clone();
    assert!(a.is_point());
    assert!(!b.is_vector());
}

#[test]
fn tuple_is_a_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    let b= a.clone();
    assert!(!a.is_point());
    assert!(b.is_vector());
}

