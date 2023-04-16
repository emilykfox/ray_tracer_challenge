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

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        *self / self.magnitude()
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

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
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

    pub fn magnitude(&self) -> f64 {
        self.tuple.magnitude()
    }

    pub fn normalize(&self) -> Vector {
        Vector {
            tuple: self.tuple.normalize(),
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

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector { tuple: -self.tuple }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            tuple: self.tuple * rhs,
        }
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            tuple: self * rhs.tuple,
        }
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            tuple: self.tuple / rhs,
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
    fn create_points_from_tuples() {
        let a1 = Tuple::new(4.0, -4.0, 3.0, 1.0);
        let a2 = Tuple::new(4.0, -4.0, 3.0, 0.0);
        assert_eq!(Point::try_from(a1), Ok(Point::new(4.0, -4.0, 3.0)));
        assert_eq!(Point::try_from(a2), Err(InvalidTupleError));
    }

    #[test]
    fn create_vectors_from_tuples() {
        let a1 = Tuple::new(4.0, -4.0, 3.0, 0.0);
        let a2 = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert_eq!(Vector::try_from(a1), Ok(Vector::new(4.0, -4.0, 3.0)));
        assert_eq!(Vector::try_from(a2), Err(InvalidTupleError));
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

    #[test]
    fn subtract_vector_from_zero() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn negate_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn mult_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(3.5 * a, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn mult_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(3.5 * v, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn mult_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
        assert_eq!(0.5 * a, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn div_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn div_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_x_unit() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_y_unit() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_z_unit() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_positive() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_negative() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_horizontal_vector() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalize(),
            Vector::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_normalized() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }
}
