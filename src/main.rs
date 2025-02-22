mod vec3;
mod color;
mod image;

use std::time::Instant;
use minifb::{Window, WindowOptions, Key, KeyRepeat};

use crate::image::*;
use crate::vec3::*;
use crate::color::*;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;

fn main() {
    let now = Instant::now();
    let image = create_area_circle();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);

    let mut window = Window::new(
        "Raytracer",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions::default()
    ).expect("Failed to create window");

    window.set_target_fps(24);

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        window.update_with_buffer(&image.bytes, IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize).expect("Failed to set buffer");
    }
}

fn create_area_circle() -> Image {
    const RADIUS: f32 = 150.0;
    let radius_squared = f32::powf(RADIUS, 2.0);

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);
    const CENTER: Vec3 = Vec3::new((IMAGE_WIDTH / 2) as f32, (IMAGE_HEIGHT / 2) as f32, 0.0);

    for x in 0..IMAGE_WIDTH {
        let f_x = x as f32;

        for y in 0..IMAGE_HEIGHT {
            if (Vec3::new(f_x, y as f32, 0.0) - CENTER).length_squared() <= radius_squared {
                image.set_pixel(x, y, Color::RED);
            } else {
                image.set_pixel(x, y, Color::WHITE);
            }
        }
    }

    image
}
