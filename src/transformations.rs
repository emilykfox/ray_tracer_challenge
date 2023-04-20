use crate::matrices::{Matrix, IDENTITY};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod test {
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
}
