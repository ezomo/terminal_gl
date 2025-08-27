use crate::geometry::{Vec2, Vec3, Color};
use crate::matrix::{Mat4, Transform};
use crate::terminal_gl::{Canvas, ColoredCoord};
use crate::geometry::{draw_line, draw_triangle_wireframe, draw_triangle_filled};

#[derive(Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub color: Color,
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            transform: Transform::new(),
        }
    }

    // キューブの作成
    pub fn create_cube(size: f32) -> Self {
        let s = size * 0.5;
        let vertices = vec![
            // Front face
            Vertex::new(Vec3::new(-s, -s, s)),
            Vertex::new(Vec3::new(s, -s, s)),
            Vertex::new(Vec3::new(s, s, s)),
            Vertex::new(Vec3::new(-s, s, s)),
            // Back face
            Vertex::new(Vec3::new(-s, -s, -s)),
            Vertex::new(Vec3::new(s, -s, -s)),
            Vertex::new(Vec3::new(s, s, -s)),
            Vertex::new(Vec3::new(-s, s, -s)),
        ];

        let triangles = vec![
            // Front face
            Triangle { vertices: [0, 1, 2], color: Color::RED },
            Triangle { vertices: [0, 2, 3], color: Color::RED },
            // Back face
            Triangle { vertices: [4, 6, 5], color: Color::GREEN },
            Triangle { vertices: [4, 7, 6], color: Color::GREEN },
            // Left face
            Triangle { vertices: [4, 0, 3], color: Color::BLUE },
            Triangle { vertices: [4, 3, 7], color: Color::BLUE },
            // Right face
            Triangle { vertices: [1, 5, 6], color: Color::YELLOW },
            Triangle { vertices: [1, 6, 2], color: Color::YELLOW },
            // Top face
            Triangle { vertices: [3, 2, 6], color: Color::CYAN },
            Triangle { vertices: [3, 6, 7], color: Color::CYAN },
            // Bottom face
            Triangle { vertices: [4, 1, 0], color: Color::MAGENTA },
            Triangle { vertices: [4, 5, 1], color: Color::MAGENTA },
        ];

        Self {
            vertices,
            triangles,
            transform: Transform::new(),
        }
    }

    // 平面の作成
    pub fn create_plane(size: f32) -> Self {
        let s = size * 0.5;
        let vertices = vec![
            Vertex::new(Vec3::new(-s, 0.0, -s)),
            Vertex::new(Vec3::new(s, 0.0, -s)),
            Vertex::new(Vec3::new(s, 0.0, s)),
            Vertex::new(Vec3::new(-s, 0.0, s)),
        ];

        let triangles = vec![
            Triangle { vertices: [0, 1, 2], color: Color::WHITE },
            Triangle { vertices: [0, 2, 3], color: Color::WHITE },
        ];

        Self {
            vertices,
            triangles,
            transform: Transform::new(),
        }
    }

    // ピラミッドの作成
    pub fn create_pyramid(size: f32) -> Self {
        let s = size * 0.5;
        let vertices = vec![
            // Base
            Vertex::new(Vec3::new(-s, -s, -s)),
            Vertex::new(Vec3::new(s, -s, -s)),
            Vertex::new(Vec3::new(s, -s, s)),
            Vertex::new(Vec3::new(-s, -s, s)),
            // Apex
            Vertex::new(Vec3::new(0.0, s, 0.0)),
        ];

        let triangles = vec![
            // Base
            Triangle { vertices: [0, 2, 1], color: Color::RED },
            Triangle { vertices: [0, 3, 2], color: Color::RED },
            // Sides
            Triangle { vertices: [0, 1, 4], color: Color::GREEN },
            Triangle { vertices: [1, 2, 4], color: Color::BLUE },
            Triangle { vertices: [2, 3, 4], color: Color::YELLOW },
            Triangle { vertices: [3, 0, 4], color: Color::CYAN },
        ];

        Self {
            vertices,
            triangles,
            transform: Transform::new(),
        }
    }

    pub fn render_wireframe(
        &self,
        canvas: &Canvas,
        view_projection: &Mat4,
        pixels: &mut Vec<ColoredCoord>
    ) {
        let model_matrix = self.transform.to_matrix();
        let mvp = view_projection.multiply(&model_matrix);

        for triangle in &self.triangles {
            let v0 = &self.vertices[triangle.vertices[0]];
            let v1 = &self.vertices[triangle.vertices[1]];
            let v2 = &self.vertices[triangle.vertices[2]];

            let p0 = mvp.project_to_screen(v0.position, canvas.width as f32, canvas.height as f32);
            let p1 = mvp.project_to_screen(v1.position, canvas.width as f32, canvas.height as f32);
            let p2 = mvp.project_to_screen(v2.position, canvas.width as f32, canvas.height as f32);

            draw_triangle_wireframe(p0, p1, p2, canvas, triangle.color, pixels);
        }
    }

    pub fn render_filled(
        &self,
        canvas: &Canvas,
        view_projection: &Mat4,
        pixels: &mut Vec<ColoredCoord>
    ) {
        let model_matrix = self.transform.to_matrix();
        let mvp = view_projection.multiply(&model_matrix);

        for triangle in &self.triangles {
            let v0 = &self.vertices[triangle.vertices[0]];
            let v1 = &self.vertices[triangle.vertices[1]];
            let v2 = &self.vertices[triangle.vertices[2]];

            // Back-face culling
            let world_v0 = model_matrix.transform_point(v0.position);
            let world_v1 = model_matrix.transform_point(v1.position);
            let world_v2 = model_matrix.transform_point(v2.position);

            let edge1 = Vec3::new(
                world_v1.x - world_v0.x,
                world_v1.y - world_v0.y,
                world_v1.z - world_v0.z,
            );
            let edge2 = Vec3::new(
                world_v2.x - world_v0.x,
                world_v2.y - world_v0.y,
                world_v2.z - world_v0.z,
            );

            let normal = edge1.cross(&edge2);
            let view_dir = Vec3::new(0.0, 0.0, 1.0); // Simplified view direction

            // Skip back-facing triangles
            if normal.dot(&view_dir) < 0.0 {
                continue;
            }

            let p0 = mvp.project_to_screen(v0.position, canvas.width as f32, canvas.height as f32);
            let p1 = mvp.project_to_screen(v1.position, canvas.width as f32, canvas.height as f32);
            let p2 = mvp.project_to_screen(v2.position, canvas.width as f32, canvas.height as f32);

            draw_triangle_filled(p0, p1, p2, canvas, triangle.color, pixels);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}