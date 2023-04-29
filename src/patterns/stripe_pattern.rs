use crate::{
    canvas::Color,
    matrices::{Transform, IDENTITY},
    shapes::Shape,
    Point,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    transform: Transform,
    inverse: Transform,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoInverseTransformError;

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern {
            a,
            b,
            transform: IDENTITY,
            inverse: IDENTITY,
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseTransformError> {
        self.inverse = transform.inverse().ok_or(NoInverseTransformError)?;
        self.transform = transform;
        Ok(())
    }

    fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &Shape, point: Point) -> Color {
        let object_point = object.get_inverse_transform() * point;
        let pattern_point = &self.inverse * object_point;
        self.stripe_at(pattern_point)
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
