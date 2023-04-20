use crate::matrices::{Matrix, IDENTITY};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x(r: f64) -> Matrix {
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_y(r: f64) -> Matrix {
    Matrix::new([
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_z(r: f64) -> Matrix {
    Matrix::new([
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shearing(
    x_by_y: f64,
    x_by_z: f64,
    y_by_x: f64,
    y_by_z: f64,
    z_by_x: f64,
    z_by_y: f64,
) -> Matrix {
    Matrix::new([
        [1.0, x_by_y, x_by_z, 0.0],
        [y_by_x, 1.0, y_by_z, 0.0],
        [z_by_x, z_by_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Builder {
    current: Matrix,
}

impl Builder {
    pub fn new() -> Self {
        Builder { current: IDENTITY }
    }

    pub fn translation(self, x: f64, y: f64, z: f64) -> Builder {
        Builder {
            current: translation(x, y, z) * self.current,
        }
    }

    pub fn scaling(self, x: f64, y: f64, z: f64) -> Builder {
        Builder {
            current: scaling(x, y, z) * self.current,
        }
    }

    pub fn rotation_x(self, r: f64) -> Builder {
        Builder {
            current: rotation_x(r) * self.current,
        }
    }

    pub fn rotation_y(self, r: f64) -> Builder {
        Builder {
            current: rotation_y(r) * self.current,
        }
    }

    pub fn rotation_z(self, r: f64) -> Builder {
        Builder {
            current: rotation_z(r) * self.current,
        }
    }

    pub fn shearing(
        self,
        x_by_y: f64,
        x_by_z: f64,
        y_by_x: f64,
        y_by_z: f64,
        z_by_x: f64,
        z_by_y: f64,
    ) -> Builder {
        Builder {
            current: shearing(x_by_y, x_by_z, y_by_x, y_by_z, z_by_x, z_by_y) * self.current,
        }
    }

    pub fn transformation(&self) -> Matrix {
        self.current
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn translate() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Ok(Point::new(2.0, 1.0, 7.0)));
    }

    #[test]
    fn inverse_translate() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse().expect("cannot take inverse");
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(inverse * p, Ok(Point::new(-8.0, 7.0, 3.0)));
    }

    #[test]
    fn translation_ignores_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, Ok(v));
    }

    #[test]
    fn scale_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Ok(Point::new(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn scale_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Ok(Vector::new(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn inverse_scale() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse().expect("cannot take inverse");
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(inverse * v, Ok(Vector::new(-2.0, 2.0, 2.0)));
    }

    #[test]
    fn reflection() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(-2.0, 3.0, 4.0)));
    }

    #[test]
    fn rotate_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Ok(Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0))
        );
        assert_eq!(full_quarter * p, Ok(Point::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn inverse_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inverse = half_quarter.inverse().expect("cannot take inverse");
        assert_eq!(
            inverse * p,
            Ok(Point::new(
                0.0,
                2.0_f64.sqrt() / 2.0,
                -(2.0_f64.sqrt()) / 2.0
            ))
        );
    }

    #[test]
    fn rotate_y() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Ok(Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0))
        );
        assert_eq!(full_quarter * p, Ok(Point::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn rotate_z() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Ok(Point::new(
                -(2.0_f64.sqrt()) / 2.0,
                2.0_f64.sqrt() / 2.0,
                0.0
            ))
        );
        assert_eq!(full_quarter * p, Ok(Point::new(-1.0, 0.0, 0.0)));
    }

    #[test]
    fn shear_x_by_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(5.0, 3.0, 4.0)));
    }

    #[test]
    fn shear_x_by_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(6.0, 3.0, 4.0)));
    }

    #[test]
    fn shear_y_by_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(2.0, 5.0, 4.0)));
    }

    #[test]
    fn shear_y_by_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(2.0, 7.0, 4.0)));
    }

    #[test]
    fn shear_z_by_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(2.0, 3.0, 6.0)));
    }

    #[test]
    fn shear_z_by_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Ok(Point::new(2.0, 3.0, 7.0)));
    }

    #[test]
    fn sequence_of_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p2 = (a * p).expect("matrix casting error");
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        let p3 = (b * p2).expect("matrix casting error");
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        let p4 = (c * p3).expect("matrix casting error");
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Ok(Point::new(15.0, 0.0, 7.0)));
    }

    #[test]
    fn build_transformation() {
        let p = Point::new(1.0, 0.0, 1.0);
        let t = Builder::new()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0)
            .shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
            .transformation();
        assert_eq!(t * p, Ok(Point::new(15.0, 15.0, 7.0)));
    }
}
