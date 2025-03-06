mod vec3;
mod color;
mod image;
mod ray;
mod hittable;
mod camera;

use std::time::Instant;
use minifb::{Window, WindowOptions, Key, KeyRepeat};

use crate::image::Image;
use crate::vec3::Vec3;
use crate::color::Color;
use crate::hittable::{Hittable, sphere::Sphere};
use crate::ray::Ray;
use crate::camera::Camera;

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
        Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.25, Color::RED),
        Sphere::new(Vec3::new(-0.25, 0.333, 1.0), 0.1, Color::BLUE),
        Sphere::new(Vec3::new(0.25, 0.333, 1.0), 0.1, Color::GREEN)
    ];

    let camera = Camera::new(1.0);
    let aspect_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);

    let left = -(camera.size * aspect_ratio) / 2.0;
    let x_step = camera.size * (IMAGE_WIDTH as f32);

    let top = camera.size / 2.0;
    let y_step = camera.size * (IMAGE_HEIGHT as f32);

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);
    let ray_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    for x in 0..IMAGE_WIDTH {
        let viewport_x = left + ((x as f32) / x_step) * aspect_ratio;

        for y in 0..IMAGE_HEIGHT {
            let viewport_y = top - (y as f32) / y_step;

            let pixel = Vec3::new(viewport_x, viewport_y, 0.0);
            let ray = Ray::new(pixel, ray_dir);

            let mut nearest_dist = f32::INFINITY;
            let mut col = Color::BLACK;

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
