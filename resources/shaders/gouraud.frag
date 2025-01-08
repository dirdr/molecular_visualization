#version 410 core

in vec3 v_normal;
out vec4 FragColor;

uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.6, 0.0, 0.0);
    vec3 regular_color = vec3(1.0, 0.0, 0.0);
    FragColor = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
