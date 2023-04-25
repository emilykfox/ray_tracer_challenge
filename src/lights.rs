use crate::{canvas::Color, Point};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        PointLight {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_light_fields() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
