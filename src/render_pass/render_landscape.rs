use ash::vk;
use serde::{Deserialize, Serialize};
use rust_engine_3d::render_pass::render_object::common;
use rust_engine_3d::renderer::push_constants::{PushConstant, PushConstantName, PushConstantParameter, PushConstant_RenderObjectBase};
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::vulkan_context::descriptor::{DescriptorDataCreateInfo, DescriptorResourceType};
use rust_engine_3d::vulkan_context::render_pass::RenderPassDataCreateInfo;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_RenderLandscape {
    pub _push_constant_base: PushConstant_RenderObjectBase
}

impl Default for PushConstant_RenderLandscape {
    fn default() -> PushConstant_RenderLandscape {
        PushConstant_RenderLandscape {
            _push_constant_base: PushConstant_RenderObjectBase::default()
        }
    }
}

impl PushConstantName for PushConstant_RenderLandscape {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_RenderLandscape"
    }
}

impl PushConstant for PushConstant_RenderLandscape {
    fn set_push_constant_parameter(&mut self, key: &str, value: &PushConstantParameter) -> bool {
        self._push_constant_base.set_push_constant_parameter(key, value)
    }

    fn update_material_parameters(&mut self, material_parameters: &serde_json::Map<String, serde_json::Value>) -> bool {
        self._push_constant_base.update_material_parameters(material_parameters)
    }
}

pub fn get_push_constant_data() -> Box<dyn PushConstant> {
    Box::new(PushConstant_RenderLandscape::default())
}

pub fn get_descriptor_data_create_infos() -> Vec<DescriptorDataCreateInfo> {
    vec![
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX0,
            _descriptor_name: String::from("textureLayerMask"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX1,
            _descriptor_name: String::from("layer0_textureBase"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX2,
            _descriptor_name: String::from("layer0_textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX3,
            _descriptor_name: String::from("layer0_textureNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX4,
            _descriptor_name: String::from("layer1_textureBase"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX5,
            _descriptor_name: String::from("layer1_textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX6,
            _descriptor_name: String::from("layer1_textureNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX7,
            _descriptor_name: String::from("layer2_textureBase"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX8,
            _descriptor_name: String::from("layer2_textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX9,
            _descriptor_name: String::from("layer2_textureNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX10,
            _descriptor_name: String::from("layer3_textureBase"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX11,
            _descriptor_name: String::from("layer3_textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX12,
            _descriptor_name: String::from("layer3_textureNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX13,
            _descriptor_name: String::from("layer4_textureBase"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX14,
            _descriptor_name: String::from("layer4_textureMaterial"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: common::USER_BINDING_INDEX15,
            _descriptor_name: String::from("layer4_textureNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        }
    ]
}

pub fn get_render_pass_data_create_infos(renderer_data: &RendererData) -> Vec<RenderPassDataCreateInfo> {
    common::get_render_pass_data_create_infos(
        renderer_data,
        "render_landscape",
        "render_landscape/render_landscape.vert",
        "render_landscape/render_landscape.frag",
        &get_push_constant_data(),
        &get_descriptor_data_create_infos()
    )
}