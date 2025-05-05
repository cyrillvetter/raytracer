use crate::{
    IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES, BOUNCES,
    primitive::*,
    util::{ProgressBar, save_png},
    Scene,
    material::Scatterable
};

use rayon::prelude::*;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

pub fn render_scene(scene: Scene) {
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT);
    let mut pixels = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];
    let bands: Vec<(usize, &mut [u32])> = pixels.chunks_mut(IMAGE_WIDTH).enumerate().collect();

    bands
        .into_par_iter()
        .for_each(|(y, band)| {
            render_line(band, y, &scene);
            progress_bar.update();
        });

    save_png(&scene.name, IMAGE_WIDTH, IMAGE_HEIGHT, pixels);
    progress_bar.end();
}

fn render_line(pixels: &mut [u32], y: usize, scene: &Scene) {
    for (x, pixel) in pixels.iter_mut().enumerate() {
        let mut color = Color::BLACK;

        for _ in 0..SAMPLES {
            let ray = scene.camera.ray_from(x, y);
            color += trace_ray(ray, BOUNCES, &scene);
        }

        *pixel = (color / SAMPLES as f32).gamma_correct().into_u32();
    }
}

fn trace_ray(ray: Ray, depth: usize, scene: &Scene) -> Color {
    if depth <= 0 {
        return Color::BLACK;
    }

    match scene.bvh.intersects(&ray) {
        Some(hit_record) => match hit_record.material_index {
            Some(index) => match scene.materials[index].scatter(&ray, &hit_record, scene) {
                (Some(reflective_ray), color) => color * trace_ray(reflective_ray, depth - 1, scene),
                (None, color) => color,
            },
            _ => FALLBACK_COLOR
        },
        _ => {
            // Sky gradient.
            let a = 0.5 * (ray.direction.y + 1.0);
            (1.0 - a) * Color::WHITE + a * Color::rgb(0.5, 0.7, 1.0)
        }
    }
}
