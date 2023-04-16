const EQUALITY_EPSILON: f64 = 0.00001;

/// A 4-dimensional tuple
/// You generally won't construct these directly but instead by casting
/// from `Point` and `Vector`
#[derive(Debug, Default, Copy, Clone)]
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

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EQUALITY_EPSILON
            && (self.y - other.y).abs() < EQUALITY_EPSILON
            && (self.z - other.z).abs() < EQUALITY_EPSILON
            && (self.w - other.w).abs() < EQUALITY_EPSILON
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

impl std::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            tuple: self.tuple - rhs.tuple,
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

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            tuple: self.tuple + rhs.tuple,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            tuple: self.tuple - rhs.tuple,
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

    #[test]
    fn add_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }
    #[test]
    fn add_vector_to_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
        assert_eq!(v + p, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn add_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtract_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }
}
