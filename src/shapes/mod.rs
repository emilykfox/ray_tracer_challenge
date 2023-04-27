#[cfg(test)]
use std::cell::RefCell;
use std::fmt::Debug;

use crate::{
    intersections::Intersections,
    material::Material,
    matrices::{NoInverseError, Transform, IDENTITY},
    rays::Ray,
};

pub mod spheres;

pub trait Model: Debug {
    fn local_intersect<'shape>(
        &self,
        shape: &'shape Shape,
        local_ray: &'_ Ray,
    ) -> Intersections<'shape>;

    fn dynamic_clone(&self) -> Box<dyn Model>;
}

#[derive(Debug)]
pub struct Shape {
    transform: Transform,
    inverse: Transform,
    pub material: Material,
    pub model: Box<dyn Model>,
    #[cfg(test)]
    saved_ray: RefCell<Ray>,
}

impl Shape {
    pub fn new(model: impl Model) -> Self {
        Shape {
            transform: IDENTITY,
            inverse: IDENTITY,
            material: Material::default(),
            model: Box::new(model),
            #[cfg(test)]
            saved_ray: RefCell::new(Ray::default()),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        let inverse = transform.inverse()?;
        self.transform = transform;
        self.inverse = inverse;
        Ok(())
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transformed(&self.inverse);
        self.model.local_intersect(self, &local_ray)
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape {
            transform: self.transform.clone(),
            inverse: self.inverse.clone(),
            material: self.material.clone(),
            model: self.model.dynamic_clone(),
            #[cfg(test)]
            saved_ray: self.saved_ray.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        matrices::IDENTITY,
        transformations::{scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
    struct TestModel;

    impl Model for TestModel {
        fn local_intersect<'shape>(
            &self,
            shape: &'shape Shape,
            local_ray: &'_ Ray,
        ) -> Intersections<'shape> {
            #[cfg(test)]
            {
                shape.saved_ray.replace(local_ray.clone());
            }
            Intersections::new(vec![])
        }

        fn dynamic_clone(&self) -> Box<dyn Model> {
            Box::new(Self)
        }
    }

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

    #[test]
    fn intersect_scaled_shape() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(TestModel);
        s.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        _ = s.intersect(&r);
        assert_eq!(s.saved_ray.borrow().origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.borrow().direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersect_translated_shape() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        _ = s.intersect(&r);
        assert_eq!(s.saved_ray.borrow().origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.borrow().direction, Vector::new(0.0, 0.0, 1.0));
    }
}
