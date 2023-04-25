use crate::{canvas::Color, lights::PointLight, Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
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
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: Point,
    eyev: Vector,
    normal: Vector,
) -> Color {
    let effective_color = material.color * light.intensity();
    let lightv = (light.position() - point).normalize();

    let ambient = effective_color * material.ambient;

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
            specular = light.intensity() * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use crate::{canvas::Color, lights::PointLight, Point, Vector};

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
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_surface_eye_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_between_light_and_surface_light_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn eye_between_light_and_surface_both_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -(2_f64.sqrt()) / 2.0, -(2_f64.sqrt()) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
