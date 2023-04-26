use crate::{
    canvas::Color,
    intersections::{Intersection, Intersections},
    lights::PointLight,
    material::lighting,
    rays::Ray,
    shapes::Shape,
    Point, Vector,
};

const SHADOW_EPSILON: f64 = 0.00001;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Shape>,
    pub light: PointLight,
}

impl World {
    pub fn new() -> Self {
        World::default()
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut vec = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray).vec.into_iter())
            .collect::<Vec<Intersection>>();
        vec.sort_by(|x, y| x.t.total_cmp(&y.t));
        Intersections::new(vec)
    }

    pub fn shade_hit(&self, hit_info: &HitInfo) -> Color {
        let is_shadowed = self.is_shadowed(hit_info.over_point);
        lighting(
            &hit_info.object.material,
            &self.light,
            hit_info.point,
            hit_info.eyev,
            hit_info.normal,
            is_shadowed,
        )
    }

    pub fn color_from(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = intersections.hit();
        let Some(hit) = hit else {
            return Color::default();
        };
        let hit_info = HitInfo::prepare(hit, ray);
        self.shade_hit(&hit_info)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let light_to_point = self.light.position - point;
        let distance = light_to_point.magnitude();
        let direction = light_to_point.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray);
        let hit = intersections.hit();
        if let Some(hit) = hit {
            hit.t < distance
        } else {
            false
        }
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
        HitInfo {
            t,
            object,
            point,
            eyev,
            normal,
            inside,
            over_point,
        }
    }
}

#[cfg(test)]
pub(crate) fn default_world() -> World {
    use crate::{shapes::spheres::Sphere, transformations::Builder};

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::new(Sphere);
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Shape::new(Sphere);
    s2.set_transform(Builder::new().scaling(0.5, 0.5, 0.5).transform())
        .unwrap();
    World {
        objects: vec![s1, s2],
        light,
    }
}

#[cfg(test)]
mod test {
    use crate::{
        canvas::Color,
        rays::Ray,
        shapes::spheres::Sphere,
        transformations::{translation, Builder},
        Point, Vector,
    };

    use super::*;

    #[test]
    fn create_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert_eq!(w.light, PointLight::default());
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Shape::new(Sphere);
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Shape::new(Sphere);
        s2.set_transform(Builder::new().scaling(0.5, 0.5, 0.5).transform())
            .unwrap();

        let w = default_world();
        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.vec.len(), 4);
        assert_eq!(xs.vec[0].t, 4.0);
        assert_eq!(xs.vec[1].t, 4.5);
        assert_eq!(xs.vec[2].t, 5.5);
        assert_eq!(xs.vec[3].t, 6.0);
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
    fn shading_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let hit_info = HitInfo::prepare(&i, &r);
        let c = w.shade_hit(&hit_info);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let hit_info = HitInfo::prepare(&i, &r);
        let c = w.shade_hit(&hit_info);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_from_miss() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_from(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_from_hit() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_from(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let inner = &w.objects[1];
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_from(&r);
        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn no_object_on_line_shadow() {
        let w = default_world();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn object_between_shadow() {
        let w = default_world();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn object_behind_light_shadow() {
        let w = default_world();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn object_other_side_shadow() {
        let w = default_world();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_given_shadowed() {
        let mut w = World::new();
        w.light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = Shape::new(Sphere);
        w.objects.push(s1);
        let mut s2 = Shape::new(Sphere);
        s2.set_transform(translation(0.0, 0.0, 10.0)).unwrap();
        w.objects.push(s2);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[1]);
        let hit_info = HitInfo::prepare(&i, &r);
        let c = w.shade_hit(&hit_info);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
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
}
