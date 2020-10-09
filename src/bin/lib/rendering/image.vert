#version 330 core

layout (location = 0) in vec3 Position;
out vec2 texPositionOut;
uniform mat3x2 transform;

void main()
{
    gl_Position = vec4(transform * vec3(Position.xy, 1.), Position.z, 1.0);
    vec2 texPositionFixed = (Position.xy + 1) / 2;
    texPositionOut =vec2(texPositionFixed.x, 1 - texPositionFixed.y);
}