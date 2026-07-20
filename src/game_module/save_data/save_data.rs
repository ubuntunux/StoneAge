use crate::game_module::actors::character_manager::CharacterSaveData;
use crate::game_module::game_scene_manager::GameSceneSaveData;
use crate::game_module::scenario::scenario::{GameScenarioCreateInfo, ScenarioType};
use crate::game_module::widgets::item_bar_widget::InventoryItemCreateInfoList;
use rust_engine_3d::scene::camera::CameraCreateInfo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GameSaveData {
    pub _player: Option<(String, CharacterSaveData)>,
    pub _camera: CameraCreateInfo,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _inventory_item_create_info_list: InventoryItemCreateInfoList,
    pub _selected_inventory_item_index: usize,
    pub _last_game_scene_data_name: String,
    pub _game_scenes: HashMap<String, GameSceneSaveData>,
    pub _game_scenarios: Vec<GameScenarioCreateInfo>,
    pub _completed_game_scenarios: HashSet<ScenarioType>,
}

impl Default for GameSaveData {
    fn default() -> GameSaveData {
        GameSaveData {
            _player: None,
            _camera: Default::default(),
            _time_of_day: 0.0,
            _temperature: 0.0,
            _date: 0,
            _inventory_item_create_info_list: Default::default(),
            _selected_inventory_item_index: 0,
            _last_game_scene_data_name: "".to_string(),
            _game_scenes: Default::default(),
            _game_scenarios: vec![],
            _completed_game_scenarios: Default::default(),
        }
    }
}
