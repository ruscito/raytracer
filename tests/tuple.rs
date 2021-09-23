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

#[test]
fn create_point() {
    let a = Tuple::point(4.0, -4.0, 3.0);
    assert!(a.x==4.0 && a.y==-4.0 && a.z==3.0 && a.w == 1.0)
}

#[test]
fn create_vector() {
    let a = Tuple::vector(4.0, -4.0, 3.0);
    assert!(a.x==4.0 && a.y==-4.0 && a.z==3.0 && a.w == 0.0)
}

#[test]
fn addig_tuple() {
    let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn subtracting_two_point() {
    let a1 = Tuple::point(3.0, 2.0, 1.0);
    let a2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_two_vector() {
    let a1 = Tuple::vector(3.0, 2.0, 1.0);
    let a2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_vector_from_point() {
    let a1 = Tuple::point(3.0, 2.0, 1.0);
    let a2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::point(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_vector_from_zero_vector() {
    let a1 = Tuple::zero_vector();
    let a2 = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(a1 - a2, Tuple::vector(-1.0, 2.0,-3.0));
}

#[test]
fn negating_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0))
}

#[test]
fn mul_tuple_by_scalar(){
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0))
}

#[test]
fn div_tuple_by_scalar(){
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}