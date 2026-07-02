use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::game_scene_manager::GameSceneDataCreateInfo;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSaveData {
    pub _player: CharacterCreateInfo,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _last_game_scene_name: String,
    pub _game_scenes: HashMap<String, GameSceneDataCreateInfo>,
}