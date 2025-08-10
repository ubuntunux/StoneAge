use ash::vk;
use serde::{Deserialize, Serialize};
use rust_engine_3d::render_pass::render_object::common;
use rust_engine_3d::render_pass::render_object::common::{USER_BINDING_INDEX0, USER_BINDING_INDEX1, USER_BINDING_INDEX2};
use rust_engine_3d::renderer::push_constants::{PushConstant, PushConstantName, PushConstantParameter, PushConstant_RenderObjectBase};
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::vulkan_context::descriptor::{DescriptorDataCreateInfo, DescriptorResourceType};
use rust_engine_3d::vulkan_context::render_pass::RenderPassDataCreateInfo;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_VegetationShader {
    pub _push_constant_base: PushConstant_RenderObjectBase
}

impl Default for PushConstant_VegetationShader {
    fn default() -> PushConstant_VegetationShader {
        PushConstant_VegetationShader {
            _push_constant_base: PushConstant_RenderObjectBase::default()
        }
    }
}

impl PushConstantName for PushConstant_VegetationShader {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_VegetationShader"
    }
}

impl PushConstant for PushConstant_VegetationShader {
    fn set_push_constant_parameter(&mut self, key: &str, value: &PushConstantParameter) -> bool {
        self._push_constant_base.set_push_constant_parameter(key, value)
    }
}

pub fn get_push_constant_data() -> Box<dyn PushConstant> {
    Box::new(PushConstant_VegetationShader::default())
}

pub fn get_descriptor_data_create_infos() -> Vec<DescriptorDataCreateInfo> {
    vec![
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX0,
            _descriptor_name: String::from("_LeafTex"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX1,
            _descriptor_name: String::from("textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX2,
            _descriptor_name: String::from("_LeafNormalMap"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        }
    ]
}

pub fn get_render_pass_data_create_infos(renderer_data: &RendererData) -> Vec<RenderPassDataCreateInfo> {
    common::get_render_pass_data_create_infos(
        renderer_data,
        "PolygonNatureBiomes_VegetationShader",
        "PolygonNatureBiomes/VegetationShader.vert",
        "PolygonNatureBiomes/VegetationShader.frag",
        &get_push_constant_data(),
        &get_descriptor_data_create_infos()
    )
}