#version 410 core

in vec2 v_uv_coordinates;
in vec3 v_world_pos;
in vec3 v_center;
in vec4 v_color;
in float v_radius;

out vec4 frag_color;

uniform vec3 light_position;
uniform vec3 camera_position;
uniform bool debug_billboard;

void main() {
    // Convert texture coordinates from [0,1] to [-1,1]
    vec2 pos = v_uv_coordinates * 2.0 - 1.0;

    // Ray origin (camera position)
    vec3 ray_origin = camera_position;

    // Ray direction
    vec3 ray_dir = normalize(v_world_pos - camera_position);

    // Sphere center
    vec3 sphere_center = v_center;

    // Ray-sphere intersection
    vec3 oc = ray_origin - sphere_center;
    float a = dot(ray_dir, ray_dir);
    float b = 2.0 * dot(ray_dir, oc);
    float c = dot(oc, oc) - v_radius * v_radius;
    float discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        if (debug_billboard) {
            frag_color = vec4(1.0, 0.0, 1.0, 1);
            return;
        } else {
            discard;
        }
    }

    // If no intersection, discard the fragment
    if (length(pos) > 1.0) {
        discard;
    }

    // Get the nearest intersection point
    float t = (-b - sqrt(discriminant)) / (2.0 * a);
    vec3 intersection = ray_origin + t * ray_dir;

    // Calculate normal at intersection point
    vec3 normal = normalize(intersection - sphere_center);

    vec3 light_dir = normalize(light_position - intersection);
    float diffuse = max(dot(normal, light_dir), 0.0);

    vec3 view_dir = normalize(camera_position - intersection);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

    float ambient = 0.3;

    frag_color = v_color * (ambient + diffuse + specular);
}
