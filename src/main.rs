use raytracer::camera::Camera;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::color::GREEN;
use raytracer::color::RED;
use raytracer::color::WHITE;
use raytracer::light::Light;
use raytracer::material::Material;
use raytracer::matrix::mat4::translate;
use raytracer::matrix::mat4::view_transform;
use raytracer::matrix::mat4::{identity, rotate_z, scale};
use raytracer::matrix::Mat4;
use raytracer::ray::Ray;
use raytracer::shape::Plane;
use raytracer::shape::Shape;
use raytracer::shape::Sphere;
use raytracer::tuple::*;
use raytracer::world::World;
use std::f64::consts::PI;

//use std::time;
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
        cv[(p.position.x as usize, (550 - p.position.y as usize))] = RED;
    }

    cv.save("./render/projectile.png").unwrap();
}

fn clock() {
    // Chapter 04 challenge:
    // Write a program that uses a rotation matrix to cmpute the position of those hours nt the clock face,
    // and draw a pixel onto a canvas for each of them.
    let width = 600usize;
    let height = 600usize;
    let mut canvas = Canvas::new(width, height);
    let radius = width as f64 * 0.375;

    // translate P(0,0,0) origin to the center of the canvas
    let clock_centered_orgin = identity().translate(300.0, 300.0, 0.0) * Point::new(0.0, 0.0, 0.0);

    // In case of rotation around z axis 12 o'clock is on the y axis
    let clock_at_12 = Point::new(0.0, 1.0, 0.0);

    // 2pi radians in a circle so each hour is rotated 2PI/12 = PI/6
    // all the point ar calculated with center being at p(0,0,0)
    for i in 1..13 {
        //position = rotate_z(PI/6.0) * clock_at_12; in
        let mut position = rotate_z(i as f64 * (PI / 6.0)) * clock_at_12;

        // moltiplicate by the radius to move far from 1 (this is a unit sphere)
        position.x = position.x * radius;
        position.y = position.y * radius;

        // translate  the position to the center
        position.x = position.x + clock_centered_orgin.x;
        position.y = position.y + clock_centered_orgin.y;

        canvas[(position.x as usize, height - position.y as usize)] = RED;
    }

    canvas.save("./render/clock.png").unwrap();
}

fn move_a_point() {
    let mut cv = Canvas::new(200, 200);

    let start_point = Point::new(100.0, 100.0, 0.0);
    let end_point = Point::new(150.0, 130.0, 0.0);
    let speed = 0.01;

    let direction = end_point - start_point;
    let velocity = direction.normalize() * speed;

    let mut position = start_point;

    while position.distance(&start_point) <= start_point.distance(&end_point) {
        cv[(position.x as usize, 200usize - position.y as usize)] = RED;
        position = position + velocity;
    }
    cv.save("./render/move_a_point.png").unwrap();
}

fn raycast_2d_sphere() {
    // Chapter 05 Challenge:
    // casts rays at a sphere and draws the picture to a canvas.
    let canvas_pixels = 100 as usize;

    let wall_z = 10.0; // unit
    let wall_size = 7.0; //unit
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let t = Mat4::identity()
        .scale(1., 0.5, 1.)
        .skew(0.5, 0., 0.5, 0., 0., 0.);

    let mut s = Sphere::new(None, None); //unit sphere
    s.set_transform(t);
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    // for each row of pixels in  the canvas
    for y in 0..canvas_pixels - 1 {
        // compute the worl y coordinate (top = +half, bottom= -half)
        let world_y = half - pixel_size * y as f64;

        // for each pixel in the row
        for x in 0..canvas_pixels - 1 {
            // compute the worl x coordinate (left = -half, right= half)
            let world_x = -half + pixel_size * x as f64;

            // describe the point in the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalize();

            let ray = Ray::new(ray_origin, direction.into());

            let xs = s.intersect(ray);

            if let Some(_) = xs.hit() {
                canvas[(x, y)] = RED;
            }
        }
    }
    canvas.save("./render/2d_red_sphere.png").unwrap();
}

fn raycast_3d_sphere() {
    // Chapter 06 Challenge:
    // casts rays at a sphere and draws the picture to a canvas.
    let canvas_pixels = 1000 as usize;
    let wall_size = 7 as usize; //unit
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0; // unit
    let pixel_size = wall_size as f64 / canvas_pixels as f64;
    let half = wall_size as f64 / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut s = Sphere::new(None, None); //unit sphere
    let m = Material::new(Some(Color::new(1.0, 0.2, 1.0)), None, None, None, None);
    s.set_material(m);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    // for each row of pixels in  the canvas
    for y in 0..canvas_pixels {
        // compute the worl y coordinate (top = +half, bottom= -half)
        let world_y = half - pixel_size * (y as f64);
        // for each pixel in the row
        for x in 0..canvas_pixels {
            // compute the worl x coordinate (left = -half, right= half)
            let world_x = -half + pixel_size * (x as f64);

            // describe the point in the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = s.intersect(ray);
            if let Some(hit) = xs.hit() {
                //let start = time::Instant::now();
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;
                canvas[(x, y)] = hit
                    .object
                    .material()
                    .lighting(light, point, eye, normal, false);
                //println!("{} elpased.", start.elapsed().as_micros());
            }
        }
    }
    canvas.save("./render/3d_red_sphere.png").unwrap();
}

