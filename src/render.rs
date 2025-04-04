use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, AA_SIZE};
use crate::primitive::*;
use crate::triangle::HitRecord;
use crate::scene::Scene;
use crate::Image;
use crate::material::Scatterable;

const FALLBACK_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const MAX_DEPTH: f32 = 5.0;
const BACKGROUND: Color = Color::BLACK;

pub fn render_scene(scene: Scene) -> Image {
    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let mut color = Color::BLACK;

            for x_offset in 0..AA_SIZE {
                for y_offset in 0..AA_SIZE {
                    let ray = scene.camera.ray_from((x * AA_SIZE) + x_offset, (y * AA_SIZE) + y_offset);
                    color += trace_ray(ray, MAX_DEPTH, &scene);
                }
            }

            image.set_pixel(x, y, (color / (AA_SIZE * AA_SIZE) as f32).gamma_correct());
        }
    }

    image
}

fn trace_ray(ray: Ray, depth: f32, scene: &Scene) -> Color {
    if depth < f32::EPSILON {
        return Color::BLACK;
    }

    let mut nearest_dist = f32::INFINITY;
    let mut nearest_hit: Option<HitRecord> = None;

    for triangle in scene.triangles.iter() {
        match triangle.hit(&ray) {
            Some(hit_record) if hit_record.t < nearest_dist => {
                nearest_dist = hit_record.t;
                nearest_hit = Some(hit_record);
            },
            _ => ()
        }
    }

    match nearest_hit {
        Some(hit_record) => match hit_record.material_index {
            Some(index) => match scene.materials[index].scatter(&ray, &hit_record, scene) {
                (Some(reflective_ray), color) => color * (0.9f32).powf(MAX_DEPTH - depth) * trace_ray(reflective_ray, depth - 1.0, scene),
                (None, color) => color,
            },
            _ => FALLBACK_COLOR
        },
        _ => BACKGROUND
    }
}
