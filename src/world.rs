use crate::{
    intersections::{Intersection, Intersections},
    lights::PointLight,
    rays::Ray,
    spheres::{IntersectingSphereError, Sphere},
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        World::default()
    }

    pub fn intersect(&self, ray: Ray) -> Result<Intersections, IntersectingSphereError> {
        let mut intersections = self
            .objects
            .iter()
            .map(|object| object.intersect(ray))
            .collect::<Result<Vec<Intersections>, IntersectingSphereError>>()?
            .into_iter()
            .flat_map(|intersections| intersections.vec.into_iter())
            .collect::<Vec<Intersection>>();
        intersections.sort_by(|x, y| x.t().total_cmp(&y.t()));
        Ok(Intersections::new(intersections))
    }
}

#[cfg(test)]
mod test {
    use crate::{canvas::Color, rays::Ray, transformations::Builder, Point, Vector};

    use super::*;

    #[test]
    fn create_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert_eq!(w.light, None);
    }

    fn default_world() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(Builder::new().scaling(0.5, 0.5, 0.5).transform());
        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(Builder::new().scaling(0.5, 0.5, 0.5).transform());

        let w = default_world();
        assert_eq!(w.light, Some(light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(r).unwrap();
        assert_eq!(xs.vec.len(), 4);
        assert_eq!(xs.vec[0].t(), 4.0);
        assert_eq!(xs.vec[1].t(), 4.5);
        assert_eq!(xs.vec[2].t(), 5.5);
        assert_eq!(xs.vec[3].t(), 6.0);
    }
}
