use ray_tracer_challenge::{
    canvas::{Canvas, Color},
    rays::Ray,
    spheres::Sphere,
    transformations::translation,
    Point, Vector,
};

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(400, 400);
    let hit_color = Color::new(1.0, 0.0, 0.0);

    let transform = translation(0.0, 0.0, 5.0);
    let mut sphere = Sphere::new();
    sphere.set_transform(transform);

    let direction = Vector::new(0.0, 0.0, 1.0);

    for x in 0..400 {
        for y in 0..400 {
            let ray = Ray::new(
                Point::new(
                    1.3 * (x as f64 / 200.0 - 1.0),
                    1.3 * (-(y as f64 / 200.0) + 1.0),
                    0.0,
                ),
                direction,
            );
            let intersections = sphere.intersect(ray).unwrap();
            if intersections.hit().is_some() {
                canvas
                    .write_pixel(x, y, hit_color)
                    .expect("cannot write pixel");
            }
        }
    }

    let output_path = std::env::args().nth(1);
    if let Some(output_path) = output_path {
        std::fs::write(output_path, canvas.to_ppm())?;
    }

    Ok(())
}
