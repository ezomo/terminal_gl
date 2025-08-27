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

    let width = 120 * 2;
    let height = 60 * 2;

    let mut canvas = Canvas::new(width, height);
    let mut scene = Scene::new(width as f32, height as f32);
    let mut renderer = Renderer::new();

    // カメラの初期位置を設定
    scene.camera.set_position(Vec3::new(3.0, 3.0, 5.0));
    scene.camera.look_at(Vec3::new(0.0, 0.0, 0.0));

    // サンプルメッシュを追加
    let mut cube = Mesh::create_cube(2.0);
    cube.transform.position = Vec3::new(0.0, 0.0, 0.0);
    scene.add_mesh(cube);

    let mut pyramid = Mesh::create_pyramid(1.5);
    pyramid.transform.position = Vec3::new(3.0, 0.0, 0.0);
    scene.add_mesh(pyramid);

    let mut plane = Mesh::create_plane(8.0);
    plane.transform.position = Vec3::new(0.0, -2.0, 0.0);
    scene.add_mesh(plane);

    canvas.init();

    // 非ブロッキング入力用のチャンネル
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let mut input = [0];
            if io::stdin().read(&mut input).is_ok() {
                if tx.send(input[0]).is_err() {
                    break;
                }
            }
        }
    });

    let mut last_time = Instant::now();
    let mut rotation_time = 0.0f32;

    'main_loop: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        rotation_time += delta_time;

        // 入力処理
        while let Ok(key) = rx.try_recv() {
            match key {
                27 => break 'main_loop, // ESC
                b'w' | b'W' => scene.camera.move_forward(0.1),
                b's' | b'S' => scene.camera.move_forward(-0.1),
                b'a' | b'A' => scene.camera.move_right(-0.1),
                b'd' | b'D' => scene.camera.move_right(0.1),
                b'q' | b'Q' => scene.camera.move_up(0.1),
                b'e' | b'E' => scene.camera.move_up(-0.1),
                b'r' | b'R' => renderer.toggle_render_mode(),
                b'f' | b'F' => renderer.toggle_fps_display(),
                _ => {}
            }
        }

        // オブジェクトのアニメーション
        if let Some(cube) = scene.meshes.get_mut(0) {
            cube.transform.rotation.x = rotation_time * 0.5;
            cube.transform.rotation.y = rotation_time * 0.3;
        }

        if let Some(pyramid) = scene.meshes.get_mut(1) {
            pyramid.transform.rotation.y = rotation_time * 0.8;
            pyramid.transform.position.y = (rotation_time * 2.0).sin() * 0.5;
        }

        // レンダリング
        renderer.render(&mut canvas, &scene);

        // フレームレート制限
        thread::sleep(Duration::from_millis(16)); // 約60FPS
    }

    // 端末を元に戻す
    print!("\x1b[?25h"); // カーソルを表示
    print!("\x1b[?1049l"); // 通常スクリーンバッファに戻る
    print!("\x1b[2J\x1b[H"); // 画面をクリア

    println!("Terminal Tiny GL terminated.");
}
