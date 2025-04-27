use crate::primitive::Color;

use glam::Vec2;
use gltf::image::{Data, Format};

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>
}

impl Texture {
    pub fn new(image_data: Data) -> Self {
        let pixels: Vec<Color> = image_data.pixels
            .chunks(get_chunk_size(image_data.format))
            .map(|p| Color::rgb_u8(p[0], p[1], p[2]).gamma_uncorrect())
            .collect();

        Self {
            width: image_data.width,
            height: image_data.height,
            pixels
        }
    }

    pub fn sample(&self, uv: Vec2) -> Color {
        let x = (uv.x.fract() * (self.width - 1) as f32).round() as usize;
        let y = (uv.y.fract() * (self.height - 1) as f32).round() as usize;

        self.pixels[y * self.width as usize + x]
    }
}

fn get_chunk_size(format: Format) -> usize {
    use Format::*;
    match format {
        R8G8B8A8 | R16G16B16A16 | R32G32B32A32FLOAT => 4,
        _ => 3
    }
}
