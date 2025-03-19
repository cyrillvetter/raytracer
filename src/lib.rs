mod primitive;

mod render;
pub use render::render_image;

mod image;
pub use image::Image;

mod hittable;

mod scene;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;
