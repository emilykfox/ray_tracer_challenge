use crate::EQUALITY_EPSILON;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

#[derive(Default, Debug, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.red - other.red).abs() < EQUALITY_EPSILON
            && (self.green - other.green).abs() < EQUALITY_EPSILON
            && (self.blue - other.blue).abs() < EQUALITY_EPSILON
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct PixelOutOfBoundsError;

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Result<Color, PixelOutOfBoundsError> {
        if x >= self.width || y >= self.height {
            Err(PixelOutOfBoundsError)
        } else {
            Ok(self.pixels[y * self.width + x])
        }
    }

    pub fn write_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Color,
    ) -> Result<(), PixelOutOfBoundsError> {
        if x >= self.width || y >= self.height {
            Err(PixelOutOfBoundsError)
        } else {
            self.pixels[y * self.width + x] = color;
            Ok(())
        }
    }

    pub fn to_ppm(&self) -> String {
        format!(
            "P3\n\
            {} {}\n\
            255\n",
            self.width, self.height,
        ) + &(0..self.height)
            .map(|y| {
                let mut line_length = 0;
                (0..self.width)
                    .map(|x| {
                        let pixel = self.pixels[y * self.width + x];
                        let colors = vec![pixel.red, pixel.green, pixel.blue];
                        colors
                            .into_iter()
                            .map(|color| {
                                // Need to manually build lines so max char length is 70
                                let color = ((color * 256.0) as i64).clamp(0, 255).to_string();
                                // Assumes first color isn't > 70 characters long
                                let pad = if line_length == 0 {
                                    line_length += color.len();
                                    "".to_string()
                                } else if line_length + 1 + color.len() > 70 {
                                    line_length = color.len();
                                    "\n".to_string()
                                } else {
                                    line_length += 1 + color.len();
                                    " ".to_string()
                                };
                                pad + &color
                            })
                            .collect::<String>()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_colors() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
        assert_eq!(2.0 * c, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        for i in 0..10 {
            for j in 0..20 {
                assert_eq!(c.pixel_at(i, j), Ok(Color::new(0.0, 0.0, 0.0)));
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red)
            .expect("cannot write: pixel out of bounds");
        assert_eq!(c.pixel_at(2, 3), Ok(red));
    }

    #[test]
    fn out_of_bounds() {
        let mut c = Canvas::new(10, 20);
        assert_eq!(
            c.write_pixel(10, 20, Color::default()),
            Err(PixelOutOfBoundsError)
        );
        assert_eq!(c.pixel_at(10, 20), Err(PixelOutOfBoundsError));
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm: String = c.to_ppm();
        assert!(ppm.starts_with(
            "P3\n\
            5 3\n\
            255\n"
        ));
    }

    #[test]
    fn ppm_from_pixels() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1)
            .expect("cannot write: pixel out of bounds");
        c.write_pixel(2, 1, c2)
            .expect("cannot write: pixel out of bounds");
        c.write_pixel(4, 2, c3)
            .expect("cannot write: pixel out of bounds");
        let ppm = c.to_ppm();
        assert_eq!(
            ppm.lines().skip(3).take(3).collect::<Vec<&str>>(),
            vec![
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
            ]
        );
    }

    #[test]
    fn ppm_split_long_lines() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, color)
                    .expect("cannot write: pixel out of bounds");
            }
        }
        let ppm = c.to_ppm();
        assert_eq!(
            ppm.lines().skip(3).take(4).collect::<Vec<&str>>(),
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ]
        );
    }

    #[test]
    fn ppm_ends_with_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with('\n'));
    }
}
