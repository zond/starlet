use crate::constants::*;
use crate::types::ShipState;
use glam::{Quat, Vec3};

/// Deterministic physics step. Runs identically on client and server.
pub fn step_ship(state: &mut ShipState, dt: f32) {
    // --- Angular dynamics ---
    // Apply angular velocity to orientation
    let av = state.angular_velocity;
    if av.length_squared() > 1e-8 {
        let pitch = Quat::from_rotation_x(av.x * dt);
        let yaw = Quat::from_rotation_y(av.y * dt);
        let roll = Quat::from_rotation_z(av.z * dt);
        state.orientation = (state.orientation * yaw * pitch * roll).normalize();
    }

    // Angular drag — spin decays without input
    let drag_factor = 1.0 - (ANGULAR_DRAG * dt).min(1.0);
    state.angular_velocity *= drag_factor;

    // --- Linear dynamics ---
    let forward = state.orientation * Vec3::NEG_Z;

    // Thrust-based acceleration (for future server-authoritative mode)
    let acceleration = forward * state.thrust * MAX_ACCELERATION;
    state.velocity += acceleration * dt;

    // Velocity steers toward current heading (drift recovery)
    let speed = state.velocity.length();
    if speed > 0.01 {
        let desired = forward * speed;
        let t = (STEER_RATE * dt).min(1.0);
        state.velocity = state.velocity.lerp(desired, t);
    }

    // Speed clamp
    let speed = state.velocity.length();
    if speed > MAX_SPEED {
        state.velocity = state.velocity.normalize() * MAX_SPEED;
    }

    // Linear drag
    state.velocity *= 1.0 - (DRAG_COEFFICIENT * dt);

    state.position += state.velocity * dt;
    state.tick += 1;
}

/// Apply player input to ship state. The server uses this to validate
/// that turn_input and desired_speed are within allowed limits.
pub fn apply_input(state: &mut ShipState, turn_input: glam::Vec3, desired_speed: f32, dt: f32) {
    // Clamp turn input to max rate (server anti-cheat)
    let clamped_turn = Vec3::new(
        turn_input.x.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
        turn_input.y.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
        turn_input.z.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
    );

    // Accelerate angular velocity toward desired turn rate
    let diff = clamped_turn - state.angular_velocity;
    let max_change = TURN_ACCELERATION * dt;
    let change = diff.clamp_length_max(max_change);
    state.angular_velocity += change;

    // Clamp angular velocity
    state.angular_velocity = Vec3::new(
        state.angular_velocity.x.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
        state.angular_velocity.y.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
        state.angular_velocity.z.clamp(-MAX_TURN_RATE, MAX_TURN_RATE),
    );

    // Set desired speed (server validates rate of change)
    state.desired_speed = desired_speed.clamp(0.0, MAX_SPEED);

    // Override velocity magnitude to desired speed
    let forward = state.orientation * Vec3::NEG_Z;
    let current_speed = state.velocity.length();
    if current_speed > 0.01 {
        let desired_vel = forward * state.desired_speed;
        let t = (STEER_RATE * dt).min(1.0);
        state.velocity = state.velocity.lerp(desired_vel, t);
    } else {
        state.velocity = forward * state.desired_speed;
    }

    // Not using thrust in direct speed mode
    state.thrust = 0.0;
}
