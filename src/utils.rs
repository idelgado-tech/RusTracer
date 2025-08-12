use std::path::Path;

use minifb::Window;

use crate::{camera::Camera, drivers::minifb_driver, io::yaml::parse, world::World};

pub fn compare_float(value1: f64, value2: f64) -> bool {
    (value1 - value2).abs() < 0.00001
}

pub fn compare_float_with_threshold(value1: f64, value2: f64, threshold: f64) -> bool {
    (value1 - value2).abs() < threshold
}

pub fn pos_from_index(index: usize, width: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

pub fn index_from_pos(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

// TODO a ranger

pub fn init_from_path(path: &Path) -> (Camera, Vec<u32>, Window) {
    let (objects, ligths, camera) = parse(path);

    //world creation
    let mut world = World::new_world();
    world.objects = objects;

    //ligth sources
    world.light_sources = ligths;

    //render result to a canvas
    // let canvas = camera.render_with_update_bar(world);
    let canvas = camera.render_par_with_update_bar(world);

    let buffer = minifb_driver::buffer_from_canvas(&canvas);
    let window = minifb_driver::new_window(&canvas);

    (camera, buffer, window)
}

pub fn init_headless_from_path(path: &Path) -> (Camera, Vec<u32>, Window) {
    let (objects, ligths, camera) = parse(path);

    //world creation
    let mut world = World::new_world();
    world.objects = objects;

    //ligth sources
    world.light_sources = ligths;

    //render result to a canvas
    // let canvas = camera.render_with_update_bar(world);
    let canvas = camera.render_par_headless(world);

    let buffer = minifb_driver::buffer_from_canvas(&canvas);
    let window = minifb_driver::new_window(&canvas);

    (camera, buffer, window)
}