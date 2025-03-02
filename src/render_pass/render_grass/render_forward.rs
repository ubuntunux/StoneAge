use std::path::PathBuf;

use ash::vk;
use rust_engine_3d::render_pass::common::render_forward;
use rust_engine_3d::renderer::renderer_data::RenderObjectType;
use rust_engine_3d::resource::resource::RenderPassDataCreateInfoMap;
use rust_engine_3d::vulkan_context::render_pass::PipelinePushConstantData;
use crate::render_pass::render_grass::push_constants::PushConstant_RenderGrass;

pub fn get_render_pass_data_create_info(
    render_object_type: RenderObjectType,
    render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap,
) {
    let render_pass_name = render_forward::get_render_pass_name(render_object_type);
    let render_pass_data_create_info = render_pass_data_create_info_map.get_mut(render_pass_name).unwrap();
    let mut pipeline_data_create_info = render_pass_data_create_info.get_pipeline_data_create_info("render_object").clone();
    pipeline_data_create_info._pipeline_data_create_info_name = String::from("render_grass");
    pipeline_data_create_info._pipeline_vertex_shader_file = PathBuf::from("render_grass.vert");
    pipeline_data_create_info._pipeline_fragment_shader_file = PathBuf::from("render_grass.frag");
    pipeline_data_create_info._push_constant_data_list = vec![PipelinePushConstantData {
        _stage_flags: vk::ShaderStageFlags::ALL,
        _offset: 0,
        _push_constant: Box::new(PushConstant_RenderGrass::default()),
    }];
    render_pass_data_create_info
        ._pipeline_data_create_infos
        .push(pipeline_data_create_info);
}
