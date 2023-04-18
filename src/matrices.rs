use crate::{tuples::Tuple, Point, Vector};

pub const IDENTITY: Matrix = Matrix {
    raw: RawMatrix {
        entries: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    },
};
const EQUALITY_EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
struct RawMatrix<const M: usize, const N: usize> {
    entries: [[f64; N]; M],
}

impl<const M: usize, const N: usize> RawMatrix<M, N> {
    pub fn new(entries: [[f64; N]; M]) -> Self {
        RawMatrix { entries }
    }

    pub fn transpose(&self) -> RawMatrix<N, M> {
        let mut entries = [[0.0; M]; N];
        for (i, row) in self.entries.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                entries[j][i] = *entry;
            }
        }
        RawMatrix { entries }
    }
}

impl<const M: usize, const N: usize> Default for RawMatrix<M, N> {
    fn default() -> Self {
        /*
        TODO?: Use this code for identity matrix.
        let mut entries = [[0.0; N]; N];
        for (i, entry) in entries.iter_mut().enumerate() {
            entry[i] = 1.0;
        }
        RawMatrix { entries }
        */
        RawMatrix {
            entries: [[0.0; N]; M],
        }
    }
}

impl<const M: usize, const N: usize> PartialEq for RawMatrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for (&x, &y) in self
            .entries
            .iter()
            .flatten()
            .zip(other.entries.iter().flatten())
        {
            if (y - x).abs() >= EQUALITY_EPSILON {
                return false;
            }
        }

        true
    }
}

impl<const M: usize, const N: usize> std::ops::Index<[usize; 2]> for RawMatrix<M, N> {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.entries[index[0]][index[1]]
    }
}

impl<const M: usize, const N: usize, const O: usize> std::ops::Mul<RawMatrix<N, O>>
    for RawMatrix<M, N>
{
    type Output = RawMatrix<M, O>;

    fn mul(self, rhs: RawMatrix<N, O>) -> Self::Output {
        let mut entries = [[0.0; O]; M];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = (0..N).map(|k| self.entries[i][k] * rhs.entries[k][j]).sum()
            }
        }
        RawMatrix { entries }
    }
}

impl From<Tuple> for RawMatrix<4, 1> {
    fn from(value: Tuple) -> Self {
        RawMatrix {
            entries: [[value.x], [value.y], [value.z], [value.w]],
        }
    }
}

impl From<RawMatrix<4, 1>> for Tuple {
    fn from(value: RawMatrix<4, 1>) -> Self {
        Tuple {
            x: value.entries[0][0],
            y: value.entries[1][0],
            z: value.entries[2][0],
            w: value.entries[3][0],
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    raw: RawMatrix<4, 4>,
}

impl Matrix {
    pub fn new(entries: [[f64; 4]; 4]) -> Matrix {
        Matrix {
            raw: RawMatrix::new(entries),
        }
    }

    pub fn transpose(&self) -> Matrix {
        Matrix {
            raw: self.raw.transpose(),
        }
    }
}

impl std::ops::Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.raw[index]
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix {
            raw: self.raw * rhs.raw,
        }
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::from(self.raw * RawMatrix::from(rhs))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct CastingMatrixError {}

/// Will return `Err(CastingMatrixError)`` if enclosed `Tuple` cannot be converted to a `Point`
impl std::ops::Mul<Point> for Matrix {
    type Output = Result<Point, CastingMatrixError>;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::try_from(self * Tuple::from(rhs)).map_err(|_| CastingMatrixError {})
    }
}

/// Will return `Err(CastingMatrixError)`` if enclosed `Tuple` cannot be converted to a `Vector`
impl std::ops::Mul<Vector> for Matrix {
    type Output = Result<Vector, CastingMatrixError>;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::try_from(self * Tuple::from(rhs)).map_err(|_| CastingMatrixError {})
    }
}

#[cfg(test)]
mod test {
    use crate::tuples::Tuple;

    use super::*;

    #[test]
    fn construct_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[[0, 0]], 1.0);
        assert_eq!(m[[0, 3]], 4.0);
        assert_eq!(m[[1, 0]], 5.5);
        assert_eq!(m[[1, 2]], 7.5);
        assert_eq!(m[[2, 2]], 11.0);
        assert_eq!(m[[3, 0]], 13.5);
        assert_eq!(m[[3, 2]], 15.5);
    }

    #[test]
    fn construct_2x2() {
        let m = RawMatrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[[0, 0]], -3.0);
        assert_eq!(m[[0, 1]], 5.0);
        assert_eq!(m[[1, 0]], 1.0);
        assert_eq!(m[[1, 1]], -2.0);
    }

    #[test]
    fn construct_3x3() {
        let m = RawMatrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(m[[0, 0]], -3.0);
        assert_eq!(m[[1, 1]], -2.0);
        assert_eq!(m[[2, 2]], 1.0);
    }

    #[test]
    fn matrix_equality() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_inequality() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    /*
    TODO: Default should be all 0's.
    Save some code for identity matrix.
    #[test]
    fn matrix_default() {
        let m = Matrix::default();
        let i = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(m, i);
    }
    */

    #[test]
    fn matrix_multiplication() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let product = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(a * b, product);
    }

    #[test]
    fn matrix_multiply_tuple() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn matrix_multiply_point() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(a * p, Ok(Point::new(18.0, 24.0, 33.0)));
    }

    #[test]
    fn matrix_multiply_vertex() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a * v, Ok(Vector::new(14.0, 22.0, 32.0)));
    }

    #[test]
    fn matrix_multiply_error() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 2.0, 0.0, 1.0],
        ]);
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a * p, Err(CastingMatrixError {}));
        assert_eq!(a * v, Err(CastingMatrixError {}));
    }

    #[test]
    fn matrix_times_identity() {
        let a = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(a * IDENTITY, a);
    }

    #[test]
    fn identity_times_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(IDENTITY * a, a);
    }

    #[test]
    fn transpose() {
        let a = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        assert_eq!(
            a.transpose(),
            Matrix::new([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ])
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(IDENTITY.transpose(), IDENTITY);
    }
}
