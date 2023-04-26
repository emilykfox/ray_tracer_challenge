use crate::{
    material::Material,
    matrices::{NoInverseError, Transform, IDENTITY},
};

pub trait Model: 'static {}

pub struct Shape {
    transform: Transform,
    inverse: Transform,
    pub material: Material,
    pub model: Box<dyn Model>,
}

impl Shape {
    pub fn new(form: impl Model) -> Self {
        Shape {
            transform: IDENTITY,
            inverse: IDENTITY,
            material: Material::default(),
            model: Box::new(form),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        let inverse = transform.inverse()?;
        self.transform = transform;
        self.inverse = inverse;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{matrices::IDENTITY, transformations::translation};

    use super::*;

    struct TestModel;

    impl Model for TestModel {}

    #[test]
    fn default_transform() {
        let s = Shape::new(TestModel);
        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn assigning_transform() {
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(2.0, 3.0, 4.0)).unwrap();
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s = Shape::new(TestModel);
        let m = Material::default();
        assert_eq!(s.material, m);
    }

    #[test]
    fn assigning_material() {
        let mut s = Shape::new(TestModel);
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}
