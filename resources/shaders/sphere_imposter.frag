#version 410 core

in vec2 v_uv_coordinates;
in vec3 v_world_pos;
in vec3 v_center;
in vec4 v_color;
in float v_radius;
in float v_depth;

out vec4 frag_color;

uniform vec3 light_position;
uniform vec3 camera_position;
uniform bool debug_billboard;
uniform mat4 projection;
uniform mat4 view;

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
            frag_color = vec4(1.0, 0.0, 1.0, 1.0);
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

    // Light calculations
    vec3 light_dir = normalize(light_position - intersection);
    float distance_to_light = distance(light_position, intersection); // Use the GLSL distance function

    // Attenuation based on distance
    float attenuation = 1.0 / (1.0 + 0.09 * distance_to_light + 0.032 * distance_to_light * distance_to_light);

    // Diffuse lighting
    float diffuse = max(dot(normal, light_dir), 0.0) * attenuation;

    // Specular lighting
    vec3 view_dir = normalize(camera_position - intersection);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float shininess = 16.0; // Lower for broader highlights
    float specular = pow(max(dot(view_dir, reflect_dir), 0.0), shininess) * attenuation;

    // Ambient light
    vec3 ambient_light = vec3(0.3, 0.3, 0.4); // Slightly bluish ambient light
    vec3 ambient = ambient_light * 0.3;

    // Final color
    frag_color = vec4(v_color.rgb * (ambient + diffuse) + specular, v_color.a);

    // Depth calculation
    vec4 clip_space = projection * view * vec4(intersection, 1.0);
    float ndc_depth = clip_space.z / clip_space.w;
    float window_depth = (ndc_depth * 0.5) + 0.5; // Map from [-1,1] to [0,1]

    // Apply a small depth bias to prevent z-fighting
    float depth_bias = 0.0001;
    gl_FragDepth = window_depth + depth_bias;
}
