use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::renderer::renderer_data::RenderObjectType;
use rust_engine_3d::resource::resource::RenderPassDataCreateInfoMap;

use crate::render_pass::{capture_height_map, depth_prepass, render_forward, render_forward_for_light_probe, render_gbuffer, render_shadow};

pub fn get_render_pass_data_create_infos(
    _renderer_context: &RendererContext,
    render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap,
) {
    depth_prepass::get_render_pass_data_create_info(
        RenderObjectType::Static,
        render_pass_data_create_info_map
    );

    render_forward::get_render_pass_data_create_info(
        RenderObjectType::Static,
        render_pass_data_create_info_map,
    );
    render_gbuffer::get_render_pass_data_create_info(
        RenderObjectType::Static,
        render_pass_data_create_info_map,
    );
    render_shadow::get_render_pass_data_create_info(
        RenderObjectType::Static,
        render_pass_data_create_info_map,
    );

    for layer in 0..6 {
        render_forward_for_light_probe::get_render_pass_data_create_info(
            RenderObjectType::Static,
            render_pass_data_create_info_map,
            layer,
        );
    }

    capture_height_map::get_render_pass_data_create_info(
        RenderObjectType::Static,
        render_pass_data_create_info_map
    );
}
