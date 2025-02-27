mod vec3;
mod color;
mod image;
mod shapes;

use std::time::Instant;

use minifb::{Window, WindowOptions, Key, KeyRepeat};

use crate::image::*;
use crate::vec3::*;
use crate::color::*;
use crate::shapes::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;

static OUT_PATH: &str = "out/image.png";

fn main() {
    let now = Instant::now();
    let image = create_circles();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);

    show_image(&image);
}

fn show_image(image: &Image) {
    let mut window = Window::new(
        "Raytracer",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions::default()
    ).expect("Failed to create window");

    window.set_target_fps(24);
    let mut image_saved = false;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        if !image_saved && (window.is_key_down(Key::LeftCtrl) || window.is_key_down(Key::RightCtrl)) && window.is_key_down(Key::S) {
            image_saved = true;
            image.save_png(OUT_PATH);
            println!("Image saved to: {}", OUT_PATH);
        }

        window
            .update_with_buffer(&image.bytes, IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
            .expect("Failed to set buffer");
    }
}

fn create_circles() -> Image {
    const RADIUS: f32 = 200.0;
    let spheres = vec![
        Sphere::new(Vec3::new(640.0, 280.0, 0.0), RADIUS, Color::RED),
        Sphere::new(Vec3::new(520.0, 440.0, 0.0), RADIUS, Color::GREEN),
        Sphere::new(Vec3::new(760.0, 440.0, 0.0), RADIUS, Color::BLUE),
    ];

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        let f_x = x as f32;

        for y in 0..IMAGE_HEIGHT {
            let mut col = Color::BLACK;
            for s in spheres.iter() {
                if (Vec3::new(f_x, y as f32, 0.0) - s.origin).length_squared() <= f32::powf(s.radius, 2.0) {
                    col = col + s.color;
                }
            }

            image.set_pixel(x, y, col);
        }
    }

    image
}
