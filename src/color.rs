use crate::utils::*;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new_color(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn normalise(&self) -> (u8, u8, u8) {
        (
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
    }
}

pub const AZURE_BLUE: Color = Color {
    red: 0.0,
    green: 0.5,
    blue: 1.0,
};

pub const LIGHT_VIOLET: Color = Color {
    red: 0.5,
    green: 0.5,
    blue: 1.0,
};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        compare_float(self.green, other.green)
            && compare_float(self.blue, other.blue)
            && compare_float(self.red, other.red)
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn color_creation() {
        let color = Color::new_color(-0.5, 0.4, -1.7);
        assert!(compare_float(color.red, -0.5));
        assert!(compare_float(color.green, 0.4));
        assert!(compare_float(color.blue, -1.7));

        let color_2 = Color::new_color(0.5, 0.4, 1.7);
        assert!(compare_float(color_2.red, 0.5));
        assert!(compare_float(color_2.green, 0.4));
        assert!(compare_float(color_2.blue, 1.7));
    }

    #[test]
    fn color_addition() {
        let color = Color::new_color(0.9, 0.6, 0.75);
        let color_2 = Color::new_color(0.7, 0.1, 0.25);
        assert_eq!(color + color_2, Color::new_color(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_substraction() {
        let color = Color::new_color(0.9, 0.6, 0.75);
        let color_2 = Color::new_color(0.7, 0.1, 0.25);
        assert_eq!(color - color_2, Color::new_color(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_mult_by_a_scalar() {
        let color = Color::new_color(0.2, 0.3, 0.4);
        assert_eq!(color * 2.0, Color::new_color(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_multiplication() {
        let color = Color::new_color(1.0, 0.2, 0.4);
        let color_2 = Color::new_color(0.9, 1.0, 0.1);
        assert_eq!(color * color_2, Color::new_color(0.9, 0.2, 0.04));
    }
}