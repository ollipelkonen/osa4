#version 330 core

in vec4 position;
out vec3 v_position;
out vec2 v_tex_coord;

void main() {
  v_position = vec3(position.xy, 0.0);
  v_tex_coord = position.zw;
}
