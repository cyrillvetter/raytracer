mod vec3;
mod color;
mod image;

use std::time::Instant;

use crate::image::*;
use crate::vec3::*;
use crate::color::*;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const OUT_PATH: &'static str = "out/test.ppm";

fn main() {
    let now = Instant::now();
    let image = create_area_circle();
    let elapsed = now.elapsed();

    image.write_ppm(OUT_PATH).unwrap();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_area_circle() -> Image {
    const RADIUS: f32 = 150.0;
    let radius_squared = f32::powf(RADIUS, 2.0);

    let mut image = Image::blank(WIDTH, HEIGHT);
    const CENTER: Vec3 = Vec3::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32, 0.0);

    for x in 0..WIDTH {
        let f_x = x as f32;

        for y in 0..HEIGHT {
            if (Vec3::new(f_x, y as f32, 0.0) - CENTER).length_squared() <= radius_squared {
                image.set_pixel(x, y, Color::RED);
            } else {
                image.set_pixel(x, y, Color::WHITE);
            }
        }
    }

    image
}
