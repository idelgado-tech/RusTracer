use std::sync::{Arc, Mutex};

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use serde_yaml::with;

use crate::matrix::memoized_inverse;
use crate::transformation;
use crate::{
    canvas::Canvas,
    color::{self, Color},
    matrix::Matrix,
    ray::Ray,
    reflection,
    tuple::Tuple,
    utils,
    world::World,
};

///virtual camera
#[derive(Debug, Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transformation: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    fn calculate_ratios(mut self) -> Self {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view
        };
        let pixel_size = (half_width * 2.0) / (self.hsize as f64);

        self.pixel_size = pixel_size;
        self.half_height = half_height;
        self.half_width = half_width;
        self
    }

    pub fn default() -> Camera {
        Camera {
            hsize: 200,
            vsize: 100,
            field_of_view: 1.5,
            transformation: Matrix::new_identity_matrix(4),
            half_width: 0.0,
            half_height: 0.0,
            pixel_size: 0.0,
        }
        .calculate_ratios()
    }

    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        Self::default()
            .with_size(hsize, vsize)
            .with_fov(field_of_view)
            .calculate_ratios()
    }

    pub fn with_size(mut self, hsize: usize, vsize: usize) -> Self {
        self.hsize = hsize;
        self.vsize = vsize;
        self.calculate_ratios()
    }

    pub fn with_fov(mut self, fov: f64) -> Self {
        self.field_of_view = fov;
        self.calculate_ratios()
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self
    }

    pub fn set_transform(&mut self, new_transformation: &Matrix) {
        self.transformation = new_transformation.clone();
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 0.0),
        );

        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = memoized_inverse(self.clone().transformation).unwrap()
            * Tuple::new_point(world_x, world_y, -1.0);
        let origin = memoized_inverse(self.clone().transformation).unwrap()
            * Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin.clone()).normalize();

        Ray::new(origin, direction)
    }

    fn color_at(&self, world: &World, col: usize, row: usize) -> Color {
        let ray = self.ray_for_pixel(col, row);
        world.color_at(&ray, 5)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new_canvas(self.hsize, self.vsize);
        println!("Starting render");
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                image.set_pixel_color(x, y, self.color_at(&world, x, y));
            }
        }
        image
    }

    // factoriser les fonctions

    pub fn render_with_update_bar(&self, world: World) -> Canvas {
        let mut image = Canvas::new_canvas(self.hsize, self.vsize);
        println!("Starting render");
        let bar = ProgressBar::new((self.hsize * self.vsize) as u64);
        bar.set_style(
            ProgressStyle::with_template("{bar:120} [{percent_precise}%] [T : {elapsed:}]")
                .unwrap(),
        );

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                image.set_pixel_color(x, y, self.color_at(&world, x, y));
            }
            bar.inc(self.hsize as u64);
        }

        println!("Done rendering");
        image
    }

    pub fn render_par_with_update_bar(&self, world: World) -> Canvas {
        const BAND_SIZE: usize = 10;
        let mut image2 = Canvas::new_canvas(self.hsize, self.vsize);

        println!("Starting render");

        let bar_style =
            ProgressStyle::with_template("{bar:120} [{percent_precise}%] [T : {elapsed:}]")
                .unwrap();

        image2
            .pixels()
            .par_chunks_mut(self.hsize * BAND_SIZE)
            .enumerate()
            .progress_with_style(bar_style)
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.hsize {
                        band[row * self.hsize + col] =
                            self.color_at(&world, col, row + i * BAND_SIZE);
                    }
                }
            });

        println!("Done rendering");
        image2
    }


    pub fn render_par_headless(&self, world: World) -> Canvas {
        const BAND_SIZE: usize = 10;
        let mut image2 = Canvas::new_canvas(self.hsize, self.vsize);

        image2
            .pixels()
            .par_chunks_mut(self.hsize * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.hsize {
                        band[row * self.hsize + col] =
                            self.color_at(&world, col, row + i * BAND_SIZE);
                    }
                }
            });

        image2
    }
}

#[cfg(test)]
mod camera_tests {
    use super::*;
    use crate::{
        color::Color,
        transformation::{create_translation, view_transform},
        utils,
    };
    use std::f64::consts::PI;

    #[test]
    ///Constructing a camera
    fn creating_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let camera = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
        assert_eq!(camera.field_of_view, field_of_view);
    }

    #[test]
    ///The pixel size for a horizontal canvas
    fn pixel_size_hosizontal() {
        let camera = Camera::new(200, 125, PI / 2.0);
        assert!(utils::compare_float(camera.pixel_size, 0.01));
    }

    #[test]
    ///The pixel size for a vertical canvas
    fn pixel_size_vertical() {
        let camera = Camera::new(125, 200, PI / 2.0);
        assert!(utils::compare_float(camera.pixel_size, 0.01));
    }

    #[test]
    ///Constructing a ray through the center of the canvas
    fn construc_ray_center() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    ///Constructing a ray through the Corner of the canvas
    fn construc_ray_corner() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(
            r.direction,
            Tuple::new_vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
        );
    }

    #[test]
    ///Constructing a ray when the camera is transformed
    fn construc_ray_tranform() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.set_transform(&create_translation(0.0, -2.0, 5.0).rotation_y(PI / 4.0));
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::new_point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::new_vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    ///Rendering a world with a camera
    fn render_world() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);

        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        c.transformation = view_transform(&from, &to, &up);
        let image = c.render(w);

        assert_eq!(
            image.pixel_at(5, 5),
            Color::new_color(0.3806611930807966, 0.47582649135099575, 0.28549589481059745)
        );
    }
}
