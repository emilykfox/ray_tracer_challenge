use crate::{rays::Ray, shapes::Shape, Point, Vector, EQUALITY_EPSILON};

const SHADOW_EPSILON: f64 = 0.00001;

#[derive(Debug, Clone)]
pub struct Intersection<'object> {
    pub t: f64,
    pub object: &'object Shape,
}

impl<'object> Intersection<'object> {
    pub fn new(t: f64, object: &'object Shape) -> Self {
        Intersection { t, object }
    }
}

impl<'object> PartialEq for Intersection<'object> {
    fn eq(&self, other: &Self) -> bool {
        (self.t - other.t) < EQUALITY_EPSILON && std::ptr::eq(self.object, other.object)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections<'objects> {
    vec: Vec<Intersection<'objects>>,
}

impl<'objects> Intersections<'objects> {
    pub fn new(mut vec: Vec<Intersection<'objects>>) -> Self {
        vec.sort_by(|x, y| x.t.total_cmp(&y.t));
        Intersections { vec }
    }

    pub fn hit(&self) -> Option<&Intersection<'objects>> {
        match self.vec.binary_search_by(|probe| probe.t.total_cmp(&0.0)) {
            Ok(index) | Err(index) => self.vec.get(index),
        }
    }
}

impl<'objects> std::ops::Deref for Intersections<'objects> {
    type Target = Vec<Intersection<'objects>>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<'objects> std::iter::IntoIterator for Intersections<'objects> {
    type Item = Intersection<'objects>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitInfo<'object> {
    pub t: f64,
    pub object: &'object Shape,
    pub point: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub reflectv: Vector,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct NormalTransformationError;

impl<'object> HitInfo<'object> {
    pub fn prepare(intersection: &Intersection<'object>, ray: &Ray) -> Self {
        let t = intersection.t;
        let object = intersection.object;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let naive_normal = object.normal_at(point);
        let inside = Vector::dot(naive_normal, eyev) < 0.0;
        let normal = if inside { -naive_normal } else { naive_normal };
        let over_point = point + normal * SHADOW_EPSILON;
        let reflectv = ray.direction.reflect(normal);
        HitInfo {
            t,
            object,
            point,
            eyev,
            normal,
            inside,
            over_point,
            reflectv,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        rays::Ray,
        shapes::{Plane, Sphere},
        transformations::{scaling, translation},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn create_intersection() {
        let s = Shape::new(Sphere);
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Shape::new(Sphere);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 1.0);
        assert_eq!(xs.vec[1].t, 2.0);
    }

    #[test]
    fn hit_all_positive() {
        let s = Shape::new(Sphere);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i1);
    }

    #[test]
    fn hit_some_negative() {
        let s = Shape::new(Sphere);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i2);
    }

    #[test]
    fn hit_all_negative() {
        let s = Shape::new(Sphere);
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_lowest_nonnegative() {
        let s = Shape::new(Sphere);
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i4);
    }

    #[test]
    fn create_hit_info() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::new(Sphere);
        let i = Intersection::new(4.0, &shape);
        let hit_info = HitInfo::prepare(&i, &r);
        assert_eq!(hit_info.t, i.t);
        assert_eq!(hit_info.object, &shape);
        assert_eq!(hit_info.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(hit_info.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(hit_info.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::new(Sphere);
        let i = Intersection::new(4.0, &shape);
        let hit_info = HitInfo::prepare(&i, &r);
        assert!(!hit_info.inside);
    }

    #[test]
    fn hit_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::new(Sphere);
        let i = Intersection::new(1.0, &shape);
        let hit_info = HitInfo::prepare(&i, &r);
        assert_eq!(hit_info.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(hit_info.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(hit_info.inside);
        assert_eq!(hit_info.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Shape::new(Sphere);
        shape.set_transform(translation(0.0, 0.0, 1.0)).unwrap();
        let i = Intersection::new(5.0, &shape);
        let hit_info = HitInfo::prepare(&i, &r);
        assert!(hit_info.over_point.z < -SHADOW_EPSILON / 2.0);
        assert!(hit_info.point.z > hit_info.over_point.z);
    }

    #[test]
    fn precompute_reflection_vector() {
        let shape = Shape::new(Plane);
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let hit_info = HitInfo::prepare(&i, &r);
        assert_eq!(
            hit_info.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn various_n1_and_n2() {
        let mut a = Sphere::new_glass();
        a.set_transform(scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = Sphere::new_glass();
        b.set_transform(translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = Sphere::new_glass();
        c.set_transform(translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ]);

        let examples = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for (index, pair) in examples.iter().enumerate() {
            let hit_info = HitInfo::prepare(&xs.vec[index], &r);
            todo!();
            //TODO: Finish test assert_eq!(hit_info.n1, pair.0);
            //assert_eq!(hit_info.n2, pair.1);
        }
    }
}
