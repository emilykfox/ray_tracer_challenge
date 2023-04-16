use ray_tracer_challenge::{Point, Vector};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Environment { gravity, wind }
    }
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    Projectile::new(position, velocity)
}

fn main() {
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize(),
    );
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    let mut num_ticks = 0;
    while projectile.position.y() > 0.0 {
        projectile = tick(environment, projectile);
        num_ticks += 1;
        println!(
            "Projectile at ({:.2}, {:.2}, {:.2}) after {} tick(s).",
            projectile.position.x(),
            projectile.position.y(),
            projectile.position.z(),
            num_ticks
        );
    }

    println!();
    println!("Projectile took {num_ticks} ticks to hit the ground!");
}
