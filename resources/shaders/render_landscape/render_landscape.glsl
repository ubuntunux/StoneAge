#include "../../engine_resources/shaders/common/random.glsl"

// bindings
layout(binding = USER_BINDING_INDEX0) uniform sampler2D layer0_textureBase;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D layer0_textureMaterial;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D layer0_textureNormal;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D layer1_textureBase;
layout(binding = USER_BINDING_INDEX4) uniform sampler2D layer1_textureMaterial;
layout(binding = USER_BINDING_INDEX5) uniform sampler2D layer1_textureNormal;
layout(binding = USER_BINDING_INDEX6) uniform sampler2D layer2_textureBase;
layout(binding = USER_BINDING_INDEX7) uniform sampler2D layer2_textureMaterial;
layout(binding = USER_BINDING_INDEX8) uniform sampler2D layer2_textureNormal;
layout(binding = USER_BINDING_INDEX9) uniform sampler2D layer3_textureBase;
layout(binding = USER_BINDING_INDEX10) uniform sampler2D layer3_textureMaterial;
layout(binding = USER_BINDING_INDEX11) uniform sampler2D layer3_textureNormal;
layout(binding = USER_BINDING_INDEX12) uniform sampler2D layer4_textureBase;
layout(binding = USER_BINDING_INDEX13) uniform sampler2D layer4_textureMaterial;
layout(binding = USER_BINDING_INDEX14) uniform sampler2D layer4_textureNormal;

// push constant
BEGIN_PUSH_CONSTANT(PushConstant_RenderLandscape)
    float tiling;
    float layer0_tiling;
    float layer1_alpha;
    float layer1_tiling;
    float layer2_alpha;
    float layer2_tiling;
    float layer3_alpha;
    float layer3_tiling;
    float layer4_alpha;
    float layer4_tiling;
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
}

#elif SHADER_STAGE_FLAG == FRAGMENT
FRAGMENT_SHADER_MAIN()
{
    const vec3 world_position = in_vertex_output.relative_position.xyz + view_constants.CAMERA_POSITION;
    const vec2 texcoord = world_position.xz * GET_PUSH_CONSTANT().tiling;
    const vec2 layer0_texcoord = texcoord * GET_PUSH_CONSTANT().layer0_tiling;
    const vec2 layer1_texcoord = texcoord * GET_PUSH_CONSTANT().layer1_tiling;
    const vec2 layer2_texcoord = texcoord * GET_PUSH_CONSTANT().layer2_tiling;
    const vec2 layer3_texcoord = texcoord * GET_PUSH_CONSTANT().layer3_tiling;
    const vec2 layer4_texcoord = texcoord * GET_PUSH_CONSTANT().layer4_tiling;
    const vec4 layer_masks = in_vertex_output.color * vec4(GET_PUSH_CONSTANT().layer1_alpha, GET_PUSH_CONSTANT().layer2_alpha, GET_PUSH_CONSTANT().layer3_alpha, GET_PUSH_CONSTANT().layer4_alpha);

    out_base_color = mix(texture(layer0_textureBase, layer0_texcoord), texture(layer1_textureBase, layer1_texcoord), layer_masks.x);
    out_base_color = mix(out_base_color, texture(layer2_textureBase, layer2_texcoord), layer_masks.y);
    out_base_color = mix(out_base_color, texture(layer3_textureBase, layer3_texcoord), layer_masks.z);
    out_base_color = mix(out_base_color, texture(layer4_textureBase, layer4_texcoord), layer_masks.w);
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2));
    out_base_color.w = 1.0;

    out_material = mix(texture(layer0_textureMaterial, layer0_texcoord), texture(layer1_textureMaterial, layer1_texcoord), layer_masks.x);
    out_material = mix(out_material, texture(layer2_textureMaterial, layer2_texcoord), layer_masks.y);
    out_material = mix(out_material, texture(layer3_textureMaterial, layer3_texcoord), layer_masks.z);
    out_material = mix(out_material, texture(layer4_textureMaterial, layer4_texcoord), layer_masks.w);

    out_tangent_normal = mix(texture(layer0_textureNormal, layer0_texcoord).xyz * 2.0 - 1.0, texture(layer1_textureNormal, layer1_texcoord).xyz * 2.0 - 1.0, layer_masks.x);
    out_tangent_normal = mix(out_tangent_normal, texture(layer2_textureNormal, layer2_texcoord).xyz * 2.0 - 1.0, layer_masks.y);
    out_tangent_normal = mix(out_tangent_normal, texture(layer3_textureNormal, layer3_texcoord).xyz * 2.0 - 1.0, layer_masks.z);
    out_tangent_normal = mix(out_tangent_normal, texture(layer4_textureNormal, layer4_texcoord).xyz * 2.0 - 1.0, layer_masks.w);
    out_tangent_normal = safe_normalize(out_tangent_normal);
}
#endif