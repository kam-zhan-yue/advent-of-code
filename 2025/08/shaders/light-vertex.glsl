#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aOffset;

uniform vec3 cameraPos;
uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

const float MIN = 5.0;
const float MAX = 200.0;
const float MAX_DISTANCE = 100.0;

void main() {
  vec4 pos = vec4(vec3(aPos + aOffset), 1.0);
  gl_Position = projection * view * model * pos;
  gl_PointSize = 10.0;
  // Change the size of the point depending on how far it is from the camera
  // float distance = length(cameraPos - pos.xyz);
  // float normalizedDist = clamp(distance / MAX_DISTANCE, 0.0, 1.0);
  // gl_PointSize = mix(MAX, MIN, normalizedDist);
}

