#include "../../engine_resources/shaders/common/random.glsl"

// bindings
layout(binding = USER_BINDING_INDEX0) uniform sampler2D layer0_textureBase;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D layer0_textureNormal;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D layer1_textureBase;
layout(binding = USER_BINDING_INDEX5) uniform sampler2D layer1_textureNormal;
layout(binding = USER_BINDING_INDEX6) uniform sampler2D layer2_textureBase;
layout(binding = USER_BINDING_INDEX8) uniform sampler2D layer2_textureNormal;
layout(binding = USER_BINDING_INDEX9) uniform sampler2D layer3_textureBase;
layout(binding = USER_BINDING_INDEX11) uniform sampler2D layer3_textureNormal;
layout(binding = USER_BINDING_INDEX12) uniform sampler2D layer4_textureBase;
layout(binding = USER_BINDING_INDEX14) uniform sampler2D layer4_textureNormal;

// push constant
BEGIN_PUSH_CONSTANT(PushConstant_RenderLandscape)
    float tiling;
    float layer0_tiling;
    float layer0_alpha;
    float layer1_tiling;
    float layer1_alpha;
    float layer2_tiling;
    float layer2_alpha;
    float layer3_tiling;
    float layer3_alpha;
    float layer4_tiling;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
}

#elif SHADER_STAGE_FLAG == FRAGMENT
FRAGMENT_SHADER_MAIN()
{
    const vec3 world_position = in_vertex_output._relative_position.xyz + view_constants.CAMERA_POSITION;
    const vec2 texcoord = world_position.xz * GET_PUSH_CONSTANT().tiling;
    const vec2 layer0_texcoord = texcoord * GET_PUSH_CONSTANT().layer0_tiling;
    const vec2 layer1_texcoord = texcoord * GET_PUSH_CONSTANT().layer1_tiling;
    const vec2 layer2_texcoord = texcoord * GET_PUSH_CONSTANT().layer2_tiling;
    const vec2 layer3_texcoord = texcoord * GET_PUSH_CONSTANT().layer3_tiling;
    const vec2 layer4_texcoord = texcoord * GET_PUSH_CONSTANT().layer4_tiling;

    const vec4 color4 = texture(layer4_textureBase, layer4_texcoord);
    const vec4 color3 = texture(layer3_textureBase, layer3_texcoord);
    const vec4 color2 = texture(layer2_textureBase, layer2_texcoord);
    const vec4 color1 = texture(layer1_textureBase, layer1_texcoord);
    const vec4 color0 = texture(layer0_textureBase, layer0_texcoord);

    const vec3 normal4 = texture(layer3_textureNormal, layer4_texcoord).xyz * 2.0 - 1.0;
    const vec3 normal3 = texture(layer3_textureNormal, layer3_texcoord).xyz * 2.0 - 1.0;
    const vec3 normal2 = texture(layer2_textureNormal, layer2_texcoord).xyz * 2.0 - 1.0;
    const vec3 normal1 = texture(layer1_textureNormal, layer1_texcoord).xyz * 2.0 - 1.0;
    const vec3 normal0 = texture(layer0_textureNormal, layer0_texcoord).xyz * 2.0 - 1.0;

    const vec4 layer_mask_weight = mix(vec4(8.0), vec4(1.0), vec4(
        normal0.z * get_luminance(color0.xyz) * color0.w,
        normal1.z * get_luminance(color1.xyz) * color1.w,
        normal2.z * get_luminance(color2.xyz) * color2.w,
        normal3.z * get_luminance(color3.xyz) * color3.w
    ));
    const vec4 layer_masks = pow(in_vertex_output._color * vec4(
        GET_PUSH_CONSTANT().layer0_alpha,
        GET_PUSH_CONSTANT().layer1_alpha,
        GET_PUSH_CONSTANT().layer2_alpha,
        GET_PUSH_CONSTANT().layer3_alpha
    ), layer_mask_weight);

    out_base_color = color4;
    out_base_color = mix(out_base_color, color3, layer_masks[3]);
    out_base_color = mix(out_base_color, color2, layer_masks[2]);
    out_base_color = mix(out_base_color, color1, layer_masks[1]);
    out_base_color = mix(out_base_color, color0, layer_masks[0]);
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2));
    out_base_color.w = 1.0;

    // x: roughness y: metalic
    out_material.x = 0.9;
    out_material.y = 0.0;

    out_tangent_normal = normal4;
    out_tangent_normal = mix(out_tangent_normal, normal3, layer_masks[3]);
    out_tangent_normal = mix(out_tangent_normal, normal2, layer_masks[2]);
    out_tangent_normal = mix(out_tangent_normal, normal1, layer_masks[1]);
    out_tangent_normal = mix(out_tangent_normal, normal0, layer_masks[0]);
    out_tangent_normal = safe_normalize(out_tangent_normal);
}
#endif
