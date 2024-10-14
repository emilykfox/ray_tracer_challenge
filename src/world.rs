use crate::{
    canvas::Color,
    intersections::{HitInfo, Intersection, Intersections},
    lights::PointLight,
    materials::lighting,
    rays::Ray,
    shapes::Shape,
    Point, Vector,
};

pub const RECURSION_DEPTH: usize = 5;

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
        let vec = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray).into_iter())
            .collect::<Vec<Intersection>>();
        Intersections::new(vec)
    }

    pub fn shade_hit(&self, hit_info: &HitInfo, remaining: usize) -> Color {
        let is_shadowed = self.is_shadowed(hit_info.over_point);
        let surface = lighting(
            &hit_info.object.material,
            hit_info.object,
            &self.light,
            hit_info.point,
            hit_info.eyev,
            hit_info.normal,
            is_shadowed,
        );

        let reflected = self.reflected_color(hit_info, remaining);
        let refracted = self.refracted_color(hit_info, remaining);

        surface + reflected + refracted
    }

    pub fn color_from(&self, ray: &Ray, remaining: usize) -> Color {
        let intersections = self.intersect(ray);
        let Some(hit_index) = intersections.hit() else {
            return Color::default();
        };
        let hit_info = HitInfo::prepare(&intersections, ray, hit_index).expect("invalid hit index");
        self.shade_hit(&hit_info, remaining)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let light_to_point = self.light.position - point;
        let distance = light_to_point.magnitude();
        let direction = light_to_point.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray);
        if let Some(hit_index) = intersections.hit() {
            intersections[hit_index].t < distance
        } else {
            false
        }
    }

    pub fn reflected_color(&self, hit_info: &HitInfo, remaining: usize) -> Color {
        if remaining == 0 || hit_info.object.material.reflective == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(hit_info.over_point, hit_info.reflectv);
        let color = self.color_from(&reflect_ray, remaining - 1);

        color * hit_info.object.material.reflective
    }

    pub fn refracted_color(&self, hit_info: &HitInfo, remaining: usize) -> Color {
        if remaining == 0 || hit_info.object.material.transparaency == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let n_ratio = hit_info.n1 / hit_info.n2;
        let cos_i = Vector::dot(hit_info.eyev, hit_info.normal);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = hit_info.normal * (n_ratio * cos_i - cos_t) - hit_info.eyev * n_ratio;
        let refract_ray = Ray::new(hit_info.under_point, direction);
        self.color_from(&refract_ray, remaining - 1) * hit_info.object.material.transparaency
    }

    pub fn schlick(hit_info: &HitInfo) -> f64 {
        let mut cos = Vector::dot(hit_info.eyev, hit_info.normal);
        if hit_info.n1 > hit_info.n2 {
            let n = hit_info.n1 / hit_info.n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let ratio = (hit_info.n1 - hit_info.n2) / (hit_info.n1 + hit_info.n2);
        let r0 = ratio * ratio;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
pub(crate) fn default_world() -> World {
    use crate::{shapes::Sphere, transformations::Builder};

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
        patterns::{Pattern, TestPattern},
        rays::Ray,
        shapes::{Plane, Sphere},
        transformations::{translation, Builder},
        Point, Vector, EQUALITY_EPSILON,
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
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let c = w.shade_hit(&hit_info, RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let c = w.shade_hit(&hit_info, RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_from_miss() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_from(&r, RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_from_hit() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_from(&r, RECURSION_DEPTH);
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
        let c = w.color_from(&r, RECURSION_DEPTH);
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
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let c = w.shade_hit(&hit_info, RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_nonreflective_material() {
        let mut w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &mut w.objects[1];
        shape.material.ambient = 1.0;
        let shape = &w.objects[1];
        let i = Intersection::new(1.0, shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.reflected_color(&hit_info, RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn reflected_color_reflective_material() {
        let mut w = default_world();
        let mut shape = Shape::new(Plane);
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape);
        let shape = &w.objects[2];
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.reflected_color(&hit_info, RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = default_world();
        let mut shape = Shape::new(Plane);
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape);
        let shape = &w.objects[2];
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.shade_hit(&hit_info, RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = PointLight::new(Point::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        let mut lower = Shape::new(Plane);
        lower.material.reflective = 1.0;
        lower.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(lower);
        let mut upper = Shape::new(Plane);
        upper.material.reflective = 1.0;
        upper.set_transform(translation(0.0, 1.0, 0.0)).unwrap();
        w.objects.push(upper);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        w.color_from(&r, RECURSION_DEPTH);
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = default_world();
        let mut shape = Shape::new(Plane);
        shape.material.reflective = 0.5;
        shape.set_transform(translation(0.0, -1.0, 0.0)).unwrap();
        w.objects.push(shape);
        let shape = &w.objects[2];
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), shape);
        let xs = Intersections::new(vec![i.clone()]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.reflected_color(&hit_info, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_opaque_material() {
        let w = default_world();
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.refracted_color(&hit_info, RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = default_world();
        let shape = &mut w.objects[0];
        shape.material.transparaency = 1.0;
        shape.material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.refracted_color(&hit_info, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_total_internal_reflection() {
        let mut w = default_world();
        let shape = &mut w.objects[0];
        shape.material.transparaency = 1.0;
        shape.material.refractive_index = 1.5;
        let shape = &w.objects[0];
        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(vec![
            Intersection::new(-(2_f64.sqrt()) / 2.0, shape),
            Intersection::new(2_f64.sqrt() / 2.0, shape),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 1).unwrap();
        let color = w.refracted_color(&hit_info, 5);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn refracted_color_from_refracted_ray() {
        let mut w = default_world();
        let a = &mut w.objects[0];
        a.material.ambient = 1.0;
        a.material.pattern = Some(Pattern::new(TestPattern));
        let b = &mut w.objects[1];
        b.material.transparaency = 1.0;
        b.material.refractive_index = 1.5;
        let a = &w.objects[0];
        let b = &w.objects[1];
        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 2).unwrap();
        let color = w.refracted_color(&hit_info, 5);
        assert_eq!(color, Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = default_world();
        let mut floor = Shape::new(Plane);
        let _ = floor.set_transform(translation(0.0, -1.0, 0.0));
        floor.material.transparaency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);
        let mut ball = Shape::new(Sphere);
        let _ = ball.set_transform(translation(0.0, -3.5, -0.5));
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        w.objects.push(ball);
        let floor = &w.objects[w.objects.len() - 2];
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new(vec![Intersection::new(2_f64.sqrt(), floor)]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let color = w.shade_hit(&hit_info, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn schlick_approx_total_internal_reflection() {
        let shape = Sphere::new_glass();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(vec![
            Intersection::new(-(2_f64.sqrt() / 2.0), &shape),
            Intersection::new(2_f64.sqrt() / 2.0, &shape),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 1).unwrap();
        let reflectance = World::schlick(&hit_info);
        assert!((reflectance - 1.0) < EQUALITY_EPSILON);
    }

    #[test]
    fn schlick_approx_perpendicular() {
        let shape = Sphere::new_glass();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ]);
        let hit_info = HitInfo::prepare(&xs, &r, 1).unwrap();
        let reflectance = World::schlick(&hit_info);
        assert!((reflectance - 0.04) < EQUALITY_EPSILON);
    }

    #[test]
    fn schlick_approx_small_angle_n2_gt_n1() {
        let shape = Sphere::new_glass();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![Intersection::new(1.8589, &shape)]);
        let hit_info = HitInfo::prepare(&xs, &r, 0).unwrap();
        let reflectance = World::schlick(&hit_info);
        assert!((reflectance - 0.48873) < EQUALITY_EPSILON);
    }
}
