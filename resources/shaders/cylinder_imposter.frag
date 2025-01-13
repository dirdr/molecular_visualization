#version 410 core

in vec2 v_uv_coordinates;
in vec3 v_world_pos;
in vec3 v_start;
in vec3 v_end;
in vec4 v_color_first_half;
in vec4 v_color_second_half;
in float v_radius;

out vec4 frag_color;

uniform vec3 camera_position;
uniform vec3 light_position;
uniform bool debug_billboard;
uniform mat4 projection;
uniform mat4 view;

// New uniform for silhouette control
uniform bool u_show_silhouette;

void main() {
    // Ray-cylinder intersection
    vec3 ray_origin = camera_position;
    vec3 ray_dir = normalize(v_world_pos - camera_position);

    vec3 cylinder_dir = normalize(v_end - v_start);
    float cylinder_length = distance(v_end, v_start);

    // Calculate cylinder intersection
    vec3 oc = ray_origin - v_start;
    vec3 ray_proj = ray_dir - dot(ray_dir, cylinder_dir) * cylinder_dir;
    vec3 oc_proj = oc - dot(oc, cylinder_dir) * cylinder_dir;
    float a = dot(ray_proj, ray_proj);
    float b = 2.0 * dot(ray_proj, oc_proj);
    float c = dot(oc_proj, oc_proj) - v_radius * v_radius;

    float discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        if (debug_billboard) {
            frag_color = vec4(1.0, 0.0, 1.0, 1.0);
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

    // Compute normalized position along the cylinder (0.0 to 1.0)
    float t_normalized = along_cylinder / cylinder_length;

    // Closest point on the cylinder axis
    vec3 closest_point = v_start + dot(intersection - v_start, cylinder_dir) * cylinder_dir;

    // Normal is the normalized vector from the closest point to the intersection point
    vec3 normal = normalize(intersection - closest_point);

    // Lighting calculations
    vec3 light_dir = normalize(light_position - intersection);
    float distance_to_light = distance(light_position, intersection);
    float attenuation = 1.0 / (1.0 + 0.09 * distance_to_light + 0.032 * distance_to_light * distance_to_light);

    // Diffuse lighting
    float diffuse = max(dot(normal, light_dir), 0.0) * attenuation;

    // Specular lighting
    vec3 view_dir = normalize(camera_position - intersection);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float shininess = 32.0; // Higher value for sharper highlights
    float specular = pow(max(dot(view_dir, reflect_dir), 0.0), shininess) * attenuation;

    // Ambient lighting
    vec3 ambient_light = vec3(0.3, 0.3, 0.4); // Slightly bluish ambient light
    vec3 ambient = ambient_light * 0.70;

    // Determine which color to use based on position
    vec4 selected_color;
    if (t_normalized < 0.5) {
        selected_color = v_color_first_half;
    } else {
        selected_color = v_color_second_half;
    }

    // Combine lighting components with selected color
    vec3 final_color = selected_color.rgb * (ambient + diffuse) + specular;

    // Silhouette parameters
    const float SILHOUETTE_THRESHOLD = 0.4;
    const vec3 SILHOUETTE_COLOR = vec3(0.0, 0.0, 0.0); // Black silhouette

    // Compute the silhouette factor
    float ndotl = dot(normal, view_dir);
    float silhouette_factor = smoothstep(SILHOUETTE_THRESHOLD, SILHOUETTE_THRESHOLD + 0.1, abs(ndotl));

    // Apply silhouette if enabled
    if (u_show_silhouette && silhouette_factor < 1.0) {
        // Option 1: Directly set to silhouette color
        // final_color = SILHOUETTE_COLOR;

        // Option 2: Blend silhouette color with final color
        final_color = mix(SILHOUETTE_COLOR, final_color, silhouette_factor);
    }

    // Calculate depth in view space
    vec4 clip_space = projection * view * vec4(intersection, 1.0);
    float ndc_depth = clip_space.z / clip_space.w;
    float window_depth = (ndc_depth * 0.5) + 0.5; // Map from [-1,1] to [0,1]

    // Apply a small depth bias to prevent z-fighting
    float depth_bias = 0.0001;
    gl_FragDepth = window_depth + depth_bias;

    frag_color = vec4(final_color, selected_color.a);
}
