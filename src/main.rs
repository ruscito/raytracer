
use std::f32::consts::PI;

use raytracer::{canvas::Canvas, color::RED, matrix::mat4::{identity, rotate_z}, tuple::{Tuple, point}};


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
    
    while p.position.y > 0.0 {
        p  = tick(&e, p);
        cv[(p.position.y, p.position.x)] = RED;
    }

    cv.save("projectile.png").unwrap();

}

fn clock() {
    // Challenge ch.4:
    // Write a program that uses a rotation matrix to cmpute the position of those hours nt the clock face,
    // and draw a pixel onto a canvas for each of them.
    let width = 600usize;
    let height = 600usize;
    let mut canvas = Canvas::new(width, height);
    let radius = width as f32 * 0.375;
    
    // translate P(0,0,0) origin to the center of the canvas
    let clock_centered_orgin = identity().translate(300.0, 300.0, 0.0) * point(0.0, 0.0, 0.0);
    
    // In case of rotation around z axis 12 o'clock is on the y axis
    let clock_at_12 = point(0.0, 1.0, 0.0);

    // 2pi radians in a circle so each hour is rotated 2PI/12 = PI/6
    // all the point ar calculated with center being at p(0,0,0)
    for i in 1..13 {
        //position = rotate_z(PI/6.0) * clock_at_12; in 
        let mut position = rotate_z(i as f32 * (PI/6.0)) * clock_at_12;

        // moltiplicate by the radius to move far from 1
        position.x = position.x * radius;
        position.y = position.y * radius;

        // move the position to the center
        position.x = position.x + clock_centered_orgin.x;
        position.y = position.y + clock_centered_orgin.y;

        canvas[(position.y, position.x)] = RED;
    }

    canvas.save("clock.png").unwrap();
}

fn move_a_point(){
    let mut cv = Canvas::new(200, 200);
    
    let start_point = point(100.0, 100.0, 0.0);
    let end_point = point(150.0, 130.0, 0.0);
    let speed = 0.01;
    
    let direction = end_point - start_point;
    let distance = start_point.distance(&end_point);
    let velocity = direction.normalize() * speed;

    let mut position = start_point;    

    println!("Start distance ={}",distance);


    while position.distance(&start_point) <= start_point.distance(&end_point) {
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
