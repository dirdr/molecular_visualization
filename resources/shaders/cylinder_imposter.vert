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

uniform mat4 view;
uniform mat4 projection;

void main() {
    vec3 cylinder_dir = normalize(instance_end_pos - instance_start_pos);
    vec3 camera_pos = vec3(inverse(view)[3]);
    vec3 camera_dir = normalize(camera_pos - instance_start_pos);

    // Create basis for billboard
    vec3 right = normalize(cross(cylinder_dir, camera_dir));
    vec3 up = normalize(cross(right, cylinder_dir));

    // Calculate cylinder length
    float length = distance(instance_end_pos, instance_start_pos);

    // Transform quad vertices
    vec3 world_pos =
        instance_start_pos
            + cylinder_dir * (length * (pos.y + 0.5))
            + right * (instance_radius * pos.x);

    v_uv_coordinates = uv_coordinates;
    v_world_pos = world_pos;
    v_start = instance_start_pos;
    v_end = instance_end_pos;
    v_color = instance_color;
    v_radius = instance_radius;

    gl_Position = projection * view * vec4(world_pos, 1.0);
}
