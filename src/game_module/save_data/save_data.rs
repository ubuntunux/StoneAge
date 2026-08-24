use crate::game_module::actors::character::CharacterSaveData;
use crate::game_module::game_scene_manager::GameSceneSaveData;
use crate::game_module::game_weather::WeatherType;
use crate::game_module::scenario::scenario::{GameScenarioCreateInfo, ScenarioType};
use crate::game_module::widgets::item_bar::{DEFAULT_INVENTORY_ROWS, InventoryItemCreateInfoList};
use rust_engine_3d::scene::camera::CameraCreateInfo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct PlayerRecords {
    pub _item_count: u32,
    pub _craft_count: u32,
    pub _death_count: u32,
    pub _taming_count: u32,
    pub _friend_count: u32,
    pub _visited_maps: HashSet<String>,
    pub _item_type_counts: HashMap<String, u32>,
    pub _monster_kill_counts: HashMap<String, u32>,
    pub _energy_balls: u32,
    pub _spirit_balls: u32,
    pub _custom_activity_counts: HashMap<String, u32>,
}

impl PlayerRecords {
    pub fn reset(&mut self) {
        *self = PlayerRecords::default();
    }

    pub fn increment_activity_stat(&mut self, key: &str, amount: u32) {
        *self._custom_activity_counts.entry(key.to_string()).or_insert(0) += amount;
    }

    pub fn get_activity_stat(&self, key: &str) -> u32 {
        self._custom_activity_counts.get(key).copied().unwrap_or(0)
    }

    pub fn add_item_count(&mut self, amount: u32) {
        self._item_count = self._item_count.saturating_add(amount);
    }

    pub fn add_item_type_count(&mut self, item_type_name: &str, amount: u32) {
        *self._item_type_counts.entry(item_type_name.to_string()).or_insert(0) += amount;
    }

    pub fn add_craft_count(&mut self, amount: u32) {
        self._craft_count = self._craft_count.saturating_add(amount);
    }

    pub fn add_death_count(&mut self, amount: u32) {
        self._death_count = self._death_count.saturating_add(amount);
    }

    pub fn add_taming_count(&mut self, amount: u32) {
        self._taming_count = self._taming_count.saturating_add(amount);
    }

    pub fn add_friend_count(&mut self, amount: u32) {
        self._friend_count = self._friend_count.saturating_add(amount);
    }

    pub fn record_map_visit(&mut self, map_name: &str) {
        if !map_name.is_empty() {
            self._visited_maps.insert(map_name.to_string());
        }
    }

    pub fn get_visited_map_count(&self) -> usize {
        self._visited_maps.len()
    }

    pub fn add_monster_kill(&mut self, monster_name: &str) {
        *self._monster_kill_counts.entry(monster_name.to_string()).or_insert(0) += 1;
    }

    pub fn add_energy_balls(&mut self, amount: u32) {
        self._energy_balls = self._energy_balls.saturating_add(amount);
    }

    pub fn add_spirit_balls(&mut self, amount: u32) {
        self._spirit_balls = self._spirit_balls.saturating_add(amount);
    }

    pub fn get_total_monster_kills(&self) -> u32 {
        self._monster_kill_counts.values().sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GameSaveData {
    pub _player: Option<(String, CharacterSaveData)>,
    pub _camera: CameraCreateInfo,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _weather_type: WeatherType,
    pub _inventory_rows: usize,
    pub _inventory_item_create_info_list: InventoryItemCreateInfoList,
    pub _selected_inventory_item_index: usize,
    pub _selected_quick_slot: Option<(usize, usize)>,
    pub _unlocked_toolbox_items: HashSet<String>,
    pub _last_opened_toolbox_tab: String,
    pub _last_game_scene_data_name: String,
    pub _game_scenes: HashMap<String, GameSceneSaveData>,
    pub _game_scenarios: Vec<GameScenarioCreateInfo>,
    pub _completed_game_scenarios: HashSet<ScenarioType>,
    pub _is_controls_visible: bool,
    pub _player_records: PlayerRecords,
}

impl Default for GameSaveData {
    fn default() -> GameSaveData {
        GameSaveData {
            _player: None,
            _camera: Default::default(),
            _time_of_day: 0.0,
            _temperature: 0.0,
            _date: 0,
            _weather_type: WeatherType::None,
            _inventory_rows: DEFAULT_INVENTORY_ROWS,
            _inventory_item_create_info_list: Default::default(),
            _selected_inventory_item_index: usize::MAX,
            _selected_quick_slot: None,
            _unlocked_toolbox_items: HashSet::new(),
            _last_opened_toolbox_tab: "".to_string(),
            _last_game_scene_data_name: "".to_string(),
            _game_scenes: Default::default(),
            _game_scenarios: vec![],
            _completed_game_scenarios: Default::default(),
            _is_controls_visible: true,
            _player_records: Default::default(),
        }
    }
}
