use std::usize;

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>
}

pub fn pos_from_index(index: usize, canvas: &Canvas) -> (usize, usize) {
    let y = index / canvas.width;
    let x = index % canvas.width;
    (x, y)
}

 pub fn index_from_pos(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

impl Canvas {
   pub  fn new_canvas(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new_color(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn new_canvas_with_color(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn set_pixel_color(&mut self, x_pos: usize, y_pos: usize, color: Color) {
        self.pixels[index_from_pos(x_pos, y_pos, self.width)] = color;
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
///Creating a canvas
    fn canvas_creation() {
        let canvas = Canvas::new_canvas(10, 20);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.width, 10);
        for color in canvas.pixels {
            assert_eq!(color, Color::new_color(0.0, 0.0, 0.0));
        }

        let canvas_2 = Canvas::new_canvas_with_color(10, 20, crate::color::AZURE_BLUE);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.width, 10);
        for color in canvas_2.pixels {
            assert_eq!(color, crate::color::AZURE_BLUE);
        }
    }
}

// Feature: Canvas

// Scenario: Creating a canvas
//   Given c ← canvas(10, 20)
//   Then c.width = 10
//     And c.height = 20
//     And every pixel of c is color(0, 0, 0)

// Scenario: Writing pixels to a canvas
//   Given c ← canvas(10, 20)
//     And red ← color(1, 0, 0)
//   When write_pixel(c, 2, 3, red)
//   Then pixel_at(c, 2, 3) = red

// Scenario: Constructing the PPM header
//   Given c ← canvas(5, 3)
//   When ppm ← canvas_to_ppm(c)
//   Then lines 1-3 of ppm are
//     """
//     P3
//     5 3
//     255
//     """

// Scenario: Constructing the PPM pixel data
//   Given c ← canvas(5, 3)
//     And c1 ← color(1.5, 0, 0)
//     And c2 ← color(0, 0.5, 0)
//     And c3 ← color(-0.5, 0, 1)
//   When write_pixel(c, 0, 0, c1)
//     And write_pixel(c, 2, 1, c2)
//     And write_pixel(c, 4, 2, c3)
//     And ppm ← canvas_to_ppm(c)
//   Then lines 4-6 of ppm are
//     """
//     255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
//     """

// Scenario: Splitting long lines in PPM files
//   Given c ← canvas(10, 2)
//   When every pixel of c is set to color(1, 0.8, 0.6)
//     And ppm ← canvas_to_ppm(c)
//   Then lines 4-7 of ppm are
//     """
//     255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
//     153 255 204 153 255 204 153 255 204 153 255 204 153
//     255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
//     153 255 204 153 255 204 153 255 204 153 255 204 153
//     """

// Scenario: PPM files are terminated by a newline character
//   Given c ← canvas(5, 3)
//   When ppm ← canvas_to_ppm(c)
//   Then ppm ends with a newline character
