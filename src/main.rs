mod terminal_gl;

use terminal_gl::camera::*;
use terminal_gl::geometry::*;
use terminal_gl::matrix::*;
use terminal_gl::mesh::*;
use terminal_gl::renderer::*;
use terminal_gl::*;

use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use std::fs::File;
use std::io::{BufRead, BufReader};

impl Mesh {
    pub fn from_obj_file(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with("v ") {
                // 頂点情報
                let parts: Vec<&str> = line[2..].split_whitespace().collect();
                if parts.len() == 3 {
                    let x: f32 = parts[0].parse().unwrap_or(0.0);
                    let y: f32 = parts[1].parse().unwrap_or(0.0);
                    let z: f32 = parts[2].parse().unwrap_or(0.0);
                    vertices.push(Vertex::new(Vec3::new(x, y, z)));
                }
            } else if line.starts_with("f ") {
                // 面情報 (三角形のみ対応)
                let parts: Vec<&str> = line[2..].split_whitespace().collect();
                if parts.len() == 3 {
                    let idx: Vec<usize> = parts
                        .iter()
                        .map(|p| p.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();
                    triangles.push(Triangle {
                        vertices: [idx[0], idx[1], idx[2]],
                        color: Color::WHITE,
                    });
                }
            }
        }

        Ok(Self {
            vertices,
            triangles,
            transform: Transform::new(),
        })
    }
}

fn main() {
    println!("Terminal Tiny GL - Rust Edition");
    println!("Controls:");
    println!("  WASD: Move camera");
    println!("  QE: Move up/down");
    println!("  Arrow keys: Rotate camera");
    println!("  R: Toggle render mode (wireframe/filled)");
    println!("  F: Toggle FPS display");
    println!("  ESC: Exit");
    println!("\nPress any key to start...");

    let mut input = [0];
    io::stdin().read(&mut input).unwrap();

    // 端末を raw モードにする
    print!("\x1b[?1049h"); // 代替スクリーンバッファを使用
    print!("\x1b[?25l"); // カーソルを隠す

    let width = 2880;
    let height = 1800 * 2;

    let mut canvas = Canvas::new(width, height);
    let mut scene = Scene::new(width as f32, height as f32);
    let mut renderer = Renderer::new();

    // カメラの初期位置を設定
    scene.camera.set_position(Vec3::new(0.0, -1.0, 5.0));
    scene.camera.look_at(Vec3::new(0.0, 0.0, 0.0));

    // サンプルメッシュを追加
    // let mut cube = Mesh::create_cube(2.0);
    // cube.transform.position = Vec3::new(-2.0, 0.0, 0.0);
    // scene.add_mesh(cube);

    // let mut cube1 = Mesh::create_cube(3.0);
    // cube1.transform.position = Vec3::new(4.0, 0.0, 0.0);
    // scene.add_mesh(cube1);

    // let mut pyramid = Mesh::create_pyramid(1.5);
    // pyramid.transform.position = Vec3::new(0., 0., 0.0);
    // scene.add_mesh(pyramid);

    let mut test = Mesh::from_obj_file("african_head.obj").unwrap();
    test.transform.position = Vec3::new(0.0, 0.0, 0.0);
    test.transform.rotation = Vec3::new(0.0, 0.0, 0.0);
    scene.add_mesh(test);

    canvas.init();
    thread::sleep(Duration::from_secs(3)); // 約60FPS

    let mut last_time = Instant::now();
    let mut rotation_time = 0.0f32;

    // loop {
    let current_time = Instant::now();
    let delta_time = current_time.duration_since(last_time).as_secs_f32();
    last_time = current_time;

    rotation_time += delta_time;

    // オブジェクトのアニメーション
    // if let Some(cube) = scene.meshes.get_mut(0) {
    //     cube.transform.rotation.x = rotation_time * 0.5;
    //     cube.transform.rotation.y = rotation_time * 0.3;
    // }

    // if let Some(pyramid) = scene.meshes.get_mut(1) {
    //     pyramid.transform.rotation.y = rotation_time * 0.8;
    //     pyramid.transform.position.y = (rotation_time * 2.0).sin() * 0.5;
    // }

    // if let Some(pyramid) = scene.meshes.get_mut(0) {
    //     pyramid.transform.rotation.y = rotation_time * 0.8;
    //     pyramid.transform.position.y = (rotation_time * 2.0).sin() * 0.5;
    // }

    // レンダリング
    renderer.render(&mut canvas, &scene);

    // フレームレート制限
    // thread::sleep(Duration::from_millis(33)); // 約60FPS

    // }

    thread::sleep(Duration::from_secs(3)); // 約60FPS
}
