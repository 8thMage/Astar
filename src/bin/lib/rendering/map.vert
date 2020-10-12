#version 330 core

layout (location = 0) in vec3 Position;
out vec2 texPosition;
uniform isampler2D tex;
uniform mat3x2 transform;

void main()
{
    gl_Position = vec4(Position, 1.0);
    vec2 transformedPosition = transform * vec3(Position.xy, 1.);

    texPosition = transformedPosition * textureSize(tex, 0).xy / 2;
}