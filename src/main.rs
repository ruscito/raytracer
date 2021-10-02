use raytracer::{canvas::Canvas, color::{BLACK, RED, WHITE}, matrix::mat4::identity, tuple::Tuple};


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


fn projectile(){
    let mut p = Projectile{ 
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.65,
    };
    
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    
    let mut cv= Canvas::new(950, 550);
    cv.backgound(WHITE);    
    
    while p.position.y > 0.0 {
        p  = tick(&e, p);
        cv[(p.position.y, p.position.x)] = BLACK;
    }

    cv.save("test.png").unwrap();

}

fn clock() {
    let width = 600usize;
    let height = 600usize;
    let mut canvas = Canvas::new(width, height);
    let mut p = Tuple::point(0.0, 0.0, 0.0);
    let origin = identity().translate(300.0, 300.0, 0.0) * p; 
    let mut degrees = 0.0 as f32;
    
    
    canvas[(origin.x, origin.y)] = RED;

    canvas.save("clock.png").unwrap();
}


fn main() {
    projectile();
    clock();
}
