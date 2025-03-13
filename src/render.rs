use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::math::Vec3;
use crate::image::Image;
use crate::color::Color;
use crate::hittable::{Hittable, Sphere, Triangle};
use crate::camera::Camera;
use crate::light::Light;

pub fn render_image() -> Image {
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(-0.575, 0.0, -1.0), 0.25, Color::rgb_u8(207, 54, 67))),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.25, Color::rgb_u8(55, 184, 57))),
        Box::new(Sphere::new(Vec3::new(0.575, 0.0, -1.0), 0.25, Color::rgb_u8(54, 55, 207))),
        Box::new(Triangle::new(Vec3::new(0.0, 0.0, -0.25), Vec3::new(0.5, 0.0, -0.25), Vec3::new(0.0, 0.5, -0.25), Color::RED)),
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
            let mut color = Color::BLACK;

            for object in &objects {
                match object.hit(&ray) {
                    Some(dist) if dist < nearest_dist => {
                        nearest_dist = dist;
                        color = object.get_color(ray.at(dist), &lights);
                    },
                    _ => ()
                }
            }

            image.set_pixel(x, y, color);
        }
    }

    image
}

