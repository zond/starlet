#version 300 es
precision highp float;

in float v_life;
out vec4 frag_color;

void main() {
    vec2 coord = gl_PointCoord - vec2(0.5);
    float dist = length(coord);
    if (dist > 0.5) discard;

    float alpha = smoothstep(0.5, 0.0, dist) * v_life;
    frag_color = vec4(1.0, 1.0, 1.0, alpha);
}
