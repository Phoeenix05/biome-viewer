#version 330

in vec2 position;
in vec4 color;

uniform vec2 offset;
uniform float zoom;

out vec4 v_color;

void main() {
    gl_Position = vec4(position + offset, 0, 0);
    v_color = color;
}
