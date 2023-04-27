use crate::{Point, Vector, EQUALITY_EPSILON};

pub const IDENTITY: Transform = Transform {
    matrix: Matrix {
        entries: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    },
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MatrixIndexError;

#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    pub entries: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(entries: [[f64; N]; M]) -> Self {
        Matrix { entries }
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut entries = [[0.0; M]; N];
        for (i, row) in self.entries.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                entries[j][i] = *entry;
            }
        }
        Matrix { entries }
    }
}

impl Matrix<2, 2> {
    pub fn submatrix(&self, i: usize, j: usize) -> Result<Matrix<1, 1>, MatrixIndexError> {
        if i > 1 || j > 1 {
            Err(MatrixIndexError)
        } else {
            Ok(Matrix {
                entries: [[self.entries[1 - i][1 - j]]],
            })
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        Ok(self.submatrix(i, j)?[[0, 0]])
    }

    pub fn cofactor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        let minor = self.minor(i, j)?;
        if (i + j) % 2 == 0 {
            Ok(minor)
        } else {
            Ok(-minor)
        }
    }

    pub fn determinant(&self) -> f64 {
        self.entries[0]
            .iter()
            .enumerate()
            .map(|(j, entry)| entry * self.cofactor(0, j).expect("matrix index error"))
            .sum()
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix<2, 2>> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return None;
        }

        let mut entries = [[0.0; 2]; 2];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                let cofactor = self.cofactor(j, i);
                *entry = cofactor.expect("matrix index error") / determinant;
            }
        }

        Some(Matrix { entries })
    }
}

impl Matrix<3, 3> {
    pub fn submatrix(&self, i: usize, j: usize) -> Result<Matrix<2, 2>, MatrixIndexError> {
        if i > 2 || j > 2 {
            Err(MatrixIndexError)
        } else {
            let mut entries = [[0.0; 2]; 2];
            for (k, row) in self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(k, row)| if k != i { Some(row) } else { None })
                .enumerate()
            {
                for (l, entry) in row
                    .iter()
                    .enumerate()
                    .filter_map(|(l, entry)| if l != j { Some(entry) } else { None })
                    .enumerate()
                {
                    entries[k][l] = *entry;
                }
            }
            Ok(Matrix { entries })
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        Ok(self.submatrix(i, j)?.determinant())
    }

    pub fn cofactor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        let minor = self.minor(i, j)?;
        if (i + j) % 2 == 0 {
            Ok(minor)
        } else {
            Ok(-minor)
        }
    }

    pub fn determinant(&self) -> f64 {
        self.entries[0]
            .iter()
            .enumerate()
            .map(|(j, entry)| entry * self.cofactor(0, j).expect("matrix index error"))
            .sum()
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix<3, 3>> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return None;
        }

        let mut entries = [[0.0; 3]; 3];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                let cofactor = self.cofactor(j, i);
                *entry = cofactor.expect("matrix index error") / determinant;
            }
        }

        Some(Matrix { entries })
    }
}

impl Matrix<4, 4> {
    pub fn submatrix(&self, i: usize, j: usize) -> Result<Matrix<3, 3>, MatrixIndexError> {
        if i > 3 || j > 3 {
            Err(MatrixIndexError)
        } else {
            let mut entries = [[0.0; 3]; 3];
            for (k, row) in self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(k, row)| if k != i { Some(row) } else { None })
                .enumerate()
            {
                for (l, entry) in row
                    .iter()
                    .enumerate()
                    .filter_map(|(l, entry)| if l != j { Some(entry) } else { None })
                    .enumerate()
                {
                    entries[k][l] = *entry;
                }
            }
            Ok(Matrix { entries })
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        Ok(self.submatrix(i, j)?.determinant())
    }

    pub fn cofactor(&self, i: usize, j: usize) -> Result<f64, MatrixIndexError> {
        let minor = self.minor(i, j)?;
        if (i + j) % 2 == 0 {
            Ok(minor)
        } else {
            Ok(-minor)
        }
    }

    pub fn determinant(&self) -> f64 {
        self.entries[0]
            .iter()
            .enumerate()
            .map(|(j, entry)| entry * self.cofactor(0, j).expect("matrix index error"))
            .sum()
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix<4, 4>> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return None;
        }

        let mut entries = [[0.0; 4]; 4];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                let cofactor = self.cofactor(j, i);
                *entry = cofactor.expect("matrix index error") / determinant;
            }
        }

        Some(Matrix { entries })
    }
}

impl<const M: usize, const N: usize> Default for Matrix<M, N> {
    fn default() -> Self {
        Matrix {
            entries: [[0.0; N]; M],
        }
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut entries = [[0.0; N]; N];
        for (i, row) in entries.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        Matrix { entries }
    }
}

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
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

impl<const M: usize, const N: usize> std::ops::Index<[usize; 2]> for Matrix<M, N> {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.entries[index[0]][index[1]]
    }
}

impl<const M: usize, const N: usize, const O: usize> std::ops::Mul<&Matrix<N, O>>
    for &Matrix<M, N>
{
    type Output = Matrix<M, O>;

    fn mul(self, rhs: &Matrix<N, O>) -> Self::Output {
        let mut entries = [[0.0; O]; M];
        for (i, row) in entries.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = (0..N).map(|k| self.entries[i][k] * rhs.entries[k][j]).sum()
            }
        }
        Matrix { entries }
    }
}

impl From<Point> for Matrix<4, 1> {
    fn from(value: Point) -> Self {
        Matrix {
            entries: [[value.x], [value.y], [value.z], [1.0]],
        }
    }
}

