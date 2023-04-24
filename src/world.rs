use crate::{
    canvas::Color,
    intersections::{Intersection, Intersections},
    lights::PointLight,
    material::lighting,
    rays::Ray,
    spheres::{IntersectingSphereError, Sphere},
    Point, Vector,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct NoLightsourceError;

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
        intersections.sort_by(|x, y| x.t.total_cmp(&y.t));
        Ok(Intersections::new(intersections))
    }

    pub fn shade_hit(&self, hit_info: HitInfo) -> Result<Color, NoLightsourceError> {
        let Some(light) = self.light else {
            return Err(NoLightsourceError);
        };

        Ok(lighting(
            hit_info.object.material,
            light,
            hit_info.point,
            hit_info.eyev,
            hit_info.normal,
        ))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitInfo<'object> {
    pub t: f64,
    pub object: &'object Sphere,
    pub point: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct NormalTransformationError;

impl<'object> HitInfo<'object> {
    pub fn prepare(
        intersection: Intersection<'object>,
        ray: Ray,
    ) -> Result<Self, NormalTransformationError> {
        let t = intersection.t;
        let object = intersection.object;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let naive_normal = object
            .normal_at(point)
            .map_err(|_| NormalTransformationError)?;
        let inside = Vector::dot(naive_normal, eyev) < 0.0;
        let normal = if inside { -naive_normal } else { naive_normal };
        Ok(HitInfo {
            t,
            object,
            point,
            eyev,
            normal,
            inside,
        })
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
        assert_eq!(xs.vec[0].t, 4.0);
        assert_eq!(xs.vec[1].t, 4.5);
        assert_eq!(xs.vec[2].t, 5.5);
        assert_eq!(xs.vec[3].t, 6.0);
    }

    #[test]
    fn create_hit_info() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let hit_info = HitInfo::prepare(i, r).unwrap();
        assert_eq!(hit_info.t, i.t);
        assert_eq!(hit_info.object, &shape);
        assert_eq!(hit_info.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(hit_info.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(hit_info.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let hit_info = HitInfo::prepare(i, r).unwrap();
        assert!(!hit_info.inside);
    }

    #[test]
    fn hit_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let hit_info = HitInfo::prepare(i, r).unwrap();
        assert_eq!(hit_info.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(hit_info.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(hit_info.inside);
        assert_eq!(hit_info.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn shading_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, &shape);
        let hit_info = HitInfo::prepare(i, r).unwrap();
        let c = w.shade_hit(hit_info);
        assert_eq!(c, Ok(Color::new(0.38066, 0.47583, 0.2855)));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let hit_info = HitInfo::prepare(i, r).unwrap();
        let c = w.shade_hit(hit_info);
        assert_eq!(c, Ok(Color::new(0.90498, 0.90498, 0.90498)));
    }
}
