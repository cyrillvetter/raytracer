mod render;
pub use render::render_image;

mod primitive;

mod triangle;

mod material;
pub use material::Material;

mod image;
pub use image::Image;

mod scene;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;
