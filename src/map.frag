#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
uniform uvec2 screen_resolution;
uniform float zoom;
uniform isampler2D tex;

void main()
{
    vec2 relativeCoordinates = gl_FragCoord.xy / screen_resolution.xy;
    vec2 gridIndex = relativeCoordinates * textureSize(tex, 0).xy;
    bvec2 lower = lessThan(abs(fract(gridIndex + 0.5) - 0.5), vec2(0.1));
    if (any(lower)) {
        Color = vec4(0, 0, 0, 1.0f);
    } else { 
        ivec2 index = ivec2(floor(gridIndex));
        //    Color = vec4(1, 1, 0.6f, 1.0f);
        int isValue = texture(tex, relativeCoordinates).r;
        if(isValue == 3){
           Color = vec4(1, 1, 0.6f, 1.0f);
        } else if(isValue == 2) {
           Color = vec4(1, 0, 0.6f, 1.0f);
        } else {
           Color = vec4(0, 0, 0.6f, 1.0f);
        }
    }
}