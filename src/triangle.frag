#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
uniform uvec2 screen_resolution;
uniform isampler2D tex;

void main()
{
    vec2 relativeCoordinates = gl_FragCoord.xy / screen_resolution.xx;
    vec2 gridIndex = relativeCoordinates * 10;
    bvec2 lower = lessThan(abs(fract(gridIndex + 0.5) - 0.5), vec2(0.1));
    if (any(lower)) {
        Color = vec4(0, 0, 0, 1.0f);
    } else { 
        ivec2 index = ivec2(floor(gridIndex));
        //    Color = vec4(1, 1, 0.6f, 1.0f);
        bool isValue = texelFetch(tex, index, 0).r == 1;
        if(isValue){
           Color = vec4(0, 0, 0.6f, 1.0f);
        } else {
           Color = vec4(1, 1, 0.6f, 1.0f);
        }
    }
}