use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT, SAMPLES};
use crate::primitive::*;
use crate::scene::Scene;
use crate::Image;
use crate::material::Scatterable;

use std::path::Path;

use minifb::{Window, WindowOptions, Key, KeyRepeat};
use rayon::prelude::*;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const MAX_DEPTH: f32 = 5.0;

static OUT_PATH: &str = "out/image.png";

pub fn render_scene(scene: Scene) {
    let out_path = Path::new(OUT_PATH);
    let mut window = Window::new(
        "Raytracer",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions::default()
    ).expect("Failed to create window");

    window.set_target_fps(60);

    let mut pixels = vec![Color::BLACK; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let mut samples = 0;

    loop {
        println!("{}", samples);
        samples += 10;
        let bands: Vec<(usize, &mut [Color])> = pixels.chunks_mut(IMAGE_WIDTH as usize).enumerate().collect();

        bands
            .into_par_iter()
            .for_each(|(y, band)| {
                render_line(band, y as u32, &scene, 10);
            });

        let buffer: Vec<u32> = pixels.iter().map(|color| {
            let c = (*color / (samples as f32)).gamma_correct();
            ((c.r * 255.0) as u32) << 16 | ((c.g * 255.0) as u32) << 8 | ((c.b * 255.0) as u32)
        }).collect();

        window.update_with_buffer(&buffer, IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize).unwrap();
        Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, buffer).save_png(out_path);
    }
}

fn render_line(pixels: &mut [Color], y: u32, scene: &Scene, samples: usize) {
    for x in 0..pixels.len() {
        let mut color = Color::BLACK;

        for _ in 0..samples {
            let ray = scene.camera.ray_from(x as u32, y as u32);
            color += trace_ray(ray, MAX_DEPTH, &scene);
        }

        pixels[x] += color;
    }
}

fn trace_ray(ray: Ray, depth: f32, scene: &Scene) -> Color {
    if depth < f32::EPSILON {
        return Color::BLACK;
    }

    match scene.bvh.intersects(&ray) {
        Some(hit_record) => match hit_record.material_index {
            Some(index) => match scene.materials[index].scatter(&ray, &hit_record, scene) {
                (Some(reflective_ray), color, att) => (color * (att * trace_ray(reflective_ray, depth - 1.0, scene))).clamp(),
                (None, color, _) => color,
            },
            _ => FALLBACK_COLOR
        },
        _ => {
            let a = 0.5 * (ray.direction.y + 1.0);
            Color::WHITE
            //(1.0 - a) * Color::WHITE + a * Color::rgb(0.5, 0.7, 1.0)
        }
    }
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

}

