#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
in vec2 texPositionOut;
uniform sampler2D tex;

void main()
{
    Color = texture(tex, texPositionOut); 
}