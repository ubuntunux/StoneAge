#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : enable

#include "scene_constants.glsl"
#include "utility.glsl"
#include "PCFKernels.glsl"
#include "shading.glsl"
#include "render_quad_common.glsl"

layout(binding = 0) uniform SceneConstants
{
    SCENE_CONSTANTS scene_constants;
};
layout(binding = 1) uniform ViewConstants
{
    VIEW_CONSTANTS view_constants;
};
layout(binding = 2) uniform LightData
{
    LIGHT_DATA light_data;
};
layout(binding = 3) uniform sampler2D textureSceneNormal;
layout(binding = 4) uniform sampler2D textureSceneDepth;
layout(binding = 5) uniform sampler2D ssaoNoise;
layout(binding = 6) uniform sampler2D texture_shadow;

// uniform_buffer_data - struct ShadowAOConstants
layout(binding = 7) uniform ShadowAOConstants
{
    vec4 SSAO_KERNEL_SAPLES[SSAO_KERNEL_SIZE];
} uboSSAOKernel;

layout(location = 0) in VERTEX_OUTPUT vs_output;

layout(location = 0) out float outColor;


void main() {
    outColor = 1;

    vec2 texCoord = vs_output.texCoord;
    float device_depth = texture(textureSceneDepth, texCoord).x;
    if(0.0 == device_depth)
    {
        return;
    }

    const vec2 inv_depth_size = 1.0 / textureSize(textureSceneDepth, 0).xy;
    const ivec2 screen_pos = ivec2(vs_output.texCoord * scene_constants.SCREEN_SIZE);
    const int timeIndex = int(mod(scene_constants.TIME, 1.0) * 65535.0);

    const vec4 relative_pos = relative_world_from_device_depth(view_constants.INV_VIEW_ORIGIN_PROJECTION_JITTER, texCoord, device_depth);
    const vec3 world_position = relative_pos.xyz + view_constants.CAMERA_POSITION;
    const vec3 normal = normalize(texture(textureSceneNormal, texCoord).xyz * 2.0 - 1.0);
    const vec3 randomVec = normalize(texture(ssaoNoise, texCoord).xyz * 2.0 - 1.0);
    const vec3 tangent = normalize(randomVec - normal * dot(randomVec, normal));
    const vec3 bitangent = normalize(cross(normal, tangent));
    const mat3 tnb = mat3(tangent, normal, bitangent);

    const float occlusion_distance_min = 0.05;
    const float occlusion_distance_max = 5.0;
    const float ssao_contrast = 1.0;

    const vec2 noise = vec2(interleaved_gradient_noise(screen_pos + timeIndex) * 2.0 - 1.0) * inv_depth_size * 2.0;
    const int sample_count = SSAO_KERNEL_SIZE;
    const float occlusion_factor = 4.0 * sample_count / 64;
    float acc_occlusion = 1.0;
    for (int i = 0; i < sample_count; ++i)
    {
        // ray
        const float t = float(i) / float(sample_count - 1);
        const float occlusion_distance = mix(occlusion_distance_min, occlusion_distance_max, t*t);
        float kernel_radius = float(i + 1) / float(sample_count);
        vec3 ray = normalize(uboSSAOKernel.SSAO_KERNEL_SAPLES[i].xyz * vec3(kernel_radius, 1.0, kernel_radius));
        vec3 pos = normalize(tnb * ray) * occlusion_distance + relative_pos.xyz;

        // project sample position:
        vec4 offset = vec4(pos, 1.0);
        offset = view_constants.VIEW_ORIGIN_PROJECTION_JITTER * offset;
        offset.xyz /= offset.w;
        offset.xy = offset.xy * 0.5 + 0.5 + noise;

        if(offset.x < 0.0 || offset.x > 1.0 || offset.y < 0.0 || offset.y > 1.0 || 1.0 < offset.z)
        {
            break;
        }

        const float occlusion_depth = texture(textureSceneDepth, offset.xy).x;
        if(occlusion_depth <= offset.z)
        {
            continue;
        }

        const vec4 occlusion_relative_pos = relative_world_from_device_depth(view_constants.INV_VIEW_ORIGIN_PROJECTION_JITTER, offset.xy, occlusion_depth);
        const float distance = length(occlusion_relative_pos - relative_pos);
        float occlusion = 1.0 - exp(-distance * occlusion_factor);
        acc_occlusion *= occlusion;
    }

    float shadow_factor = get_shadow_factor(
        scene_constants.TIME,
        screen_pos,
        world_position,
        light_data.SHADOW_VIEW_PROJECTION,
        light_data.INV_SHADOW_VIEW_PROJECTION,
        light_data.SHADOW_SAMPLES,
        SHADOW_BIAS,
        texture_shadow
    );

    outColor = saturate(1 * shadow_factor);
}