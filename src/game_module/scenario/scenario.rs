use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, ItemCreateInfoMap, PropCreateInfoMap};
use crate::game_module::scenario::game_scenarios::scenario_intro::day_one::ScenarioDayOne;
use crate::game_module::scenario::game_scenarios::scenario_intro::intro::ScenarioIntro;
use crate::game_module::scenario::game_scenarios::scenario_intro::revolution::ScenarioRevolution;
use crate::game_module::scenario::game_scenarios::scenario_intro::ufo::ScenarioUfo;
use crate::game_module::scenario::game_scenarios::scenario_using_oumuamua::ScenarioUsingOumuamua;
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
    ScenarioUsingOumuamua,
}

impl ScenarioType {
    pub fn get_scenario_data_name(&self) -> &str {
        match *self {
            ScenarioType::ScenarioIntro_Intro => "scenario/scenario_intro/intro",
            ScenarioType::ScenarioIntro_Ufo => "scenario/scenario_intro/ufo",
            ScenarioType::ScenarioIntro_Revolution => "scenario/scenario_intro/revolution",
            ScenarioType::ScenarioIntro_DayOne => "scenario/scenario_intro/day_one",
            ScenarioType::ScenarioWrapUpTheDay => "scenario/wrap_up_the_day",
            ScenarioType::ScenarioUsingOumuamua => "scenario/using_oumuamua",
        }
    }
}

