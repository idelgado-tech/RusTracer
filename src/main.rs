use std::path::Path;

use minifb::Key;
use rustracer::utils::init_from_path;

fn main() {
    let path = Path::new("scenes/ch11_refraction.yml");

    let (camera, buffer, mut window) = init_from_path(path);

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, camera.hsize, camera.vsize)
            .unwrap();
    }
}
