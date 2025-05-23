#include "../../engine_resources/shaders/common/random.glsl"

layout( push_constant ) uniform PushConstant_RenderGrass
{
    PushConstant_RenderObjectBase push_constant_base;
} pushConstant;

// bindings
layout(binding = USER_BINDING_INDEX0) uniform sampler2D textureBase;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D textureMaterial;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D textureNormal;

// material functions
IMPL_GET_PUSH_CONSTANT_BASE()
{
    return pushConstant.push_constant_base;
}

IMPL_GET_TEXCOORD()
{
    return texcoord;
}

IMPL_GET_BASE_COLOR()
{
    vec4 base_color = texture(textureBase, texcoord);
    base_color.xyz = pow(base_color.xyz, vec3(2.2));
    return base_color;
}

IMPL_GET_MATERIAL()
{
    return texture(textureMaterial, texcoord);
}

IMPL_GET_TANGENT_NORMAL()
{
    return (texture(textureNormal, texcoord).xyz * 2.0 - 1.0);
}

IMPL_GET_WORLD_OFFSET()
{
    return vec3(0.0); // vec3(pow(vertex_position.y, 2.0) * sin(scene_constants.TIME + random(local_latrix[3].xyz) * 13.1423), 0.0, 0.0);
}