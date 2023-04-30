use crate::Vector;

use super::ShapeModel;

const PARALLEL_EPSILON: f64 = 0.00001;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Plane;

impl ShapeModel for Plane {
    fn local_intersect(&self, local_ray: &crate::rays::Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < PARALLEL_EPSILON {
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;
            vec![t]
        }
    }

    fn local_normal_at(&self, _local_point: crate::Point) -> crate::Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::{rays::Ray, Point};

    use super::*;

    #[test]
    fn constant_normal() {
        let p = Plane;
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_parallel_ray() {
        let p = Plane;
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_coplanar_ray() {
        let p = Plane;
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_from_above() {
        let p = Plane;
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn intersect_from_below() {
        let p = Plane;
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
