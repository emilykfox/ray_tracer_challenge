use ray_tracer_challenge::{
    canvas::{Canvas, Color},
    lights::PointLight,
    material::{lighting, Material},
    rays::Ray,
    spheres::Sphere,
    transformations::{translation, Builder},
    Point,
};

fn main() -> std::io::Result<()> {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 10.0;

    let canvas_pixels = 600;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut sphere = Sphere::new();
    sphere.material = Material {
        color: Color::new(1.0, 0.5, 1.0),
        shininess: 100.0,
        ..Material::default()
    };
    let transform = Builder::new()
        .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .scaling(1.0, 0.8, 0.7)
        .transformation();
    sphere.set_transform(transform);

    let light_position = Point::new(8.0, 10.0, -10.0);
    let light_color = Color::new(0.8, 1.0, 0.8);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = sphere.intersect(ray).unwrap();
            if let Some(hit) = intersections.hit() {
                let point = ray.position(hit.t());
                let normal = hit.object().normal_at(point).unwrap();
                let eye = -ray.direction();
                let color = lighting(hit.object().material, light, point, eye, normal);
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
