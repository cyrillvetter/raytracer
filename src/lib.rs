pub const IMAGE_WIDTH: usize = 1920;
pub const IMAGE_HEIGHT: usize = 1080;

pub const BOUNCES: usize = 6;

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

mod light;
use light::Light;
