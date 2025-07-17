mod camera;
mod canvas;
mod color;
mod drivers;
mod error;
mod io;
mod matrix;
mod pattern;
mod ppm;
mod ray;
mod reflection;
mod refraction;
mod shape;
mod transformation;
mod tuple;
mod utils;
mod world;

use std::{f64::consts::PI, path::Path};

use camera::Camera;
use color::Color;
use drivers::minifb_driver;
use minifb::Key;
use reflection::Material;
use transformation::{create_rotation_x, create_scaling, create_translation, view_transform};
use world::World;

use crate::{
    color::{BLACK, WHITE},
    io::yaml::parse,
    pattern::Pattern,
    shape::object::Object,
    tuple::*,
};

// TODO
// Better Unwrap
// progesss system (en  parallelle ?)

fn main() {

    let path = Path::new("/home/vanvan/RusTracer/scenes/ch11.yml");
    let (objects, ligths, camera) = parse(path);

    println!("Parsed value in {:?}", path);
    println!("Objects {:?}", objects);
    println!("Ligths {:?}", ligths);
    println!("Camera {:?}", camera);

    //world creation
    let mut world = World::new_world();
    world.objects = objects;

    //ligth source
    world.light_sources = ligths;

    //canvas
    let canvas_size_pixels_width = camera.hsize;
    let canvas_size_pixels_height = camera.vsize;

    //render result to a canvas
    // let canvas = camera.render_with_update_bar(world);
    let canvas = camera.render_par_with_update_bar(world);

    let buffer = minifb_driver::buffer_from_canvas(&canvas);
    let mut window = minifb_driver::new_window(&canvas);

    window
        .update_with_buffer(&buffer, camera.hsize, camera.vsize)
        .unwrap();

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, canvas_size_pixels_width, canvas_size_pixels_height)
            .unwrap();
    }
}
