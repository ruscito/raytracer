use raytracer::matrix::Matrix;


#[test]
fn creating_matrix() {
    let m = Matrix::from_buffer(4, 4, 
        &[1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]).unwrap();
    assert_eq!(m[[0,3]], 4.0);
}   