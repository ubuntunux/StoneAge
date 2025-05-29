#include "../../engine_resources/shaders/common/random.glsl"

layout(binding = USER_BINDING_INDEX0) uniform sampler2D textureBase;
layout(binding = USER_BINDING_INDEX1) uniform sampler2D textureMaterial;
layout(binding = USER_BINDING_INDEX2) uniform sampler2D textureNormal;

BEGIN_PUSH_CONSTANT(PushConstant_RenderGrass)
END_PUSH_CONSTANT()

#if SHADER_STAGE_FLAG == VERTEX
VERTEX_SHADER_MAIN()
{
    // out_world_offset = vec3(pow(in_relative_position.y - in_local_latrix[3].y, 2.0) * sin(scene_constants.TIME + random(in_local_latrix[3].xyz) * 13.1423), 0.0, 0.0);
}

#elif SHADER_STAGE_FLAG == FRAGMENT
FRAGMENT_SHADER_MAIN()
{
    out_base_color = texture(textureBase, in_vertex_output.texCoord);
    out_base_color.xyz = pow(out_base_color.xyz, vec3(2.2));
    out_material = texture(textureMaterial, in_vertex_output.texCoord);
    out_tangent_normal = texture(textureNormal, in_vertex_output.texCoord).xyz * 2.0 - 1.0;
}
#endif