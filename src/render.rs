use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::primitive::*;
use crate::Image;
use crate::hittable::{Hittable, HitRecord, Sphere};
use crate::scene::{Scene, Camera, Light};

const BACKGROUND: Color = Color::BLACK;

pub fn render_image() -> Image {
    let mut scene = Scene::import("scenes/cube.gltf");

    // Temporarily add spheres manually.
    scene.objects.push(Box::new(Sphere::new(Vec3::new(-0.575, 0.0, -1.0), 0.25, Color::rgb_u8(207, 54, 67))));
    scene.objects.push(Box::new(Sphere::new(Vec3::new(0.575, 0.0, -1.0), 0.25, Color::rgb_u8(54, 55, 207))));

    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let ray = scene.camera.ray_from(x, y);

            let mut nearest_dist = f32::INFINITY;
            let mut nearest_hit: Option<HitRecord> = None;
            let mut nearest_object: Option<&Box<dyn Hittable>> = None;

            for object in scene.objects.iter() {
                match object.hit(&ray) {
                    Some(r) if r.t < nearest_dist => {
                        nearest_dist = r.t;
                        nearest_hit = Some(r);
                        nearest_object = Some(object);
                    },
                    _ => ()
                }
            }

            let color = match nearest_object {
                Some(object) => object.get_color(nearest_hit.unwrap(), &scene),
                None => BACKGROUND
            };

            image.set_pixel(x, y, color);
        }
    }

    image
}

#[allow(dead_code)]
fn create_scene() -> Scene {
    Scene {
        camera: Camera::new(1.0),
        lights: vec![
            Light::new(Vec3::new(-10.0, 7.0, 12.0), Color::rgb(0.992, 0.973, 0.918), 1.0),
        ],
        objects: vec![
            Box::new(Sphere::new(Vec3::new(-0.575, 0.0, -1.0), 0.25, Color::rgb_u8(207, 54, 67))),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.25, Color::rgb_u8(55, 184, 57))),
            Box::new(Sphere::new(Vec3::new(0.575, 0.0, -1.0), 0.25, Color::rgb_u8(54, 55, 207))),
        ]
    }
}
