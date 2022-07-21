mod canvas;
mod color;
mod error;
mod matrix;
mod minifb_als;
mod ppm;
mod ray;
mod transformation;
mod tuple;
mod utils;

use minifb::{Key, Window, WindowOptions};
use ray::Sphere;

use crate::{
    ray::{intersect, Ray},
    tuple::*,
};

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
    Proj { position, velocity }
}

fn main() {
    // Variables cast
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_size_pixels = 640.0;
    let pixel_size = wall_size / canvas_size_pixels;
    let half = wall_size / 2.0;

    let mut canvas = canvas::Canvas::new_canvas_with_color(
        canvas_size_pixels as usize,
        canvas_size_pixels as usize,
        color::Color::new_color(1.0, 1.0, 1.0),
    );

    let shape = Sphere::sphere();

    for y in 0..canvas_size_pixels as isize {
        println!("Here elem y = {:?} ", y);
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_size_pixels as isize {
            // println!("Here elem x = {:?} ", x);

            let world_x = half - pixel_size * x as f64;
            let position = Tuple::new_point(world_x, world_y, wall_z);
            let r = Ray::new(
                ray_origin.clone(),
                (position - ray_origin.clone()).normalize(),
            );
            let xs = intersect(&shape, r);

            if xs.len() > 0 {
                canvas.set_pixel_color(
                    x as usize,
                    y as usize,
                    color::LIGHT_VIOLET,
                );
            }
        }
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
