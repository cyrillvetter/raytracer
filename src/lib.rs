mod render;
pub use render::render_scene;

mod primitive;

mod triangle;

mod material;
pub use material::Material;

mod image;
pub use image::Image;

pub mod scene;

mod camera;
pub use camera::Camera;

mod light;
pub use light::Light;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;
const AA_SIZE: u32 = 4;
