use crate::{canvas::Color, Point};

use super::PatternModel;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stripes {
    pub a: Color,
    pub b: Color,
}

impl Stripes {
    pub fn new(a: Color, b: Color) -> Self {
        Stripes { a, b }
    }
}

impl PatternModel for Stripes {
    fn at(&self, point: Point) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
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
    fn create_stripe_pattern() {
        let model = Stripes::new(WHITE, BLACK);
        assert_eq!(model.a, WHITE);
        assert_eq!(model.b, BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let model = Stripes::new(WHITE, BLACK);
        assert_eq!(model.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(model.at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(model.at(Point::new(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let model = Stripes::new(WHITE, BLACK);
        assert_eq!(model.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(model.at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(model.at(Point::new(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let model = Stripes::new(WHITE, BLACK);
        assert_eq!(model.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(model.at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(model.at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(model.at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(model.at(Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(model.at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }
}
