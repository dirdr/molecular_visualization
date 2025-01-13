#version 410 core

// Quad geometry attributes
layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv_coordinates;

layout(location = 2) in vec3 instance_pos;
layout(location = 3) in vec4 instance_color;
layout(location = 4) in float instance_radius;

out vec2 v_uv_coordinates;
out vec3 v_world_pos;
out vec3 v_center;
out vec4 v_color;
out float v_radius;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 model;

void main() {
    vec3 transformed_instance_pos = (model * vec4(instance_pos, 1.0)).xyz;

    // Adjust camera basis vectors
    vec3 camera_right = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 camera_up = vec3(view[0][1], view[1][1], view[2][1]);

    float scale_x = length(vec3(model[0][0], model[1][0], model[2][0]));
    // Assuming uniform scaling, scale_x, scale_y, and scale_z should be equal
    float scaled_radius = instance_radius * scale_x;

    // Compute the world position of the quad vertices
    vec3 world_pos = transformed_instance_pos
            + camera_right * pos.x * scaled_radius * 2.0
            + camera_up * pos.y * scaled_radius * 2.0;

    v_uv_coordinates = uv_coordinates;
    v_world_pos = world_pos;
    v_center = transformed_instance_pos;
    v_color = instance_color;
    v_radius = scaled_radius;

    gl_Position = projection * view * vec4(world_pos, 1.0);
}
