use crate::{
    canvas::Color, lights::PointLight, patterns::StripePattern, shapes::Shape, Point, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<StripePattern>,
}

impl Material {
    pub fn new() -> Self {
        Material::default()
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
}

pub fn lighting(
    material: &Material,
    object: &Shape,
    light: &PointLight,
    point: Point,
    eyev: Vector,
    normal: Vector,
    in_shadow: bool,
) -> Color {
    let color = material.pattern.as_ref().map_or(material.color, |pattern| {
        pattern.stripe_at_object(object, point)
    });
    let effective_color = color * light.intensity;
    let lightv = (light.position - point).normalize();

    let ambient = effective_color * material.ambient;
    if in_shadow {
        return ambient;
    }

    let diffuse;
    let specular;
    let light_dot_normal = Vector::dot(lightv, normal);
    if light_dot_normal < 0.0 {
        diffuse = Color::default();
        specular = Color::default();
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normal);
        let reflect_dot_eye = Vector::dot(reflectv, eyev);
        if reflect_dot_eye <= 0.0 {
            specular = Color::default();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use crate::{
        canvas::{Color, BLACK, WHITE},
        lights::PointLight,
        patterns::StripePattern,
        shapes::Sphere,
        Point, Vector,
    };

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_surface_eye_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_between_light_and_surface_light_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn eye_between_light_and_surface_both_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -(2_f64.sqrt()) / 2.0, -(2_f64.sqrt()) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_shadow() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            position,
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern() {
        let m = Material {
            pattern: Some(StripePattern::new(WHITE, BLACK)),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Material::default()
        };
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), WHITE);
        let c1 = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normal,
            false,
        );
        let c2 = lighting(
            &m,
            &Shape::new(Sphere),
            &light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normal,
            false,
        );
        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
}
