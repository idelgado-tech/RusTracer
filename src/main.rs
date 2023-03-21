mod camera;
mod canvas;
mod color;
mod error;
mod matrix;
mod minifb_als;
mod ppm;
mod ray;
mod reflection;
mod transformation;
mod tuple;
mod utils;
mod world;
mod shape;


use std::f64::consts::PI;

use camera::Camera;
use color::Color;
use matrix::Matrix;
use minifb::{Key, Window, WindowOptions};
use ray::Sphere;
use reflection::Material;
use transformation::{create_scaling, create_translation, view_transform};
use world::World;

use crate::{
    ray::{hit_intersections, intersect, Ray},
    tuple::*,
};

fn main() {

    //floor
    let mut floor = Sphere::sphere();
    floor.set_transform(&create_scaling(10.0, 0.01, 10.0));
    floor.material = Material::material();
    floor.material.color = Color::new_color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    //left wall
    let mut left_wall = Sphere::sphere();
    left_wall.set_transform(
        &create_scaling(10.0, 0.01, 10.0)
            .rotation_x(PI / 2.0)
            .rotation_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    left_wall.material = floor.material.clone();

    //rigth wall
    let mut right_wall = Sphere::sphere();
    right_wall.set_transform(
        &create_scaling(10.0, 0.01, 10.0)
            .rotation_x(PI / 2.0)
            .rotation_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    right_wall.material = right_wall.material.clone();

    //large sphere
    let mut middle = Sphere::sphere();
    middle.set_transform(&create_translation(-0.5, 1.0, 0.5));
    middle.material = Material::material();
    middle.material.color = Color::new_color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    //small sphere
    let mut right = Sphere::sphere();
    right.set_transform(&create_scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5));
    right.material = Material::material();
    right.material.color = Color::new_color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    //smaller sphere
    let mut left = Sphere::sphere();
    left.set_transform(&create_scaling(0.33, 0.33, 0.33).translation(-1.5, 0.33, -0.75));
    left.material = Material::material();
    left.material.color = Color::new_color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    //world creation
    let mut world = World::world();
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

    //ligth source
    let light_position = Tuple::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new_color(1.0, 1.0, 1.0);
    let light = reflection::PointLight::new_point_light(light_color, light_position);
    world.light_sources.push(light.clone());

    //camera
    let canvas_size_pixels_width = 700;
    let canvas_size_pixels_height = 700;
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
    let buffer = minifb_als::buffer_from_canvas(&canvas);
    let mut window = minifb_als::new_window(&canvas);

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
