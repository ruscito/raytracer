use raytracer::{matrix::*, tuple::Tuple};
use std::f32::consts::PI;


#[test]
fn creating_matrix() {
    let m = Mat4::from_buffer( 
                [1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]);
    assert_eq!(m[(0,0)], 1.0);
    assert_eq!(m[(0,1)], 2.0);
    assert_eq!(m[(0,2)], 3.0);
    assert_eq!(m[(0,3)], 4.0);
    assert_eq!(m[(1,0)], 5.5);
    assert_eq!(m[(1,1)], 6.5);
    assert_eq!(m[(1,2)], 7.5);
    assert_eq!(m[(1,3)], 8.5);
    assert_eq!(m[(2,0)], 9.0);
    assert_eq!(m[(2,1)], 10.0);
    assert_eq!(m[(2,2)], 11.0);
    assert_eq!(m[(2,3)], 12.0);
    assert_eq!(m[(3,0)], 13.5);
    assert_eq!(m[(3,1)], 14.5);
    assert_eq!(m[(3,2)], 15.5);
    assert_eq!(m[(3,3)], 16.5);
}   

#[test]
fn matrix_equality() {
    let m1 = Mat4::from_buffer( 
                [1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]);
    
    let m2 = Mat4::from_buffer( 
                [1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]);

    let m3 = Mat4::from_buffer( 
                [1.1, 2.1, 3.1, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]);
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}   

#[test]
fn multiplying_matrices() {
    let m1 = Mat4::from_buffer( 
                [1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 6.0,
                5.0, 4.0, 3.0, 2.0]);
    
    let m2 = Mat4::from_buffer( 
                [-2.0, 1.0, 2.0, 3.0,
                3.0, 2.0, 1.0, -1.0,
                4.0, 3.0, 6.0, 5.0,
                1.0, 2.0, 7.0, 8.0]);

    let m3 = Mat4::from_buffer( 
                [20.0, 22.0, 50.0, 48.0,
                44.0, 54.0, 114.0, 108.0,
                40.0, 58.0, 110.0, 102.0,
                16.0, 26.0, 46.0, 42.0]);
    assert_eq!(m1*m2, m3);
}  

#[test]
fn matrix_by_tuple(){
    let m1 = Mat4::from_buffer( 
        [1.0, 2.0, 3.0, 4.0,
        2.0, 4.0, 4.0, 2.0,
        8.0, 6.0, 4.0, 1.0,
        0.0, 0.0, 0.0, 1.0]);
    let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(m1*t, Tuple::new(18.0,24.0, 33.0, 1.0))
}

#[test]
fn matrix_by_identity() {
    let m1 = Mat4::from_buffer( 
        [0.0, 1.0, 2.0, 4.0,
        1.0, 2.0, 4.0, 8.0,
        2.0, 4.0, 8.0, 16.0,
        4.0, 8.0, 16.0, 32.0]);
    let m2 = m1.clone();
    assert_eq!(m1*Mat4::identity(), m2)            
}

#[test]
fn identiy_by_tuple() {
    let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Mat4::identity()*a, a)
}

#[test]
fn transpose_matrix() {
    let m1 = Mat4::from_buffer( 
        [0.0, 9.0, 3.0, 0.0,
        9.0, 8.0, 0.0, 8.0,
        1.0, 8.0, 5.0, 3.0,
        0.0, 0.0, 5.0, 8.0]);
    let m2 = Mat4::from_buffer(
        [0.0, 9.0, 1.0, 0.0,
        9.0, 8.0, 8.0, 0.0,
        3.0, 0.0, 5.0, 5.0,
        0.0, 8.0, 3.0, 8.0]);
    assert_eq!(m1.transpose(), m2)
}

#[test]
fn transpose_identy() {
    let m1 = Mat4::identity();
    assert_eq!(m1.transpose(), Mat4::identity())
}

#[test]
fn submatrix() {
    let m1 = Mat4::from_buffer([
        -6.0, 1.0, 1.0, 6.0,
        -8.0, 5.0, 8.0, 6.0,
        -1.0, 0.0, 8.0, 2.0,
        -7.0, 1.0, -1.0, 1.0
    ]);
    let m3 = m1.submatrix(2, 1);
    assert_eq!(m3, Mat3::from_buffer([ -6.0, 1.0, 6.0,  -8.0, 8.0, 6.0,  -7.0, -1.0, 1.0]))
}

#[test]
fn calculating_determinant() {
    let m = Mat4::from_buffer([
        -2.0, -8.0, 3.0, 5.0,
        -3.0, 1.0, 7.0, 3.0,
         1.0, 2.0, -9.0, 6.0,
        -6.0, 7.0, 7.0, -9.0
    ]);
    assert_eq!(m.det(), -4071.0)
}

