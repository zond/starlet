/// Unified input state consumed by the simulation each frame.
#[derive(Default)]
pub struct InputState {
    /// Desired turn rates in rad/s (joystick deflection mapped to angular velocity).
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    /// Desired speed (0..MAX_SPEED).
    pub speed: f32,
}

pub trait InputBackend {
    fn poll(&mut self) -> InputState;
}
