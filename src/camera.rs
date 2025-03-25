use crate::primitive::Ray;
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};

use glam::Vec3;

#[derive(Debug, Clone)]
pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera)
}

#[derive(Debug, Clone)]
pub struct PerspectiveCamera {
}

#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    half_width: f32,
    half_height: f32,
    meter_per_pixel: f32,
    z_near: f32,
}

impl Camera {
    pub fn orthographic(y_mag: f32, z_near: f32) -> Self {
        Self::Orthographic(OrthographicCamera {
            half_width: (IMAGE_WIDTH as f32) / 2.0,
            half_height: (IMAGE_HEIGHT as f32) / 2.0,
            meter_per_pixel: (y_mag * 2.0) / (IMAGE_HEIGHT as f32),
            z_near,
        })
    }

    pub fn ray_from(&self, x: u32, y: u32) -> Ray {
        match self {
            Camera::Orthographic(cam) => {
                let world_x = ((x as f32) - cam.half_width) * cam.meter_per_pixel;
                let world_y = (cam.half_height - (y as f32)) * cam.meter_per_pixel;
                Ray::new(Vec3::new(world_x, world_y, cam.z_near), Vec3::NEG_Z)
            },
            _ => unimplemented!()
        }
    }
}
