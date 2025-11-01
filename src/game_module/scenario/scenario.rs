use std::collections::HashMap;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use rust_engine_3d::utilities::system::{newRcRefCell, RcRefCell};
use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, GameSceneManager, ItemCreateInfoMap, PropCreateInfoMap};
use crate::game_module::scenario::game_scenarios::scenario_intro::ScenarioIntro;

pub type GameSceneCreateInfoMap = HashMap<String, GameSceneCreateInfo>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneCreateInfo {
    pub _game_scene_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ScenarioDataCreateInfo {
    pub _characters: CharacterCreateInfoMap,
    pub _items: ItemCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _props: PropCreateInfoMap,
    pub _game_scenes: GameSceneCreateInfoMap
}
pub trait ScenarioBase {
    fn set_scenario_data(&mut self, next_scenario_phase: &str);
    fn update_game_scenario_start(&mut self);
    fn update_game_scenario_end(&mut self);
    fn update_game_scenario(&mut self, _delta_time: f64);
}

pub fn create_scenario<'a>(game_scene_manager: *const GameSceneManager<'a>, scenario_name: &str, scenario_create_info: &ScenarioDataCreateInfo) -> RcRefCell<dyn ScenarioBase + 'a> {
    newRcRefCell(ScenarioIntro::create_game_scenario(game_scene_manager, scenario_name, scenario_create_info))
}