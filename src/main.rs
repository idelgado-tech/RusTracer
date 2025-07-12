mod camera;
mod canvas;
mod color;
mod drivers;
mod error;
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

use std::f64::consts::PI;

use camera::Camera;
use color::Color;
use drivers::minifb_driver;
use minifb::Key;
use reflection::Material;
use transformation::{create_rotation_x, create_scaling, create_translation, view_transform};
use world::World;

use crate::{
    color::{BLACK, WHITE},
    pattern::Pattern,
    shape::object::Object,
    tuple::*,
};

// TODO
// Better Unwrap
// progesss system (en  parallelle ?)

fn main() {
    //floor
    let mut floor = Object::new_plane();
    floor.material = Material::default_material();
    floor.material.color = Color::new_color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    //left wall
    let mut left_wall = Object::new_plane();
    left_wall.set_transform(
        &create_rotation_x(PI / 2.0)
            .rotation_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    left_wall.material = floor.material.set_reflective(0.8).clone();
    floor.material.pattern = Some(Pattern::new_checker_pattern(WHITE, BLACK));

    //rigth wall
    let mut right_wall = Object::new_plane();
    right_wall.set_transform(
        &create_rotation_x(PI / 2.0)
            .rotation_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    right_wall.material = right_wall.material.set_reflective(0.9).clone();
    right_wall.material.pattern = Some(Pattern::new_radial_gradiant_pattern(
        color::BLACK,
        color::WHITE,
    ));

    //large sphere
    let mut middle = Object::new_sphere();
    middle.set_transform(&create_translation(-0.5, 1.0, 0.5));
    middle.material = Material::default_material().clone();
    middle.material.color = Color::new_color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.pattern = Some(Pattern::new_checker_pattern(
        color::AZURE_BLUE,
        color::WHITE,
    ));
    middle.set_refractive_index(2.5);
    middle.set_transparency(0.9);

    //middle inner sphere
    let mut middle_inner = Object::new_sphere();
    middle_inner.set_transform(&create_translation(-0.5, 1.0, 0.5).scaling(0.5, 0.5, 0.5));
    middle_inner.material = Material::default_material().set_reflective(0.5).clone();
    middle_inner.material.color = Color::new_color(1.0, 0.0, 0.0);
    middle_inner.material.diffuse = 0.7;
    middle_inner.material.specular = 0.3;
    middle_inner.set_refractive_index(1.5);
    middle_inner.set_transparency(0.5);

    //small sphere
    let mut right = Object::new_sphere();
    right.set_transform(&create_scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5));
    right.material = Material::default_material();
    right.material.color = Color::new_color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.pattern = Some(Pattern::new_stripe_pattern(vec![
        color::AZURE_BLUE,
        color::LIGHT_VIOLET,
    ]));
    right.set_refractive_index(1.5);
    right.set_transparency(0.6);

    //smaller sphere
    let mut left = Object::new_sphere();
    left.set_transform(&create_scaling(0.33, 0.33, 0.33).translation(-1.5, 0.33, -0.75));
    left.material = Material::default_material();
    left.material.color = Color::new_color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    //world creation
    let mut world = World::new_world();
    world.objects = vec![
        floor,
        left_wall,
        right_wall,
        middle,
        right,
        left,
        middle_inner,
    ];

    //ligth source
    let light_position = Tuple::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new_color(1.0, 1.0, 1.0);
    let light = reflection::PointLight::new_point_light(light_color, light_position);
    world.light_sources.push(light.clone());

    //camera
    let canvas_size_pixels_width = 1800;
    let canvas_size_pixels_height = 1000;
    let mut camera = Camera::new(
        canvas_size_pixels_width,
        canvas_size_pixels_height,
        PI / 2.0,
    );
    camera.transformation = view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0.0, 1.0, 0.0),
        &Tuple::new_vector(0.0, 1.0, 0.0),
    );

    //render result to a canvas
    // let canvas = camera.render_with_update_bar(world);
    let canvas = camera.render_par_with_update_bar(world);

    let buffer = minifb_driver::buffer_from_canvas(&canvas);
    let mut window = minifb_driver::new_window(&canvas);

    window
        .update_with_buffer(&buffer, canvas_size_pixels_width, canvas_size_pixels_height)
        .unwrap();

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, canvas_size_pixels_width, canvas_size_pixels_height)
            .unwrap();
    }
}
