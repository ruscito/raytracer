use raytracer::tuple::Tuple;


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


fn main() {
    println!("raytracer");
    let  mut projectile = Projectile { 
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut i = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(&environment, projectile);
        i = i + 1;
        println!("{} {:?}", i, projectile.position)
    }
}
