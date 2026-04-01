use glam::Mat4;

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

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov_y, aspect, self.near, self.far)
    }
}
