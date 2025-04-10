use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u32>
}

impl Image {
    pub fn new(width: u32, height: u32, bytes: Vec<u32>) -> Self {
        Self { width, height, bytes }
    }

    pub fn save_png(&self, out_path: &Path) {
        let file = File::create(out_path).unwrap();
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
