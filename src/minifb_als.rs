use crate::canvas::*;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const MAX_WIDTH: usize = 640;
const MAX_HEIGHT: usize = 360;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn from_u8_tuble_rgb(tuple: (u8, u8, u8)) -> u32 {
    let (r, g, b) = tuple;
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn buffer_from_canvas(canvas: &Canvas) -> Vec<u32> {
    let mut buffer = vec![0; canvas.width * canvas.height];
    for it_tuple in canvas.pixels.iter().zip(buffer.iter_mut()) {
        let (ai, bi) = it_tuple;
        *bi = from_u8_tuble_rgb(ai.normalise());
    }
    buffer
}