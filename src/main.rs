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
mod reflection;

use std::f64::consts::PI;

use minifb::{Key, Window, WindowOptions};
use ray::Sphere;

use crate::{
    ray::{hit_intersections, intersect, Ray},
    tuple::*,
};

fn main() {
    // Variables cast
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_size_pixels_width = 700.0;
    let canvas_size_pixels_height = 700.0;

    let pixel_size_width = wall_size / canvas_size_pixels_width;
    let pixel_size_height = wall_size / canvas_size_pixels_height;

    let half = wall_size / 2.0;

    let mut canvas = canvas::Canvas::new_canvas_with_color(
        canvas_size_pixels_width as usize,
        canvas_size_pixels_height as usize,
        color::Color::new_color(1.0, 1.0, 1.0),
    );

    let mut shape = Sphere::sphere();
    let transformation = transformation::create_shearing(1.0, 1.0, 0.0, 0.0, 0.0, 2.0).scaling(0.5, 0.5, 0.5);
    shape.set_transform(&transformation);

    for y in 0..canvas_size_pixels_height as isize {
        // println!("Here elem y = {:?} ", y);
        let world_y = half - pixel_size_height * y as f64;

        for x in 0..canvas_size_pixels_width as isize {
            // println!("Here elem x = {:?} ", x);

            let world_x = (-1.0 * half) + pixel_size_width * x as f64;
            let position = Tuple::new_point(world_x, world_y, wall_z);
            let r = Ray::new(
                ray_origin.clone(),
                (position - ray_origin.clone()).normalize(),
            );
            let xs = intersect(&shape, r);

            if hit_intersections(xs).is_some() {
                canvas.set_pixel_color(x as usize, canvas.height - y as usize, color::LIGHT_VIOLET);
            }
        }
    }

    // let buffer = minifb_als::buffer_from_canvas(&canvas);

    // let mut window = minifb_als::new_window(&canvas);

    // window
    //     .update_with_buffer(
    //         &buffer,
    //         canvas_size_pixels_width as usize,
    //         canvas_size_pixels_height as usize,
    //     )
    //     .unwrap();

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(
    //             &buffer,
    //             canvas_size_pixels_width as usize,
    //             canvas_size_pixels_height as usize,
    //         )
    //         .unwrap();
    // }
}
