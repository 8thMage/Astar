#version 330 core

layout (location = 0) in vec3 Position;
uniform vec2 scale;
uniform vec2 offset;
out vec2 texPositionOut;
uniform uvec2 screen_resolution;

void main()
{
    gl_Position = vec4((Position.xy * scale + offset) * vec2(1, screen_resolution.x / screen_resolution.y),Position.z, 1.0);
    vec2 texPositionFixed = (Position.xy + 1) / 2;
    texPositionOut =vec2(texPositionFixed.x, 1 - texPositionFixed.y);
}