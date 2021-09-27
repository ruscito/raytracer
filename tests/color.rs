use raytracer::color::Color;

#[test]
fn create_color() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(c.r, -0.5);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 1.7);
}

#[test]
fn add_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn sub_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
}

#[test]
fn mul_scalar() {
    assert_eq!(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
}

#[test]
fn mul_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
}

#[test]
fn color_to_byte() {
    let c1 = Color::new(1.1, -0.1, 0.5);
    let t: (u8, u8, u8) = c1.to_bytes() ;
    println!("{:?}", c1);
    assert_eq!(t, (255, 0, 128));
}

#[test]
fn from_tuple_to_color() {
    let c1 = Color::from((255,0,128));
    let c2: Color = (255,0,128).into();
    assert_eq!(c1, Color::new(1.0, 0.0, 0.5));
    assert_eq!(c2, Color::new(1.0, 0.0, 0.5));
}