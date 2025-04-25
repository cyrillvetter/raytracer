use std::time::Instant;
use std::fs::read_dir;
use std::io::stdin;
use std::path::{Path, PathBuf};

use raytracer::{IMAGE_HEIGHT, IMAGE_WIDTH, Image, render_scene, scene::Scene, util::Statistics};

static SCENES_PATH: &str = "scenes/";

fn main() {
    let mut statistics = Statistics::new();
    statistics.add_str("Resolution", format!("{}x{}", IMAGE_WIDTH, IMAGE_HEIGHT));

    let scene_path = pick_scene_path();
    let mut now = Instant::now();

    let scene = Scene::import(&scene_path);
    let bvh_elapsed = now.elapsed();
    statistics.add("Triangles", &scene.bvh.triangles.len());
    statistics.add("BVH nodes", &scene.bvh.nodes_used);
    statistics.add_str("BVH construction time", format!("{:.2?}", bvh_elapsed));
    statistics.print();

    render_scene(scene);
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
