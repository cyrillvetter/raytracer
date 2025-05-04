use crate::{
    IMAGE_WIDTH, IMAGE_HEIGHT,
    primitive::Ray,
};

use glam::{Vec3A, Affine3A};
use fastrand::f32;

#[derive(Debug)]
pub struct Camera {
    half_width: f32,
    half_height: f32,
    meter_per_pixel: f32,
    focal_length: f32,
    transform: Affine3A
}

// TODO: Try to simplify the orthographic/perspective calculation.
impl Camera {
    pub fn new(aspect_ratio: f32, y_fov: f32, transform: Affine3A) -> Self {
        let h = aspect_ratio.recip();

        let pixel_height = IMAGE_HEIGHT as f32;
        let pixel_width = IMAGE_WIDTH as f32;

        Camera {
            half_width: pixel_width / 2.0,
            half_height: pixel_height / 2.0,
            meter_per_pixel: h / pixel_height,
            focal_length: (h / 2.0) / f32::tan(y_fov / 2.0),
            transform,
        }
    }

    pub fn ray_from(&self, x: usize, y: usize) -> Ray {
        let x_offset = f32() - 0.5;
        let y_offset = f32() - 0.5;

        let plane_x = ((x as f32) + x_offset - self.half_width) * self.meter_per_pixel;
        let plane_y = (self.half_height - (y as f32) + y_offset) * self.meter_per_pixel;

        Ray::new(
            self.transform.translation,
            self.transform.transform_vector3a(Vec3A::new(plane_x, plane_y, -self.focal_length).normalize())
        )
    }
}
