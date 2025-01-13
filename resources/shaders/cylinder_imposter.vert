#version 410 core

// Quad geometry attributes
layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv_coordinates;

layout(location = 2) in vec3 instance_start_pos;
layout(location = 3) in vec3 instance_end_pos;
layout(location = 4) in vec4 instance_color;
layout(location = 5) in float instance_radius;

out vec2 v_uv_coordinates;
out vec3 v_world_pos;
out vec3 v_start;
out vec3 v_end;
out vec4 v_color;
out float v_radius;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 model;

void main() {
    vec3 transformed_start_pos = (model * vec4(instance_start_pos, 1.0)).xyz;
    vec3 transformed_end_pos = (model * vec4(instance_end_pos, 1.0)).xyz;

    // Calculate cylinder direction
    vec3 cylinder_dir = normalize(transformed_end_pos - transformed_start_pos);

    vec3 camera_pos = vec3(inverse(view)[3]);
    vec3 camera_dir = normalize(camera_pos - transformed_start_pos);

    // Create basis for billboard
    vec3 right = normalize(cross(cylinder_dir, camera_dir));
    vec3 up = normalize(cross(right, cylinder_dir));

    float length_cylinder = distance(transformed_end_pos, transformed_start_pos);

    float scale_x = length(vec3(model[0][0], model[1][0], model[2][0]));
    float scaled_radius = instance_radius * scale_x;

    // Compute the world position of the quad vertices
    vec3 world_pos =
        transformed_start_pos
            + cylinder_dir * (length_cylinder * (pos.y + 0.5))
            + right * (scaled_radius * pos.x * 2.0);

    // Pass data to the fragment shader
    v_uv_coordinates = uv_coordinates;
    v_world_pos = world_pos;
    v_start = transformed_start_pos;
    v_end = transformed_end_pos;
    v_color = instance_color;
    v_radius = instance_radius * model[0][0];
    gl_Position = projection * view * vec4(world_pos, 1.0);
}
