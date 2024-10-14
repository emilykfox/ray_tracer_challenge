use clap::Parser;
use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    canvas::Color,
    lights::PointLight,
    materials::Material,
    patterns::{Checkers, Gradient, Pattern, Rings},
    shapes::{Plane, Shape, Sphere},
    transformations::{translation, view_transform, Builder},
    world::World,
    Point, Vector,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "image.ppm")]
    output: String,

    #[arg(long, default_value = "480")]
    width: usize,

    #[arg(long, default_value = "270")]
    height: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut floor = Shape::new(Plane);
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    floor.material.pattern = Some(Pattern::new(Checkers::new(
        Color::new(1.0, 0.9, 0.9),
        Color::new(0.0, 0.1, 0.1),
    )));
    floor.material.reflective = 0.8;

    let mut back_wall = Shape::new(Plane);
    back_wall
        .set_transform(
            Builder::new()
                .rotation_x(PI / 2.0)
                .translation(0.0, 0.0, 5.0)
                .transform(),
        )
        .unwrap();
    back_wall.material = Material::new();
    back_wall.material.color = Color::new(1.0, 1.0, 1.0);
    back_wall.material.diffuse = 0.2;
    back_wall.material.ambient = 0.1;
    back_wall.material.specular = 0.0;
    back_wall.material.reflective = 1.0;

    let mut middle = Shape::new(Sphere);
    middle.set_transform(translation(-0.5, 1.0, 3.0)).unwrap();
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut middle_pattern = Pattern::new(Gradient::new(
        Color::new(0.1, 1.0, 0.5),
        Color::new(0.9, 0.0, 0.5),
    ));
    middle_pattern
        .set_transform(
            Builder::new()
                .scaling(2.0, 2.0, 2.0)
                .translation(-1.0, 0.0, 0.0)
                .rotation_z(PI / 4.0)
                .transform(),
        )
        .expect("no inverse for transform");
    middle.material.pattern = Some(middle_pattern);

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
    let mut right_pattern = Pattern::new(Rings::new(
        Color::new(0.5, 1.0, 0.1),
        Color::new(0.3, 0.8, 0.3),
    ));
    right_pattern
        .set_transform(
            Builder::new()
                .scaling(0.3, 0.1, 0.1)
                .shearing(0.2, 0.0, 0.0, 0.0, 0.0, 0.0)
                .rotation_x(PI / 2.0)
                .transform(),
        )
        .expect("no inverse for transform");
    right.material.pattern = Some(right_pattern);

    let mut left = Shape::new(Sphere);
    left.set_transform(
        Builder::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-0.75, 0.33, 1.0)
            .transform(),
    )
    .unwrap();
    left.material = Material::new();
    left.material.color = Color::new(0.25, 0.2, 0.025);
    left.material.diffuse = 0.1;
    left.material.specular = 0.3;
    left.material.reflective = 0.1;
    left.material.transparaency = 0.9;
    left.material.refractive_index = 1.05;

    let mut world = World::new();
    world.objects = vec![floor, back_wall, middle, right, left];
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
