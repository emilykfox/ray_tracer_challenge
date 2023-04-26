use crate::{material::Material, matrices::Transform};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct NoInverseError;

pub trait Shape {
    fn transform(&self) -> &Transform;

    fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError>;

    fn material(&self) -> &Material;

    fn set_material(&mut self, material: Material);
}

#[cfg(test)]
mod test {
    use crate::{matrices::IDENTITY, transformations::translation};

    use super::*;

    struct TestShape {
        transform: Transform,
        material: Material,
    }

    impl TestShape {
        pub fn new() -> TestShape {
            TestShape {
                transform: IDENTITY,
                material: Material::default(),
            }
        }
    }

    impl Shape for TestShape {
        fn transform(&self) -> &Transform {
            &self.transform
        }

        fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
            self.transform = transform;
            Ok(())
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }
    }

    #[test]
    fn default_transform() {
        let s: Box<dyn Shape> = Box::new(TestShape::new());
        assert_eq!(*s.transform(), IDENTITY);
    }

    #[test]
    fn assigning_transform() {
        let mut s: Box<dyn Shape> = Box::new(TestShape::new());
        s.set_transform(translation(2.0, 3.0, 4.0)).unwrap();
        assert_eq!(*s.transform(), translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s: Box<dyn Shape> = Box::new(TestShape::new());
        let m = Material::default();
        assert_eq!(*s.material(), m);
    }

    #[test]
    fn assigning_material() {
        let mut s: Box<dyn Shape> = Box::new(TestShape::new());
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        s.set_material(m.clone());
        assert_eq!(*s.material(), m);
    }
}
