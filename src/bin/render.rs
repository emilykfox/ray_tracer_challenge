use clap::Parser;
use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    canvas::Color,
    lights::PointLight,
    material::Material,
    shapes::{Plane, Shape, Sphere},
    transformations::{translation, view_transform, Builder},
    world::World,
    Point, Vector,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "image.ppm")]
    output: String,

    #[arg(long, default_value = "100")]
    width: usize,

    #[arg(long, default_value = "50")]
    height: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut floor = Shape::new(Plane);
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut middle = Shape::new(Sphere);
    middle.set_transform(translation(-0.5, 1.0, 0.5)).unwrap();
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::new(Sphere);
    right
        .set_transform(
            Builder::new()
                .scaling(0.5, 0.5, 0.5)
                .translation(1.5, 0.5, -0.5)
                .transform(),
        )
        .unwrap();
    right.material = Material::new();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::new(Sphere);
    left.set_transform(
        Builder::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75)
            .transform(),
    )
    .unwrap();
    left.material = Material::new();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.objects = vec![floor, middle, right, left];
    world.light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let mut camera = Camera::new(args.width, args.height, PI / 3.0);
    camera
        .set_transform(view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ))
        .expect("no inverse error");

    let canvas = camera.render(&world);

    std::fs::write(args.output, canvas.to_ppm())?;

    Ok(())
}