impl From<Vector> for Matrix<4, 1> {
    fn from(value: Vector) -> Self {
        Matrix {
            entries: [[value.x], [value.y], [value.z], [0.0]],
        }
    }
}

impl From<Matrix<4, 1>> for Point {
    fn from(value: Matrix<4, 1>) -> Self {
        Point::new(
            value.entries[0][0],
            value.entries[1][0],
            value.entries[2][0],
        )
    }
}

impl From<Matrix<4, 1>> for Vector {
    fn from(value: Matrix<4, 1>) -> Self {
        Vector::new(
            value.entries[0][0],
            value.entries[1][0],
            value.entries[2][0],
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transform {
    matrix: Matrix<4, 4>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct CastingTransformError;

impl Transform {
    pub fn new(entries: [[f64; 4]; 4]) -> Result<Self, CastingTransformError> {
        if entries[3] != [0.0, 0.0, 0.0, 1.0] {
            Err(CastingTransformError)
        } else {
            Ok(Transform {
                matrix: Matrix::new(entries),
            })
        }
    }

    pub fn submatrix(&self, i: usize, j: usize) -> Result<Matrix<3, 3>, MatrixIndexError> {
        self.matrix.submatrix(i, j)
    }

    pub fn invertible(&self) -> bool {
        self.matrix.invertible()
    }

    pub fn inverse(&self) -> Option<Transform> {
        Some(Transform {
            matrix: self.matrix.inverse()?,
        })
    }
}

impl std::ops::Mul for &Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        let matrix = &self.matrix * &rhs.matrix;
        debug_assert_eq!(matrix.entries[3], [0.0, 0.0, 0.0, 1.0]);

        Transform { matrix }
    }
}

impl std::ops::Mul<Point> for &Transform {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let column = &self.matrix * &Matrix::from(rhs);
        debug_assert_eq!(column[[3, 0]], 1.0);

        column.into()
    }
}

impl std::ops::Mul<Vector> for &Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let column = &self.matrix * &Matrix::from(rhs);
        debug_assert_eq!(column[[3, 0]], 0.0);

        column.into()
    }
}

#[cfg(test)]
mod test {
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
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[[0, 0]], -3.0);
        assert_eq!(m[[0, 1]], 5.0);
        assert_eq!(m[[1, 0]], 1.0);
        assert_eq!(m[[1, 1]], -2.0);
    }

    #[test]
    fn construct_3x3() {
        let m = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

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
        assert_eq!(&a * &b, product);
    }

    #[test]
    fn matrix_multiply_point() {
        let a = Transform::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(&a * p, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn matrix_multiply_vertex() {
        let a = Transform::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(&a * v, Vector::new(14.0, 22.0, 32.0));
    }

    #[test]
    fn matrix_times_identity() {
        let a = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(&a * &Matrix::identity(), a);
    }

    #[test]
    fn identity_times_point() {
        let a = Point::new(1.0, 2.0, 3.0);
        assert_eq!(&IDENTITY * a, a);
    }

    #[test]
    fn identity_times_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(&IDENTITY * a, a);
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
        assert_eq!(Matrix::<4, 4>::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn determinant_of_2x2() {
        let a = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(
            a.submatrix(0, 2),
            Ok(Matrix::new([[-3.0, 2.0], [0.0, 6.0],]))
        );
    }

    #[test]
    fn submatrix_of_4x4() {
        let a = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(
            a.submatrix(2, 1),
            Ok(Matrix::new([
                [-6.0, 1.0, 6.0],
                [-8.0, 8.0, 6.0],
                [-7.0, -1.0, 1.0],
            ]))
        );
    }

    #[test]
    fn minor_of_3x3() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0).expect("matrix index error");
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), Ok(25.0));
    }

    #[test]
    fn cofactor_of_3x3() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), Ok(-12.0));
        assert_eq!(a.cofactor(0, 0), Ok(-12.0));
        assert_eq!(a.minor(1, 0), Ok(25.0));
        assert_eq!(a.cofactor(1, 0), Ok(-25.0));
    }

    #[test]
    fn determinant_of_3x3() {
        let a = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), Ok(56.0));
        assert_eq!(a.cofactor(0, 1), Ok(12.0));
        assert_eq!(a.cofactor(0, 2), Ok(-46.0));
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_4x4() {
        let a = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.cofactor(0, 0), Ok(690.0));
        assert_eq!(a.cofactor(0, 1), Ok(447.0));
        assert_eq!(a.cofactor(0, 2), Ok(210.0));
        assert_eq!(a.cofactor(0, 3), Ok(51.0));
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn invertible() {
        let a = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.invertible());
    }

    #[test]
    fn not_invertible() {
        let a = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.invertible());
    }

    #[test]
    fn inverse() {
        let a = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse().expect("no inverse");
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), Ok(-160.0));
        assert_eq!(b[[3, 2]], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), Ok(105.0));
        assert_eq!(b[[2, 3]], 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix::new([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn inverse2() {
        let a = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        assert_eq!(
            a.inverse(),
            Some(Matrix::new([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ]))
        );
    }

    #[test]
    fn inverse3() {
        let a = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        assert_eq!(
            a.inverse(),
            Some(Matrix::new([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ]))
        );
    }

    #[test]
    fn inverse_is_inverse() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = &a * &b;
        assert_eq!(&c * &b.inverse().expect("not invertable"), a);
    }

    #[test]
    fn fail_to_invert() {
        let a = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.inverse(), None);
    }
}
