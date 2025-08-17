#include "../../engine_resources/shaders/common/random.glsl"

layout(binding = USER_BINDING_INDEX0) uniform sampler2D _Top;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D _TopNormal;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D _Slides;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D _SlidesNormal;

BEGIN_PUSH_CONSTANT(PushConstant_TriplanarBasic)
    vec4 _Color;
    vec2 _Tiling;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
    // out_world_offset = vec3(pow(in_relative_position.y - in_local_latrix[3].y, 2.0) * sin(scene_constants.TIME + random(in_local_latrix[3].xyz) * 13.1423), 0.0, 0.0);
}

#elif SHADER_STAGE_FLAG == FRAGMENT

inline float4 TriplanarSampling8( sampler2D topTexMap, sampler2D midTexMap, sampler2D botTexMap, float3 worldPos, float3 worldNormal, float falloff, float2 tiling, float3 normalScale, float3 index )
{
    float3 projNormal = ( pow( abs( worldNormal ), falloff ) );
    projNormal /= ( projNormal.x + projNormal.y + projNormal.z ) + 0.00001;
    float3 nsign = sign( worldNormal );
    float negProjNormalY = max( 0, projNormal.y * -nsign.y );
    projNormal.y = max( 0, projNormal.y * nsign.y );
    half4 xNorm; half4 yNorm; half4 yNormN; half4 zNorm;
    xNorm  = tex2D( midTexMap, tiling * worldPos.zy * float2(  nsign.x, 1.0 ) );
    yNorm  = tex2D( topTexMap, tiling * worldPos.xz * float2(  nsign.y, 1.0 ) );
    yNormN = tex2D( botTexMap, tiling * worldPos.xz * float2(  nsign.y, 1.0 ) );
    zNorm  = tex2D( midTexMap, tiling * worldPos.xy * float2( -nsign.z, 1.0 ) );
    return xNorm * projNormal.x + yNorm * projNormal.y + yNormN * negProjNormalY + zNorm * projNormal.z;
}


inline float4 TriplanarSampling5( sampler2D topTexMap, sampler2D midTexMap, sampler2D botTexMap, float3 worldPos, float3 worldNormal, float falloff, float2 tiling, float3 normalScale, float3 index )
{
    float3 projNormal = ( pow( abs( worldNormal ), falloff ) );
    projNormal /= ( projNormal.x + projNormal.y + projNormal.z ) + 0.00001;
    float3 nsign = sign( worldNormal );
    float negProjNormalY = max( 0, projNormal.y * -nsign.y );
    projNormal.y = max( 0, projNormal.y * nsign.y );
    half4 xNorm; half4 yNorm; half4 yNormN; half4 zNorm;
    xNorm  = tex2D( midTexMap, tiling * worldPos.zy * float2(  nsign.x, 1.0 ) );
    yNorm  = tex2D( topTexMap, tiling * worldPos.xz * float2(  nsign.y, 1.0 ) );
    yNormN = tex2D( botTexMap, tiling * worldPos.xz * float2(  nsign.y, 1.0 ) );
    zNorm  = tex2D( midTexMap, tiling * worldPos.xy * float2( -nsign.z, 1.0 ) );
    return xNorm * projNormal.x + yNorm * projNormal.y + yNormN * negProjNormalY + zNorm * projNormal.z;
}

FRAGMENT_SHADER_MAIN()
{
    float3 ase_worldPos = i.worldPos;
    float3 ase_worldNormal = WorldNormalVector( i, float3( 0, 0, 1 ) );
    float4 triplanar8 = TriplanarSampling8( _TopNormal, _SidesNormal, _SidesNormal, ase_worldPos, ase_worldNormal, _FallOff, _Tiling, float3( 1,1,1 ), float3(0,0,0) );
    o.Normal = UnpackNormal( triplanar8 );
    float4 triplanar5 = TriplanarSampling5( _Top, _Sides, _Sides, ase_worldPos, ase_worldNormal, _FallOff, _Tiling, float3( 1,1,1 ), float3(0,0,0) );
    o.Albedo = triplanar5.xyz;
    o.Alpha = 1;

    out_base_color = texture(_LeafTex, in_vertex_output._texCoord);
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2));
    out_material = texture(textureMaterial, in_vertex_output._texCoord);
    out_tangent_normal = texture(_LeafNormalMap, in_vertex_output._texCoord).xyz * 2.0 - 1.0;
}
#endif