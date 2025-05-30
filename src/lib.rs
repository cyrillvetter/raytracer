pub const IMAGE_WIDTH: usize = 1024;
pub const IMAGE_HEIGHT: usize = 1024;

pub const BOUNCES: usize = 6;

#[cfg(debug_assertions)]
pub const SAMPLES: usize = 32;

#[cfg(not(debug_assertions))]
pub const SAMPLES: usize = 4096;

mod render;
pub use render::render_scene;

pub mod scene;
pub use scene::Scene;

pub mod util;

mod primitive;

mod triangle;

mod material;
use material::Material;

mod sampler;
use sampler::{Texture, Sampler};

mod bvh;
use bvh::Bvh;

mod camera;
use camera::Camera;
