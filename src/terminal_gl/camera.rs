use crate::geometry::Vec3;
use crate::matrix::Mat4;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
    pub right: Vec3,

    // Projection parameters
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,

    // Camera angles for FPS-style movement
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            forward: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::new(1.0, 0.0, 0.0),

            fov: 45.0_f32.to_radians(),
            aspect_ratio: screen_width / screen_height,
            near_plane: 0.1,
            far_plane: 100.0,

            yaw: -90.0_f32.to_radians(), // Point towards -Z initially
            pitch: 0.0,
        };

        camera.update_vectors();
        camera
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.update_target();
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;

        // Calculate yaw and pitch from position to target
        let direction = Vec3::new(
            target.x - self.position.x,
            target.y - self.position.y,
            target.z - self.position.z,
        )
        .normalize();

        self.yaw = direction.z.atan2(direction.x);
        self.pitch = direction.y.asin();

        self.update_vectors();
    }

    pub fn move_forward(&mut self, distance: f32) {
        let forward_movement = Vec3::new(
            self.forward.x * distance,
            self.forward.y * distance,
            self.forward.z * distance,
        );

        self.position.x += forward_movement.x;
        self.position.y += forward_movement.y;
        self.position.z += forward_movement.z;

        self.update_target();
    }

    pub fn move_right(&mut self, distance: f32) {
        let right_movement = Vec3::new(
            self.right.x * distance,
            self.right.y * distance,
            self.right.z * distance,
        );

        self.position.x += right_movement.x;
        self.position.y += right_movement.y;
        self.position.z += right_movement.z;

        self.update_target();
    }

    pub fn move_up(&mut self, distance: f32) {
        let up_movement = Vec3::new(
            self.up.x * distance,
            self.up.y * distance,
            self.up.z * distance,
        );

        self.position.x += up_movement.x;
        self.position.y += up_movement.y;
        self.position.z += up_movement.z;

        self.update_target();
    }

    pub fn rotate(&mut self, yaw_delta: f32, pitch_delta: f32) {
        self.yaw += yaw_delta;
        self.pitch += pitch_delta;

        // Constrain pitch to avoid gimbal lock
        self.pitch = self
            .pitch
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        self.update_vectors();
        self.update_target();
    }

    pub fn set_fov(&mut self, fov_degrees: f32) {
        self.fov = fov_degrees.to_radians();
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.target, self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn get_view_projection_matrix(&self) -> Mat4 {
        let view = self.get_view_matrix();
        let projection = self.get_projection_matrix();
        projection.multiply(&view)
    }

    fn update_vectors(&mut self) {
        // Calculate the new forward vector
        self.forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        // Calculate right vector (cross product of forward and world up)
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        self.right = self.forward.cross(&world_up).normalize();

        // Calculate up vector (cross product of right and forward)
        self.up = self.right.cross(&self.forward).normalize();
    }

    fn update_target(&mut self) {
        self.target = Vec3::new(
            self.position.x + self.forward.x,
            self.position.y + self.forward.y,
            self.position.z + self.forward.z,
        );
    }
}
