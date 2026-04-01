#version 300 es
precision highp float;

in float v_brightness;
in float v_color_temp;
out vec4 frag_color;

void main() {
    vec2 coord = gl_PointCoord - vec2(0.5);
    float dist = length(coord);
    if (dist > 0.5) discard;

    float alpha = smoothstep(0.5, 0.1, dist) * v_brightness;

    // Spectral class colors: O/B (blue-white) through K/M (orange-red)
    vec3 blue_white = vec3(0.6, 0.7, 1.0);
    vec3 white      = vec3(1.0, 1.0, 1.0);
    vec3 yellow     = vec3(1.0, 0.95, 0.7);
    vec3 orange     = vec3(1.0, 0.75, 0.4);
    vec3 red        = vec3(1.0, 0.5, 0.3);

    vec3 color;
    if (v_color_temp < 0.25) {
        color = mix(blue_white, white, v_color_temp * 4.0);
    } else if (v_color_temp < 0.5) {
        color = mix(white, yellow, (v_color_temp - 0.25) * 4.0);
    } else if (v_color_temp < 0.75) {
        color = mix(yellow, orange, (v_color_temp - 0.5) * 4.0);
    } else {
        color = mix(orange, red, (v_color_temp - 0.75) * 4.0);
    }

    // Bright stars get a subtle glow core
    float core = smoothstep(0.3, 0.0, dist);
    color = mix(color, vec3(1.0), core * v_brightness * 0.3);

    frag_color = vec4(color, alpha);
}
