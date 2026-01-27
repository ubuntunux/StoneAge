use crate::game_module::game_scene_manager::{
    CharacterCreateInfoMap, GameSceneManager, ItemCreateInfoMap, PropCreateInfoMap,
};
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::scenario::game_scenarios::scenario_intro::ScenarioIntro;
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{newRcRefCell, RcRefCell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type GameSceneCreateInfoMap = HashMap<String, GameSceneCreateInfo>;

pub struct ScenarioTrack<T: Copy + PartialEq> {
    pub _scenario_phase: T,
    pub _phase_time: f32,
    pub _phase_duration: Option<f32>,
}

impl<T: Copy + PartialEq> ScenarioTrack<T> {
    pub fn set_scenario_phase(&mut self, next_scenario_phase: T, phase_duration: Option<f32>) {
        if next_scenario_phase != self._scenario_phase {
            self._scenario_phase = next_scenario_phase;
            self._phase_time = 0.0;
            self._phase_duration = phase_duration;
        }
    }

    pub fn get_phase_ratio(&self) -> f32 {
        if let Some(phase_duration) = self._phase_duration.as_ref() {
            if 0.0f32 < *phase_duration {
                return 0f32.max(1f32.min(self._phase_time / phase_duration));
            }
        }
        0.0
    }

    pub fn update_scenario_track(&mut self, delta_time: f32) {
        if let Some(phase_duration) = self._phase_duration.as_ref() {
            self._phase_time = (*phase_duration).min(self._phase_time + delta_time as f32);
        }
    }
}

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
    pub _game_scenes: GameSceneCreateInfoMap,
}
pub trait ScenarioBase {
    fn is_end_of_scenario(&self) -> bool;
    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>);
    fn update_game_scenario_begin(&mut self);
    fn update_game_scenario_end(&mut self);
    fn update_game_scenario(&mut self, game_ui_manager: &mut GameUIManager, any_key_hold: bool, any_key_pressed: bool, delta_time: f64);
}

pub fn create_scenario<'a>(
    game_scene_manager: *const GameSceneManager<'a>,
    scenario_name: &str,
    scenario_create_info: &ScenarioDataCreateInfo,
) -> RcRefCell<dyn ScenarioBase + 'a> {
    newRcRefCell(ScenarioIntro::create_game_scenario(
        game_scene_manager,
        scenario_name,
        scenario_create_info,
    ))
}
