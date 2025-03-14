mod primitive;

mod render;
pub use render::render_image;

mod image;
pub use image::Image;

mod hittable;

mod camera;
pub use camera::Camera;

mod light;
pub use light::Light;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;
