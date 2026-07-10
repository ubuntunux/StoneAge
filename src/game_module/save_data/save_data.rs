use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rust_engine_3d::scene::camera::CameraCreateInfo;
use crate::game_module::actors::character_manager::{CharacterCreateInfo, CharacterID};
use crate::game_module::actors::items::{ItemCreateInfo, ItemID};
use crate::game_module::actors::props::{PropCreateInfo, PropID};
use crate::game_module::scenario::scenario::ScenarioType;
use crate::game_module::widgets::item_bar_widget::InventoryItemCreateInfo;

pub type CharacterCreateInfoIDMap = HashMap<CharacterID, CharacterCreateInfo>;
pub type ItemCreateInfoIDMap = HashMap<ItemID, ItemCreateInfo>;
pub type PropCreateInfoIDMap = HashMap<PropID, PropCreateInfo>;
pub type ScenarioSaveDataList = Vec<GameScenarioSaveData>;
pub type InventoryItemCreateInfoList = HashMap<usize, Vec<InventoryItemCreateInfo>>;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSaveData {
    pub _player: CharacterCreateInfo,
    pub _camera: CameraCreateInfo,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _inventory_item_create_info_list: InventoryItemCreateInfoList,
    pub _last_game_scene_data_name: String,
    pub _game_scenes: HashMap<String, GameSceneSaveData>,
    pub _game_scenarios: ScenarioSaveDataList,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneSaveData {
    pub _characters: CharacterCreateInfoIDMap,
    pub _items: ItemCreateInfoIDMap,
    pub _props: PropCreateInfoIDMap
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
 pub struct GameScenarioSaveData {
     pub _scenario_type: ScenarioType,
     pub _scenario_phase: String,
 }