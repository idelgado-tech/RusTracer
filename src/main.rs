mod canvas;
mod color;
mod minifb_als;
mod ppm;
mod tuple;
mod utils;
mod matrix;

use minifb::{Key, Window, WindowOptions};

use crate::canvas::*;
use crate::tuple::*;

#[derive(Debug, Clone)]
pub struct Env {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug, Clone)]
pub struct Proj {
    pub position: Tuple,
    pub velocity: Tuple,
}

fn tick(env: &Env, proj: &Proj) -> Proj {
    let position = proj.position.clone() + proj.velocity.clone();
    let velocity = proj.velocity.clone() + env.gravity.clone() + env.wind.clone();
    return Proj { position, velocity };
}

fn main() {
    let gravity = tuple::Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = tuple::Tuple::new_vector(-0.01, 0.0, 0.0);
    let env = Env { gravity, wind };

    let  position = tuple::Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = tuple::Tuple::new_vector(3.0, 3.0, 0.0);
    let mut proj = Proj {
        position: position.clone(),
        velocity,
    };

    let mut position_vec: Vec<Tuple> = Vec::with_capacity(100);
    position_vec.push(position.clone());

    while position_vec.last().unwrap().y > 0.0 {
        println!("Here loop = {:?} ", position_vec.len());
        let new_proj = tick(&env, &proj);
        proj = new_proj;
        position_vec.push(proj.position.clone())
    }
    println!("Here list = {:?} ", position_vec);

    let mut canvas = canvas::Canvas::new_canvas_with_color(
        minifb_als::MAX_WIDTH,
        minifb_als::MAX_HEIGHT,
        color::Color::new_color(1.0, 1.0, 1.0),
    );

    for elem in position_vec {
        println!("Here elem = {:?} ", elem);

        canvas.set_pixel_color(
        
            elem.x as usize,
            canvas.height - std::cmp::max(elem.y as usize, 1),
            color::AZURE_BLUE,
        );
    }

    let buffer = minifb_als::buffer_from_canvas(&canvas);

    let mut window = minifb_als::new_window(&canvas);

    window
        .update_with_buffer(&buffer, minifb_als::MAX_WIDTH, minifb_als::MAX_HEIGHT)
        .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, minifb_als::MAX_WIDTH, minifb_als::MAX_HEIGHT)
            .unwrap();
    }
}
