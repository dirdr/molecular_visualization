#version 410 core

in vec2 v_tex_coords;
uniform sampler2D u_texture;

out vec4 frag_color;
void main() {
    frag_color = texture(u_texture, v_tex_coords);
}
