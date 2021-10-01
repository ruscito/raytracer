use raytracer::{matrix::Mat2};



#[test]
fn creating_matrix() {
    let m = Mat2::from_buffer( 
                [1.0, 2.0,
                3.0, 4.0]);
    assert_eq!(m[(0,0)], 1.0);
    assert_eq!(m[(0,1)], 2.0);
    assert_eq!(m[(1,0)], 3.0);
    assert_eq!(m[(1,1)], 4.0);

}   

#[test]
fn matrix_equality() {
    let m1 = Mat2::from_buffer( 
                [1.0, 2.0, 
                3.0, 4.0]);
    
    let m2 = Mat2::from_buffer( 
                [1.0, 2.0, 
                3.0, 4.0]);

    let m3 = Mat2::from_buffer( 
                [1.1, 2.1, 
                3.1, 4.0]);
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}   

#[test]
fn multiplying_matrices() {
    let m1 = Mat2::from_buffer( 
                [1.0, 2.0,
                3.0, 4.0]);
    
    let m2 = Mat2::from_buffer( 
                [-2.0, 1.0, 
                2.0, 3.0]);

    let m3 = Mat2::from_buffer( 
                [2.0, 7.0,
                2.0, 15.0]);
    assert_eq!(m1*m2, m3);
}  

#[test]
fn matrix_by_identity() {
    let m1 = Mat2::from_buffer( 
        [0.0, 1.0, 
        2.0, 4.0]);
    let m2 = m1.clone();
    assert_eq!(m1*Mat2::identiy(), m2)            
}

#[test]
fn transpose_matrix() {
    let m1 = Mat2::from_buffer( 
        [0.0, 9.0, 
        3.0, 0.0,]);
    let m2 = Mat2::from_buffer(
        [0.0, 3.0, 9.0, 0.0,]);
    assert_eq!(m1.transpose(), m2)
}

#[test]
fn transpose_identy() {
    let m1 = Mat2::identiy();
    assert_eq!(m1.transpose(), Mat2::identiy())
}

#[test]
fn determinant(){
    let m1 = Mat2::from_buffer( 
        [1.0, 5.0, 
        -3.0, 2.0]);
    assert_eq!(m1.det(), 17.0)
}