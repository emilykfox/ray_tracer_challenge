/// A 4-dimensional tuple
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// Returns `true` if tuple represents a 3-dimensional point
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Returns `true` if tuple represents a 3-dimensional vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
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
}
