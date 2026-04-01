use wasm_bindgen::JsCast;

use crate::input::abstraction::InputBackend;
use crate::renderer::Renderer;
use crate::simulation::LocalSimulation;

pub struct App {
    pub renderer: Renderer,
    pub input: Box<dyn InputBackend>,
    pub simulation: LocalSimulation,
}

impl App {
    pub fn new() -> Self {
        let renderer = Renderer::new();
        let canvas = web_sys::window()
            .expect("window")
            .document()
            .expect("document")
            .get_element_by_id("starlet-canvas")
            .expect("canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("cast");
        let input = crate::input::create_input(&canvas);
        let simulation = LocalSimulation::new();

        Self {
            renderer,
            input,
            simulation,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let input_state = self.input.poll();
        self.simulation.apply_input(&input_state);
        self.simulation.step(dt);
        self.renderer.particles.update(
            dt,
            self.simulation.ship.position,
            self.simulation.ship.velocity,
            self.simulation.ship.orientation,
        );
    }

    pub fn draw(&self) {
        self.renderer.draw(
            self.simulation.ship.position,
            self.simulation.ship.velocity,
            self.simulation.ship.orientation,
        );
    }
}
