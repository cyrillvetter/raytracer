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
    half_width: f32,
    half_height: f32,
    meter_per_pixel: f32,
    focal_length: f32,
    transform: Affine3A,
}

#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    half_width: f32,
    half_height: f32,
    meter_per_pixel: f32,
    z_near: f32,
    transform: Affine3A,
}

// TODO: Try to simplify the orthographic/perspective calculation.

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

    pub fn perspective(aspect_ratio: f32, y_fov: f32, transform: Affine3A) -> Self {
        let h = aspect_ratio.recip();
        Self::Perspective(PerspectiveCamera {
            half_width: (IMAGE_WIDTH as f32) / 2.0,
            half_height: (IMAGE_HEIGHT as f32) / 2.0,
            meter_per_pixel: h / (IMAGE_HEIGHT as f32),
            focal_length: (h / 2.0) / f32::tan(y_fov / 2.0),
            transform,
        })
    }

    pub fn ray_from(&self, x: u32, y: u32) -> Ray {
        match self {
            Camera::Orthographic(o) => {
                let world_x = ((x as f32) - o.half_width) * o.meter_per_pixel;
                let world_y = (o.half_height - (y as f32)) * o.meter_per_pixel;
                Ray::new(
                    o.transform.transform_point3(Vec3::new(world_x, world_y, o.z_near)),
                    o.transform.transform_vector3(Vec3::NEG_Z)
                )
            },
            Camera::Perspective(p) => {
                let plane_x = ((x as f32) - p.half_width) * p.meter_per_pixel;
                let plane_y = (p.half_height - (y as f32)) * p.meter_per_pixel;
                Ray::new(
                    p.transform.translation.into(),
                    p.transform.transform_vector3(Vec3::new(plane_x, plane_y, -p.focal_length).normalize())
                )
            }
        }
    }
}
