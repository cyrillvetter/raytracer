mod vec3;
mod color;
mod image;
mod ray;
mod hittable;

use std::time::Instant;
use minifb::{Window, WindowOptions, Key, KeyRepeat};

use crate::image::Image;
use crate::vec3::Vec3;
use crate::color::Color;
use crate::hittable::{Hittable, sphere::Sphere};
use crate::ray::Ray;

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
    let spheres = [
        Sphere::new(Vec3::new(640.0, 360.0, -300.0), 200.0, Color::RED),
        Sphere::new(Vec3::new(520.0, 240.0, -250.0), 100.0, Color::GREEN),
        Sphere::new(Vec3::new(760.0, 240.0, -250.0), 100.0, Color::BLUE)
    ];

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        let f_x = x as f32;

        for y in 0..IMAGE_HEIGHT {
            let ray = Ray::new(Vec3::new(f_x, y as f32, 0.0), Vec3::new(0.0, 0.0, 1.0));

            let mut col = Color::BLACK;
            let mut nearest_dist = f32::MAX;

            for s in spheres.iter() {
                match s.hit(&ray) {
                    Some(dist) if dist < nearest_dist => {
                        nearest_dist = dist;
                        col = s.color;
                    },
                    _ => ()
                };
            }

            image.set_pixel(x, y, col);
        }
    }

    image
}
