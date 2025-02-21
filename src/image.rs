use std::fs::File;
use std::io::{BufWriter, Write};

use crate::color::Color;

const RGB: u32 = 3;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>
}

impl Image {
    pub fn blank(width: u32, height: u32) -> Self {
        let bytes = vec![0; ((width * height) * RGB) as usize];
        Self { width, height, bytes }
    }

    // TODO: Return a result instead of an option.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        let start = ((y * self.width) + x) * RGB;
        let end = start + RGB;
        let len = self.bytes.len();

        if start as usize > len || end as usize > len {
            None
        } else {
            let slice = &self.bytes[start as usize..end as usize];
            Some(Color::rgb(
                slice[0],
                slice[1],
                slice[2]
            ))
        }
    }

    // TODO: Return a result instead of a boolean.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        let start = ((y * self.width) + x) * RGB;

        if x >= self.width || y >= self.height {
            false
        } else {
            let u_start = start as usize;
            self.bytes[u_start] = color.r;
            self.bytes[u_start + 1] = color.g;
            self.bytes[u_start + 2] = color.b;

            true
        }
    }

    pub fn write_ppm(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for i in 0..(self.width * self.height) {
            let j = (i * RGB) as usize;
            writeln!(writer, "{} {} {}", self.bytes[j], self.bytes[j + 1], self.bytes[j + 2])?;
        }

        writer.flush()?;
        Ok(())
    }
}
