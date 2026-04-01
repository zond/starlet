#version 300 es
precision highp float;

layout(location = 0) in vec3 a_position;
layout(location = 1) in float a_brightness;
layout(location = 2) in float a_color_temp;

uniform mat4 u_view_projection;

out float v_brightness;
out float v_color_temp;

void main() {
    gl_Position = u_view_projection * vec4(a_position, 1.0);
    gl_PointSize = mix(1.0, 6.0, a_brightness * a_brightness);
    v_brightness = a_brightness;
    v_color_temp = a_color_temp;
}
