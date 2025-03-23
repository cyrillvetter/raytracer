use std::time::Instant;
use std::fs::read_dir;
use std::io::stdin;
use std::path::Path;

use minifb::{Window, WindowOptions, Key, KeyRepeat};

use raytracer::{Image, render_scene, scene::Scene};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

static SCENES_PATH: &str = "scenes/";
static OUT_PATH: &str = "out/image.png";

fn main() {
    let scene = pick_scene();

    let now = Instant::now();
    let image = render_scene(scene);
    println!("Elapsed: {:.2?}", now.elapsed());

    show_image(&image);
}

fn pick_scene() -> Scene {
    let paths = read_dir(SCENES_PATH).expect("No scenes found");
    let scene_paths = paths
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| path
            .extension()
            .map_or(false, |ext| ext == "gltf")
            .then_some(path))
        .collect::<Vec<_>>();

    if scene_paths.is_empty() {
        panic!("No scenes found.");
    }

    if scene_paths.len() == 1 {
        return Scene::import(&scene_paths[0]);
    }

    println!("Pick a scene:");
    for (i, path) in scene_paths.iter().enumerate() {
        println!("{}: {}", i + 1, path.file_stem().unwrap().to_str().unwrap());
    }

    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let i = input.trim().parse::<usize>().expect("Cannot parse input");
    println!("");

    if i < 1 || i > scene_paths.len() {
        panic!("Scene number out of range");
    }

    Scene::import(&scene_paths[i - 1])
}

fn show_image(image: &Image) {
    let mut window = Window::new(
        "Raytracer",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions::default()
    ).expect("Failed to create window");

    window.set_target_fps(30);
    let mut image_saved = false;

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        if !image_saved && window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            image_saved = true;
            image.save_png(Path::new(OUT_PATH));
            println!("Image saved to: {}", OUT_PATH);
        }

        window
            .update_with_buffer(&image.bytes, image.width as usize, image.height as usize)
            .expect("Failed to set buffer");
    }
}
