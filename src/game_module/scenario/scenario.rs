use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, GameSceneManager, ItemCreateInfoMap, PropCreateInfoMap};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{RcRefCell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use strum_macros::{Display, EnumString};
use crate::game_module::scenario::game_scenarios::scenario_day_one::ScenarioDayOne;
use crate::game_module::scenario::game_scenarios::scenario_intro::ScenarioIntro;
use crate::game_module::scenario::game_scenarios::scenario_revolution::ScenarioRevolution;
use crate::game_module::scenario::game_scenarios::scenario_ufo::ScenarioUfo;

pub type GameSceneCreateInfoMap = HashMap<String, GameSceneCreateInfo>;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy)]
pub enum ScenarioType {
    ScenarioIntro,
    ScenarioUfo,
    ScenarioRevolution,
    ScenarioDayOne,
}

impl ScenarioType {
    pub fn get_scenario_data_name(&self) -> &str {
        match *self {
            ScenarioType::ScenarioIntro => "scenario/intro",
            ScenarioType::ScenarioUfo => "scenario/ufo",
            ScenarioType::ScenarioRevolution => "scenario/revolution",
            ScenarioType::ScenarioDayOne => "scenario/day_one",
        }
    }
}


pub struct ScenarioTrack<T: Copy + PartialEq + Hash> {
    pub _scenario_phase: T,
    pub _phase_time: f32,
    pub _phase_duration: Option<f32>,
}

impl<T: Copy + PartialEq + Hash> ScenarioTrack<T> {
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

    pub fn get_phase_time(&self) -> f32 {
        self._phase_time
    }

    pub fn update_scenario_track(&mut self, delta_time: f32) {
        self._phase_time += delta_time;
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

impl ScenarioDataCreateInfo {
    pub fn get_game_scene_data_name(&self) -> String {
        if self._game_scenes.is_empty() {
            return String::from("");
        }
        self._game_scenes.values().last().as_ref().unwrap()._game_scene_data_name.clone()
    }
}

pub trait ScenarioBase<'a> {
    fn get_scenario_type(&self) -> ScenarioType;
    fn is_play_scenario_mode(&self) -> bool;
    fn is_end_of_scenario(&self) -> bool;
    fn destroy_game_scenario(&mut self);
    fn on_close_game_scene(&mut self, game_scene_data_name: &str);
    fn on_open_game_scene(&mut self, game_scene_data_name: &str);
    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>);
    fn update_game_scenario_begin(&mut self);
    fn update_game_scenario_end(&mut self);
    fn update_game_scenario(&mut self, any_key_hold: bool, any_key_pressed: bool, delta_time: f64);
}

pub fn create_scenario<'a>(
    game_scene_manager: *const GameSceneManager<'a>,
    scenario_type: ScenarioType,
    scenario_create_info: &ScenarioDataCreateInfo,
) -> RcRefCell<dyn ScenarioBase<'a> + 'a> {
    match scenario_type {
        ScenarioType::ScenarioIntro => {
            ScenarioIntro::create_game_scenario(game_scene_manager, scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioUfo => {
            ScenarioUfo::create_game_scenario(game_scene_manager, scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioRevolution => {
            ScenarioRevolution::create_game_scenario(game_scene_manager, scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioDayOne => {
            ScenarioDayOne::create_game_scenario(game_scene_manager, scenario_type, scenario_create_info)
        }
    }

}
