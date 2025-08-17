use ash::vk;
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use rust_engine_3d::render_pass::render_object::common;
use rust_engine_3d::render_pass::render_object::common::{USER_BINDING_INDEX0, USER_BINDING_INDEX1, USER_BINDING_INDEX2, USER_BINDING_INDEX3};
use rust_engine_3d::renderer::push_constants::{PushConstant, PushConstantName, PushConstantParameter, PushConstant_RenderObjectBase};
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::vulkan_context::descriptor::{DescriptorDataCreateInfo, DescriptorResourceType};
use rust_engine_3d::vulkan_context::render_pass::RenderPassDataCreateInfo;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_TriplanarBasic {
    pub _push_constant_base: PushConstant_RenderObjectBase,
    pub _Color: Vector4<f32>,
    pub _Glossiness: f32,
    pub _Metallic: f32,
    pub _Tiling: f32,
    pub _TopScaleX: f32,
    pub _TopScaleY: f32,
    pub _TopNormalScaleX: f32,
    pub _TopNormalScaleY: f32,
    pub _SidesScaleX: f32,
    pub _SidesScaleY: f32,
    pub _SidesNormalScaleX: f32,
    pub _SidesNormalScaleY: f32,
    pub _reserved0: f32
}

impl Default for PushConstant_TriplanarBasic {
    fn default() -> PushConstant_TriplanarBasic {
        PushConstant_TriplanarBasic {
            _push_constant_base: PushConstant_RenderObjectBase::default(),
            _Color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            _Glossiness: 0.0,
            _Metallic: 0.0,
            _Tiling: 1.0,
            _TopScaleX: 1.0,
            _TopScaleY: 1.0,
            _TopNormalScaleX: 1.0,
            _TopNormalScaleY: 1.0,
            _SidesScaleX: 1.0,
            _SidesScaleY: 1.0,
            _SidesNormalScaleX: 1.0,
            _SidesNormalScaleY: 1.0,
            _reserved0: 0.0,
        }
    }
}

impl PushConstantName for PushConstant_TriplanarBasic {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_TriplanarBasic"
    }
}

impl PushConstant for PushConstant_TriplanarBasic {
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
        } else if "_Tiling" == key {
            if let PushConstantParameter::Float(value) = value {
                self._Tiling = *value;
            }
        } else if "_TopScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TopScaleX = *value;
            }
        } else if "_TopScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TopScaleY = *value;
            }
        } else if "_TopNormalScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TopNormalScaleX = *value;
            }
        } else if "_TopNormalScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TopNormalScaleY = *value;
            }
        } else if "_SidesScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._SidesScaleX = *value;
            }
        } else if "_SidesScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._SidesScaleY = *value;
            }
        } else if "_SidesNormalScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._SidesNormalScaleX = *value;
            }
        } else if "_SidesNormalScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._SidesNormalScaleY = *value;
            }
        } else {
            return self._push_constant_base.set_push_constant_parameter(key, value);
        }
        true
    }
}

pub fn get_push_constant_data() -> Box<dyn PushConstant> {
    Box::new(PushConstant_TriplanarBasic::default())
}

pub fn get_descriptor_data_create_infos() -> Vec<DescriptorDataCreateInfo> {
    vec![
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX0,
            _descriptor_name: String::from("_Top"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX1,
            _descriptor_name: String::from("_TopNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX2,
            _descriptor_name: String::from("_Sides"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX3,
            _descriptor_name: String::from("_SidesNormal"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        }
    ]
}

pub fn get_render_pass_data_create_infos(renderer_data: &RendererData) -> Vec<RenderPassDataCreateInfo> {
    common::get_render_pass_data_create_infos(
        renderer_data,
        "PolygonNatureBiomes_TriplanarBasic",
        "PolygonNatureBiomes/TriplanarBasic.vert",
        "PolygonNatureBiomes/TriplanarBasic.frag",
        &get_push_constant_data(),
        &get_descriptor_data_create_infos()
    )
}