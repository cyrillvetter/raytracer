use crate::{
    IMAGE_WIDTH, IMAGE_HEIGHT, BOUNCES, AA_SIZE,
    primitive::*,
    Scene,
    material::Scatterable
};

use rayon::prelude::*;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

pub fn render_scene(scene: &Scene) -> Vec<u32> {
    let mut pixels = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];
    let bands: Vec<(usize, &mut [u32])> = pixels.chunks_mut(IMAGE_WIDTH).enumerate().collect();

    bands
        .into_par_iter()
        .for_each(|(y, band)| {
            render_line(band, y, scene);
        });

    pixels
}

fn render_line(pixels: &mut [u32], y: usize, scene: &Scene) {
    for (x, pixel) in pixels.iter_mut().enumerate() {
        let mut color = Color::BLACK;

        for x_offset in 0..AA_SIZE {
            for y_offset in 0..AA_SIZE {
                let ray = scene.camera.ray_from((x * AA_SIZE) + x_offset, (y * AA_SIZE) + y_offset);
                color += trace_ray(ray, BOUNCES, &scene);
            }
        }

        *pixel = (color / (AA_SIZE * AA_SIZE) as f32).gamma_correct().into_u32();
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
