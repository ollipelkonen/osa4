#version 140

uniform float time

in vec3 v_position;
in vec2 v_tex_coords;

out vec4 color;

#include <sdf_func.glsl>

void main() {
  color = vec4(1,1,1,1);
}
