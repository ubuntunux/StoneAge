use std::collections::HashMap;

use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use crate::game_module::actors::item_updater::ItemUpdaterBase;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type ItemMap<'a> = HashMap<u64, RcRefCell<Item<'a>>>;

#[derive(Serialize, Deserialize, Hash, Eq, Clone, Copy, Debug, EnumIter, Display, PartialEq)]
pub enum ItemDataType {
    None,
    Meat,
    Rock,
    Wood,
    SpiritBall
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ItemCreateInfo {
    pub _item_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ItemData {
    pub _item_type: ItemDataType,
    pub _model_data_name: String,
    pub _name: String,
}

pub struct ItemProperties {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_on_ground: bool,
}

pub struct Item<'a> {
    pub _item_name: String,
    pub _item_id: u64,
    pub _item_data: RcRefCell<ItemData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _item_properties: Box<ItemProperties>,
    pub _item_updater: Box<dyn ItemUpdaterBase>,
}

pub struct ItemManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _id_generator: u64,
    pub _items: ItemMap<'a>,
}
