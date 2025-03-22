use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::primitive::Color;
use crate::Image;
use crate::triangle::Triangle;
use crate::scene::Scene;

use glam::Vec3;

const BACKGROUND: Color = Color::BLACK;
const AMBIENT_FACTOR: f32 = 0.05;

pub fn render_image() -> Image {
    let scene = Scene::import("scenes/monkey.gltf");
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
    let mut color = triangle.color * AMBIENT_FACTOR;

    for light in scene.lights.iter() {
        let s = (light.origin - hit).normalize();
        let diffuse = triangle.color * s.dot(triangle.v1.normal).max(0.0) * light.color * light.intensity;
        color += diffuse;
    }

    color.clamp()
}
