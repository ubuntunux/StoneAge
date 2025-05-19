use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::RenderPassDataCreateInfoMap;
use crate::render_pass::render_grass;

pub fn get_render_pass_data_create_infos(
    renderer_context: &RendererContext,
    render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap,
) {
    let render_grass_render_pass_data_create_infos = render_grass::get_render_pass_data_create_infos(renderer_context.get_renderer_data());
    for render_grass_render_pass_data_create_info in render_grass_render_pass_data_create_infos.iter() {
        let render_pass_data_create_info = render_pass_data_create_info_map.get_mut(&render_grass_render_pass_data_create_info._render_pass_create_info_name).unwrap();
        for pipeline_data_create_info in render_grass_render_pass_data_create_info._pipeline_data_create_infos.iter() {
            render_pass_data_create_info._pipeline_data_create_infos.push(pipeline_data_create_info.clone());
        }
    }
}
