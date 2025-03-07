use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f32,
    left: f32,
    x_step: f32,
    top: f32,
    y_step: f32
}

impl Camera {
    pub const fn new(size: f32) -> Self {
        let aspect_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);

        Self {
            aspect_ratio,
            left: -(size * aspect_ratio) / 2.0,
            x_step: size * (IMAGE_WIDTH as f32),
            top: size / 2.0,
            y_step: size * (IMAGE_HEIGHT as f32)
        }
    }

    pub fn in_world(&self, x: u32, y: u32) -> Vec3 {
        let viewport_x = self.left + ((x as f32) / self.x_step) * self.aspect_ratio;
        let viewport_y = self.top - (y as f32) / self.y_step;
        Vec3::new(viewport_x, viewport_y, 0.0)
    }
}
