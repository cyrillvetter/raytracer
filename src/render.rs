use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::primitive::*;
use crate::triangle::Triangle;
use crate::scene::Scene;
use crate::Image;

use glam::Vec3;

const BACKGROUND: Color = Color::BLACK;
const DEFAULT_COLOR: Color = Color::gray(0.8);
const AMBIENT_FACTOR: f32 = 0.05;

pub fn render_scene(scene: Scene) -> Image {
    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let ray = scene.camera.ray_from(x, y);

            let mut nearest_dist = f32::INFINITY;
            let mut nearest_triangle: Option<&Triangle> = None;

            for triangle in scene.triangles.iter() {
                match triangle.hit(&ray) {
                    Some(dist) if dist < nearest_dist => {
                        nearest_dist = dist;
                        nearest_triangle = Some(triangle);
                    },
                    _ => ()
                }
            }

            let color = match nearest_triangle {
                Some(triangle) => calculate_color(triangle, &scene, ray.at(nearest_dist)),
                _ => BACKGROUND
            };

            image.set_pixel(x, y, color);
        }
    }

    image
}

fn calculate_color(triangle: &Triangle, scene: &Scene, hit: Vec3) -> Color {
    let triangle_color = triangle
        .material_index
        .map_or(DEFAULT_COLOR, |index| scene.materials[index].color_at(hit));
    let mut color = triangle_color * AMBIENT_FACTOR;

    for light in scene.lights.iter() {
        let light_dir = (light.origin - hit).normalize();
        let shadow_ray = Ray::new(hit + light_dir * f32::EPSILON, light_dir);
        let mut in_shadow = false;

        for triangle in scene.triangles.iter() {
            if triangle.hit(&shadow_ray).is_some() {
                color += triangle_color * 0.5;
                in_shadow = true;
                break;
            }
        }

        if !in_shadow {
            let s = (light.origin - hit).normalize();
            let diffuse = triangle_color * s.dot(triangle.v1.normal).max(0.0) * light.color * light.intensity;
            color += diffuse;
        }
    }

    color.clamp()
}
