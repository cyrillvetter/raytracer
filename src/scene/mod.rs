pub mod camera;
pub use camera::Camera;

pub mod light;
pub use light::Light;

use crate::hittable::Hittable;

use std::path::Path;
use gltf::Gltf;

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn import(path: &str) -> Self {
        let gltf = Gltf::open(Path::new(path)).unwrap();
        let scene = gltf.scenes().next().unwrap();
        todo!()
    }
}
