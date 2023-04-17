use std::{fs::File, io::Write};

use ray_tracer_challenge::{
    canvas::{Canvas, Color},
    Point, Vector,
};

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

fn main() -> std::io::Result<()> {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile::new(start, velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550);

    let mut num_ticks = 0;
    _ = canvas.write_pixel(
        projectile.position.x() as usize,
        549 - projectile.position.y() as usize,
        Color::new(0.75, 0.3, 0.0),
    );
    while projectile.position.y() > 0.0 {
        projectile = tick(environment, projectile);
        num_ticks += 1;
        _ = canvas.write_pixel(
            projectile.position.x() as usize,
            549 - projectile.position.y() as usize,
            Color::new(0.75, 0.3, 0.0),
        );
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

    let output_path = std::env::args().nth(1);
    if let Some(output_path) = output_path {
        let mut file = File::create(output_path)?;
        file.write_all(canvas.to_ppm().as_bytes())?;
    }

    Ok(())
}
