use crate::color::Color;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u32>
}

impl Image {
    pub fn blank(width: u32, height: u32) -> Self {
        Self { width, height, bytes: vec![0; (width * height) as usize] }
    }

    // TODO: Return a result instead of an option.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        let pos = ((y * self.width) + x) as usize;

        if pos > self.bytes.len() {
            None
        } else {
            let be_bytes = &self.bytes[pos].to_be_bytes();
            Some(Color::rgba(
                be_bytes[3],
                be_bytes[2],
                be_bytes[1],
                be_bytes[0]
            ))
        }
    }

    // TODO: Return a result instead of a boolean.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        let pos = ((y * self.width) + x) as usize;

        if x >= self.width || y >= self.height {
            false
        } else {
            self.bytes[pos] = u32::from_be_bytes([color.a, color.b, color.g, color.r]);
            true
        }
    }
}
