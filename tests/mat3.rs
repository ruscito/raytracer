use raytracer::{matrix::{Mat3, Mat2}};



#[test]
fn creating_matrix() {
    let m = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0]);
    assert_eq!(m[(0,0)], 1.0);
    assert_eq!(m[(0,1)], 2.0);
    assert_eq!(m[(0,2)], 3.0);
    assert_eq!(m[(1,0)], 4.0);
    assert_eq!(m[(1,1)], 5.0);
    assert_eq!(m[(1,2)], 6.0);
    assert_eq!(m[(2,0)], 7.0);
    assert_eq!(m[(2,1)], 8.0);
    assert_eq!(m[(2,2)], 9.0);

}   

#[test]
fn matrix_equality() {
    let m1 = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0]);
    
    let m2 = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0]);

    let m3 = Mat3::from_buffer( 
                [1.1, 2.0, 3.0, 
                4.0, 7.0, 6.0,
                7.0, 8.0, 2.0]);
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}   

#[test]
fn multiplying_matrices() {
    let m1 = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                1.0, 2.0, 3.0]);
    let m2 = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                1.0, 2.0, 3.0]);

    let m3 = Mat3::from_buffer( 
                [12.0, 18.0, 24.0, 
                30.0, 45.0, 60.0,
                12.0, 18.0, 24.0]);
    assert_eq!(m1*m2, m3);
}  

#[test]
fn matrix_by_identity() {
    let m1 = Mat3::from_buffer( 
                [1.0, 2.0, 3.0, 
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0]);
    let m2 = m1.clone();
    assert_eq!(m1*Mat3::identiy(), m2)            
}

#[test]
fn transpose_matrix() {
    let m1 = Mat3::from_buffer( 
        [1.0, 2.0, 3.0, 
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0]);
    let m2 = Mat3::from_buffer(
        [1.0, 4.0, 7.0, 
        2.0, 5.0, 8.0,
        3.0, 6.0, 9.0]);
    assert_eq!(m1.transpose(), m2)
}

#[test]
fn transpose_identy() {
    let m1 = Mat3::identiy();
    assert_eq!(m1.transpose(), Mat3::identiy())
}

#[test]
fn submatrix() {
    let m3 = Mat3::from_buffer([
        1.0, 5.0, 0.0,
        -3.0, 2.0, 7.0,
        0.0, 6.0, -3.0
    ]);
    let m2 = m3.submatrix(0, 2);
    assert_eq!(m2, Mat2::from_buffer([ -3.0, 2.0, 0.0, 6.0]))
}

#[test]
fn calculating_minor(){
    let m3 = Mat3::from_buffer([
        3.0, 5.0, 0.0,
        2.0, -1.0, -7.0,
        6.0, -1.0, 5.0]);
    assert_eq!(m3.minor(1, 0), 25.0)
}

#[test]
fn calculating_cofactor(){
    let m3 = Mat3::from_buffer([
        3.0, 5.0, 0.0,
        2.0, -1.0, -7.0,
        6.0, -1.0, 5.0]);
    assert_eq!(m3.minor(1, 0), 25.0);
    assert_eq!(m3.cofactor(1, 0), -25.0);
    assert_eq!(m3.minor(0, 0), -12.0);
    assert_eq!(m3.cofactor(0, 0), -12.0)
}

#[test]
fn calculating_determinant(){
    let m3 = Mat3::from_buffer([
        1.0, 2.0, 6.0,
        -5.0, 8.0, -4.0,
        2.0, 6.0, 4.0]);
    assert_eq!(m3.det(), -196.0)
}