mod render;
pub use render::render_scene;

mod primitive;

mod triangle;

mod material;
pub use material::Material;

mod image;
pub use image::Image;

pub mod scene;

pub mod bvh;

mod camera;
pub use camera::Camera;

mod light;
pub use light::Light;

pub mod util;

pub const IMAGE_WIDTH: u32 = 1920;
pub const IMAGE_HEIGHT: u32 = 1080;

#[cfg(debug_assertions)]
pub const SAMPLES: usize = 32;

#[cfg(not(debug_assertions))]
pub const SAMPLES: usize = 4096;
