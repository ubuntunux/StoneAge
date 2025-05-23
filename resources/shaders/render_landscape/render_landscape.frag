#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_GOOGLE_include_directive : enable

// shader predefined
#include "../../engine_resources/shaders/render_object/render_object_common.glsl"

// user defined shader
#include "render_landscape.glsl"

// shader entry point
#include "../../engine_resources/shaders/render_object/render_object_common.frag"