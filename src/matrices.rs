const EQUALITY_EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
struct RawMatrix<const M: usize, const N: usize> {
    entries: [[f64; N]; M],
}

impl<const M: usize, const N: usize> RawMatrix<M, N> {
    pub fn new(entries: [[f64; N]; M]) -> Self {
        RawMatrix { entries }
    }
}

impl<const M: usize, const N: usize> Default for RawMatrix<M, N> {
    fn default() -> Self {
        /*
        TODO: Use this code for identity matrix.
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
}

impl std::ops::Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.raw[index]
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
}
