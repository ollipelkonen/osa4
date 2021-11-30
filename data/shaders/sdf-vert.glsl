#version 150


in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform float time;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;


void main() {
  gl_Position = vec4(position, 1.0f);
  v_position = position;
  v_normal = normal;
  v_tex_coords = tex_coords;
}
