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

// material functions
struct UserData
{
    vec2 layer0_texcoord;
    vec2 layer1_texcoord;
    vec2 layer2_texcoord;
    vec2 layer3_texcoord;
    vec2 layer4_texcoord;
    vec4 layer_masks;
} user_data;

IMPL_INITALIZE_USER_DATA()
{
    const vec3 world_position = vs_output.relative_position.xyz + view_constants.CAMERA_POSITION;
    vec2 texcoord = world_position.xz * pushConstant.tiling;
    user_data.layer0_texcoord = texcoord * pushConstant.layer0_tiling;
    user_data.layer1_texcoord = texcoord * pushConstant.layer1_tiling;
    user_data.layer2_texcoord = texcoord * pushConstant.layer2_tiling;
    user_data.layer3_texcoord = texcoord * pushConstant.layer3_tiling;
    user_data.layer4_texcoord = texcoord * pushConstant.layer4_tiling;
    user_data.layer_masks = vs_output.color * vec4(pushConstant.layer1_alpha, pushConstant.layer2_alpha, pushConstant.layer3_alpha, pushConstant.layer4_alpha);
}

IMPL_GET_PUSH_CONSTANT_BASE()
{
    return pushConstant.push_constant_base;
}

IMPL_GET_BASE_COLOR()
{
    vec4 base_color = mix(texture(layer0_textureBase, user_data.layer0_texcoord), texture(layer1_textureBase, user_data.layer1_texcoord), user_data.layer_masks.x);
    base_color = mix(base_color, texture(layer2_textureBase, user_data.layer2_texcoord), user_data.layer_masks.y);
    base_color = mix(base_color, texture(layer3_textureBase, user_data.layer3_texcoord), user_data.layer_masks.z);
    base_color = mix(base_color, texture(layer4_textureBase, user_data.layer4_texcoord), user_data.layer_masks.w);

    base_color.xyz = pow(base_color.xyz, vec3(2.2));
    base_color.w = 1.0;
    return base_color;
}

IMPL_GET_MATERIAL()
{
    vec4 material = mix(texture(layer0_textureMaterial, user_data.layer0_texcoord), texture(layer1_textureMaterial, user_data.layer1_texcoord), user_data.layer_masks.x);
    material = mix(material, texture(layer2_textureMaterial, user_data.layer2_texcoord), user_data.layer_masks.y);
    material = mix(material, texture(layer3_textureMaterial, user_data.layer3_texcoord), user_data.layer_masks.z);
    material = mix(material, texture(layer4_textureMaterial, user_data.layer4_texcoord), user_data.layer_masks.w);

    return material;
}

IMPL_GET_TANGENT_NORMAL()
{
    vec3 tangent_normal = mix(texture(layer0_textureNormal, user_data.layer0_texcoord).xyz * 2.0 - 1.0, texture(layer1_textureNormal, user_data.layer1_texcoord).xyz * 2.0 - 1.0, user_data.layer_masks.x);
    tangent_normal = mix(tangent_normal, texture(layer2_textureNormal, user_data.layer2_texcoord).xyz * 2.0 - 1.0, user_data.layer_masks.y);
    tangent_normal = mix(tangent_normal, texture(layer3_textureNormal, user_data.layer3_texcoord).xyz * 2.0 - 1.0, user_data.layer_masks.z);
    tangent_normal = mix(tangent_normal, texture(layer4_textureNormal, user_data.layer4_texcoord).xyz * 2.0 - 1.0, user_data.layer_masks.w);
    return safe_normalize(tangent_normal);
}

IMPL_GET_WORLD_OFFSET()
{
    return vec3(0.0);
}