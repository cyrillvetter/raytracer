use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::primitive::*;
use crate::triangle::HitRecord;
use crate::scene::Scene;
use crate::Image;
use crate::material::Scatterable;

const BACKGROUND: Color = Color::BLACK;
const DEFAULT_COLOR: Color = Color::gray(0.8);

pub fn render_scene(scene: Scene) -> Image {
    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let ray = scene.camera.ray_from(x, y);
            let color = trace_ray(ray, 0.0, &scene);
            image.set_pixel(x, y, color.gamma_correct());
        }
    }

    image
}

fn trace_ray(ray: Ray, depth: f32, scene: &Scene) -> Color {
    if depth >= 5.0 {
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
                (Some(reflective_ray), color) => color * trace_ray(reflective_ray, depth - 1.0, scene),
                (None, color) => color,
            },
            _ => DEFAULT_COLOR
        },
        _ => BACKGROUND,
    }
}
