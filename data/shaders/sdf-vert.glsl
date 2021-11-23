#version 150

//in vec4 position;
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform float time;


void main() {
  gl_Position = vec4(
    position.x* 10000.0f * sin(position.y*0.7f+time*1241.0f),
    position.y*1000.0f*cos(position.x+time*100.0f),
    1000.0f * sin(time*1000.0f), 1.0);
  gl_Position = vec4(position.xy,0.0, 1.0);
}
