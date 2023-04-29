use std::fmt::Debug;

pub mod stripe_pattern;

pub use stripe_pattern::StripePattern;

use crate::matrices::{Transform, IDENTITY};

pub trait PatternModel: Clone + Debug + PartialEq {}

#[derive(Debug)]
pub struct Pattern {
    transform: Transform,
    inverse: Transform,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoInverseTransformError;

impl Pattern {
    pub fn new(model: impl PatternModel) -> Pattern {
        Pattern {
            transform: IDENTITY,
            inverse: IDENTITY,
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseTransformError> {
        self.inverse = transform.inverse().ok_or(NoInverseTransformError)?;
        self.transform = transform;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{matrices::IDENTITY, transformations::translation};

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestPattern;

    impl PatternModel for TestPattern {}

    #[test]
    fn default_pattern() {
        let pattern = Pattern::new(TestPattern);
        assert_eq!(pattern.transform, IDENTITY);
    }

    #[test]
    fn assign_transform() {
        let mut pattern = Pattern::new(TestPattern);
        pattern.set_transform(translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform, translation(1.0, 2.0, 3.0));
    }
}
