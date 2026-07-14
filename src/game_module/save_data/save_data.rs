use crate::game_module::actors::character_manager::CharacterCreateInfo;
use crate::game_module::game_scene_manager::{GameSceneSaveData, ScenarioSaveDataList};
use crate::game_module::scenario::scenario::ScenarioType;
use crate::game_module::widgets::item_bar_widget::InventoryItemCreateInfoList;
use rust_engine_3d::scene::camera::CameraCreateInfo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
    pub _completed_game_scenarios: HashSet<ScenarioType>,
}
