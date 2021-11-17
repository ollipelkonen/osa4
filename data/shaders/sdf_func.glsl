
#define saturate(x) clamp(x, 0, 1)

float sdRoundBox( vec3 p, vec3 b, float r )
{
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - r;
}

float sdVerticalCapsule( vec3 p, float h, float r )
{
  p.y -= clamp( p.y, 0.0, h );
  return length( p ) - r;
}

float sdBox( vec3 p, vec3 b )
{
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdSphere( vec3 p, float r )
{
  return length(p) - r;
}

float sdEllipsoid( in vec3 p, in vec3 r )
{
  return (length(p/r) - 1.0) * min(min(r.x,r.y),r.z);
}

// Distance to line segment between <a> and <b>, used for fCapsule() version 2below
float sdLineSegment(vec3 p, vec3 a, vec3 b) {
  vec3 ab = b - a;
  float t = saturate(dot(p - a, ab) / dot(ab, ab));
  return length((ab*t + a) - p);
}

// Capsule version 2: between two end points <a> and <b> with radius r 
float sdCapsule(vec3 p, vec3 a, vec3 b, float r) {
  return sdLineSegment(p, a, b) - r;
}

// Torus in the XZ-plane
float sdTorus(vec3 p, float smallRadius, float largeRadius) {
  return length(vec2(length(p.xz) - largeRadius, p.y)) - smallRadius;
}

// A circle line. Can also be used to make a torus by subtracting the smaller radius of the torus.
float sdCircle(vec3 p, float r) {
  float l = length(p.xz) - r;
  return length(vec2(p.y, l));
}

// A circular disc with no thickness (i.e. a cylinder with no height).
// Subtract some value to make a flat disc with rounded edge.
float sdDisc(vec3 p, float r) {
  float l = length(p.xz) - r;
  return l < 0 ? abs(p.y) : length(vec2(p.y, l));
}
