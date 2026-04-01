use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

pub fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(shader_type).expect("create_shader");
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if !gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let log = gl.get_shader_info_log(&shader).unwrap_or_default();
        panic!("Shader compile error: {log}");
    }
    shader
}

pub fn link_program(gl: &GL, vert: &WebGlShader, frag: &WebGlShader) -> WebGlProgram {
    let program = gl.create_program().expect("create_program");
    gl.attach_shader(&program, vert);
    gl.attach_shader(&program, frag);
    gl.link_program(&program);
    if !gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let log = gl.get_program_info_log(&program).unwrap_or_default();
        panic!("Program link error: {log}");
    }
    program
}
