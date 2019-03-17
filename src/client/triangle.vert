#version 120

attribute vec2 a_Pos;
attribute vec3 a_Color;
varying vec4 v_Color;
uniform mat2 u_Transform;

void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = vec4(a_Pos * u_Transform, 0.0, 1.0);
}
