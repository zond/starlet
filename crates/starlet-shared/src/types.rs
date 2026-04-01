use glam::{Quat, Vec3};

pub type EntityId = u64;
pub type Tick = u64;

#[derive(Debug, Clone)]
pub struct ShipState {
    pub id: EntityId,
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation: Quat,
    /// Angular velocity in local space (pitch, yaw, roll) in rad/s.
    pub angular_velocity: Vec3,
    pub thrust: f32,
    /// Desired speed set by player (for direct speed control mode).
    pub desired_speed: f32,
    pub tick: Tick,
}

impl ShipState {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            orientation: Quat::IDENTITY,
            angular_velocity: Vec3::ZERO,
            thrust: 0.0,
            desired_speed: 0.0,
            tick: 0,
        }
    }
}

/// What the client sends each tick. The server validates these values
/// are within allowed limits (max turn rate, max turn acceleration, etc).
#[derive(Debug, Clone)]
pub struct PlayerInput {
    /// Joystick deflection: desired angular velocity (pitch, yaw, roll).
    /// Server validates magnitude <= MAX_TURN_RATE.
    pub turn_input: Vec3,
    /// Desired speed. Server validates rate of change <= MAX_SPEED_CHANGE.
    pub desired_speed: f32,
    pub fire: bool,
    pub tick: Tick,
}
