use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3};
use rust_engine_3d::scene::model::ModelData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::socket::Socket;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub type WeaponMap<'a> = HashMap<u64, RcRefCell<Weapon<'a>>>;

#[derive(Serialize, Deserialize, Hash, Eq, Clone, Copy, Debug, EnumIter, Display, PartialEq)]
pub enum WeaponDataType {
    None,
    WoodenClub
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponDataCreateInfo {
    pub _damage: f32,
    pub _model_data_name: String,
    pub _weapon_data_type: WeaponDataType
}

pub struct WeaponData<'a> {
    pub _damage: f32,
    pub _model_data: RcRefCell<ModelData<'a>>,
    pub _weapon_data_type: WeaponDataType
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponCreateInfo {
    pub _weapon_socket_name: String,
    pub _weapon_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>
}

pub struct Weapon<'a> {
    pub _weapon_socket: RcRefCell<Socket>,
    pub _weapon_data: RcRefCell<WeaponData<'a>>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _transform: Matrix4<f32>
}