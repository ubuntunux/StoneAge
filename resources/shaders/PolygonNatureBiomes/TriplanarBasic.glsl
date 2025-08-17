#include "../../engine_resources/shaders/common/random.glsl"

layout(binding = USER_BINDING_INDEX0) uniform sampler2D _Top;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D _TopNormal;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D _Sides;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D _SidesNormal;

BEGIN_PUSH_CONSTANT(PushConstant_TriplanarBasic)
    vec4 _Color;
    float _Glossiness;
    float _Metallic;
    float _Tiling;
    float _TopScaleX;
    float _TopScaleY;
    float _TopNormalScaleX;
    float _TopNormalScaleY;
    float _SidesScaleX;
    float _SidesScaleY;
    float _SidesNormalScaleX;
    float _SidesNormalScaleY;
    float _reserved0;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
}

#elif SHADER_STAGE_FLAG == FRAGMENT


vec3 TriplanarSamplingNormal( sampler2D topTexMap, sampler2D midTexMap, sampler2D botTexMap, vec3 worldPos, vec3 worldNormal, float falloff, vec2 tiling )
{
    vec3 projNormal = safe_normalize( pow( abs( worldNormal ), vec3(falloff) ) );
    vec3 nsign = sign( worldNormal );
    float negProjNormalY = max( 0, projNormal.y * -nsign.y );
    projNormal.y = max( 0, projNormal.y * nsign.y );
    vec3 xNorm, yNorm, yNormN, zNorm;
    xNorm  = texture( midTexMap, tiling * worldPos.zy * vec2(  nsign.x, 1.0 ) ).xyz * 2.0 - 1.0;
    yNorm  = texture( topTexMap, tiling * worldPos.xz * vec2(  nsign.y, 1.0 ) ).xyz * 2.0 - 1.0;
    yNormN = texture( botTexMap, tiling * worldPos.xz * vec2(  nsign.y, 1.0 ) ).xyz * 2.0 - 1.0;
    zNorm  = texture( midTexMap, tiling * worldPos.xy * vec2( -nsign.z, 1.0 ) ).xyz * 2.0 - 1.0;
    return safe_normalize(xNorm * projNormal.x + yNorm * projNormal.y + yNormN * negProjNormalY + zNorm * projNormal.z);
}

vec4 TriplanarSampling( sampler2D topTexMap, sampler2D midTexMap, sampler2D botTexMap, vec3 worldPos, vec3 worldNormal, float falloff, vec2 tiling )
{
    vec3 projNormal = safe_normalize( pow( abs( worldNormal ), vec3(falloff) ) );
    vec3 nsign = sign( worldNormal );
    float negProjNormalY = max( 0, projNormal.y * -nsign.y );
    projNormal.y = max( 0, projNormal.y * nsign.y );
    vec4 xNorm, yNorm, yNormN, zNorm;
    xNorm  = texture( midTexMap, tiling * worldPos.zy * vec2(  nsign.x, 1.0 ) );
    yNorm  = texture( topTexMap, tiling * worldPos.xz * vec2(  nsign.y, 1.0 ) );
    yNormN = texture( botTexMap, tiling * worldPos.xz * vec2(  nsign.y, 1.0 ) );
    zNorm  = texture( midTexMap, tiling * worldPos.xy * vec2( -nsign.z, 1.0 ) );
    return xNorm * projNormal.x + yNorm * projNormal.y + yNormN * negProjNormalY + zNorm * projNormal.z;
}

FRAGMENT_SHADER_MAIN()
{
    vec3 ase_worldPos = in_vertex_output._relative_position.xyz + view_constants.CAMERA_POSITION;
    vec3 ase_worldNormal = vec3(0,1,0);//normalize(in_vertex_output._tangent_to_world[2]);
    float _FallOff = 10.0;

    vec4 baseColor = TriplanarSampling(_Top, _Sides, _Sides, ase_worldPos, ase_worldNormal, _FallOff, vec2(push_constant._Tiling));
    out_base_color.xyz = pow(baseColor.xyz, vec3(2.2));
    out_base_color.w = 1.0;

    // x: roughness, y: metallic, z: emissive intensity
    const float emissive_intensity = 0.0;
    out_material = vec4(1.0 - push_constant._Glossiness, push_constant._Metallic, emissive_intensity, 1.0);

    out_tangent_normal = TriplanarSamplingNormal(_TopNormal, _SidesNormal, _SidesNormal, ase_worldPos, ase_worldNormal, _FallOff, vec2(push_constant._Tiling));
}
#endif