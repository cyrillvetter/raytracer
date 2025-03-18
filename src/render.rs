use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::primitive::*;
use crate::image::Image;
use crate::hittable::{Hittable, Sphere, Triangle};
use crate::camera::Camera;
use crate::light::Light;

const BACKGROUND: Color = Color::BLACK;

pub fn render_image() -> Image {
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(-0.575, 0.0, -1.0), 0.25, Color::rgb_u8(207, 54, 67))),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.25, Color::rgb_u8(55, 184, 57))),
        Box::new(Sphere::new(Vec3::new(0.575, 0.0, -1.0), 0.25, Color::rgb_u8(54, 55, 207))),
    ];

    let lights = [
        Light::new(Vec3::new(-10.0, 7.0, 12.0), Color::rgb(0.992, 0.973, 0.918), 1.0),
    ];

    let camera = Camera::new(1.0);
    let mut image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let ray = camera.ray_from(x, y);

            let mut nearest_dist = f32::INFINITY;
            let mut nearest_object: Option<&Box<dyn Hittable>> = None;

            for object in &objects {
                match object.hit(&ray) {
                    Some(dist) if dist < nearest_dist => {
                        nearest_dist = dist;
                        nearest_object = Some(object);
                    },
                    _ => ()
                }
            }

            let color = match nearest_object {
                Some(object) => object.get_color(ray.at(nearest_dist), &lights),
                None => BACKGROUND
            };

            image.set_pixel(x, y, color);
        }
    }

    image
}
