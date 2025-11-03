use ash::vk;
use nalgebra::Vector4;
use rust_engine_3d::render_pass::render_object::common;
use rust_engine_3d::render_pass::render_object::common::{
    USER_BINDING_INDEX0, USER_BINDING_INDEX1, USER_BINDING_INDEX2, USER_BINDING_INDEX3,
    USER_BINDING_INDEX4, USER_BINDING_INDEX5,
};
use rust_engine_3d::renderer::push_constants::{
    PushConstant, PushConstantName, PushConstantParameter, PushConstant_RenderObjectBase,
};
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::vulkan_context::descriptor::{
    DescriptorDataCreateInfo, DescriptorResourceType,
};
use rust_engine_3d::vulkan_context::render_pass::RenderPassDataCreateInfo;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_VegetationShader {
    pub _push_constant_base: PushConstant_RenderObjectBase,
    pub _BaseColour: Vector4<f32>,
    pub _EmissiveColour: Vector4<f32>,
    pub _TrunkBaseColour: Vector4<f32>,
    pub _GustFreq: f32,
    pub _GustLargeFreq: f32,
    pub _LeafSmoothness: f32,
    pub _TrunkSmoothness: f32,
    pub _EmissiveMaskScaleX: f32,
    pub _EmissiveMaskScaleY: f32,
    pub _GustNoiseMapScaleX: f32,
    pub _GustNoiseMapScaleY: f32,
    pub _LeafNormalMapScaleX: f32,
    pub _LeafNormalMapScaleY: f32,
    pub _LeafTexScaleX: f32,
    pub _LeafTexScaleY: f32,
    pub _TrunkNormalMapScaleX: f32,
    pub _TrunkNormalMapScaleY: f32,
    pub _TunkTexScaleX: f32,
    pub _TunkTexScaleY: f32,
}

impl Default for PushConstant_VegetationShader {
    fn default() -> PushConstant_VegetationShader {
        PushConstant_VegetationShader {
            _push_constant_base: PushConstant_RenderObjectBase::default(),
            _BaseColour: Vector4::new(1.0, 1.0, 1.0, 1.0),
            _EmissiveColour: Vector4::new(0.0, 0.0, 0.0, 1.0),
            _TrunkBaseColour: Vector4::new(1.0, 1.0, 1.0, 1.0),
            _GustFreq: 0.0,
            _GustLargeFreq: 0.0,
            _LeafSmoothness: 0.0,
            _TrunkSmoothness: 0.0,
            _EmissiveMaskScaleX: 1.0,
            _EmissiveMaskScaleY: 1.0,
            _GustNoiseMapScaleX: 1.0,
            _GustNoiseMapScaleY: 1.0,
            _LeafNormalMapScaleX: 1.0,
            _LeafNormalMapScaleY: 1.0,
            _LeafTexScaleX: 1.0,
            _LeafTexScaleY: 1.0,
            _TrunkNormalMapScaleX: 1.0,
            _TrunkNormalMapScaleY: 1.0,
            _TunkTexScaleX: 1.0,
            _TunkTexScaleY: 1.0,
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
        if "_BaseColour" == key {
            if let PushConstantParameter::Float4(value) = value {
                self._BaseColour = *value;
            }
        } else if "_EmissiveColour" == key {
            if let PushConstantParameter::Float4(value) = value {
                self._EmissiveColour = *value;
            }
        } else if "_TrunkBaseColour" == key {
            if let PushConstantParameter::Float4(value) = value {
                self._TrunkBaseColour = *value;
            }
        } else if "_GustFreq" == key {
            if let PushConstantParameter::Float(value) = value {
                self._GustFreq = *value;
            }
        } else if "_GustLargeFreq" == key {
            if let PushConstantParameter::Float(value) = value {
                self._GustLargeFreq = *value;
            }
        } else if "_LeafSmoothness" == key {
            if let PushConstantParameter::Float(value) = value {
                self._LeafSmoothness = *value;
            }
        } else if "_TrunkSmoothness" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TrunkSmoothness = *value;
            }
        } else if "_EmissiveMaskScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._EmissiveMaskScaleX = *value;
            }
        } else if "_EmissiveMaskScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._EmissiveMaskScaleY = *value;
            }
        } else if "_GustNoiseMapScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._GustNoiseMapScaleX = *value;
            }
        } else if "_GustNoiseMapScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._GustNoiseMapScaleY = *value;
            }
        } else if "_LeafNormalMapScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._LeafNormalMapScaleX = *value;
            }
        } else if "_LeafNormalMapScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._LeafNormalMapScaleY = *value;
            }
        } else if "_LeafTexScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._LeafTexScaleX = *value;
            }
        } else if "_LeafTexScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._LeafTexScaleY = *value;
            }
        } else if "_TrunkNormalMapScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TrunkNormalMapScaleX = *value;
            }
        } else if "_TrunkNormalMapScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TrunkNormalMapScaleY = *value;
            }
        } else if "_TunkTexScaleX" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TunkTexScaleX = *value;
            }
        } else if "_TunkTexScaleY" == key {
            if let PushConstantParameter::Float(value) = value {
                self._TunkTexScaleY = *value;
            }
        } else {
            return self
                ._push_constant_base
                .set_push_constant_parameter(key, value);
        }
        true
    }
}

pub fn get_push_constant_data() -> Box<dyn PushConstant> {
    Box::new(PushConstant_VegetationShader::default())
}

pub fn get_descriptor_data_create_infos() -> Vec<DescriptorDataCreateInfo> {
    vec![
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX0,
            _descriptor_name: String::from("_EmissiveMask"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX1,
            _descriptor_name: String::from("_GustNoiseMap"),
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
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX3,
            _descriptor_name: String::from("_LeafTex"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX4,
            _descriptor_name: String::from("_TrunkNormalMap"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
        DescriptorDataCreateInfo {
            _descriptor_binding_index: USER_BINDING_INDEX5,
            _descriptor_name: String::from("_TunkTex"),
            _descriptor_resource_type: DescriptorResourceType::Texture,
            _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
    ]
}

pub fn get_render_pass_data_create_infos(
    renderer_data: &RendererData,
) -> Vec<RenderPassDataCreateInfo> {
    common::get_render_pass_data_create_infos(
        renderer_data,
        vk::CullModeFlags::NONE,
        "PolygonNatureBiomes_VegetationShader",
        "PolygonNatureBiomes/VegetationShader.vert",
        "PolygonNatureBiomes/VegetationShader.frag",
        &get_push_constant_data(),
        &get_descriptor_data_create_infos(),
    )
}
