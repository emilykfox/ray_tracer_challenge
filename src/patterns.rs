use crate::{canvas::Color, Point};

pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern { a, b }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
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
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }
}
