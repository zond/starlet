#version 300 es
precision highp float;

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_color;

uniform mat4 u_model;
uniform mat4 u_view_projection;
uniform mat3 u_normal_matrix;

out vec3 v_normal;
out vec3 v_color;
out vec3 v_world_pos;

void main() {
    vec4 world_pos = u_model * vec4(a_position, 1.0);
    gl_Position = u_view_projection * world_pos;
    v_world_pos = world_pos.xyz;
    v_normal = normalize(u_normal_matrix * a_normal);
    v_color = a_color;
}
