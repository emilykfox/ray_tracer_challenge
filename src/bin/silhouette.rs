use ray_tracer_challenge::{
    canvas::{Canvas, Color},
    rays::Ray,
    spheres::Sphere,
    Point,
};

fn main() -> std::io::Result<()> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::new();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = shape.intersect(&ray).unwrap();
            if intersections.hit().is_some() {
                canvas
                    .write_pixel(x, y, color)
                    .expect("unable to write pixel");
            }
        }
    }

    let output_path = std::env::args().nth(1);
    if let Some(output_path) = output_path {
        std::fs::write(output_path, canvas.to_ppm())?;
    }

    Ok(())
}
