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

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const AA_SIZE: u32 = 1;
