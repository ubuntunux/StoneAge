use std::collections::HashMap;

use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub type WeaponMap<'a> = HashMap<u64, RcRefCell<Weapon<'a>>>;

#[derive(Serialize, Deserialize, Hash, Eq, Clone, Copy, Debug, EnumIter, Display, PartialEq)]
pub enum WeaponDataType {
    WoodenClub
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponCreateInfo {
    pub _weapon_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponData {
    pub _weapon_type: WeaponDataType,
    pub _model_data_name: String,
}

pub struct WeaponProperties {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Weapon<'a> {
    pub _weapon_name: String,
    pub _weapon_id: u64,
    pub _weapon_data: RcRefCell<WeaponData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _weapon_properties: Box<WeaponProperties>,
}