use crate::primitive::*;
use crate::scene::Scene;

use super::{Hittable, Triangle};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub color: Color,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, color: Color) -> Self {
        Self { triangles, color }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let mut nearest = f32::INFINITY;
        let mut hit = false;

        for triangle in self.triangles.iter() {
            match triangle.hit(ray) {
                Some(q) if q < nearest => {
                    hit = true;
                    nearest = q;
                },
                _ => ()
            }
        }

        if hit {
            Some(nearest)
        } else {
            None
        }
    }

    fn get_color(&self, _q: Vec3, _scene: &Scene) -> Color {
        Color::BLACK
    }
}
