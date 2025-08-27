use crate::geometry::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        mat.m[0][3] = x;
        mat.m[1][3] = y;
        mat.m[2][3] = z;
        mat
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        mat.m[1][1] = cos_a;
        mat.m[1][2] = -sin_a;
        mat.m[2][1] = sin_a;
        mat.m[2][2] = cos_a;
        mat
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        mat.m[0][0] = cos_a;
        mat.m[0][2] = sin_a;
        mat.m[2][0] = -sin_a;
        mat.m[2][2] = cos_a;
        mat
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        mat.m[0][0] = cos_a;
        mat.m[0][1] = -sin_a;
        mat.m[1][0] = sin_a;
        mat.m[1][1] = cos_a;
        mat
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        mat.m[0][0] = x;
        mat.m[1][1] = y;
        mat.m[2][2] = z;
        mat
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut mat = Self::zero();
        let f = 1.0 / (fov * 0.5).tan();
        mat.m[0][0] = f / aspect;
        mat.m[1][1] = f;
        mat.m[2][2] = (far + near) / (near - far);
        mat.m[2][3] = (2.0 * far * near) / (near - far);
        mat.m[3][2] = -1.0;
        mat
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = Vec3 {
            x: center.x - eye.x,
            y: center.y - eye.y,
            z: center.z - eye.z,
        }
        .normalize();

        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        let mut mat = Self::identity();
        mat.m[0][0] = s.x;
        mat.m[1][0] = s.y;
        mat.m[2][0] = s.z;
        mat.m[0][1] = u.x;
        mat.m[1][1] = u.y;
        mat.m[2][1] = u.z;
        mat.m[0][2] = -f.x;
        mat.m[1][2] = -f.y;
        mat.m[2][2] = -f.z;
        mat.m[0][3] = -s.dot(&eye);
        mat.m[1][3] = -u.dot(&eye);
        mat.m[2][3] = f.dot(&eye);
        mat
    }

    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }

    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let x =
            self.m[0][0] * point.x + self.m[0][1] * point.y + self.m[0][2] * point.z + self.m[0][3];
        let y =
            self.m[1][0] * point.x + self.m[1][1] * point.y + self.m[1][2] * point.z + self.m[1][3];
        let z =
            self.m[2][0] * point.x + self.m[2][1] * point.y + self.m[2][2] * point.z + self.m[2][3];
        let w =
            self.m[3][0] * point.x + self.m[3][1] * point.y + self.m[3][2] * point.z + self.m[3][3];

        if w != 0.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }

    pub fn project_to_screen(&self, point: Vec3, width: f32, height: f32) -> Vec2 {
        let transformed = self.transform_point(point);
        Vec2::new(
            (transformed.x + 1.0) * width * 0.5,
            (1.0 - transformed.y) * height * 0.5,
        )
    }
}

#[derive(Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        let translation = Mat4::translation(self.position.x, self.position.y, self.position.z);
        let rotation_x = Mat4::rotation_x(self.rotation.x);
        let rotation_y = Mat4::rotation_y(self.rotation.y);
        let rotation_z = Mat4::rotation_z(self.rotation.z);
        let scale = Mat4::scale(self.scale.x, self.scale.y, self.scale.z);

        translation
            .multiply(&rotation_z.multiply(&rotation_y.multiply(&rotation_x.multiply(&scale))))
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
