use ash::vk;
use nalgebra::Vector4;
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
pub struct PushConstant_Standard {
    pub _push_constant_base: PushConstant_RenderObjectBase,
    pub _Color: Vector4<f32>,
    pub _Glossiness: f32,
    pub _Metallic: f32,
    pub _MainTexScaleX: f32,
    pub _MainTexScaleY: f32,
    pub _BumpMapScaleX: f32,
    pub _BumpMapScaleY: f32,
    pub _EmissionMapScaleX: f32,
    pub _EmissionMapScaleY: f32
}

impl Default for PushConstant_Standard {
    fn default() -> PushConstant_Standard {
        PushConstant_Standard {
            _push_constant_base: PushConstant_RenderObjectBase::default(),
            _Color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            _Glossiness: 0.0,
            _Metallic: 0.0,
            _MainTexScaleX: 1.0,
            _MainTexScaleY: 1.0,
            _BumpMapScaleX: 1.0,
            _BumpMapScaleY: 1.0,
            _EmissionMapScaleX: 1.0,
            _EmissionMapScaleY: 1.0,
        }
    }
}

impl PushConstantName for PushConstant_Standard {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_Standard"
    }
}

impl PushConstant for PushConstant_Standard {
    fn set_push_constant_parameter(&mut self, key: &str, value: &PushConstantParameter) -> bool {
        if "_Color" == key {
            if let PushConstantParameter::Float4(value) = value {
                self._Color = *value;
            }
        } else if "_Glossiness" == key {
            if let PushConstantParameter::Float(value) = value {
                self._Glossiness = *value;
            }
        } else if "_Metallic" == key {
            if let PushConstantParameter::Float(value) = value {
                self._Metallic = *value;
            }
        } else if "_MainTexScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._MainTexScaleX = *value;
            }
        } else if "_MainTexScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._MainTexScaleY = *value;
            }
        } else if "_BumpMapScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._BumpMapScaleX = *value;
            }
        } else if "_BumpMapScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._BumpMapScaleY = *value;
            }
        } else if "_EmissionMapScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._EmissionMapScaleX = *value;
            }
        } else if "_EmissionMapScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._EmissionMapScaleY = *value;
            }
        } else {
            return self._push_constant_base.set_push_constant_parameter(key, value);
        }
        true
    }
}

pub fn get_push_constant_data() -> Box<dyn PushConstant> {
    Box::new(PushConstant_Standard::default())
}

pub fn get_descriptor_data_create_infos() -> Vec<DescriptorDataCreateInfo> {
    vec![
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX0,
            _descriptor_name: String::from("_MainTex"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX1,
            _descriptor_name: String::from("_EmissionMap"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX2,
            _descriptor_name: String::from("_BumpMap"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        }
    ]
}

pub fn get_render_pass_data_create_infos(renderer_data: &RendererData) -> Vec<RenderPassDataCreateInfo> {
    common::get_render_pass_data_create_infos(
        renderer_data,
        vk::CullModeFlags::BACK,
        "PolygonNatureBiomes_Standard",
        "PolygonNatureBiomes/Standard.vert",
        "PolygonNatureBiomes/Standard.frag",
        &get_push_constant_data(),
        &get_descriptor_data_create_infos()
    )
}