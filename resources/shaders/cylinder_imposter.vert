#version 410 core

// Quad geometry attributes
layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv_coordinates;

// Instance attributes
layout(location = 2) in vec3 instance_start_pos;
layout(location = 3) in vec3 instance_end_pos;
layout(location = 4) in vec4 instance_color;
layout(location = 5) in float instance_radius;

// Outputs to fragment shader
out vec2 v_uv_coordinates;
out vec3 v_world_pos;
out vec3 v_start;
out vec3 v_end;
out vec4 v_color;
out float v_radius;

// Uniforms
uniform mat4 view;
uniform mat4 projection;
uniform mat4 scene_model; // Scene model matrix

void main() {
    // Apply scene model transformation to instance positions
    vec3 transformed_start_pos = (scene_model * vec4(instance_start_pos, 1.0)).xyz;
    vec3 transformed_end_pos = (scene_model * vec4(instance_end_pos, 1.0)).xyz;

    // Calculate cylinder direction
    vec3 cylinder_dir = normalize(transformed_end_pos - transformed_start_pos);

    // Calculate camera position and direction
    vec3 camera_pos = vec3(inverse(view)[3]);
    vec3 camera_dir = normalize(camera_pos - transformed_start_pos);

    // Create basis for billboard
    vec3 right = normalize(cross(cylinder_dir, camera_dir));
    vec3 up = normalize(cross(right, cylinder_dir));

    // Calculate cylinder length
    float length = distance(transformed_end_pos, transformed_start_pos);

    // Transform quad vertices
    vec3 world_pos =
        transformed_start_pos
            + cylinder_dir * (length * (pos.y + 0.5))
            + right * (instance_radius * scene_model[0][0] * pos.x); // Scale radius by scene scale

    v_uv_coordinates = uv_coordinates;
    v_world_pos = world_pos;
    v_start = transformed_start_pos;
    v_end = transformed_end_pos;
    v_color = instance_color;
    v_radius = instance_radius * scene_model[0][0]; // Scale radius

    gl_Position = projection * view * vec4(world_pos, 1.0);
}
