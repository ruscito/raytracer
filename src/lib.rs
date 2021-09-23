pub mod tuple;

pub fn f32eq(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}
