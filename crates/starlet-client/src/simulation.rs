use glam::Vec3;
use starlet_shared::physics;
use starlet_shared::types::ShipState;

use crate::input::abstraction::InputState;

pub struct LocalSimulation {
    pub ship: ShipState,
}

impl LocalSimulation {
    pub fn new() -> Self {
        Self {
            ship: ShipState::new(0),
        }
    }

    pub fn apply_input(&mut self, input: &InputState) {
        let turn_input = Vec3::new(input.pitch, input.yaw, input.roll);
        let dt = starlet_shared::constants::PHYSICS_DT;
        physics::apply_input(&mut self.ship, turn_input, input.speed, dt);
    }

    pub fn step(&mut self, dt: f32) {
        physics::step_ship(&mut self.ship, dt);
    }
}
