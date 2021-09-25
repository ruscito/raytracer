use raytracer::{canvas::Canvas, color::{BLACK, BLUE, Color, RED, WHITE, YELLOW}};

#[test]
fn creating_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn index_access() {
    let mut c = Canvas::new(10, 20);
    assert_eq!(c[[10,10]], Color::from(BLACK));
    c[[10,10]] = WHITE.into();
    assert_eq!(c[[10,10]], Color::from((255, 255, 255)));
}

#[test]
fn writing_pixel() {
    let mut c = Canvas::new(10, 20);
    c[[0, 0]] = RED.into();
    assert_eq!(c[[0, 0]], RED.into());
}

#[test]
fn set_canvas_background() {
    let mut c = Canvas::new(10, 20);
    c.backgound(RED.into());
    assert_eq!(c[[0, 0]], RED.into());
}

#[test]
fn save_canvas() {
    let mut c = Canvas::new(400, 400);
    c.backgound(BLUE.into());
    c.save("my_image.png").unwrap();
    assert_eq!(c[[0, 0]], BLUE.into());
}