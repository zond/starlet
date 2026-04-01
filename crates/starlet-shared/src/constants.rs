pub const PHYSICS_TICK_RATE: f32 = 60.0;
pub const PHYSICS_DT: f32 = 1.0 / PHYSICS_TICK_RATE;
pub const MAX_SPEED: f32 = 500.0;
pub const MAX_ACCELERATION: f32 = 100.0;
pub const DRAG_COEFFICIENT: f32 = 0.02;

/// Max angular velocity in rad/s (per axis) at zero speed.
pub const MAX_TURN_RATE: f32 = 1.0;
/// Turn rate multiplier at max speed (1.0 = same turn rate at all speeds).
pub const HIGH_SPEED_TURN_FACTOR: f32 = 1.0;
/// How fast angular velocity can change, in rad/s².
pub const TURN_ACCELERATION: f32 = 8.0;
/// Angular drag at zero speed.
pub const ANGULAR_DRAG: f32 = 4.0;
/// Additional angular drag at max speed (stabilizes at high speed).
pub const ANGULAR_DRAG_SPEED_BONUS: f32 = 3.0;
/// How fast velocity realigns to heading at zero speed, per second.
pub const STEER_RATE_LOW: f32 = 6.0;
/// How fast velocity realigns to heading at max speed, per second.
pub const STEER_RATE_HIGH: f32 = 0.8;
/// Max speed change per second via desired_speed input.
pub const MAX_SPEED_CHANGE: f32 = 500.0;
