#version 140

//in vec3 v_position;
//  in vec2 v_tex_coord;
out vec4 color;


in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
uniform vec3 u_light;
uniform sampler2D diffuse_tex;
uniform sampler2D normal_tex;

uniform float time;


void main() {
  color = vec4(1.0, 1.0, 1.0, 1.0);
}

