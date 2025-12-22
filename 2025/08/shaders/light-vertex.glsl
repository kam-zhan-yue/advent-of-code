#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColour;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec3 vColour;

const float MIN = 5.0;
const float MAX = 200.0;
const float MAX_DISTANCE = 100.0;

void main() {
  vec4 pos = vec4(vec3(aPos), 1.0);
  gl_Position = projection * view * model * pos;
  gl_PointSize = 5.0;
  vColour = aColour;
}

