pub mod camera;
pub mod context;
pub mod hud;
pub mod particles;
pub mod shader;
pub mod ship;
pub mod ship_mesh;
pub mod starfield;

use glam::{Mat4, Quat, Vec3};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

use camera::Camera;
use hud::Hud;
use particles::ParticleSystem;
use ship::ShipRenderer;
use starfield::Starfield;

/// Camera offset behind and above the ship (in ship-local space).
const CHASE_OFFSET: Vec3 = Vec3::new(0.0, 0.8, 3.5);

pub struct Renderer {
    gl: GL,
    canvas: HtmlCanvasElement,
    camera: Camera,
    starfield: Starfield,
    pub particles: ParticleSystem,
    ship_renderer: ShipRenderer,
    hud: Hud,
}

impl Renderer {
    pub fn new() -> Self {
        let (canvas, gl) = context::setup_canvas();
        let camera = Camera::new();
        let starfield = Starfield::new(&gl);
        let particles = ParticleSystem::new(&gl);
        let ship_renderer = ShipRenderer::new(&gl);
        let hud = Hud::new();
        Self {
            gl,
            canvas,
            camera,
            starfield,
            particles,
            ship_renderer,
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

        // Chase camera: positioned behind the ship along the velocity direction.
        // When drifting, the camera follows velocity but the ship model shows orientation.
        let speed = velocity.length();
        let cam_orientation = if speed > 0.5 {
            // Camera mostly follows ship orientation, with a subtle velocity bias
            // so the ship visibly turns relative to the camera when drifting.
            let vel_dir = velocity / speed;
            let vel_quat = quat_look_to(vel_dir, orientation * Vec3::Y);
            orientation.slerp(vel_quat, 0.95)
        } else {
            orientation
        };

        // Camera sits behind and above the ship
        let cam_pos = position
            + cam_orientation * Vec3::new(CHASE_OFFSET.x, CHASE_OFFSET.y, CHASE_OFFSET.z);
        let cam_forward = cam_orientation * Vec3::NEG_Z;
        let cam_up = cam_orientation * Vec3::Y;
        let view = Mat4::look_to_rh(cam_pos, cam_forward, cam_up);
        let vp = proj * view;

        // Stars: view with no translation (at infinity)
        let sky_view = Mat4::look_to_rh(Vec3::ZERO, cam_forward, cam_up);
        let sky_vp = proj * sky_view;
        self.starfield.draw(&self.gl, &sky_vp);

        // Particles
        let speed_factor = (speed / starlet_shared::constants::MAX_SPEED).clamp(0.0, 1.0);
        self.particles.draw(&self.gl, &vp, speed_factor);

        // Ship model — draw with depth testing
        self.gl.enable(GL::DEPTH_TEST);
        self.ship_renderer
            .draw(&self.gl, &vp, position, orientation);
        self.gl.disable(GL::DEPTH_TEST);

        // HUD overlay
        let forward = orientation * Vec3::NEG_Z;
        let heading = forward.x.atan2(-forward.z).to_degrees();
        let pitch = forward.y.asin().to_degrees();
        self.hud.draw(heading, pitch, speed);
    }
}

/// Build a quaternion that looks along `dir` with the given `up` hint.
fn quat_look_to(dir: Vec3, up: Vec3) -> Quat {
    let forward = -dir.normalize();
    let right = up.cross(forward).normalize();
    if right.length_squared() < 1e-6 {
        return Quat::IDENTITY;
    }
    let up = forward.cross(right);
    Quat::from_mat3(&glam::Mat3::from_cols(right, up, forward))
}
