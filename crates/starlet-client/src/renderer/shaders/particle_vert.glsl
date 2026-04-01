#version 300 es
precision highp float;

layout(location = 0) in vec3 a_position;
layout(location = 1) in float a_life;
layout(location = 2) in float a_size;

uniform mat4 u_view_projection;
uniform float u_speed_factor;

out float v_life;

void main() {
    gl_Position = u_view_projection * vec4(a_position, 1.0);
    gl_PointSize = a_size * mix(2.0, 6.0, u_speed_factor);
    v_life = a_life;
}
