use std::f64::consts::PI;

use ray_tracer_challenge::{
    canvas::{Canvas, Color},
    transformations::rotation_z,
    Point,
};

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(400, 400);
    let transform = rotation_z(-PI / 6.0);
    let mut point = Point::new(0.0, 170.0, 0.0);
    for _ in 1..=12 {
        point = (transform * point).unwrap();
        canvas
            .write_pixel(
                (200.0 + point.x()) as usize,
                (200.0 - point.y()) as usize,
                Color::new(1.0, 1.0, 1.0),
            )
            .unwrap();
    }

    let output_path = std::env::args().nth(1);
    if let Some(output_path) = output_path {
        std::fs::write(output_path, canvas.to_ppm())?;
    }

    Ok(())
}
