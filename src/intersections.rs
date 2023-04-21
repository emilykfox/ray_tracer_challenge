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
}
