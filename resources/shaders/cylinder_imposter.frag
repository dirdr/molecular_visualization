#version 410 core
in vec2 v_uv_coordinates;
in vec3 v_world_pos;
in vec3 v_start;
in vec3 v_end;
in vec4 v_color;
in float v_radius;

out vec4 frag_color;

uniform vec3 camera_position;
uniform vec3 light_position;
uniform bool debug_billboard;
uniform mat4 projection;
uniform mat4 view;

void main() {
    // Ray-cylinder intersection
    vec3 ray_origin = camera_position;
    vec3 ray_dir = normalize(v_world_pos - camera_position);

    vec3 cylinder_dir = normalize(v_end - v_start);
    vec3 cylinder_center = (v_start + v_end) * 0.5;
    float cylinder_length = distance(v_end, v_start);

    // Calculate cylinder intersection
    vec3 oc = ray_origin - v_start;
    float a = dot(ray_dir - dot(ray_dir, cylinder_dir) * cylinder_dir,
            ray_dir - dot(ray_dir, cylinder_dir) * cylinder_dir);
    float b = 2.0 * dot(ray_dir - dot(ray_dir, cylinder_dir) * cylinder_dir,
                oc - dot(oc, cylinder_dir) * cylinder_dir);
    float c = dot(oc - dot(oc, cylinder_dir) * cylinder_dir,
            oc - dot(oc, cylinder_dir) * cylinder_dir) - v_radius * v_radius;

    float discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        if (debug_billboard) {
            frag_color = vec4(1.0, 0.0, 1.0, 1);
            return;
        } else {
            discard;
        }
    }

    float t = (-b - sqrt(discriminant)) / (2.0 * a);
    vec3 intersection = ray_origin + t * ray_dir;

    // Check if intersection is within cylinder length
    float along_cylinder = dot(intersection - v_start, cylinder_dir);
    if (along_cylinder < 0.0 || along_cylinder > cylinder_length) {
        discard;
    }

    float length = distance(v_end, v_start);
    t = clamp(t, 0.0, length);

    // Closest point on the cylinder axis
    vec3 closest_point = v_start + t * cylinder_dir;

    // Normal is the normalized vector from the closest point to the intersection point
    vec3 normal = normalize(intersection - closest_point);

    vec3 light_dir = normalize(light_position - intersection);
    float diffuse = max(dot(normal, light_dir), 0.0);

    vec3 view_dir = normalize(camera_position - intersection);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

    float ambient = 0.4;
    vec3 final_color = v_color.rgb * (ambient + diffuse + specular);

    // Calculate depth in view space
    // Convert world position to clip space to get correct depth
    vec4 clip_space = projection * view * vec4(intersection, 1.0);
    float ndc_depth = clip_space.z / clip_space.w;
    float window_depth = (ndc_depth * 0.5) + 0.5; // Map from [-1,1] to [0,1]

    // Apply a small depth bias to prevent z-fighting
    float depth_bias = 0.0001;
    gl_FragDepth = window_depth + depth_bias;

    frag_color = vec4(final_color, v_color.a);
}
