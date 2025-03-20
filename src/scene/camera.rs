use crate::primitive::Ray;
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};

use glam::Vec3;

const RAY_DIR: Vec3 = Vec3::new(0.0, 0.0, -1.0);

pub struct Camera {
    half_width: f32,
    half_height: f32,
    meter_per_pixel: f32
}

impl Camera {
    pub const fn new(size: f32) -> Self {
        Self {
            half_width: (IMAGE_WIDTH as f32) / 2.0,
            half_height: (IMAGE_HEIGHT as f32) / 2.0,
            meter_per_pixel: size / (IMAGE_HEIGHT as f32)
        }
    }

    pub fn ray_from(&self, x: u32, y:u32) -> Ray {
        let world_x = ((x as f32) - self.half_width) * self.meter_per_pixel;
        let world_y = (self.half_height - (y as f32)) * self.meter_per_pixel;
        Ray::new(Vec3::new(world_x, world_y, 0.0), RAY_DIR)
    }
}
