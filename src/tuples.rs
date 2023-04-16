/// A 4-dimensional tuple
/// You generally won't construct these directly but instead by casting
/// from `Point` and `Vector`
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// Returns a new `Tuple`
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    /// Returns `true` if tuple represents a 3-dimensional point
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Returns `true` if tuple represents a 3-dimensional vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl From<Point> for Tuple {
    fn from(value: Point) -> Self {
        value.tuple
    }
}

impl From<Vector> for Tuple {
    fn from(value: Vector) -> Self {
        value.tuple
    }
}

pub struct InvalidTupleError;

/// A 3-dimensional point
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    /// Returns a new `Point`
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            tuple: Tuple::new(x, y, z, 1.0),
        }
    }
}

impl TryFrom<Tuple> for Point {
    type Error = InvalidTupleError;

    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        if !value.is_point() {
            Err(InvalidTupleError)
        } else {
            Ok(Point { tuple: value })
        }
    }
}

/// A 3-dimensional vector
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector {
    tuple: Tuple,
}

impl Vector {
    /// Returns a new `Vector `
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self {
        Vector {
            tuple: Tuple::new(dx, dy, dz, 0.0),
        }
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = InvalidTupleError;

    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        if !value.is_vector() {
            Err(InvalidTupleError)
        } else {
            Ok(Vector { tuple: value })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_points() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn determine_vectors() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn create_points() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(Tuple::from(p), Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn create_vectors() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(Tuple::from(v), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
}
