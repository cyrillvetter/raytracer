use std::time::Instant;
use std::fs::read_dir;
use std::io::stdin;
use std::path::{Path, PathBuf};

use minifb::{Window, WindowOptions, Key, KeyRepeat};

use raytracer::{Image, render_scene, scene::Scene, util::Statistics};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

static SCENES_PATH: &str = "scenes/";
static OUT_PATH: &str = "out/image.png";

fn main() {
    let mut statistics = Statistics::new();

    let scene_path = pick_scene_path();
    let mut now = Instant::now();

    let scene = Scene::import(&scene_path);
    let bvh_elapsed = now.elapsed();
    statistics.add("Triangles", &scene.bvh.triangles.len());
    statistics.add("Bvh nodes", &scene.bvh.nodes_used);
    statistics.add_str("Bvh construction time", format!("{:.2?}", bvh_elapsed));

    now = Instant::now();
    let image = render_scene(scene);
    let render_elapsed = now.elapsed();
    statistics.add_str("Render time", format!("{:.2?}", render_elapsed));
    statistics.add_str("Total time", format!("{:.2?}", bvh_elapsed + render_elapsed));

    statistics.print();
    show_image(&image);
}

fn pick_scene_path() -> PathBuf {
    let paths = read_dir(SCENES_PATH).expect("No scenes found");
    let scene_paths: Vec<PathBuf> = paths
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| path
            .extension()
            .map_or(false, |ext| ext == "gltf")
            .then_some(path))
        .collect();

    if scene_paths.is_empty() {
        panic!("No scenes found.");
    }

    if scene_paths.len() == 1 {
        return scene_paths[0].clone();
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

    scene_paths[i - 1].clone()
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
