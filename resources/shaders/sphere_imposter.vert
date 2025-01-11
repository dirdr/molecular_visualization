#version 410 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 tex_coords;

out vec2 v_tex_coords;
out vec3 v_world_pos;
out vec3 v_center;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float sphere_radius;

void main() {
    vec3 camera_right = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 camera_up = vec3(view[0][1], view[1][1], view[2][1]);

    vec4 center = model * vec4(0.0, 0.0, 0.0, 1.0);
    v_center = center.xyz;

    // Scale the billboard by the sphere radius
    vec3 world_pos = center.xyz
                   + camera_right * position.x * sphere_radius * 2.0
                   + camera_up * position.y * sphere_radius * 2.0;

    v_world_pos = world_pos;
    gl_Position = projection * view * vec4(world_pos, 1.0);
    v_tex_coords = tex_coords;
}
