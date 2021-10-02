
use raytracer::{canvas::Canvas, color::{BLACK, RED, WHITE}, matrix::mat4::identity, tuple::{Tuple, point}};


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

    cv.save("projectile.png").unwrap();

}

fn clock() {
    let width = 600usize;
    let height = 600usize;
    let mut canvas = Canvas::new(width, height);
    let p = Tuple::point(0.0, 0.0, 0.0);
    let origin = identity().translate(300.0, 300.0, 0.0) * p; 
    //let mut degrees = 0.0 as f32;
    
    
    canvas[(origin.y, origin.x)] = RED;
    let origin = identity().translate(150.0, 0.0, 0.0) * origin; 
    canvas[(origin.y, origin.x)] = RED;

    canvas.save("clock.png").unwrap();
}

fn move_a_point(){
    let mut cv = Canvas::new(200, 200);
    
    let start_point = point(100.0, 100.0, 0.0);
    let end_point = point(150.0, 130.0, 0.0);
    let speed = 0.50;
    
    let direction = end_point - start_point;
    let distance = start_point.distance(&end_point);
    let velocity = direction.normalize() * speed;

    let mut position = start_point;    

    println!("Start distance ={}",distance);


    while position.distance(&end_point) > 1.0 {
        //println!("distance ={}",position.distance(&end_point));
        //println!("main row:{} - col:{}", position.y, position.x);
        cv[(position.y, position.x)]= RED;
        position = position + velocity;
    }
    
    
    




    cv.save("move_a_point.png").unwrap();
}


fn main() {
    move_a_point();
    projectile();
    clock();
}
