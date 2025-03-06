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

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 720;

const IMAGE_WIDTH: u32 = 1920;
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
    let camera = Camera::new(1.0);
    let wh_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);

    let top_left = Vec3::new(-(camera.size * wh_ratio) / 2.0, camera.size / 2.0, 0.0);
    println!("{}", wh_ratio);
    println!("{}", top_left);

    let spheres = [
        Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5, Color::RED),
        //Sphere::new(Vec3::new(-0.25, 0.333, 1.0), 0.1, Color::BLUE),
        //Sphere::new(Vec3::new(0.25, 0.333, 1.0), 0.1, Color::GREEN)
    ];

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);
    let ray_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    let mut min_vy = f32::INFINITY;
    let mut max_vx = f32::NEG_INFINITY;

    for x in 0..IMAGE_WIDTH {
        let vx = top_left.x + (x as f32) / (camera.size * (IMAGE_WIDTH as f32)) * wh_ratio;
        max_vx = max_vx.max(vx);

        for y in 0..IMAGE_HEIGHT {
            let vy = top_left.y - (y as f32) / (camera.size * (IMAGE_HEIGHT as f32));
            min_vy = min_vy.min(vy);

            let pixel = Vec3::new(vx, vy, 0.0);
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

    println!("");
    println!("max");
    println!("x: {}", max_vx);
    println!("y: {}", min_vy);

    image
}
