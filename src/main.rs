mod vec3;
mod color;
mod image;
mod ray;
mod hittable;
mod camera;

use std::time::Instant;
use minifb::{Window, WindowOptions, Key, KeyRepeat};
use rand::Rng;
use rayon::prelude::*;

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

const LIGHT_ORIGIN: Vec3 = Vec3::new(-10.0, 7.0, 10.0);

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
    const OBJECTS_AMOUNT: usize = 10000;
    let mut rng = rand::rng();
    let mut objects: Vec<Sphere> = Vec::with_capacity(OBJECTS_AMOUNT);

    for _ in 0..OBJECTS_AMOUNT {
        let x = rng.random_range(-0.9..0.9);
        let y = rng.random_range(-0.50..0.50);
        let z = rng.random_range(1.0..2.0);
        let radius = rng.random_range(0.0075..0.025);

        let r = rng.random();
        let g = rng.random();
        let b = rng.random();

        objects.push(Sphere::new(Vec3::new(x, y, z), radius, Color::rgb(r, g, b)));
    }

    let camera = Camera::new(1.0);

    let mut pixels = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let bands: Vec<(usize, &mut [u32])> = pixels.chunks_mut(IMAGE_WIDTH as usize).enumerate().collect();

    bands
        .into_par_iter()
        .for_each(|(y, band)| {
            render_line(band, y as u32, &camera, &objects);
        });

    Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, pixels)
}

fn render_line(pixels: &mut [u32], y: u32, camera: &Camera, objects: &Vec<Sphere>) {
    let ray_dir = Vec3::new(0.0, 0.0, -1.0).normalize();

    for x in 0..pixels.len() {

        let pixel = camera.in_world(x as u32, y);
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

        pixels[x] = ((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32);
    }
}