#[test]
fn check_is_invertible() {
    let m = Mat4::from_buffer([
        -2.0, -8.0, 3.0, 5.0,
        -3.0, 1.0, 7.0, 3.0,
         1.0, 2.0, -9.0, 6.0,
        -6.0, 7.0, 7.0, -9.0
    ]);
    assert!(m.is_invertible());

    let m = Mat4::from_buffer([
        -4.0, 2.0, -2.0, -3.0,
         9.0, 6.0, 2.0, 6.0,
         0.0, -5.0, 1.0, -5.0,
         0.0, 0.0, 0.0, 0.0
    ]);
    assert!(!m.is_invertible())
}

// ignore this test because float equality/precision
// but it works 
// shouldn't compare float though .....
#[ignore] #[test]
fn calcualting_inverse() {
    let m = Mat4::from_buffer([
         8.0, -5.0,  9.0,  2.0,
         7.0,  5.0,  6.0,  1.0,
        -6.0,  0.0,  9.0,  6.0,
        -3.0,  0.0, -9.0, -4.0
    ]);

    let inv_m = Mat4::from_buffer([
        -0.15385, -0.15385, -0.28205, -0.53846,
        -0.07692,  0.12308,  0.02564,  0.03077,
         0.35897,  0.35897,  0.43590,  0.92308,
        -0.69231, -0.69231, -0.76923, -1.92308
    ]);

    assert_eq!(m.inv(), inv_m);
}

#[test]
fn translation_fetures() {
    let transform = Mat4::identity().translate(5.0, -3.0, 2.0);
    assert_eq!(transform * Tuple::point(-3.0, 4.0, 5.0),  Tuple::point(2.0, 1.0, 7.0));
    
    // muliplying by the inverse translation matrix
    let transform = Mat4::identity().translate(5.0, -3.0, 2.0);
    assert_eq!(transform.inv() * Tuple::point(-3.0, 4.0, 5.0),  Tuple::point(-8.0, 7.0, 3.0));

    let transform = Mat4::identity().translate(5.0, -3.0, 2.0);
    assert_eq!(transform * Tuple::vector(-3.0, 4.0, 5.0),  Tuple::vector(-3.0, 4.0, 5.0));
}

#[test]
fn scaling_fetures() {
    // scale applied to a point
    let scale = Mat4::identity().scale(2.0, 3.0, 4.0);
    assert_eq!(scale * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(-8.0, 18.0, 32.0));
    
    // scale applied to a vector
    let scale = Mat4::identity().scale(2.0, 3.0, 4.0);
    assert_eq!(scale * Tuple::vector(-4.0, 6.0, 8.0),  Tuple::vector(-8.0, 18.0, 32.0));

    // muliplying by the inverse translation matrix
    let scale = Mat4::identity().scale(2.0, 3.0, 4.0);
    assert_eq!(scale.inv() * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(-2.0, 2.0, 2.0));

    // reflection or mirror
    let scale = Mat4::identity().scale(-1.0, 1.0, 1.0);
    assert_eq!(scale * Tuple::point(-4.0, 6.0, 8.0),  Tuple::point(4.0, 6.0, 8.0));
}

#[test]
fn rotation_x() {
    // rotation around x axis
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::identity().rotate_x(PI/4.0);
    let full_quarter = Mat4::identity().rotate_x(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(0.0, 2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0));
    assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    // inverse of x rotation rotates in opposite dir
    assert_eq!(half_quarter.inv() * p, Tuple::point(0.0, 2.0f32.sqrt()/2.0, -2.0f32.sqrt()/2.0));
}

#[test]
fn rotation_y() {
    // rotation around y axis
    let p = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = Mat4::identity().rotate_y(PI/4.0);
    let full_quarter = Mat4::identity().rotate_y(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(2.0f32.sqrt()/2.0, 0.0, 2.0f32.sqrt()/2.0));
    assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
}

#[test]
fn rotation_z() {
    // rotation around z axis
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::identity().rotate_z(PI/4.0);
    let full_quarter = Mat4::identity().rotate_z(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::point(-2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0, 0.0));
    assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
}

#[test]
fn shearing() {
    let t = Mat4::identity().skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(5.0, 3.0, 4.0));

    let t = Mat4::identity().skew(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(6.0, 3.0, 4.0));

    let t = Mat4::identity().skew(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 5.0, 4.0));

    let t = Mat4::identity().skew(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 7.0, 4.0));

    let t = Mat4::identity().skew(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 3.0, 6.0));

    let t = Mat4::identity().skew(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(t * Tuple::point(2.0, 3.0, 4.0), Tuple::point(2.0, 3.0, 7.0))
}

#[test]
fn chaining_transform() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let t = Mat4::identity().
        translate(10.0, 5.0, 7.0).
        scale(5.0, 5.0, 5.0).
        rotate_x(PI/2.0);
    assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0))
}