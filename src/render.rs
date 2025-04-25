use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES};
use crate::util::ProgressBar;
use crate::primitive::*;
use crate::scene::Scene;
use crate::Image;
use crate::material::Scatterable;

use std::path::Path;
use rayon::prelude::*;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const MAX_DEPTH: f32 = 5.0;

static OUT_PATH: &str = "out/image.png";

pub fn render_scene(scene: Scene) {
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT as usize);
    let mut pixels = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let bands: Vec<(usize, &mut [u32])> = pixels.chunks_mut(IMAGE_WIDTH as usize).enumerate().collect();

    bands
        .into_par_iter()
        .for_each(|(y, band)| {
            render_line(band, y as u32, &scene, SAMPLES);
            progress_bar.update();
        });

    progress_bar.end();
    Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, pixels).save_png(Path::new(OUT_PATH));
}

fn render_line(pixels: &mut [u32], y: u32, scene: &Scene, samples: usize) {
    for x in 0..pixels.len() {
        let mut color = Color::BLACK;

        for _ in 0..samples {
            let ray = scene.camera.ray_from(x as u32, y as u32);
            color += trace_ray(ray, MAX_DEPTH, &scene);
        }

        pixels[x] = (color / SAMPLES as f32).gamma_correct().into_u32();
    }
}

fn trace_ray(ray: Ray, depth: f32, scene: &Scene) -> Color {
    if depth < f32::EPSILON {
        return Color::BLACK;
    }

    match scene.bvh.intersects(&ray) {
        Some(hit_record) => match hit_record.material_index {
            Some(index) => match scene.materials[index].scatter(&ray, &hit_record, scene) {
                (Some(reflective_ray), color) => color * trace_ray(reflective_ray, depth - 1.0, scene),
                (None, color) => color,
            },
            _ => FALLBACK_COLOR
        },
        _ => {
            // Uncomment to enable sky gradient.
            //let a = 0.5 * (ray.direction.y + 1.0);
            //(1.0 - a) * Color::WHITE + a * Color::rgb(0.5, 0.7, 1.0)
            Color::WHITE
        }
    }
}
