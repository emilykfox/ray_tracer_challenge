use crate::{
    canvas::{Canvas, PixelOutOfBoundsError},
    matrices::{Transform, IDENTITY},
    rays::Ray,
    world::{World, RECURSION_DEPTH},
    Point,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Transform,
    inverse: Transform,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct NoInverseError;

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: IDENTITY,
            inverse: IDENTITY,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Result<Ray, PixelOutOfBoundsError> {
        if x > self.hsize || y > self.vsize {
            return Err(PixelOutOfBoundsError);
        }

        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = &self.inverse * Point::new(world_x, world_y, -1.0);
        let origin = &self.inverse * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ok(Ray::new(origin, direction))
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        let inverse = transform.inverse().ok_or(NoInverseError)?;
        self.transform = transform;
        self.inverse = inverse;
        Ok(())
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y).expect("pixel out of bounds");
                let color = world.color_from(&ray, RECURSION_DEPTH);
                image.write_pixel(x, y, color).expect("pixel out of bounds");
            }
        }

        image
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{
        canvas::Color,
        transformations::{view_transform, Builder},
        world::default_world,
        Point, Vector, EQUALITY_EPSILON,
    };

    use super::*;

    #[test]
    fn construct_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, IDENTITY);
    }

    #[test]
    fn pixel_size_for_horizontal() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EQUALITY_EPSILON);
    }

    #[test]
    fn pixel_size_for_vertical() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EQUALITY_EPSILON);
    }

    #[test]
    fn ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0).unwrap();
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_after_transform() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(
            Builder::new()
                .translation(0.0, -2.0, 5.0)
                .rotation_y(PI / 4.0)
                .transform(),
        )
        .unwrap();
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, -(2_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn render() {
        let w = default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.set_transform(view_transform(from, to, up)).unwrap();
        let image = c.render(&w);
        assert_eq!(
            image.pixel_at(5, 5),
            Ok(Color::new(0.38066, 0.47583, 0.2855))
        );
    }
}
