use crate::{
    matrices::{NoInverseError, Transform},
    rays::Ray,
    transformations::IDENTITY,
    Point,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Transform,
    inverse: Option<Transform>,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RayForPixelError {
    PixelOutOfBounds,
    NoInverse,
    CastingTransform,
}

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
            inverse: Some(IDENTITY),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Result<Ray, RayForPixelError> {
        if x > self.hsize || y > self.vsize {
            return Err(RayForPixelError::PixelOutOfBounds);
        }

        let inverse = self.inverse.as_ref().ok_or(RayForPixelError::NoInverse)?;

        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = (inverse * Point::new(world_x, world_y, -1.0))
            .map_err(|_| RayForPixelError::CastingTransform)?;
        let origin = (inverse * Point::new(0.0, 0.0, 0.0))
            .map_err(|_| RayForPixelError::CastingTransform)?;
        let direction = (pixel - origin).normalize();

        Ok(Ray::new(origin, direction))
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        self.transform = transform;
        self.inverse = Some(self.transform.inverse()?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{transformations::Builder, Point, Vector, EQUALITY_EPSILON};

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
        assert_eq!(c.transform, Transform::identity());
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
}
