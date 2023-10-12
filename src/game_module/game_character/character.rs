use serde::{ Serialize, Deserialize };

use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    AnkyloSaurus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _max_hp: i32,
}

pub struct CharacterProperty {
    pub _hp: f32,
}

pub struct CharacterController {
    pub _transform_object: TransformObjectData,
}

pub struct Character {
    pub _character_name: String,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _character_property: CharacterProperty,
    pub _controller: CharacterController,
}