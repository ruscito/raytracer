use raytracer::{matrix::*, tuple::Tuple};



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
    assert_eq!(m1*Mat4::identiy(), m2)            
}

#[test]
fn identiy_by_tuple() {
    let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Mat4::identiy()*a, a)
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
    let m1 = Mat4::identiy();
    assert_eq!(m1.transpose(), Mat4::identiy())
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
 
#[test]
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