fn ch7() {
    // FLOOR
    let t = scale(10.0, 0.01, 10.0);
    let mtr = Material::new(Some(Color::new(1.0, 0.9, 0.9)), None, None, Some(0.0), None);
    let floor = Box::new(Sphere::new(Some(t), Some(mtr)));

    // LEFT WALL
    let mut left_wall = Box::new(Sphere::new(None, None));
    left_wall.set_transform(
        translate(0.0, 0.0, 5.0)
            .rotate_y(-PI / 4.0)
            .rotate_x(PI / 2.0)
            .scale(10.0, 0.01, 10.0),
    );
    left_wall.set_material(Material::new(
        Some(Color::new(1.0, 0.9, 0.9)),
        None,
        None,
        Some(0.0),
        None,
    ));
    // RIGHT WALL
    let mut right_wall = Box::new(Sphere::new(None, None));
    right_wall.set_transform(
        translate(0.0, 0.0, 5.0)
            .rotate_y(PI / 4.0)
            .rotate_x(PI / 2.0)
            .scale(10.0, 0.01, 10.0),
    );
    right_wall.set_material(floor.material());
    // MIDDLE SPHERE
    let mut middle = Box::new(Sphere::new(None, None));
    middle.set_transform(translate(-0.5, 1.0, 0.5));
    middle.set_material(Material::new(
        Some(Color::new(0.1, 1.0, 0.5)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // RIGHT SPHERE
    let mut right = Box::new(Sphere::new(None, None));
    right.set_transform(translate(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5));
    right.set_material(Material::new(
        Some(Color::new(0.5, 1.0, 0.1)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // LEFT SPHERE
    let mut left = Box::new(Sphere::new(None, None));
    left.set_transform(translate(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33));
    left.set_material(Material::new(
        Some(Color::new(1.0, 0.8, 0.1)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // THE WORLD
    let world = World::new(
        Some(Light::new(Point::new(-10., 10.0, -10.0), WHITE)),
        Some(vec![floor, left_wall, right_wall, middle, right, left]),
    );
    // THE CAMERA
    let mut camera = Camera::new(3000, 1500, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0., 1.5, -5.),
        Point::new(0., 1., 0.),
        Vector::new(0., 1., 0.),
    ));
    //THE CANVAS
    let canvas = camera.render(world);
    canvas.save("./render/ch7.png").unwrap();
}

fn ch9() {
    // FLOOR
    let mtr = Material::new(Some(GREEN), Some(0.2), None, Some(1.0), Some(1.0));
    let floor = Box::new(Plane::new(None, Some(mtr)));

    // MIDDLE SPHERE
    let mut middle = Box::new(Sphere::new(None, None));
    middle.set_transform(translate(-0.5, 1.0, 0.5));
    middle.set_material(Material::new(
        Some(Color::new(0.1, 1.0, 0.5)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // RIGHT SPHERE
    let mut right = Box::new(Sphere::new(None, None));
    right.set_transform(translate(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5));
    right.set_material(Material::new(
        Some(Color::new(0.5, 1.0, 0.1)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // LEFT SPHERE
    let mut left = Box::new(Sphere::new(None, None));
    left.set_transform(translate(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33));
    left.set_material(Material::new(
        Some(Color::new(1.0, 0.8, 0.1)),
        None,
        Some(0.7),
        Some(0.3),
        None,
    ));
    // THE WORLD
    let world = World::new(
        Some(Light::new(Point::new(-10., 10.0, -10.0), WHITE)),
        Some(vec![floor, middle, right, left]),
    );
    // THE CAMERA
    let mut camera = Camera::new(3000, 1500, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0., 1.5, -5.),
        Point::new(0., 1., 0.),
        Vector::new(0., 1., 0.),
    ));
    //THE CANVAS
    let canvas = camera.render(world);
    canvas.save("./render/ch9.png").unwrap();
}

fn main() {
    //move_a_point();
    //projectile();
    //clock();
    //raycast_2d_sphere();
    //raycast_3d_sphere();
    //ch7();
    //ch9();
}
