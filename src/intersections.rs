use crate::{shapes::Shape, EQUALITY_EPSILON};

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
    pub vec: Vec<Intersection<'objects>>,
}

impl<'objects> Intersections<'objects> {
    pub fn new(vec: Vec<Intersection<'objects>>) -> Self {
        Intersections { vec }
    }

    pub fn hit(&self) -> Option<&Intersection<'objects>> {
        self.vec
            .iter()
            .filter(|intersection| intersection.t >= 0.0)
            .min_by(|x, y| x.t.total_cmp(&y.t))
    }
}

#[cfg(test)]
mod test {
    use crate::spheres::Sphere;

    use super::*;

    #[test]
    fn create_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.vec.len(), 2);
        assert_eq!(xs.vec[0].t, 1.0);
        assert_eq!(xs.vec[1].t, 2.0);
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i1);
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i2);
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_lowest_nonnegative() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i4);
    }
}
