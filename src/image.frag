#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
uniform uvec2 screen_resolution;
uniform sampler2D tex;

void main()
{
    vec2 relativeCoordinates = gl_FragCoord.xy / screen_resolution.xy;
    vec2 gridIndex = relativeCoordinates * textureSize(tex, 0).xy;
    bvec2 lower = lessThan(abs(fract(gridIndex + 0.5) - 0.5), vec2(0.1));
    Color = texture(tex, relativeCoordinates); 
}