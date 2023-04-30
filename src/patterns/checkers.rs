use crate::{canvas::Color, Point};

use super::PatternModel;

#[derive(Debug, Clone, PartialEq)]
pub struct Checkers {
    pub a: Color,
    pub b: Color,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Self {
        Checkers { a, b }
    }
}

const CHECKERS_EPSILON: f64 = 0.00001;

impl PatternModel for Checkers {
    fn at(&self, point: Point) -> Color {
        if ((point.x + CHECKERS_EPSILON).floor()
            + (point.y + CHECKERS_EPSILON).floor()
            + (point.z + CHECKERS_EPSILON).floor()) as i64
            % 2
            == 0
        {
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
    fn checkers_repeat_in_x() {
        let checkers = Checkers::new(WHITE, BLACK);
        assert_eq!(checkers.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(checkers.at(Point::new(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(checkers.at(Point::new(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_repeat_in_y() {
        let checkers = Checkers::new(WHITE, BLACK);
        assert_eq!(checkers.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(checkers.at(Point::new(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(checkers.at(Point::new(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_repeat_in_z() {
        let checkers = Checkers::new(WHITE, BLACK);
        assert_eq!(checkers.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(checkers.at(Point::new(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(checkers.at(Point::new(0.0, 0.0, 1.01)), BLACK);
    }
}
