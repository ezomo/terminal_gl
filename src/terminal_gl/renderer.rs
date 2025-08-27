use crate::camera::Camera;
use crate::geometry::{Color, Vec3};
use crate::mesh::Mesh;
use crate::terminal_gl::{Canvas, ColoredCoord};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum RenderMode {
    Wireframe,
    Filled,
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub camera: Camera,
    pub background_color: Color,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            meshes: Vec::new(),
            camera: Camera::new(width, height),
            background_color: Color::BLACK,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn clear_meshes(&mut self) {
        self.meshes.clear();
    }
}

pub struct Renderer {
    pub render_mode: RenderMode,
    pub show_fps: bool,
    frame_count: u32,
    last_fps_time: Instant,
    current_fps: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            render_mode: RenderMode::Wireframe,
            show_fps: false,
            frame_count: 0,
            last_fps_time: Instant::now(),
            current_fps: 0.0,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas, scene: &Scene) {
        canvas.clear();

        let view_projection = scene.camera.get_view_projection_matrix();
        let mut pixels = Vec::with_capacity(10000);

        // Render all meshes
        for mesh in &scene.meshes {
            match self.render_mode {
                RenderMode::Wireframe => {
                    mesh.render_wireframe(canvas, &view_projection, &mut pixels);
                }
                RenderMode::Filled => {
                    mesh.render_filled(canvas, &view_projection, &mut pixels);
                }
            }
        }

        canvas.set_pixels(&mut pixels);

        // Update FPS
        self.update_fps();

        if self.show_fps {
            self.render_fps_counter(canvas);
        }

        canvas.present();
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let delta = now.duration_since(self.last_fps_time).as_secs_f32();

        if delta >= 1.0 {
            self.current_fps = self.frame_count as f32 / delta;
            self.frame_count = 0;
            self.last_fps_time = now;
        }
    }

    fn render_fps_counter(&self, canvas: &Canvas) {
        let fps_text = format!("FPS: {:.1}", self.current_fps);
        self.render_text(canvas, &fps_text, 2, 2, Color::WHITE);
    }

    fn render_text(&self, canvas: &Canvas, text: &str, x: i32, y: i32, color: Color) {
        // Simple text rendering using cursor positioning
        print!(
            "\x1b[{};{}H\x1b[38;2;{};{};{}m{}\x1b[0m",
            y + 1,
            x + 1,
            color.r,
            color.g,
            color.b,
            text
        );
    }

    pub fn toggle_render_mode(&mut self) {
        self.render_mode = match self.render_mode {
            RenderMode::Wireframe => RenderMode::Filled,
            RenderMode::Filled => RenderMode::Wireframe,
        };
    }

    pub fn toggle_fps_display(&mut self) {
        self.show_fps = !self.show_fps;
    }

    pub fn get_fps(&self) -> f32 {
        self.current_fps
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

// ライティング計算用の構造体
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn calculate_lighting(
        &self,
        surface_pos: Vec3,
        surface_normal: Vec3,
        view_pos: Vec3,
    ) -> f32 {
        let light_dir = Vec3::new(
            self.position.x - surface_pos.x,
            self.position.y - surface_pos.y,
            self.position.z - surface_pos.z,
        )
        .normalize();

        let view_dir = Vec3::new(
            view_pos.x - surface_pos.x,
            view_pos.y - surface_pos.y,
            view_pos.z - surface_pos.z,
        )
        .normalize();

        // Lambertian diffuse lighting
        let diffuse = surface_normal.dot(&light_dir).max(0.0);

        // Simple specular highlighting (Phong)
        let reflect_dir = Vec3::new(
            2.0 * surface_normal.dot(&light_dir) * surface_normal.x - light_dir.x,
            2.0 * surface_normal.dot(&light_dir) * surface_normal.y - light_dir.y,
            2.0 * surface_normal.dot(&light_dir) * surface_normal.z - light_dir.z,
        );

        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0);

        (diffuse * 0.8 + specular * 0.2) * self.intensity
    }
}
