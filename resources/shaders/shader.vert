#version 410 core

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    v_normal = transpose(inverse(mat3(model))) * normal;
    gl_Position = projection * view * model * vec4(position, 1.0);
}
