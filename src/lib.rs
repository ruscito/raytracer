use std::sync::atomic::{AtomicUsize, Ordering};

pub trait Tuple {}

pub mod tuple {
    pub use point::Point;
    pub use vector::Vector;

    pub mod point;
    pub mod vector;
}

pub mod canvas;
pub mod color;
pub mod matrix {
    pub use mat2::Mat2;
    pub use mat3::Mat3;
    pub use mat4::Mat4;

    pub mod mat2;
    pub mod mat3;
    pub mod mat4;
}

pub mod shape {
    pub use shape::Shape;
    pub use sphere::Sphere;
    pub mod shape;
    pub mod sphere;
}

pub mod camera;
pub mod comps;
pub mod intersection;
pub mod light;
pub mod material;
pub mod ray;
pub mod world;

pub const EPSILON: f64 = 0.00001;

pub fn f64eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

fn f64_to_u8(c: f64) -> u8 {
    // https://newbedev.com/converting-color-value-from-float-0-1-to-byte-0-255
    if c >= 1.0 {
        return 255;
    } else if c <= 0.0 {
        return 0;
    } else {
        (c * 256.0) as u8
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);

// https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805
pub fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
