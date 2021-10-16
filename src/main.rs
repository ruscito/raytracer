use std::{f32::consts::PI};

use raytracer::canvas::Canvas; 
use raytracer::color::RED; 
use raytracer::matrix::{Mat4, mat4::{ identity, rotate_z}}; 
use raytracer::ray::Ray;
use raytracer:: shape::Shape; 
use raytracer::shape::Sphere;
use raytracer:: tuple::{Point, Vector};

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn projectile() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.65,
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut cv = Canvas::new(950, 550);

    while p.position.y > 0.0 {
        p = tick(&e, p);
        cv[(p.position.y, p.position.x)] = RED;
    }

    cv.save("projectile.png").unwrap();
}

fn clock() {
    // Chapter 04 challenge:
    // Write a program that uses a rotation matrix to cmpute the position of those hours nt the clock face,
    // and draw a pixel onto a canvas for each of them.
    let width = 600usize;
    let height = 600usize;
    let mut canvas = Canvas::new(width, height);
    let radius = width as f32 * 0.375;

    // translate P(0,0,0) origin to the center of the canvas
    let clock_centered_orgin = identity().translate(300.0, 300.0, 0.0) * Point::new(0.0, 0.0, 0.0);

    // In case of rotation around z axis 12 o'clock is on the y axis
    let clock_at_12 = Point::new(0.0, 1.0, 0.0);

    // 2pi radians in a circle so each hour is rotated 2PI/12 = PI/6
    // all the point ar calculated with center being at p(0,0,0)
    for i in 1..13 {
        //position = rotate_z(PI/6.0) * clock_at_12; in
        let mut position = rotate_z(i as f32 * (PI / 6.0)) * clock_at_12;

        // moltiplicate by the radius to move far from 1 (this is a unit sphere)
        position.x = position.x * radius;
        position.y = position.y * radius;

        // translate  the position to the center
        position.x = position.x + clock_centered_orgin.x;
        position.y = position.y + clock_centered_orgin.y;

        canvas[(position.y, position.x)] = RED;
    }

    canvas.save("clock.png").unwrap();
}

fn move_a_point() {
    let mut cv = Canvas::new(200, 200);

    let start_point = Point::new(100.0, 100.0, 0.0);
    let end_point = Point::new(150.0, 130.0, 0.0);
    let speed = 0.01;

    let direction = end_point - start_point;
    let distance = start_point.distance(&end_point);
    let velocity = direction.normalize() * speed;

    let mut position = start_point;

    println!("Start distance ={}", distance);

    while position.distance(&start_point) <= start_point.distance(&end_point) {
        cv[(position.y, position.x)] = RED;
        position = position + velocity;
    }
    cv.save("move_a_point.png").unwrap();
}

fn raycast_2d_sphere() {
    // Chapter 05 Challenge:
    // casts rays at a sphere and draws the picture to a canvas.
    let canvas_pixels = 100 as usize;
    
    let wall_z = 10.0; // unit
    let wall_size = 7.0; //unit
    let pixel_size = wall_size  / canvas_pixels as f32;
    let half = wall_size / 2.0;
    
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let t = Mat4::identity().scale(1., 0.5, 1.).skew(0.5, 0., 0.5, 0., 0., 0.);

    let mut s = Sphere::new(); //unit sphere
    s.transform = t;
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    // for each row of pixels in  the canvas
    for y in 0..canvas_pixels -1 {
        // compute the worl y coordinate (top = +half, bottom= -half)
        let world_y = half - pixel_size * y as f32;
        
        // for each pixel in the row 
        for x in 0..canvas_pixels - 1 {
            // compute the worl x coordinate (left = -half, right= half)
            let world_x = -half + pixel_size * x as f32;

            // describe the point in the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalize();

            let ray = Ray::new(ray_origin, direction.into());
            
            let xs = s.intersect(ray);
            
            if let Some(_) = xs.hit()  {
                canvas[(x, y)] = RED;
            }
        }
    }
    canvas.save("2d_red_sphere.png").unwrap();


}

fn main() {
    move_a_point();
    projectile();
    clock();
    raycast_2d_sphere();
}
