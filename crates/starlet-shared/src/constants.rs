pub const PHYSICS_TICK_RATE: f32 = 60.0;
pub const PHYSICS_DT: f32 = 1.0 / PHYSICS_TICK_RATE;
pub const MAX_SPEED: f32 = 500.0;
pub const MAX_ACCELERATION: f32 = 100.0;
pub const DRAG_COEFFICIENT: f32 = 0.02;

/// Max angular velocity in rad/s (per axis).
pub const MAX_TURN_RATE: f32 = 1.0;
/// How fast angular velocity can change, in rad/s² (joystick responsiveness).
pub const TURN_ACCELERATION: f32 = 8.0;
/// Angular drag — how fast the ship stops spinning without input.
pub const ANGULAR_DRAG: f32 = 4.0;
/// How fast velocity realigns to heading (drift recovery), per second.
pub const STEER_RATE: f32 = 3.0;
/// Max speed change per second via desired_speed input.
pub const MAX_SPEED_CHANGE: f32 = 500.0;
