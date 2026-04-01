use crate::constants::*;
use crate::types::ShipState;
use glam::{Quat, Vec3};

/// Speed as a 0..1 fraction of MAX_SPEED.
fn speed_fraction(state: &ShipState) -> f32 {
    (state.velocity.length() / MAX_SPEED).clamp(0.0, 1.0)
}

/// Effective max turn rate, reduced at high speed.
fn effective_turn_rate(speed_frac: f32) -> f32 {
    MAX_TURN_RATE * (1.0 - speed_frac * (1.0 - HIGH_SPEED_TURN_FACTOR))
}

/// Effective steer rate (drift recovery), much slower at high speed.
fn effective_steer_rate(speed_frac: f32) -> f32 {
    STEER_RATE_LOW + (STEER_RATE_HIGH - STEER_RATE_LOW) * speed_frac
}

/// Effective angular drag, higher at speed for stabilization.
fn effective_angular_drag(speed_frac: f32) -> f32 {
    ANGULAR_DRAG + ANGULAR_DRAG_SPEED_BONUS * speed_frac
}

/// Deterministic physics step. Runs identically on client and server.
pub fn step_ship(state: &mut ShipState, dt: f32) {
    let sf = speed_fraction(state);

    // --- Angular dynamics ---
    let av = state.angular_velocity;
    if av.length_squared() > 1e-8 {
        let pitch = Quat::from_rotation_x(av.x * dt);
        let yaw = Quat::from_rotation_y(av.y * dt);
        let roll = Quat::from_rotation_z(av.z * dt);
        state.orientation = (state.orientation * yaw * pitch * roll).normalize();
    }

    // Angular drag — stronger at high speed
    let drag_factor = 1.0 - (effective_angular_drag(sf) * dt).min(1.0);
    state.angular_velocity *= drag_factor;

    // --- Linear dynamics ---
    let forward = state.orientation * Vec3::NEG_Z;

    // Thrust-based acceleration (for future server-authoritative mode)
    let acceleration = forward * state.thrust * MAX_ACCELERATION;
    state.velocity += acceleration * dt;

    // Velocity steers toward current heading — much slower at high speed (drift!)
    let speed = state.velocity.length();
    if speed > 0.01 {
        let desired = forward * speed;
        let t = (effective_steer_rate(sf) * dt).min(1.0);
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
pub fn apply_input(state: &mut ShipState, turn_input: Vec3, desired_speed: f32, dt: f32) {
    let sf = speed_fraction(state);
    let max_turn = effective_turn_rate(sf);

    // Clamp turn input to effective max rate (reduced at high speed)
    let clamped_turn = Vec3::new(
        turn_input.x.clamp(-max_turn, max_turn),
        turn_input.y.clamp(-max_turn, max_turn),
        turn_input.z.clamp(-max_turn, max_turn),
    );

    // Accelerate angular velocity toward desired turn rate
    let diff = clamped_turn - state.angular_velocity;
    let max_change = TURN_ACCELERATION * dt;
    let change = diff.clamp_length_max(max_change);
    state.angular_velocity += change;

    // Clamp angular velocity to effective rate
    state.angular_velocity = Vec3::new(
        state.angular_velocity.x.clamp(-max_turn, max_turn),
        state.angular_velocity.y.clamp(-max_turn, max_turn),
        state.angular_velocity.z.clamp(-max_turn, max_turn),
    );

    // Set desired speed
    state.desired_speed = desired_speed.clamp(0.0, MAX_SPEED);

    // Override velocity magnitude to desired speed
    let forward = state.orientation * Vec3::NEG_Z;
    let current_speed = state.velocity.length();
    if current_speed > 0.01 {
        let desired_vel = forward * state.desired_speed;
        let t = (effective_steer_rate(sf) * dt).min(1.0);
        state.velocity = state.velocity.lerp(desired_vel, t);
    } else {
        state.velocity = forward * state.desired_speed;
    }

    state.thrust = 0.0;
}
