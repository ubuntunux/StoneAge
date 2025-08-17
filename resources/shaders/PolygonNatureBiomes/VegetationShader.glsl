#include "../../engine_resources/shaders/common/random.glsl"

layout(binding = USER_BINDING_INDEX0) uniform sampler2D _EmissiveMask;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D _GustNoiseMap;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D _LeafNormalMap;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D _LeafTex;
layout(binding = USER_BINDING_INDEX4) uniform sampler2D _TrunkNormalMap;
layout(binding = USER_BINDING_INDEX5) uniform sampler2D _TunkTex;

BEGIN_PUSH_CONSTANT(PushConstant_VegetationShader)
    vec4 _BaseColour;
    vec4 _EmissiveColour;
    vec4 _TrunkBaseColour;
    float _GustFreq;
    float _GustLargeFreq;
    float _LeafSmoothness;
    float _TrunkSmoothness;
    float _EmissiveMaskScaleX;
    float _EmissiveMaskScaleY;
    float _GustNoiseMapScaleX;
    float _GustNoiseMapScaleY;
    float _LeafNormalMapScaleX;
    float _LeafNormalMapScaleY;
    float _LeafTexScaleX;
    float _LeafTexScaleY;
    float _TrunkNormalMapScaleX;
    float _TrunkNormalMapScaleY;
    float _TunkTexScaleX;
    float _TunkTexScaleY;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
}

#elif SHADER_STAGE_FLAG == FRAGMENT
FRAGMENT_SHADER_MAIN()
{
    const float blend_factor = 0.5 < in_vertex_output._color.z ? 1.0 : 0.0;
    const vec3 base_colour = mix(GET_PUSH_CONSTANT()._TrunkBaseColour, GET_PUSH_CONSTANT()._BaseColour, blend_factor).xyz;
    out_base_color = mix(
        texture(_TunkTex, in_vertex_output._texCoord * vec2(GET_PUSH_CONSTANT()._TunkTexScaleX, GET_PUSH_CONSTANT()._TunkTexScaleY)),
        texture(_LeafTex, in_vertex_output._texCoord * vec2(GET_PUSH_CONSTANT()._LeafTexScaleX, GET_PUSH_CONSTANT()._LeafTexScaleY)),
        blend_factor
    );
    //out_base_color.xyz *= base_colour;
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2));
    out_material = vec4(0.0);
    // roughness
    out_material.x = 1.0 - mix(GET_PUSH_CONSTANT()._TrunkSmoothness, GET_PUSH_CONSTANT()._LeafSmoothness, blend_factor);
    out_tangent_normal = mix(
        texture(_TrunkNormalMap, in_vertex_output._texCoord * vec2(GET_PUSH_CONSTANT()._TrunkNormalMapScaleX, GET_PUSH_CONSTANT()._TrunkNormalMapScaleY)).xyz * 2.0 - 1.0,
        texture(_LeafNormalMap, in_vertex_output._texCoord * vec2(GET_PUSH_CONSTANT()._LeafNormalMapScaleX, GET_PUSH_CONSTANT()._LeafNormalMapScaleY)).xyz * 2.0 - 1.0,
        blend_factor
    );
}
#endif