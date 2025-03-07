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
const WINDOW_HEIGHT: u32 = 1080;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;

const LIGHT_ORIGIN: Vec3 = Vec3::new(-10.0, 7.0, 12.0);

static OUT_PATH: &str = "out/image.png";

fn main() {
    let now = Instant::now();
    let image = render_spheres();
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

    window.set_target_fps(30);
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

fn render_spheres() -> Image {
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(-0.575, 0.0, -1.0), 0.25, Color::rgb_u8(207, 54, 67))),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.25, Color::rgb_u8(55, 184, 57))),
        Box::new(Sphere::new(Vec3::new(0.575, 0.0, -1.0), 0.25, Color::rgb_u8(54, 55, 207))),
    ];

    let camera = Camera::new(1.0);

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0).normalize();

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let pixel = camera.in_world(x, y);
            let ray = Ray::new(pixel, ray_dir);

            let mut nearest_dist = f32::INFINITY;
            let mut color = Color::BLACK;

            for object in objects.iter() {
                match object.hit(&ray) {
                    Some(dist) if dist < nearest_dist => {
                        nearest_dist = dist;
                        color = object.get_color(ray.at(dist));
                    },
                    _ => ()
                };
            }

            image.set_pixel(x, y, color);
        }
    }

    image
}
