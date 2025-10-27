use std::collections::HashMap;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, ItemCreateInfoMap, PropCreateInfoMap};

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

pub struct ScenarioData {

}

impl ScenarioData {
    pub fn create_scenario_data(_scenario_data_create_info: &ScenarioDataCreateInfo) -> ScenarioData {
        ScenarioData {

        }
    }

    pub fn update_scenario_data(&mut self, _delta_time: f64) {

    }
}