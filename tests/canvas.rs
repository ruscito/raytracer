use raytracer::{canvas::Canvas, color::{BLACK, BLUE, Color, RED, WHITE}};

#[test]
fn creating_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn index_access() {
    let mut c = Canvas::new(10, 20);
    assert_eq!(c[(10,10)], BLACK);
    c[(10,10)] = WHITE;
    assert_eq!(c[(10,10)], Color::from((255, 255, 255)));
}

#[test]
fn writing_pixel() {
    let mut c = Canvas::new(10, 20);
    c[(0, 19)] = RED;
    assert_eq!(c[(0, 19)], RED);
}

#[test]
fn set_canvas_background() {
    let mut c = Canvas::new(10, 20);
    c.backgound(RED);
    assert_eq!(c[(0, 0)], RED);
}

#[test]
fn save_canvas() {
    let mut c = Canvas::new(400, 400);
    c.backgound(BLUE);
    c.save("my_image.png").unwrap();
    assert_eq!(c[(0, 0)], BLUE);
}