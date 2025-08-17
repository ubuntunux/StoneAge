#include "../../engine_resources/shaders/common/random.glsl"

layout(binding = USER_BINDING_INDEX0) uniform sampler2D _MainTex;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D _EmissionMap;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D _BumpMap;

BEGIN_PUSH_CONSTANT(PushConstant_Standard)
    vec4 _Color;
    float _Glossiness;
    float _Metallic;
    float _MainTexScaleX;
    float _MainTexScaleY;
    float _BumpMapScaleX;
    float _BumpMapScaleY;
    float _EmissionMapScaleX;
    float _EmissionMapScaleY;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
}

#elif SHADER_STAGE_FLAG == FRAGMENT
FRAGMENT_SHADER_MAIN()
{
    out_base_color = texture(_MainTex, in_vertex_output._texCoord * vec2(push_constant._MainTexScaleX, push_constant._MainTexScaleY));
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2)) * push_constant._Color.xyz;

    vec4 emission = texture(_EmissionMap, in_vertex_output._texCoord * vec2(push_constant._EmissionMapScaleX, push_constant._EmissionMapScaleY));
    emission.xyz = pow(emission.xyz, vec3(2.2)) * emission.w;
    const float emissive_intensity = length(emission.xyz);

    out_base_color.xyz += emission.xyz;

    // x: roughness, y: metallic, z: emissive intensity
    out_material = vec4(1.0 - push_constant._Glossiness, push_constant._Metallic, emissive_intensity, 1.0);
    out_tangent_normal = texture(_BumpMap, in_vertex_output._texCoord * vec2(push_constant._BumpMapScaleX, push_constant._BumpMapScaleY)).xyz * 2.0 - 1.0;
}
#endif