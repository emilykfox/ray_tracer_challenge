use clap::Parser;
use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    canvas::Color,
    lights::PointLight,
    material::Material,
    spheres::Sphere,
    transformations::{scaling, translation, view_transform, Builder},
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

    let mut floor = Sphere::new();
    floor.set_transform(scaling(10.0, 0.01, 10.0)).unwrap();
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall
        .set_transform(
            Builder::new()
                .scaling(10.0, 0.01, 10.0)
                .rotation_x(PI / 2.0)
                .rotation_y(-PI / 4.0)
                .translation(0.0, 0.0, 5.0)
                .transform(),
        )
        .unwrap();
    left_wall.material = floor.material.clone();

    let mut right_wall = Sphere::new();
    right_wall
        .set_transform(
            Builder::new()
                .scaling(10.0, 0.01, 10.0)
                .rotation_x(PI / 2.0)
                .rotation_y(PI / 4.0)
                .translation(0.0, 0.0, 5.0)
                .transform(),
        )
        .unwrap();
    right_wall.material = floor.material.clone();

    let mut middle = Sphere::new();
    middle.set_transform(translation(-0.5, 1.0, 0.5)).unwrap();
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
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

    let mut left = Sphere::new();
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
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];
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
