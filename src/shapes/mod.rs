use crate::{
    intersections::{Intersection, Intersections},
    material::Material,
    matrices::{NoInverseError, Transform, IDENTITY},
    rays::Ray,
};

#[cfg(test)]
use std::cell::RefCell;
use std::{any::Any, fmt::Debug};

pub mod spheres;

#[cfg(test)]
thread_local! {
    static SAVED_RAY: RefCell<Ray> = RefCell::new(Ray::default());
}

pub trait ModelDynamicClone {
    fn dynamic_clone(&self) -> Box<dyn Model>;
}

impl<T: Model + Clone> ModelDynamicClone for T {
    fn dynamic_clone(&self) -> Box<dyn Model> {
        Box::new(self.clone())
    }
}

pub trait ModelDynamicPartialEq {
    fn dynamic_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Model + PartialEq> ModelDynamicPartialEq for T {
    fn dynamic_eq(&self, other: &dyn Any) -> bool {
        if let Some(other) = other.downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

pub trait Model: Debug + Any + ModelDynamicClone + ModelDynamicPartialEq + ModelAsAny {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;
}

#[derive(Debug)]
pub struct Shape {
    transform: Transform,
    inverse: Transform,
    pub material: Material,
    pub model: Box<dyn Model>,
}

impl Shape {
    pub fn new(model: impl Model) -> Self {
        Shape {
            transform: IDENTITY,
            inverse: IDENTITY,
            material: Material::default(),
            model: Box::new(model),
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
        Intersections::new(
            self.model
                .local_intersect(&local_ray)
                .into_iter()
                .map(|t| Intersection::new(t, self))
                .collect(),
        )
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape {
            transform: self.transform.clone(),
            inverse: self.inverse.clone(),
            material: self.material.clone(),
            model: self.model.dynamic_clone(),
        }
    }
}

pub trait ModelAsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Model> ModelAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse == other.inverse
            && self.material == other.material
            && self.model.dynamic_eq(other.model.as_ref().as_any())
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
        fn local_intersect(&self, local_ray: &'_ Ray) -> Vec<f64> {
            #[cfg(test)]
            {
                SAVED_RAY.with(|saved_ray| saved_ray.replace(local_ray.clone()));
            }
            vec![]
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
        SAVED_RAY
            .with(|saved_ray| assert_eq!(saved_ray.borrow().origin, Point::new(0.0, 0.0, -2.5)));
        SAVED_RAY
            .with(|saved_ray| assert_eq!(saved_ray.borrow().direction, Vector::new(0.0, 0.0, 0.5)));
    }

    #[test]
    fn intersect_translated_shape() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        _ = s.intersect(&r);
        SAVED_RAY
            .with(|saved_ray| assert_eq!(saved_ray.borrow().origin, Point::new(-5.0, 0.0, -5.0)));
        SAVED_RAY
            .with(|saved_ray| assert_eq!(saved_ray.borrow().direction, Vector::new(0.0, 0.0, 1.0)));
    }
}
