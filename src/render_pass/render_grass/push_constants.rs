use std::fmt::Debug;

use nalgebra::Vector4;
use rust_engine_3d::renderer::push_constants::{
    PushConstant, PushConstantName, PushConstantParameter,
};
use rust_engine_3d::utilities::json::convert_json_value_to_push_constant_parameter;
use serde::{Deserialize, Serialize};
use serde_json;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_RenderGrass {
    pub _transform_offset_index: u32,
    pub _bone_count: u32,
    pub _reserved0: u32,
    pub _reserved1: u32,
    pub _color: Vector4<f32>,
}

impl Default for PushConstant_RenderGrass {
    fn default() -> PushConstant_RenderGrass {
        PushConstant_RenderGrass {
            _transform_offset_index: 0,
            _bone_count: 0,
            _reserved0: 0,
            _reserved1: 0,
            _color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl PushConstantName for PushConstant_RenderGrass {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_RenderGrass"
    }
}

impl PushConstant for PushConstant_RenderGrass {
    fn set_push_constant_parameter(&mut self, key: &str, value: &PushConstantParameter) {
        if "_transform_offset_index" == key {
            if let PushConstantParameter::Int(transform_offset_index) = value {
                self._transform_offset_index = *transform_offset_index as u32;
            }
        } else if "_bone_count" == key {
            if let PushConstantParameter::Int(bone_count) = value {
                self._bone_count = *bone_count as u32;
            }
        } else {
            panic!("Not implemented for {:?}", key);
        }
    }

    fn update_material_parameters(
        &mut self,
        material_parameters: &serde_json::Map<String, serde_json::Value>,
    ) {
        if let PushConstantParameter::Float4(value) =
            convert_json_value_to_push_constant_parameter(material_parameters, "_color")
        {
            self._color = value;
        }
    }
}