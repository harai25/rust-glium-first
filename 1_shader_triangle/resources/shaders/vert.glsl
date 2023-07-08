#version 140

in vec2 position;
in vec3 color;

out vec3 vertColor;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    vertColor = color;
}