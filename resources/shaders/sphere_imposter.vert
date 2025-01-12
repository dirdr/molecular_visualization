#version 410 core
// Quad geometry attributes
layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv_coordinates;

// Instance attributes
layout(location = 2) in vec3 instance_pos;
layout(location = 3) in vec4 instance_color;
layout(location = 4) in float instance_radius;

// Outputs to fragment shader
out vec2 v_uv_coordinates;
out vec3 v_world_pos;
out vec3 v_center;
out vec4 v_color;
out float v_radius;

// Camera uniforms
uniform mat4 view;
uniform mat4 projection;

void main() {
    vec3 camera_right = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 camera_up = vec3(view[0][1], view[1][1], view[2][1]);

    vec3 world_pos = instance_pos
            + camera_right * pos.x * instance_radius * 2.0 // *2 the have the needed space in the billboard
            + camera_up * pos.y * instance_radius * 2.0;

    v_uv_coordinates = uv_coordinates;
    v_world_pos = world_pos;
    v_center = instance_pos;
    v_color = instance_color;
    v_radius = instance_radius;
    gl_Position = projection * view * vec4(world_pos, 1.0);
}
