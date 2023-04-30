use crate::{canvas::Color, Point};

use super::PatternModel;

#[derive(Debug, Clone, PartialEq)]
pub struct Rings {
    a: Color,
    b: Color,
}

impl Rings {
    pub fn new(a: Color, b: Color) -> Self {
        Rings { a, b }
    }
}

impl PatternModel for Rings {
    fn at(&self, point: Point) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod test {
    use crate::canvas::{BLACK, WHITE};

    use super::*;

    #[test]
    fn ring_extends_in_x_and_z() {
        let rings = Rings::new(WHITE, BLACK);
        assert_eq!(rings.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(rings.at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(rings.at(Point::new(0.0, 0.0, 1.0)), BLACK);
        // 0.708 is just slightly more than 2_f64.sqrt() / 2.0
        assert_eq!(rings.at(Point::new(0.708, 0.0, 0.708)), BLACK);
    }
}
