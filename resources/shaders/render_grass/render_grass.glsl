#include "../../engine_resources/shaders/common/random.glsl"

layout( push_constant ) uniform PushConstant_RenderGrass
{
    PushConstant_RenderObjectBase push_constant_base;
} pushConstant;

// bindings
layout(binding = USER_BINDING_INDEX0) uniform sampler2D textureBase;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D textureMaterial;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D textureNormal;

IMPL_GET_PUSH_CONSTANT_BASE()
{
    return pushConstant.push_constant_base;
}

#if SHADER_STAGE_FLAG == VERTEX
IMPL_GET_WORLD_OFFSET()
{
    return vec3(0.0); // vec3(pow(vertex_position.y, 2.0) * sin(scene_constants.TIME + random(local_latrix[3].xyz) * 13.1423), 0.0, 0.0);
}

#elif SHADER_STAGE_FLAG == FRAGMENT
struct UserData
{
    vec2 texcoord;
} user_data;

IMPL_INITALIZE_USER_DATA()
{
    user_data.texcoord = vs_output.texCoord;
}

IMPL_GET_BASE_COLOR()
{
    vec4 base_color = texture(textureBase, user_data.texcoord);
    base_color.xyz = pow(base_color.xyz, vec3(2.2));
    return base_color;
}

IMPL_GET_MATERIAL()
{
    return texture(textureMaterial, user_data.texcoord);
}

IMPL_GET_TANGENT_NORMAL()
{
    return (texture(textureNormal, user_data.texcoord).xyz * 2.0 - 1.0);
}
#endif
