use crate::{
    matrices::{CastingMatrixError, Transform},
    tuples::{Point, Vector},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn transformed(&self, transform: Transform) -> Result<Ray, CastingMatrixError> {
        Ok(Ray {
            origin: (transform * self.origin)?,
            direction: (transform * self.direction)?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        transformations::{scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn position() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transformed(m).expect("casting matrix error");
        assert_eq!(r2.origin(), Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction(), Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transformed(m).expect("casting matrix error");
        assert_eq!(r2.origin(), Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction(), Vector::new(0.0, 3.0, 0.0));
    }
}
