mod camera;
mod canvas;
mod color;
mod error;
mod matrix;
mod drivers;
mod ppm;
mod ray;
mod reflection;
mod shape;
mod transformation;
mod tuple;
mod utils;
mod pattern;
mod world;

use std::f64::consts::PI;

use camera::Camera;
use color::Color;
use minifb::Key;
use reflection::Material;
use shape::{plane::Plane, shape::Shape, sphere::Sphere};
use transformation::{create_rotation_x, create_scaling, create_translation, view_transform};
use world::World;
use drivers::minifb_driver;

use crate::{color::{BLACK, WHITE}, pattern::Pattern, tuple::*};

// TODO
// Better Unwrap
// progesss system (en  parallelle ?)

fn main() {
    //floor
    let mut floor = Plane::plane();
    floor.material = Material::default_material();
    floor.material.color = Color::new_color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    //left wall
    let mut left_wall = Plane::plane();
    left_wall.set_transform(
        &create_rotation_x(PI / 2.0)
            .rotation_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    left_wall.material = floor.material.clone();
    floor.material.pattern = Some(Pattern::new_checker_pattern(WHITE, BLACK));


    //rigth wall
    let mut right_wall = Plane::plane();
    right_wall.set_transform(
        &create_rotation_x(PI / 2.0)
            .rotation_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    right_wall.material = right_wall.material.clone();
    right_wall.material.pattern = Some (Pattern::new_radial_gradiant_pattern(color::BLACK, color::WHITE));

    //large sphere
    let mut middle = Sphere::sphere();
    middle.set_transform(&create_translation(-0.5, 1.0, 0.5));
    middle.material = Material::default_material();
    middle.material.color = Color::new_color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.pattern = Some (Pattern::new_checker_pattern(color::AZURE_BLUE, color::WHITE));

    //small sphere
    let mut right = Sphere::sphere();
    right.set_transform(&create_scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5));
    right.material = Material::default_material();
    right.material.color = Color::new_color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.pattern = Some (Pattern::new_stripe_pattern(color::AZURE_BLUE, color::LIGHT_VIOLET));

    //smaller sphere
    let mut left = Sphere::sphere();
    left.set_transform(&create_scaling(0.33, 0.33, 0.33).translation(-1.5, 0.33, -0.75));
    left.material = Material::default_material();
    left.material.color = Color::new_color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    //world creation
    let mut world = World::new_world();
    world.objects = vec![
        Box::new(floor),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(middle),
        Box::new(right),
        Box::new(left),
    ];

    //ligth source
    let light_position = Tuple::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new_color(1.0, 1.0, 1.0);
    let light = reflection::PointLight::new_point_light(light_color, light_position);
    world.light_sources.push(light.clone());

    //camera
    let canvas_size_pixels_width = 500;
    let canvas_size_pixels_height = 500;
    let mut camera = Camera::new(
        canvas_size_pixels_width,
        canvas_size_pixels_height,
        PI / 3.0,
    );
    camera.transformation = view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0.0, 1.0, 0.0),
        &Tuple::new_vector(0.0, 1.0, 0.0),
    );

    //render result to a canvas
    let canvas = camera.render(world);
    let buffer = minifb_driver::buffer_from_canvas(&canvas);
    let mut window = minifb_driver::new_window(&canvas);

    window
        .update_with_buffer(&buffer, canvas_size_pixels_width, canvas_size_pixels_height)
        .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, canvas_size_pixels_width, canvas_size_pixels_height)
            .unwrap();
    }
}
