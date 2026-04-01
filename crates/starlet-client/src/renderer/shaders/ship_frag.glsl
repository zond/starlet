#version 300 es
precision highp float;

in vec3 v_normal;
in vec3 v_color;
in vec3 v_world_pos;
out vec4 frag_color;

void main() {
    // Simple directional light from upper-front-right
    vec3 light_dir = normalize(vec3(0.3, 0.8, -0.5));
    float diffuse = max(dot(v_normal, light_dir), 0.0);

    // Ambient + diffuse
    vec3 lit = v_color * (0.3 + 0.7 * diffuse);

    // Subtle rim light from behind
    vec3 view_dir = normalize(-v_world_pos);
    float rim = pow(1.0 - max(dot(v_normal, view_dir), 0.0), 3.0);
    lit += vec3(0.15, 0.2, 0.3) * rim;

    frag_color = vec4(lit, 1.0);
}
