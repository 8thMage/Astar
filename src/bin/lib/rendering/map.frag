#version 330 core

out vec4 Color;
in vec4 gl_FragCoord;
in vec2 texPosition;
uniform isampler2D tex;

void main()
{
    vec2 gridIndex = texPosition;
    bvec2 lower = lessThan(abs(fract(gridIndex + 0.5) - 0.5), vec2(0.1));
    if (any(lower)) {
        Color = vec4(0, 0, 0, 1.0f);
    } else { 
        // ivec2 index = ivec2(floor(gridIndex));
        //    Color = vec4(1, 1, 0.6f, 1.0f);
        int isValue = texture(tex, gridIndex / textureSize(tex, 0) + 0.5).r;
        if(isValue == 3){
            Color = vec4(0.8, 0.0, 0, 1.);
        } else if(isValue == 2) {
            Color = vec4(0.5, 0.4, 0, 1.);
        } else {
            Color = vec4(0.5, 0.4, isValue, 1.);
        }
    }
}