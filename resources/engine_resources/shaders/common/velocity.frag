#include "utility.glsl"
#include "scene_constants.glsl"
#include "quad.glsl"

uniform sampler2D texture_depth;

layout (location = 0) in VERTEX_OUTPUT vs_output;
layout (location = 0) out vec2 fs_output;

void main() {
    vec2 tex_coord = vs_output.tex_coord.xy;
    float depth = texture2D(texture_depth, tex_coord).x;

    vec4 world_pos = relative_world_from_device_depth(view_constants.INV_VIEW_ORIGIN_PROJECTION, tex_coord.xy, depth);
    world_pos.xyz += view_constants.CAMERA_POSITION;
    world_pos.w = 1.0;

    vec4 clip_coord_prev = PREV_VIEW_PROJECTION * world_pos;
    clip_coord_prev.xyz /= clip_coord_prev.w;

    vec2 tex_coord_prev = clip_coord_prev.xy * 0.5 + 0.5;
    fs_output.xy = tex_coord - tex_coord_prev;

    // jitter offset
    fs_output.xy -= JITTER_DELTA;
}