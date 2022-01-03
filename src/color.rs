use crate::utils::*;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, Debug, Clone)]
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

const AZURE_BLUE: Color = Color {
    red: 0.0,
    green: 0.5,
    blue: 1.0,
};
const LIGHT_VIOLET: Color = Color {
    red: 0.5,
    green: 0.5,
    blue: 1.0,
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
}

// ​Scenario​: Adding colors
// ​ 	  ​Given​ c1 ← color(0.9, 0.6, 0.75)
// ​ 	    ​And​ c2 ← color(0.7, 0.1, 0.25)
// ​ 	   ​Then​ c1 + c2 = color(1.6, 0.7, 1.0)
// ​
// ​ 	​Scenario​: Subtracting colors
// ​ 	  ​Given​ c1 ← color(0.9, 0.6, 0.75)
// ​ 	    ​And​ c2 ← color(0.7, 0.1, 0.25)
// ​ 	   ​Then​ c1 - c2 = color(0.2, 0.5, 0.5)
// ​
// ​ 	​Scenario​: Multiplying a color by a scalar
// ​ 	  ​Given​ c ← color(0.2, 0.3, 0.4)
// ​ 	  ​Then​ c * 2 = color(0.4, 0.6, 0.8)

// ​Scenario​: Multiplying colors
// ​ 	  ​Given​ c1 ← color(1, 0.2, 0.4)
// ​ 	    ​And​ c2 ← color(0.9, 1, 0.1)
// ​ 	   ​Then​ c1 * c2 = color(0.9, 0.2, 0.04)
