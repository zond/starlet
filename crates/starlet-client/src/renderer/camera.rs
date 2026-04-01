use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov_y: 70.0_f32.to_radians(),
            near: 0.1,
            far: 50_000.0,
        }
    }

    pub fn view_matrix(&self, position: Vec3, orientation: Quat) -> Mat4 {
        let forward = orientation * Vec3::NEG_Z;
        let up = orientation * Vec3::Y;
        Mat4::look_to_rh(position, forward, up)
    }

    /// View matrix with translation zeroed — for objects at infinity (stars).
    pub fn sky_view_matrix(&self, orientation: Quat) -> Mat4 {
        let forward = orientation * Vec3::NEG_Z;
        let up = orientation * Vec3::Y;
        Mat4::look_to_rh(Vec3::ZERO, forward, up)
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov_y, aspect, self.near, self.far)
    }
}
