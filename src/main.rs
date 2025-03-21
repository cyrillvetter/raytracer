use std::time::Instant;
use minifb::{Window, WindowOptions, Key, KeyRepeat};

use raytracer::render_image;
use raytracer::Image;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

static OUT_PATH: &str = "out/image.png";

fn main() {
    let now = Instant::now();
    let image = render_image();

    println!("Elapsed: {:.2?}", now.elapsed());

    show_image(&image);
}

fn show_image(image: &Image) {
    let mut window = Window::new(
        "Raytracer",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions::default()
    ).expect("Failed to create window");

    window.set_target_fps(30);
    let mut image_saved = false;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        if !image_saved && window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            image_saved = true;
            image.save_png(OUT_PATH);
            println!("Image saved to: {}", OUT_PATH);
        }

        window
            .update_with_buffer(&image.bytes, image.width as usize, image.height as usize)
            .expect("Failed to set buffer");
    }
}
