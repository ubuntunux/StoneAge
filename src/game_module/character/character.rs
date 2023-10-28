use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SpawnPointData {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>
}

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    UrsusArctos,
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
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Character {
    pub _character_name: String,
    pub _character_id: u64,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _character_property: Box<CharacterProperty>,
    pub _controller: Box<CharacterController>,
}