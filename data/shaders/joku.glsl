#version 140

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform vec3 u_light;
uniform sampler2D diffuse_tex;
uniform sampler2D normal_tex;
uniform float time;

out vec4 color;

void main() {
  color = vec4(0.5*sin(v_position.x+time), 0, 0.5*cos(v_position.y*time/10.0+time), 1.0);
}

