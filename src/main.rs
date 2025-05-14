use std::{
    time::Instant,
    fs::read_dir,
    io::stdin,
    path::PathBuf
};

use raytracer::{
    BOUNCES, IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES,
    Scene,
    render_scene,
    util::{save_png, Statistics}
};

static SCENES_PATH: &str = "scenes/";

fn main() {
    let mut statistics = Statistics::new();
    statistics.add_str("Resolution", &format!("{}x{}", IMAGE_WIDTH, IMAGE_HEIGHT));
    statistics.add("Samples", &SAMPLES);
    statistics.add("Bounces", &BOUNCES);

    let scene_path = pick_scene_path();
    let mut now = Instant::now();

    let scene = Scene::import(&scene_path);
    let bvh_elapsed = now.elapsed();
    statistics.add("Triangles", &scene.bvh.triangles.len());
    statistics.add("BVH nodes", &scene.bvh.nodes_used);
    statistics.add_str("Scene construction time", &format!("{:.2?}", bvh_elapsed));

    now = Instant::now();
    let pixels = render_scene(&scene);
    let render_elapsed = now.elapsed();

    save_png(&scene.name, IMAGE_WIDTH, IMAGE_HEIGHT, pixels);

    statistics.add_str("Render time", &format!("{:.2?}", render_elapsed));
    statistics.add_str("Total time", &format!("{:.2?}", bvh_elapsed + render_elapsed));
    statistics.print();
}

fn pick_scene_path() -> PathBuf {
    let paths = read_dir(SCENES_PATH).expect("No scenes found");
    let mut scene_paths: Vec<PathBuf> = paths
        .filter_map(|res| res.ok().map(|entry| entry.path()))
        .filter(|path| path
            .extension()
            .is_some_and(|ext| ext == "glb"))
        .collect();
    scene_paths.sort_unstable();

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

    scene_paths.get(i - 1).expect("Scene number out of range").clone()
}
