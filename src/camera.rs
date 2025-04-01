use crate::primitive::Ray;
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, AA_SIZE};

use glam::{Vec3, Affine3A};

#[derive(Debug, Clone)]
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

        let pixel_height = (IMAGE_HEIGHT * AA_SIZE) as f32;
        let pixel_width = (IMAGE_WIDTH * AA_SIZE) as f32;

        Camera {
            half_width: pixel_width / 2.0,
            half_height: pixel_height / 2.0,
            meter_per_pixel: h / pixel_height,
            focal_length: (h / 2.0) / f32::tan(y_fov / 2.0),
            transform,
        }
    }

    pub fn ray_from(&self, x: u32, y: u32) -> Ray {
        let plane_x = ((x as f32) - self.half_width) * self.meter_per_pixel;
        let plane_y = (self.half_height - (y as f32)) * self.meter_per_pixel;
        Ray::new(
            self.transform.translation.into(),
            self.transform.transform_vector3(Vec3::new(plane_x, plane_y, -self.focal_length).normalize())
        )
    }
}
