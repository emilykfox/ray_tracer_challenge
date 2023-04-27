use crate::{
    intersections::{Intersection, Intersections},
    material::Material,
    matrices::{Matrix, NoInverseError, Transform, IDENTITY},
    rays::Ray,
    Point, Vector,
};

#[cfg(test)]
use std::cell::RefCell;
use std::{any::Any, fmt::Debug};

pub mod spheres;

#[cfg(test)]
thread_local! {
    static SAVED_RAY: RefCell<Ray> = RefCell::new(Ray::default());
}

/*
pub trait ModelDynamicClone {
    fn dynamic_clone(&self) -> Box<dyn Model>;
}

impl<T: Model + Clone + 'static> ModelDynamicClone for T {
    fn dynamic_clone(&self) -> Box<dyn Model> {
        Box::new(self.clone())
    }
}

pub trait ModelDynamicPartialEq {
    fn dynamic_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Model + PartialEq + 'static> ModelDynamicPartialEq for T {
    fn dynamic_eq(&self, other: &dyn Any) -> bool {
        if let Some(other) = other.downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}
    */

pub trait ModelAsAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Model + 'static> ModelAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait DynamicModel: Debug + ModelAsAny {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;

    fn local_normal_at(&self, local_point: Point) -> Vector;

    fn dynamic_clone(&self) -> Box<dyn DynamicModel>;

    fn dynamic_eq(&self, other: &dyn DynamicModel) -> bool;
}

impl<T: Model + Clone + Debug + PartialEq + 'static> DynamicModel for T {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        self.local_intersect(local_ray)
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        self.local_normal_at(local_point)
    }

    fn dynamic_clone(&self) -> Box<dyn DynamicModel> {
        Box::new(self.clone())
    }

    fn dynamic_eq(&self, other: &dyn DynamicModel) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

pub trait Model: Debug + 'static {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;

    fn local_normal_at(&self, local_point: Point) -> Vector;
}

#[derive(Debug)]
pub struct Shape {
    transform: Transform,
    inverse: Transform,
    pub material: Material,
    pub model: Box<dyn DynamicModel>,
}

impl Shape {
    pub fn new(model: impl DynamicModel) -> Self {
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

    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.inverse * point;
        let local_normal = self.model.local_normal_at(local_point);
        let local_normal_matrix =
            Matrix::new([[local_normal.x], [local_normal.y], [local_normal.z]]);
        let world_normal_matrix = &self
            .inverse
            .submatrix(3, 3)
            .expect("matrix index error")
            .transpose()
            * &local_normal_matrix;
        Vector::new(
            world_normal_matrix[[0, 0]],
            world_normal_matrix[[1, 0]],
            world_normal_matrix[[2, 0]],
        )
        .normalize()
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

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse == other.inverse
            && self.material == other.material
            && self.model.dynamic_eq(other.model.as_ref())
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::{
        matrices::IDENTITY,
        transformations::{rotation_z, scaling, translation},
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

        fn local_normal_at(&self, local_point: Point) -> Vector {
            Vector::new(local_point.x, local_point.y, local_point.z)
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

    #[test]
    fn normal_translated_shape() {
        let mut s = Shape::new(TestModel);
        s.set_transform(translation(0.0, 1.0, 0.0)).unwrap();
        let n = s.normal_at(Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_transformed_shape() {
        let mut s = Shape::new(TestModel);
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transform(m).unwrap();
        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
