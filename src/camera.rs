use crate::primitive::Ray;
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};

use glam::{Vec3, Affine3A};

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
    transform: Affine3A
}

impl Camera {
    pub fn orthographic(y_mag: f32, z_near: f32, transform: Affine3A) -> Self {
        Self::Orthographic(OrthographicCamera {
            half_width: (IMAGE_WIDTH as f32) / 2.0,
            half_height: (IMAGE_HEIGHT as f32) / 2.0,
            meter_per_pixel: (y_mag * 2.0) / (IMAGE_HEIGHT as f32),
            z_near,
            transform,
        })
    }

    pub fn ray_from(&self, x: u32, y: u32) -> Ray {
        match self {
            Camera::Orthographic(orth) => {
                let world_x = ((x as f32) - orth.half_width) * orth.meter_per_pixel;
                let world_y = (orth.half_height - (y as f32)) * orth.meter_per_pixel;
                Ray::new(
                    orth.transform.transform_point3(Vec3::new(world_x, world_y, orth.z_near)),
                    orth.transform.transform_vector3(Vec3::NEG_Z)
                )
            },
            _ => unimplemented!()
        }
    }
}
