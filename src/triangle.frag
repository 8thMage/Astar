#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
uniform uvec2 screen_resolution;

void main()
{
    vec2 relativeCoordinates = gl_FragCoord.xy / screen_resolution;
    bvec2 lower = lessThan(abs(fract(relativeCoordinates * 10 + 0.5) - 0.5), vec2(0.1));
    if (any(lower)) {
        Color = vec4(0, 0, 0, 1.0f);
    } else { 
           Color = vec4(gl_FragCoord.x / screen_resolution.x, gl_FragCoord.y / screen_resolution.y, 0.6f, 1.0f);
    }
}