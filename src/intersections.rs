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
}

impl<'objects> Intersections<'objects> {
    pub fn new(intersections: Vec<Intersection<'objects>>) -> Self {
        Intersections { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
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
}
