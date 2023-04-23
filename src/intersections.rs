use crate::spheres::Sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection<'object> {
    t: f64,
    object: &'object Sphere,
}

impl<'object> Intersection<'object> {
    pub fn new(t: f64, object: &'object Sphere) -> Self {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'object Sphere {
        self.object
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections<'objects> {
    intersections: Vec<Intersection<'objects>>,
    first_hit: Option<Intersection<'objects>>,
}

impl<'objects> Intersections<'objects> {
    pub fn new(intersections: Vec<Intersection<'objects>>) -> Self {
        let first_hit = intersections
            .iter()
            .filter(|intersection| intersection.t() >= 0.0)
            .min_by(|x, y| x.t().total_cmp(&y.t()))
            .copied();
        Intersections {
            intersections,
            first_hit,
        }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn hit(&self) -> Option<Intersection<'objects>> {
        self.first_hit
    }
}

impl<'objects> std::ops::Index<usize> for Intersections<'objects> {
    type Output = Intersection<'objects>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
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
        assert_eq!(i.t(), 3.5);
        assert_eq!(i.object(), &s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[1].t(), 2.0);
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, Some(i1));
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, Some(i2));
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
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(i, Some(i4));
    }
}
