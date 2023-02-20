#version 140
        
out vec4 color;

uniform vec3 ourColor;
in vec3 vertColor;

void main() {
    // color = vec4(1.0, 0.0, 0.0, 1.0);
    // color = ourColor;
    // color = vec4(vertColor.r + ourColor.r, vertColor.g + ourColor.g, vertColor.b + ourColor.b, 1.0);
    color = vec4(vertColor + ourColor, 1.0);
}