use crate::color::Color;

#[derive(PartialEq, Debug, Clone)]
pub struct Canvas {
   pub width: usize,
   pub height: usize,
   pub pixels: Vec<Color>,
}

impl Canvas {
    fn new_canvas(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new_color(0.0, 0.0, 0.0); width * height],
        }
    }

    fn new_canvas_with_color(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }
}

// Scenario​: Creating a canvas
// ​ 	  ​Given​ c ← canvas(10, 20)
// ​ 	  ​Then​ c.width = 10
// ​ 	    ​And​ c.height = 20
// ​ 	    ​And​ every pixel of c is color(0, 0, 0)
