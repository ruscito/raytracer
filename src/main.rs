use raytracer::{canvas::Canvas, color::{BLUE, YELLOW}, matrix::Mat4, tuple::Tuple};


#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}


fn demo(){
    let mut p = Projectile{ 
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.65,
    };
    
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    
    let mut cv= Canvas::new(950, 550);
    cv.backgound(YELLOW);    
    
    while p.position.y > 0.0 {
        p  = tick(&e, p);
        cv[(p.position.x, p.position.y)] = BLUE;
    }

    cv.save("test.png").unwrap();

}
fn main() {
    demo();
    let mut m=Mat4::new();
    m[(0,3)] = 4.3;
}
