use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{
    CharacterCreateInfoMap, GameSceneManager, ItemCreateInfoMap, PropCreateInfoMap,
};
use crate::game_module::scenario::game_scenarios::scenario_intro::day_one::ScenarioDayOne;
use crate::game_module::scenario::game_scenarios::scenario_intro::intro::ScenarioIntro;
use crate::game_module::scenario::game_scenarios::scenario_intro::revolution::ScenarioRevolution;
use crate::game_module::scenario::game_scenarios::scenario_intro::ufo::ScenarioUfo;
use crate::game_module::scenario::game_scenarios::scenario_wrap_up_the_day::ScenarioWrapUpTheDay;
use crate::game_module::scenario::scenario_track::ScenarioTrackCreateInfo;
use nalgebra::Vector3;
use rust_engine_3d::scene::scene_manager::SceneDataCreateInfo;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use strum_macros::{Display, EnumString};

pub type GameSceneCreateInfoMap = HashMap<String, GameSceneCreateInfo>;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy, Default)]
pub enum ScenarioType {
    #[default]
    ScenarioIntro_Intro,
    ScenarioIntro_Ufo,
    ScenarioIntro_Revolution,
    ScenarioIntro_DayOne,
    ScenarioWrapUpTheDay,
}

impl ScenarioType {
    pub fn get_scenario_data_name(&self) -> &str {
        match *self {
            ScenarioType::ScenarioIntro_Intro => "scenario/scenario_intro/intro",
            ScenarioType::ScenarioIntro_Ufo => "scenario/scenario_intro/ufo",
            ScenarioType::ScenarioIntro_Revolution => "scenario/scenario_intro/revolution",
            ScenarioType::ScenarioIntro_DayOne => "scenario/scenario_intro/day_one",
            ScenarioType::ScenarioWrapUpTheDay => "scenario/wrap_up_the_day",
        }
    }
}

pub fn create_scenario<'a>(
    game_scene_manager: *const GameSceneManager<'a>,
    game_resources: *const GameResources<'a>,
    scenario_type: ScenarioType,
    scenario_create_info: &ScenarioDataCreateInfo,
) -> RcRefCell<dyn ScenarioBase<'a> + 'a> {
    match scenario_type {
        ScenarioType::ScenarioIntro_Intro => {
            ScenarioIntro::create_game_scenario(game_scene_manager, game_resources, scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioIntro_Ufo => {
            ScenarioUfo::create_game_scenario(game_scene_manager, game_resources, scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioIntro_Revolution => ScenarioRevolution::create_game_scenario(
            game_scene_manager,
            game_resources,
            scenario_type,
            scenario_create_info,
        ),
        ScenarioType::ScenarioIntro_DayOne => ScenarioDayOne::create_game_scenario(
            game_scene_manager,
            game_resources,
            scenario_type,
            scenario_create_info,
        ),
        ScenarioType::ScenarioWrapUpTheDay => ScenarioWrapUpTheDay::create_game_scenario(
            game_scene_manager,
            game_resources,
            scenario_type,
            scenario_create_info,
        ),
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
    pub _scene: SceneDataCreateInfo,
}

impl ScenarioDataCreateInfo {
    pub fn get_game_scene_data_name(&self) -> String {
        if self._game_scenes.is_empty() {
            return String::from("");
        }
        self._game_scenes.values().last().as_ref().unwrap()._game_scene_data_name.clone()
    }
    pub fn reset(&mut self) {
        *self = ScenarioDataCreateInfo::default();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameScenarioCreateInfo {
    pub _scenario_type: ScenarioType,
    pub _scenario_create_info: ScenarioDataCreateInfo,
    pub _scenario_track_create_info: ScenarioTrackCreateInfo,
    pub _scenario_data: String,
}

pub trait ScenarioBase<'a> {
    fn get_scenario_type(&self) -> ScenarioType;
    fn get_scenario_phase_as_string(&self) -> String;
    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String);
    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo);
    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo;
    fn is_play_scenario_mode(&self) -> bool;
    fn is_end_of_scenario(&self) -> bool;
    fn destroy_game_scenario(&mut self);
    fn on_close_game_scene(&mut self, game_scene_data_name: &str);
    fn on_open_game_scene(&mut self, game_scene_data_name: &str);
    fn update_game_scenario(&mut self, any_key_hold: bool, any_key_pressed: bool, delta_time: f64);
}
