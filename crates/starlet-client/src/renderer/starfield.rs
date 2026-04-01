use glam::Mat4;
use web_sys::{
    WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

use super::shader::{compile_shader, link_program};

const STAR_COUNT: usize = 3000;
const STAR_RADIUS: f32 = 10_000.0;

pub struct Starfield {
    program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    u_view_projection: WebGlUniformLocation,
    _buffer: WebGlBuffer,
}

impl Starfield {
    pub fn new(gl: &GL) -> Self {
        let vert = compile_shader(gl, GL::VERTEX_SHADER, include_str!("shaders/star_vert.glsl"));
        let frag =
            compile_shader(gl, GL::FRAGMENT_SHADER, include_str!("shaders/star_frag.glsl"));
        let program = link_program(gl, &vert, &frag);

        let u_view_projection = gl
            .get_uniform_location(&program, "u_view_projection")
            .expect("u_view_projection");

        // Generate star positions on a sphere
        let mut data: Vec<f32> = Vec::with_capacity(STAR_COUNT * 5);
        let mut seed: u32 = 0xDEAD_BEEF;
        for _ in 0..STAR_COUNT {
            // Simple xorshift PRNG
            let (x, y, z) = loop {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                let x = (seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                let y = (seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                let z = (seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
                let len = (x * x + y * y + z * z).sqrt();
                if len > 0.01 && len <= 1.0 {
                    break (x / len * STAR_RADIUS, y / len * STAR_RADIUS, z / len * STAR_RADIUS);
                }
            };
            // brightness: most stars dim, a few very bright
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let b = seed as f32 / u32::MAX as f32;
            let brightness = if b > 0.95 {
                // ~5% are bright stars (0.7 - 1.0)
                0.7 + (b - 0.95) * 6.0
            } else if b > 0.8 {
                // ~15% are medium stars (0.3 - 0.7)
                0.3 + (b - 0.8) * (0.4 / 0.15)
            } else {
                // ~80% are dim background stars (0.05 - 0.3)
                0.05 + b * (0.25 / 0.8)
            };
            // color temperature (biased toward white/yellow, fewer blue and red)
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let color_temp = seed as f32 / u32::MAX as f32;

            data.push(x);
            data.push(y);
            data.push(z);
            data.push(brightness);
            data.push(color_temp);
        }

        let vao = gl.create_vertex_array().expect("create_vertex_array");
        gl.bind_vertex_array(Some(&vao));

        let buffer = gl.create_buffer().expect("create_buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let view = js_sys::Float32Array::view(&data);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &view, GL::STATIC_DRAW);
        }

        let stride = 5 * 4; // 5 floats * 4 bytes
        // a_position
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, stride, 0);
        // a_brightness
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, stride, 12);
        // a_color_temp
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_with_i32(2, 1, GL::FLOAT, false, stride, 16);

        gl.bind_vertex_array(None);

        Self {
            program,
            vao,
            u_view_projection,
            _buffer: buffer,
        }
    }

    pub fn draw(&self, gl: &GL, view_projection: &Mat4) {
        gl.use_program(Some(&self.program));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_view_projection),
            false,
            &view_projection.to_cols_array(),
        );
        gl.bind_vertex_array(Some(&self.vao));
        gl.draw_arrays(GL::POINTS, 0, STAR_COUNT as i32);
        gl.bind_vertex_array(None);
    }
}
