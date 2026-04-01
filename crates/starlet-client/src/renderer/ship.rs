use glam::{Mat3, Mat4, Quat, Vec3};
use web_sys::{
    WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

use super::shader::{compile_shader, link_program};
use super::ship_mesh;

pub struct ShipRenderer {
    program: WebGlProgram,
    vao: WebGlVertexArrayObject,
    u_model: WebGlUniformLocation,
    u_view_projection: WebGlUniformLocation,
    u_normal_matrix: WebGlUniformLocation,
    _buffer: WebGlBuffer,
}

/// Transform to reorient the Kenney model:
/// OBJ has Y as the long axis (ship length), we want -Z as forward.
/// Rotate -90° around X so Y→-Z, then center the model.
fn model_base_transform() -> Mat4 {
    // Center the model: OBJ bounds are X:0..2, Y:0..6, Z:0..1
    let center = Vec3::new(1.0, 3.0, 0.5);
    let centering = Mat4::from_translation(-center);
    // Rotate so Y-forward becomes -Z-forward: rotate 90° around X
    let rotate = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    // Scale down to a reasonable size
    let scale = Mat4::from_scale(Vec3::splat(0.15));
    scale * rotate * centering
}

impl ShipRenderer {
    pub fn new(gl: &GL) -> Self {
        let vert = compile_shader(
            gl,
            GL::VERTEX_SHADER,
            include_str!("shaders/ship_vert.glsl"),
        );
        let frag = compile_shader(
            gl,
            GL::FRAGMENT_SHADER,
            include_str!("shaders/ship_frag.glsl"),
        );
        let program = link_program(gl, &vert, &frag);

        let u_model = gl
            .get_uniform_location(&program, "u_model")
            .expect("u_model");
        let u_view_projection = gl
            .get_uniform_location(&program, "u_view_projection")
            .expect("u_view_projection");
        let u_normal_matrix = gl
            .get_uniform_location(&program, "u_normal_matrix")
            .expect("u_normal_matrix");

        let vao = gl.create_vertex_array().expect("create_vertex_array");
        gl.bind_vertex_array(Some(&vao));

        let buffer = gl.create_buffer().expect("create_buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let view = js_sys::Float32Array::view(&ship_mesh::VERTICES);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &view, GL::STATIC_DRAW);
        }

        let stride = (ship_mesh::STRIDE * 4) as i32;
        // a_position
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, stride, 0);
        // a_normal
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, stride, 12);
        // a_color
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_with_i32(2, 3, GL::FLOAT, false, stride, 24);

        gl.bind_vertex_array(None);

        Self {
            program,
            vao,
            u_model,
            u_view_projection,
            u_normal_matrix,
            _buffer: buffer,
        }
    }

    /// Draw the ship model.
    /// `position`: ship world position
    /// `orientation`: ship orientation quaternion
    /// `view_projection`: camera VP matrix (using velocity-based view)
    pub fn draw(
        &self,
        gl: &GL,
        view_projection: &Mat4,
        position: Vec3,
        orientation: Quat,
    ) {
        let base = model_base_transform();
        let world = Mat4::from_rotation_translation(orientation, position);
        let model = world * base;

        // Normal matrix = transpose(inverse(upper-left 3x3 of model))
        let normal_matrix = Mat3::from_mat4(model).inverse().transpose();

        gl.use_program(Some(&self.program));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_model),
            false,
            &model.to_cols_array(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_view_projection),
            false,
            &view_projection.to_cols_array(),
        );
        gl.uniform_matrix3fv_with_f32_array(
            Some(&self.u_normal_matrix),
            false,
            &normal_matrix.to_cols_array(),
        );

        gl.bind_vertex_array(Some(&self.vao));
        gl.draw_arrays(GL::TRIANGLES, 0, ship_mesh::VERTEX_COUNT as i32);
        gl.bind_vertex_array(None);
    }
}
