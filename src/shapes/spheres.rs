use crate::{rays::Ray, Point, Vector};

use super::Model;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sphere;

impl Model for Sphere {
    fn local_intersect(&self, local_ray: &'_ Ray) -> Vec<f64> {
        let sphere_to_ray = local_ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = Vector::dot(local_ray.direction, local_ray.direction);
        let b = 2.0 * Vector::dot(local_ray.direction, sphere_to_ray);
        let c = Vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        local_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::{
        matrices::IDENTITY,
        rays::Ray,
        shapes::Shape,
        transformations::{rotation_z, scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn intersect_twice() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn from_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::new(Sphere);
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert!(std::ptr::eq(xs.vec[0].object, &s));
        assert!(std::ptr::eq(xs.vec[1].object, &s));
    }

    #[test]
    fn intersect_scaled() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(Sphere);
        s.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 3.0);
        assert_eq!(xs.vec[1].t, 7.0);
    }

    #[test]
    fn intersect_translated() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::new(Sphere);
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        let xs = s.intersect(&r);
        assert_eq!(xs.vec.len(), 0);
    }

    #[test]
    fn normal_on_x_axis() {
        todo!() /*
                let s = Shape::new(Sphere);
                let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
                assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
                */
    }

    #[test]
    fn normal_on_y_axis() {
        todo!(); /*
                 let s = Sphere::new();
                 let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
                 assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
                 */
    }

    #[test]
    fn normal_on_z_axis() {
        todo!() /*
                let s = Sphere::new();
                let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
                assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
                */
    }

    #[test]
    fn normal_nonaxial() {
        todo!() /*
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
                */
    }

    #[test]
    fn normal_is_normalized() {
        todo!(); /*
                 let s = Sphere::new();
                 let n = s.normal_at(Point::new(
                     3.0_f64.sqrt() / 3.0,
                     3.0_f64.sqrt() / 3.0,
                     3.0_f64.sqrt() / 3.0,
                 ));
                 assert_eq!(n, n.normalize());
                 */
    }
}
