#version 330 core

in vec4 position;
out vec2 Tex_coord;

void main() {
  gl_Position = vec4(position.xy, 0.0, 1.0);
  Tex_coord = position.zw;
}
