use raytracer::matrix::Mat4;


#[test]
fn creating_matrix() {
    let m = Mat4::from_buffer( 
                &[1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5,16.5]);
    assert_eq!(m[(0,3)], 13.5);
}   