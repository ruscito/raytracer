use raytracer::{f32eq, tuple::*};

#[test]
fn tuple_is_a_point() {
    let a = Point::new(4.3, -4.2, 3.1);
    assert_eq!(a.w, 1.0);
}

#[test]
fn tuple_is_a_vector() {
    let a = Vector::new(4.3, -4.2, 3.1);
    assert_eq!(a.w, 0.0);
}

#[test]
fn create_point() {
    let a = Point::new(4.0, -4.0, 3.0);
    assert!(a.x==4.0 && a.y==-4.0 && a.z==3.0 && a.w == 1.0)
}

#[test]
fn create_vector() {
    let a = Vector::new(4.0, -4.0, 3.0);
    assert!(a.x==4.0 && a.y==-4.0 && a.z==3.0 && a.w == 0.0)
}

#[test]
fn addig_tuple() {
    let a1 = Point::new(3.0, -2.0, 5.0);
    let a2 = Vector::new(-2.0, 3.0, 1.0);
    assert_eq!(a1 + a2, Point::new(1.0, 1.0, 6.0));
}

#[test]
fn subtracting_two_point() {
    let a1 = Point::new(3.0, 2.0, 1.0);
    let a2 = Point::new(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Vector::new(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_two_vector() {
    let a1 = Vector::new(3.0, 2.0, 1.0);
    let a2 = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Vector::new(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_vector_from_point() {
    let a1 = Point::new(3.0, 2.0, 1.0);
    let a2 = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Point::new(-2.0, -4.0,-6.0));
}

#[test]
fn subtracting_vector_from_zero_vector() {
    let a1 = Vector::new(0., 0., 0.);
    let a2 = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(a1 - a2, Vector::new(-1.0, 2.0,-3.0));
}

#[test]
fn negating_tuple() {
    let a = Point::new(1.0, -2.0, 3.0);
    assert_eq!(-a, Point::new(-1.0, 2.0, -3.0));
    let a = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
}

#[test]
fn mul_tuple_by_scalar(){
    let a = Point::new(1.0, -2.0, 3.0);
    assert_eq!(a * 3.5, Point::new(3.5, -7.0, 10.5));
    assert_eq!(a * 0.5, Point::new(0.5, -1.0, 1.5));
    let a = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(a * 3.5, Vector::new(3.5, -7.0, 10.5));
    assert_eq!(a * 0.5, Vector::new(0.5, -1.0, 1.5));
}

#[test]
fn div_tuple_by_scalar(){
    let a = Point::new(1.0, -2.0, 3.0);
    assert_eq!(a / 2.0, Point::new(0.5, -1.0, 1.5,));
    let a = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(a / 2.0, Vector::new(0.5, -1.0, 1.5,));
}

#[test]
fn computing_magnitude(){
    let v = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);
    let v = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);
    let v = Vector::new(0.0, 0.0, 1.0);
    assert_eq!(v.magnitude(), 1.0);
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.magnitude(), 14f32.sqrt());
    let v = Vector::new(-1.0, -2.0, -3.0);
    assert_eq!(v.magnitude(), 14f32.sqrt());
}

#[test]
fn normalizing_vector() {
    let v = Vector::new(4.0, 0.0, 0.0);
    assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    let v = Vector::new(1.0, 2.0, 3.0).normalize();
    assert!(f32eq(v.x, 0.26726));
    assert!(f32eq(v.y, 0.53452));
    assert!(f32eq(v.z, 0.80178));
   
    let mut v = Vector::new(1.0, 2.0, 3.0);
    v.mut_normalize();
    assert!(f32eq(v.x, 0.26726));
    assert!(f32eq(v.y, 0.53452));
    assert!(f32eq(v.z, 0.80178));
}

#[test]
fn dot_product_of_tuples() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(a.dot(&b), 20.0 );
}

#[test]
fn cross_product_of_tuples() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(a.cross(&b), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(&a), Vector::new(1.0, -2.0, 1.0));
}

#[test]
fn reflecting_a_vector_at_45() {
    let v = Vector::new(1.0, -1.0, 0.0);
    let n = Vector::new(0.0, 1.0, 0.0);
    let r = v.reflect(n);
    assert_eq!(r, Vector::new(1., 1., 0.));
}


#[test]
fn reflecting_a_vector_of_slabt_surface() {
    let v = Vector::new(0.0, -1.0, 0.0);
    let n = Vector::new(2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0, 0.);
    let r = v.reflect(n);
    assert_eq!(r, Vector::new(1., 0., 0.));
}

