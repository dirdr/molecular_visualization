// Fragment Shader
#version 410 core
in vec2 v_tex_coords;
in vec3 v_world_pos;
in vec3 v_center;

out vec4 frag_color;

uniform vec3 light_position;
uniform vec3 camera_position;
uniform vec3 sphere_color;
uniform float sphere_radius;

void main() {
    // Convert texture coordinates from [0,1] to [-1,1]
    vec2 pos = v_tex_coords * 2.0 - 1.0;
    
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
    float c = dot(oc, oc) - sphere_radius * sphere_radius;
    float discriminant = b * b - 4.0 * a * c;
    
    // If no intersection, discard the fragment
    if (discriminant < 0.0 || length(pos) > 1.0) {
        discard;
    }
    
    // Get the nearest intersection point
    float t = (-b - sqrt(discriminant)) / (2.0 * a);
    vec3 intersection = ray_origin + t * ray_dir;
    
    // Calculate normal at intersection point
    vec3 normal = normalize(intersection - sphere_center);
    
    // Basic lighting calculation
    vec3 light_dir = normalize(light_position - intersection);
    float diffuse = max(dot(normal, light_dir), 0.0);
    
    vec3 view_dir = normalize(camera_position - intersection);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    
    // Ambient light
    float ambient = 0.1;
    
    // Final color
    vec3 color = sphere_color * (ambient + diffuse + specular);
    frag_color = vec4(color, 1.0);
}
