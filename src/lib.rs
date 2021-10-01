pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrix{
    pub use mat2::Mat2 as Mat2;
    pub use mat3::Mat3 as Mat3;
    pub use mat4::Mat4 as Mat4;

    pub mod mat2;
    pub mod mat3;
    pub mod mat4;
}
pub const EPSILON: f32 = 0.00001;

pub fn f32eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn f32_to_u8(c: f32) -> u8 {
    // https://newbedev.com/converting-color-value-from-float-0-1-to-byte-0-255
    if c >= 1.0 {
        return 255;
    } else if c <= 0.0 {
        return 0;
    } else {
        (c * 256.0) as u8 
    }
}  