pub fn create_scenario<'a>(
    scenario_type: ScenarioType,
    scenario_create_info: &ScenarioDataCreateInfo,
) -> RcRefCell<dyn ScenarioBase<'a> + 'a> {
    match scenario_type {
        ScenarioType::ScenarioIntro_Intro => ScenarioIntro::create_game_scenario(scenario_type, scenario_create_info),
        ScenarioType::ScenarioIntro_Ufo => ScenarioUfo::create_game_scenario(scenario_type, scenario_create_info),
        ScenarioType::ScenarioIntro_Revolution => {
            ScenarioRevolution::create_game_scenario(scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioIntro_DayOne => ScenarioDayOne::create_game_scenario(scenario_type, scenario_create_info),
        ScenarioType::ScenarioWrapUpTheDay => {
            ScenarioWrapUpTheDay::create_game_scenario(scenario_type, scenario_create_info)
        }
        ScenarioType::ScenarioUsingOumuamua => {
            ScenarioUsingOumuamua::create_game_scenario(scenario_type, scenario_create_info)
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
    #[serde(skip_serializing_if = "SceneDataCreateInfo::is_empty")]
    pub _scene: SceneDataCreateInfo,
}

impl ScenarioDataCreateInfo {
    pub fn get_game_scene_data_name(&self) -> String {
        self._game_scenes.values().last().map(|scene| scene._game_scene_data_name.clone()).unwrap_or_default()
    }

    pub fn reset(&mut self) {
        *self = ScenarioDataCreateInfo::default();
    }

    pub fn is_empty(&self) -> bool {
        self._characters.is_empty()
            && self._items.is_empty()
            && self._player.is_empty()
            && self._props.is_empty()
            && self._game_scenes.is_empty()
            && self._scene.is_empty()
    }

    pub fn get_save_data(&self) -> ScenarioDataCreateInfo {
        ScenarioDataCreateInfo {
            _characters: self._characters.clone(),
            _items: self._items.clone(),
            _player: self._player.clone(),
            _props: self._props.clone(),
            _game_scenes: self._game_scenes.clone(),
            _scene: SceneDataCreateInfo::default(),
        }
    }

    pub fn restore_static_scene_from_template(&mut self, scenario_type: ScenarioType) {
        if !self.is_empty() {
            let template_data = crate::game_module::game_service_locator::get_game_resources()
                .get_scenario_data(scenario_type.get_scenario_data_name());
            self._scene = template_data.borrow()._scene.clone();
        }
    }

    pub fn spawn_and_reset_if_matching(&mut self, game_scene_data_name: &str) {
        if self.get_game_scene_data_name() == game_scene_data_name {
            crate::game_module::game_service_locator::get_game_scene_manager_mut().spawn_game_scenario_objects(self);
            self.reset();
        }
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
    fn set_scenario_phase_as_string(&mut self, scenario_phase: &str);
    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo);
    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo;
    fn is_play_scenario_mode(&self) -> bool;
    fn is_available_sleep(&self) -> bool {
        false
    }
    fn is_use_table_storage_as_player_inventory(&self) -> bool {
        false
    }
    fn request_sleep(&mut self) {}
    fn set_sleep_phase(&mut self) {}
    fn set_skip_wakeup(&mut self, _skip_wakeup: bool) {}
    fn is_sleeping(&self) -> bool {
        false
    }
    fn complete_sub_quest_wrap_up_the_day(&mut self) {}
    fn complete_sub_quest_cooking(&mut self) {
        self.complete_sub_quest_ask_ewa_to_cook();
    }
    fn complete_sub_quest_ask_ewa_to_cook(&mut self) {}
    fn complete_sub_quest_eat_food(&mut self) {}
    fn complete_sub_quest_sleep(&mut self) {}
    fn is_end_of_scenario(&self) -> bool;
    fn destroy_game_scenario(&mut self);
    fn on_close_game_scene(&mut self, game_scene_data_name: &str);
    fn on_open_game_scene(&mut self, game_scene_data_name: &str);
    fn update_game_scenario(&mut self, any_key_hold: bool, any_key_pressed: bool, delta_time: f64);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_module::actors::character::CharacterCreateInfo;
    use crate::game_module::actors::items::ItemCreateInfo;
    use crate::game_module::actors::props::PropCreateInfo;
    use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectSaveData};

    #[test]
    fn test_scenario_save_data_excludes_static_scene_objects_and_preserves_dynamic_entities() {
        let mut scenario_create_info = ScenarioDataCreateInfo::default();

        // 1. Add dynamic entities (character, item, prop, player)
        scenario_create_info._characters.insert(
            String::from("dino"),
            CharacterCreateInfo {
                _character_data_name: String::from("characters/trex"),
                ..Default::default()
            },
        );
        scenario_create_info._items.insert(
            String::from("apple"),
            ItemCreateInfo {
                _item_data_name: String::from("items/apple"),
                ..Default::default()
            },
        );
        scenario_create_info._props.insert(
            String::from("torch"),
            PropCreateInfo {
                _prop_data_name: String::from("props/campfire"),
                ..Default::default()
            },
        );
        scenario_create_info._player.insert(
            String::from("player"),
            CharacterCreateInfo {
                _character_data_name: String::from("characters/aru"),
                ..Default::default()
            },
        );

        // 2. Add static scene object (terrain / static mesh)
        scenario_create_info._scene._static_objects.insert(
            String::from("terrain_mountain"),
            RenderObjectCreateInfo {
                _model_data_name: String::from("environments/mountain"),
                ..Default::default()
            },
        );
        assert!(!scenario_create_info._scene.is_empty());

        // 3. Obtain save data via get_save_data()
        let save_data = scenario_create_info.get_save_data();

        // 4. Verify dynamic entities are preserved
        assert_eq!(save_data._characters.len(), 1);
        assert_eq!(save_data._items.len(), 1);
        assert_eq!(save_data._props.len(), 1);
        assert_eq!(save_data._player.len(), 1);

        // 5. Verify static scene objects are stripped
        assert!(save_data._scene.is_empty());

        // 6. Verify serialized JSON omits _scene entirely
        let json_str = serde_json::to_string(&save_data).expect("Serialization failed");
        assert!(!json_str.contains("terrain_mountain"));
        assert!(!json_str.contains("\"_scene\":"));
        assert!(json_str.contains("characters/trex"));
        assert!(json_str.contains("items/apple"));
        assert!(json_str.contains("props/campfire"));

        // 7. Verify deserialization restores dynamic data safely
        let deserialized: ScenarioDataCreateInfo = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(deserialized._characters.len(), 1);
        assert_eq!(deserialized._items.len(), 1);
        assert_eq!(deserialized._props.len(), 1);
        assert_eq!(deserialized._player.len(), 1);
        assert!(deserialized._scene.is_empty());
    }

    #[test]
    fn test_render_object_save_data_skips_animation_buffer_and_transforms() {
        use rust_engine_3d::scene::animation::AnimationPlayInfoSaveData;

        let play_info = AnimationPlayInfoSaveData {
            _prev_animation_play_time: 1.0,
            _animation_play_time: 1.5,
            _animation_end_time: Some(2.0),
            _animation_transforms: Vec::new(),
            _last_animation_transforms: Vec::new(),
            _animation_index: 0,
            ..Default::default()
        };

        let render_save_data = RenderObjectSaveData {
            _render_object_name: String::from("actor_character"),
            _animation_play_infos: vec![play_info],
            _animation_buffer: None,
        };

        let json_str = serde_json::to_string(&render_save_data).expect("Serialization failed");
        assert!(!json_str.contains("_animation_buffer"));
        assert!(!json_str.contains("_animation_transforms"));

        let deserialized: RenderObjectSaveData = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(deserialized._render_object_name, "actor_character");
        assert!(deserialized._animation_buffer.is_none());
        assert_eq!(deserialized._animation_play_infos.len(), 1);
        assert!(deserialized._animation_play_infos[0]._animation_transforms.is_empty());
        assert!(deserialized._animation_play_infos[0]._last_animation_transforms.is_empty());
    }
}
