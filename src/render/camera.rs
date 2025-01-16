use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        let look_at_target = self.position + self.forward;
        Mat4::look_at_lh(self.position, look_at_target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    pub fn update_direction(&mut self) {
        // Convert yaw and pitch to radians
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        // Calculate the forward vector
        self.forward = Vec3::new(
            yaw_rad.sin() * pitch_rad.cos(), // X (Right)
            yaw_rad.cos() * pitch_rad.cos(), // Y (Forward)
            pitch_rad.sin(),                 // Z (Up/Down)
        )
        .normalize();

        self.right = self.forward.cross(Vec3::new(0.0, 0.0, 1.0)).normalize(); // Right from forward and global up
        self.up = self.right.cross(self.forward).normalize(); // Up from right and forward
    }
}
