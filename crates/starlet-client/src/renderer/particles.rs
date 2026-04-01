use glam::{Mat4, Quat, Vec3};
use web_sys::{
    WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

use super::shader::{compile_shader, link_program};

const MAX_PARTICLES: usize = 400;
const SPAWN_DISTANCE: f32 = 80.0;
const SPAWN_SPREAD: f32 = 40.0;

struct Particle {
    position: Vec3,
    life: f32,
    size: f32,
}

pub struct ParticleSystem {
    program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    buffer: WebGlBuffer,
    u_view_projection: WebGlUniformLocation,
    u_speed_factor: WebGlUniformLocation,
    particles: Vec<Particle>,
    seed: u32,
}

impl ParticleSystem {
    pub fn new(gl: &GL) -> Self {
        let vert = compile_shader(
            gl,
            GL::VERTEX_SHADER,
            include_str!("shaders/particle_vert.glsl"),
        );
        let frag = compile_shader(
            gl,
            GL::FRAGMENT_SHADER,
            include_str!("shaders/particle_frag.glsl"),
        );
        let program = link_program(gl, &vert, &frag);

        let u_view_projection = gl
            .get_uniform_location(&program, "u_view_projection")
            .expect("u_view_projection");
        let u_speed_factor = gl
            .get_uniform_location(&program, "u_speed_factor")
            .expect("u_speed_factor");

        let vao = gl.create_vertex_array().expect("create_vertex_array");
        gl.bind_vertex_array(Some(&vao));

        let buffer = gl.create_buffer().expect("create_buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        // 5 floats each: x, y, z, life, size
        gl.buffer_data_with_i32(
            GL::ARRAY_BUFFER,
            (MAX_PARTICLES * 5 * 4) as i32,
            GL::DYNAMIC_DRAW,
        );

        let stride = 5 * 4;
        // a_position
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, stride, 0);
        // a_life
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, stride, 12);
        // a_size
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_with_i32(2, 1, GL::FLOAT, false, stride, 16);

        gl.bind_vertex_array(None);

        Self {
            program,
            vao,
            buffer,
            u_view_projection,
            u_speed_factor,
            particles: Vec::with_capacity(MAX_PARTICLES),
            seed: 0xCAFE_BABE,
        }
    }

    fn rand_f32(&mut self) -> f32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed as f32 / u32::MAX as f32
    }

    fn spawn_particle(&mut self, ship_pos: Vec3, ship_orientation: Quat) -> Particle {
        let forward = ship_orientation * Vec3::NEG_Z;
        let right = ship_orientation * Vec3::X;
        let up = ship_orientation * Vec3::Y;

        let offset_forward = SPAWN_DISTANCE * (0.5 + self.rand_f32());
        let offset_right = SPAWN_SPREAD * (self.rand_f32() - 0.5);
        let offset_up = SPAWN_SPREAD * (self.rand_f32() - 0.5);

        // Size varies: mostly small, a few larger
        let r = self.rand_f32();
        let size = if r > 0.95 { 1.5 + r * 0.5 } else { 0.3 + r * r * 1.0 };

        let life = 0.5 + self.rand_f32() * 0.5;

        Particle {
            position: ship_pos + forward * offset_forward + right * offset_right + up * offset_up,
            life,
            size,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        ship_pos: Vec3,
        ship_velocity: Vec3,
        ship_orientation: Quat,
    ) {
        let speed = ship_velocity.length();
        let speed_factor = (speed / starlet_shared::constants::MAX_SPEED).clamp(0.0, 1.0);

        for p in &mut self.particles {
            p.life -= dt * (0.5 + speed_factor * 2.0);
        }

        self.particles.retain(|p| p.life > 0.0);

        let target_count = (MAX_PARTICLES as f32 * speed_factor) as usize;
        let spawn_count = target_count.saturating_sub(self.particles.len());
        for _ in 0..spawn_count {
            let p = self.spawn_particle(ship_pos, ship_orientation);
            self.particles.push(p);
        }
    }

    pub fn draw(&self, gl: &GL, view_projection: &Mat4, speed_factor: f32) {
        if self.particles.is_empty() {
            return;
        }

        let mut data: Vec<f32> = Vec::with_capacity(self.particles.len() * 5);
        for p in &self.particles {
            data.push(p.position.x);
            data.push(p.position.y);
            data.push(p.position.z);
            data.push(p.life);
            data.push(p.size);
        }

        gl.use_program(Some(&self.program));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_view_projection),
            false,
            &view_projection.to_cols_array(),
        );
        gl.uniform1f(Some(&self.u_speed_factor), speed_factor);

        gl.bind_vertex_array(Some(&self.vao));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        unsafe {
            let view = js_sys::Float32Array::view(&data);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(GL::ARRAY_BUFFER, 0, &view);
        }
        gl.draw_arrays(GL::POINTS, 0, self.particles.len() as i32);
        gl.bind_vertex_array(None);
    }
}
