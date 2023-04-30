use crate::{canvas::Color, Point};

use super::PatternModel;

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Gradient { a, b }
    }
}

impl PatternModel for Gradient {
    fn at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod test {
    use crate::canvas::{BLACK, WHITE};

    use super::*;

    #[test]
    fn gradient_linearly_interpolates() {
        let gradient = Gradient::new(WHITE, BLACK);
        assert_eq!(gradient.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            gradient.at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            gradient.at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            gradient.at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
