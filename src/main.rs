mod canvas;
mod color;
mod minifb_als;
mod ppm;
mod tuple;
mod utils;

extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let azure_blue = from_u8_rgb(0, 127, 255);
    let light_violet = from_u8_rgb(127, 127, 255);

    let mut buffer: Vec<u32> = vec![azure_blue; WIDTH * HEIGHT];
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = light_violet; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
