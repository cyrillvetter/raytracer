mod math;

mod render;
pub use render::render_image;

mod color;
pub use color::Color;

mod image;
pub use image::Image;

mod hittable;

mod camera;
pub use camera::Camera;

mod light;
pub use light::Light;

pub const IMAGE_WIDTH: u32 = 1280;
pub const IMAGE_HEIGHT: u32 = 720;
