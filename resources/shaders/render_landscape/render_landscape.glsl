#include "../../engine_resources/shaders/common/random.glsl"

layout( push_constant ) uniform PushConstant_RenderLandscape
{
    PushConstant_RenderObjectBase push_constant_base;
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
} pushConstant;

// bindings
layout(binding = USER_BINDING_INDEX0) uniform sampler2D textureLayerMask;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D layer0_textureBase;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D layer0_textureMaterial;
layout(binding = USER_BINDING_INDEX3) uniform sampler2D layer0_textureNormal;
layout(binding = USER_BINDING_INDEX4) uniform sampler2D layer1_textureBase;
layout(binding = USER_BINDING_INDEX5) uniform sampler2D layer1_textureMaterial;
layout(binding = USER_BINDING_INDEX6) uniform sampler2D layer1_textureNormal;
layout(binding = USER_BINDING_INDEX7) uniform sampler2D layer2_textureBase;
layout(binding = USER_BINDING_INDEX8) uniform sampler2D layer2_textureMaterial;
layout(binding = USER_BINDING_INDEX9) uniform sampler2D layer2_textureNormal;
layout(binding = USER_BINDING_INDEX10) uniform sampler2D layer3_textureBase;
layout(binding = USER_BINDING_INDEX11) uniform sampler2D layer3_textureMaterial;
layout(binding = USER_BINDING_INDEX12) uniform sampler2D layer3_textureNormal;
layout(binding = USER_BINDING_INDEX13) uniform sampler2D layer4_textureBase;
layout(binding = USER_BINDING_INDEX14) uniform sampler2D layer4_textureMaterial;
layout(binding = USER_BINDING_INDEX15) uniform sampler2D layer4_textureNormal;

// material functions
IMPL_GET_PUSH_CONSTANT_BASE()
{
    return pushConstant.push_constant_base;
}

IMPL_GET_TEXCOORD()
{
    const vec3 world_position = vs_output.relative_position.xyz + view_constants.CAMERA_POSITION;
    return world_position.xz * pushConstant.tiling;
}

IMPL_GET_BASE_COLOR()
{
    vec4 base_color = mix(texture(layer0_textureBase, texcoord), texture(layer1_textureBase, texcoord), vs_output.color.x);
    base_color.xyz = pow(base_color.xyz, vec3(2.2));
    base_color.w = 1.0;
    return base_color;
}

IMPL_GET_MATERIAL()
{
    return mix(texture(layer0_textureMaterial, texcoord), texture(layer1_textureMaterial, texcoord), vs_output.color.x);
}

IMPL_GET_TANGENT_NORMAL()
{
    vec3 tangent_normal = mix(texture(layer0_textureNormal, texcoord), texture(layer1_textureNormal, texcoord), vs_output.color.x).xyz;
    return (tangent_normal * 2.0 - 1.0);
}

IMPL_GET_WORLD_OFFSET()
{
    return vec3(0.0);
}