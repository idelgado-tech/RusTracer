use crate::canvas::*;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

pub const MAX_WIDTH: usize = 640;
pub const MAX_HEIGHT: usize = 360;

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn from_u8_tuble_rgb(tuple: (u8, u8, u8)) -> u32 {
    let (r, g, b) = tuple;
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn buffer_from_canvas(canvas: &Canvas) -> Vec<u32> {
    let mut buffer = vec![0; canvas.width * canvas.height];
    for it_tuple in canvas.pixels.iter().zip(buffer.iter_mut()) {
        let (ai, bi) = it_tuple;
        *bi = from_u8_tuble_rgb(ai.normalise());
    }
    buffer
}

pub fn new_window(canvas: &Canvas) -> Window {
    Window::new(
        "Test - ESC to exit",
        canvas.width,
        canvas.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
}

// let mut buffer: Vec<u32> = vec![azure_blue; WIDTH * HEIGHT];
// window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

// // Limit to max ~60 fps update rate
// window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
// while window.is_open() && !window.is_key_down(Key::Escape) {
//     for i in buffer.iter_mut() {
//         *i = light_violet; // write something more funny here!
//     }

//     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
//     window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
// }
