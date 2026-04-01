pub mod camera;
pub mod context;
pub mod hud;
pub mod particles;
pub mod shader;
pub mod starfield;

use glam::{Quat, Vec3};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

use camera::Camera;
use hud::Hud;
use particles::ParticleSystem;
use starfield::Starfield;

pub struct Renderer {
    gl: GL,
    canvas: HtmlCanvasElement,
    camera: Camera,
    starfield: Starfield,
    pub particles: ParticleSystem,
    hud: Hud,
}

impl Renderer {
    pub fn new() -> Self {
        let (canvas, gl) = context::setup_canvas();
        let camera = Camera::new();
        let starfield = Starfield::new(&gl);
        let particles = ParticleSystem::new(&gl);
        let hud = Hud::new();
        Self {
            gl,
            canvas,
            camera,
            starfield,
            particles,
            hud,
        }
    }

    pub fn draw(&self, position: Vec3, velocity: Vec3, orientation: Quat) {
        context::resize_canvas(&self.canvas, &self.gl);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let w = self.canvas.width();
        let h = self.canvas.height();
        if w == 0 || h == 0 {
            return;
        }
        let aspect = w as f32 / h as f32;
        let proj = self.camera.projection_matrix(aspect);

        // Stars: view with no translation (at infinity)
        let sky_view = self.camera.sky_view_matrix(orientation);
        let sky_vp = proj * sky_view;
        self.starfield.draw(&self.gl, &sky_vp);

        // Particles: full view with position
        let view = self.camera.view_matrix(position, orientation);
        let vp = proj * view;
        let speed = velocity.length();
        let speed_factor = (speed / starlet_shared::constants::MAX_SPEED).clamp(0.0, 1.0);
        self.particles.draw(&self.gl, &vp, speed_factor);

        // HUD overlay
        let forward = orientation * Vec3::NEG_Z;
        let heading = forward.x.atan2(-forward.z).to_degrees();
        let pitch = forward.y.asin().to_degrees();
        self.hud.draw(heading, pitch, speed);
    }
}
