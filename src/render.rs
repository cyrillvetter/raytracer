use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, AA_SIZE};
use crate::primitive::*;
use crate::scene::Scene;
use crate::Image;
use crate::material::Scatterable;

use rayon::prelude::*;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const MAX_DEPTH: f32 = 5.0;

pub fn render_scene(scene: Scene) -> Image {
    let mut pixels = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let bands: Vec<(usize, &mut [u32])> = pixels.chunks_mut(IMAGE_WIDTH as usize).enumerate().collect();

    bands
        .into_par_iter()
        .for_each(|(y, band)| {
            render_line(band, y as u32, &scene);
        });

    Image::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, pixels)
}

fn render_line(pixels: &mut [u32], y: u32, scene: &Scene) {
    for x in 0..pixels.len() {
        let mut color = Color::BLACK;

        for x_offset in 0..AA_SIZE {
            for y_offset in 0..AA_SIZE {
                let ray = scene.camera.ray_from(((x as u32) * AA_SIZE) + x_offset, (y * AA_SIZE) + y_offset);
                color += trace_ray(ray, MAX_DEPTH, &scene);
            }
        }

        color = (color / (AA_SIZE * AA_SIZE) as f32).gamma_correct();
        pixels[x] = ((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32);
    }
}

fn trace_ray(ray: Ray, depth: f32, scene: &Scene) -> Color {
    if depth < f32::EPSILON {
        return Color::BLACK;
    }

    match scene.bvh.intersects(&ray) {
        Some(hit_record) => match hit_record.material_index {
            Some(index) => match scene.materials[index].scatter(&ray, &hit_record, scene) {
                (Some(reflective_ray), color) => color * (0.9f32).powf(MAX_DEPTH - depth) * trace_ray(reflective_ray, depth - 1.0, scene),
                (None, color) => color,
            },
            _ => FALLBACK_COLOR
        },
        _ => {
            let a = 0.5 * (ray.direction.y + 1.0);
            (1.0 - a) * Color::WHITE + a * Color::rgb(0.5, 0.7, 1.0)
        }
    }
}
