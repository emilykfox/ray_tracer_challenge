use crate::{
    intersections::{Intersection, Intersections},
    material::Material,
    matrices::{Matrix, NoInverseError, Transform, IDENTITY},
    rays::Ray,
    Point, Vector,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sphere {
    pub material: Material,
    transform: Transform,
    inverse: Transform,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            material: Material::default(),
            transform: IDENTITY,
            inverse: IDENTITY,
        }
    }

    pub fn set_transform(&mut self, transform: Transform) -> Result<(), NoInverseError> {
        let inverse = transform.inverse()?;
        self.transform = transform;
        self.inverse = inverse;
        Ok(())
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let ray2 = ray.transformed(&self.inverse);
        let sphere_to_ray = ray2.origin - Point::new(0.0, 0.0, 0.0);

        let a = Vector::dot(ray2.direction, ray2.direction);
        let b = 2.0 * Vector::dot(ray2.direction, sphere_to_ray);
        let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            Intersections::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ])
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let object_point = &self.inverse * point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let object_normal_matrix =
            Matrix::new([[object_normal.x], [object_normal.y], [object_normal.z]]);
        let world_normal_matrix = &self
            .inverse
            .submatrix(3, 3)
            .expect("matrix index error")
            .transpose()
            * &object_normal_matrix;
        Vector::new(
            world_normal_matrix[[0, 0]],
            world_normal_matrix[[1, 0]],
            world_normal_matrix[[2, 0]],
        )
        .normalize()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::{
        matrices::IDENTITY,
        rays::Ray,
        transformations::{rotation_z, scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn intersect_twice() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 4.0);
        assert_eq!(xs.vec[1].t, 6.0);
    }

    #[test]
    fn tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 5.0);
        assert_eq!(xs.vec[1].t, 5.0);
    }

    #[test]
    fn miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 0);
    }

    #[test]
    fn from_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, -1.0);
        assert_eq!(xs.vec[1].t, 1.0);
    }

    #[test]
    fn behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, -6.0);
        assert_eq!(xs.vec[1].t, -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].object, &s);
        assert_eq!(xs.vec[1].object, &s);
    }

    #[test]
    fn default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn change_transform() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone()).unwrap();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 3.0);
        assert_eq!(xs.vec[1].t, 7.0);
    }

    #[test]
    fn intersect_translated() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 0);
    }

    #[test]
    fn normal_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_nonaxial() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0)).unwrap();
        let n = s.normal_at(Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);
        s.set_transform(m).unwrap();
        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn default_material() {
        let s = Sphere::new();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = Sphere::new();
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}
