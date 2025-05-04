use std::{
    path::PathBuf,
    fs::File,
    io::BufWriter
};

static OUT_PATH: &str = "out/";

pub fn save_png(image_name: &str, width: usize, height: usize, pixels: Vec<u32>) {
    let mut out_path = PathBuf::new();
    out_path.push(OUT_PATH);
    out_path.push(format!("{}.png", image_name));

    let file = File::create(out_path).unwrap();
    let w = BufWriter::new(file);

    // TODO: Check these settings.
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);
    encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);

    let mut image_buffer: Vec<u8> = Vec::with_capacity(width * height * 3);
    for pixel in pixels.iter() {
        let r = (pixel >> 16 & 0xFF) as u8;
        let g = (pixel >> 8 & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;

        image_buffer.push(r);
        image_buffer.push(g);
        image_buffer.push(b);
    }

    encoder
        .write_header().unwrap()
        .write_image_data(&image_buffer).unwrap();
}
