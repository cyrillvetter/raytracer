use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use crate::primitive::*;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u32>
}

impl Image {
    pub fn blank(width: u32, height: u32) -> Self {
        Self { width, height, bytes: vec![0; (width * height) as usize] }
    }

    #[allow(dead_code)]
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let pos = ((y * self.width) + x) as usize;

        let pixel = &self.bytes[pos];
        let r = ((pixel >> 16 & 0xFF) as f32) / 255.0;
        let g = ((pixel >> 8 & 0xFF) as f32) / 255.0;
        let b = ((pixel & 0xFF) as f32) / 255.0;

        Color::rgb(r, g, b)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let pos = ((y * self.width) + x) as usize;
        self.bytes[pos] = ((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32);
    }

    pub fn save_png(&self, out_path: &str) {
        let path = Path::new(out_path);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);

        // TODO: Check these settings.
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut image_buffer: Vec<u8> = Vec::with_capacity((self.width * self.height * 4) as usize);
        for pixel in self.bytes.iter() {
            let r = (pixel >> 16 & 0xFF) as u8;
            let g = (pixel >> 8 & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            image_buffer.push(r);
            image_buffer.push(g);
            image_buffer.push(b);
            image_buffer.push(255);
        }

        encoder
            .write_header().unwrap()
            .write_image_data(&image_buffer).unwrap();
    }
}
