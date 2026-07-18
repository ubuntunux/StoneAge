use crate::game_module::actors::character_manager::CharacterCreateInfo;
use crate::game_module::game_scene_manager::GameSceneDataCreateInfo;
use crate::game_module::scenario::scenario::{GameScenarioCreateInfo, ScenarioType};
use crate::game_module::widgets::item_bar_widget::InventoryItemCreateInfoList;
use rust_engine_3d::scene::camera::CameraCreateInfo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::game_module::game_constants::{CHARACTER_DATA_NAME_MONKEY_ARU};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GameSaveData {
    pub _player: (String, CharacterCreateInfo),
    pub _camera: CameraCreateInfo,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _inventory_item_create_info_list: InventoryItemCreateInfoList,
    pub _last_game_scene_data_name: String,
    pub _game_scenes: HashMap<String, GameSceneDataCreateInfo>,
    pub _game_scenarios: Vec<GameScenarioCreateInfo>,
    pub _completed_game_scenarios: HashSet<ScenarioType>,
}

impl Default for GameSaveData {
    fn default() -> GameSaveData {
        GameSaveData {
            _player: ("player".to_string(), CharacterCreateInfo {
                _character_data_name: CHARACTER_DATA_NAME_MONKEY_ARU.to_string(),
                ..Default::default()
            }),
            _camera: Default::default(),
            _time_of_day: 0.0,
            _temperature: 0.0,
            _date: 0,
            _inventory_item_create_info_list: Default::default(),
            _last_game_scene_data_name: "".to_string(),
            _game_scenes: Default::default(),
            _game_scenarios: vec![],
            _completed_game_scenarios: Default::default(),
        }
    }
}