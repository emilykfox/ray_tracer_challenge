use std::{any::Any, fmt::Debug};

mod gradients;
mod rings;
mod stripes;

pub use gradients::Gradient;
pub use rings::Rings;
pub use stripes::Stripes;

use crate::{
    canvas::Color,
    matrices::{Transform, IDENTITY},
    shapes::Shape,
    Point,
};

pub trait PatternModel: Clone + Debug + PartialEq + 'static {
    fn at(&self, point: Point) -> Color;
}

trait DynamicPatternModel: Debug {
    fn at(&self, point: Point) -> Color;

    fn as_any(&self) -> &dyn Any;

    fn dynamic_clone(&self) -> Box<dyn DynamicPatternModel>;

    fn dynamic_eq(&self, other: &dyn DynamicPatternModel) -> bool;
}

impl<T: PatternModel> DynamicPatternModel for T {
    fn at(&self, point: Point) -> Color {
        self.at(point)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dynamic_clone(&self) -> Box<dyn DynamicPatternModel> {
        Box::new(self.clone())
    }

    fn dynamic_eq(&self, other: &dyn DynamicPatternModel) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other| self == other)
    }
}

#[derive(Debug)]
pub struct Pattern {
    transform: Transform,
    inverse: Transform,
    model: Box<dyn DynamicPatternModel>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoInverseTransformError;

impl Pattern {
    pub fn new(model: impl PatternModel) -> Pattern {
        Pattern {
            transform: IDENTITY,
            inverse: IDENTITY,
            model: Box::new(model),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseTransformError> {
        self.inverse = transform.inverse().ok_or(NoInverseTransformError)?;
        self.transform = transform;
        Ok(())
    }

    pub fn at_shape(&self, shape: &Shape, point: Point) -> Color {
        let shape_point = shape.get_inverse_transform() * point;
        let pattern_point = &self.inverse * shape_point;
        self.model.at(pattern_point)
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Pattern {
            transform: self.transform.clone(),
            inverse: self.inverse.clone(),
            model: self.model.dynamic_clone(),
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse == other.inverse
            && self.model.dynamic_eq(other.model.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        matrices::IDENTITY,
        shapes::{Shape, Sphere},
        transformations::{scaling, translation},
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestPattern;

    impl PatternModel for TestPattern {
        fn at(&self, point: Point) -> Color {
            Color::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn default_pattern() {
        let pattern = Pattern::new(TestPattern);
        assert_eq!(pattern.transform, IDENTITY);
    }

    #[test]
    fn assign_transform() {
        let mut pattern = Pattern::new(TestPattern);
        pattern.set_transform(translation(1.0, 2.0, 3.0)).unwrap();
        assert_eq!(pattern.transform, translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut shape = Shape::new(Sphere);
        shape.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        let pattern = Pattern::new(TestPattern);
        let c = pattern.at_shape(&shape, Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = Shape::new(Sphere);
        let mut pattern = Pattern::new(TestPattern);
        pattern.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        let c = pattern.at_shape(&shape, Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_object_and_pattern_transformation() {
        let mut shape = Shape::new(Sphere);
        shape.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        let mut pattern = Pattern::new(TestPattern);
        pattern.set_transform(translation(0.5, 1.0, 1.5)).unwrap();
        let c = pattern.at_shape(&shape, Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
