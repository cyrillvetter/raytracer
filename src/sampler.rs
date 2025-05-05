use crate::{primitive::Color, Scene};

use glam::Vec2;
use gltf::image::{Data, Format};

#[derive(Debug)]
pub enum Sampler {
    Color(Color),
    Texture(usize)
}

#[derive(Debug)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>
}

impl Sampler {
    pub fn sample(&self, uv: Option<Vec2>, scene: &Scene) -> Color {
        match self {
            Sampler::Color(color) => *color,
            Sampler::Texture(index) => {
                let Some(uv) = uv else {
                    panic!("Missing uv coordinates required for texture sampling");
                };

                scene.textures[*index].sample(uv)
            }
        }
    }
}

impl Texture {
    pub fn new(image_data: &Data) -> Self {
        let pixels = image_data.pixels
            .chunks(channels_amount(image_data.format))
            .map(|p| Color::rgb_u8(p[0], p[1], p[2]).gamma_uncorrect())
            .collect();

        Self {
            width: image_data.width as usize,
            height: image_data.height as usize,
            pixels
        }
    }

    pub fn sample(&self, uv: Vec2) -> Color {
        let x = (uv.x.fract() * (self.width - 1) as f32).round() as usize;
        let y = (uv.y.fract() * (self.height - 1) as f32).round() as usize;

        self.pixels[y * self.width + x]
    }
}

fn channels_amount(format: Format) -> usize {
    use Format::*;
    match format {
        R8G8B8A8 | R16G16B16A16 | R32G32B32A32FLOAT => 4,
        _ => 3
    }
}
