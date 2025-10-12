use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop, AudioManager};
use rust_engine_3d::begin_block;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::scene_manager::{RenderObjectCreateInfoMap, SceneDataCreateInfo, SceneManager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};
use crate::application::application::Application;
use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::actors::items::{ItemCreateInfo, ItemManager};
use crate::game_module::actors::props::{PropCreateInfo, PropManager};
use crate::game_module::game_constants::{TEMPERATURE_MAX, TEMPERATURE_MIN, TIME_OF_DAY_SPEED};
use crate::game_module::game_resource::GameResources;

type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;
type ItemCreateInfoMap = HashMap<String, ItemCreateInfo>;
type PropCreateInfoMap = HashMap<String, PropCreateInfo>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameSceneState {
    None,
    Loading,
    Playing
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _characters: CharacterCreateInfoMap,
    pub _items: ItemCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _props: PropCreateInfoMap,
    pub _scene: SceneDataCreateInfo,
    pub _terrain: RenderObjectCreateInfoMap
}

pub struct GameSceneManager<'a> {
    pub _audio_manager: *const AudioManager<'a>,
    pub _effect_manager: *const EffectManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _character_manager: Box<CharacterManager<'a>>,
    pub _item_manager: Box<ItemManager<'a>>,
    pub _prop_manager: Box<PropManager<'a>>,
    pub _current_game_scene_data: Option<RcRefCell<GameSceneDataCreateInfo>>,
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>,
    pub _spawn_point: Vector3<f32>,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _teleport_stage: Option<String>,
    pub _teleport_gate: Option<String>,
    pub _game_scene_state: GameSceneState
}

impl<'a> GameSceneManager<'a> {
    pub fn get_game_resources(&self) -> &GameResources<'a> {
        ptr_as_ref(self._game_resources)
    }

    pub fn get_game_resources_mut(&self) -> &mut GameResources<'a> {
        ptr_as_mut(self._game_resources)
    }

    pub fn get_scene_manager(&self) -> &SceneManager<'a> {
        ptr_as_ref(self._scene_manager)
    }

    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        self._character_manager.as_ref()
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager.as_ref())
    }

    pub fn get_prop_manager(&self) -> &PropManager<'a> {
        self._prop_manager.as_ref()
    }

    pub fn get_prop_manager_mut(&self) -> &mut PropManager<'a> {
        ptr_as_mut(self._prop_manager.as_ref())
    }

    pub fn get_item_manager(&self) -> &ItemManager<'a> {
        self._item_manager.as_ref()
    }

    pub fn get_item_manager_mut(&self) -> &mut ItemManager<'a> {
        ptr_as_mut(self._item_manager.as_ref())
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager<'a>> {
        Box::new(GameSceneManager {
            _audio_manager: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _current_game_scene_data: None,
            _character_manager: CharacterManager::create_character_manager(),
            _item_manager: ItemManager::create_item_manager(),
            _prop_manager: PropManager::create_prop_manager(),
            _ambient_sound: None,
            _spawn_point: Vector3::new(0.0, 0.0, 0.0),
            _time_of_day: 10.0,
            _temperature: 30.0,
            _date: 1,
            _teleport_stage: None,
            _teleport_gate: None,
            _game_scene_state: GameSceneState::None,
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application<'a>,
        engine_core: &EngineCore<'a>,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._audio_manager = engine_core.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        engine_core.get_scene_manager_mut().initialize_scene_manager(
            engine_core.get_renderer_context(),
            engine_core.get_audio_manager(),
            engine_core.get_effect_manager(),
            engine_core.get_engine_resources(),
            window_size,
        );

        self._game_resources = application.get_game_resources();
        self._character_manager.initialize_character_manager(engine_core, application);
        self._item_manager.initialize_item_manager(engine_core, application);
        self._prop_manager.initialize_prop_manager(engine_core, application);
    }

    pub fn play_bgm(&mut self, audio_name: &str, volume: Option<f32>) {
        ptr_as_mut(self._audio_manager).play_bgm(audio_name, volume);
    }

    pub fn play_ambient_sound(&mut self, audio_name: &str, volume: Option<f32>) {
        self._ambient_sound = ptr_as_mut(self._audio_manager).play_audio_bank(audio_name, AudioLoop::LOOP, volume);
    }

    pub fn stop_bgm(&self) {
        ptr_as_mut(self._audio_manager).stop_bgm();
    }

    pub fn stop_ambient_sound(&mut self) {
        if let Some(audio_instance_refcell) = self._ambient_sound.as_ref() {
            ptr_as_mut(self._audio_manager).stop_audio_instance(audio_instance_refcell);
        }
        self._ambient_sound = None;
    }

    pub fn get_spawn_point(&self) -> &Vector3<f32> {
        &self._spawn_point
    }

    pub fn get_time_of_day(&self) -> f32 {
        self._time_of_day
    }

    pub fn get_temperature(&self) -> f32 {
        self._temperature
    }

    pub fn get_date(&self) -> u32 {
        self._date
    }

    pub fn get_game_scene_state(&self) -> GameSceneState {
        self._game_scene_state
    }

    pub fn set_game_scene_state(&mut self, state: GameSceneState) {
        self._game_scene_state = state;
    }

    pub fn is_game_scene_state(&self, state: GameSceneState) -> bool {
        self._game_scene_state == state
    }

    pub fn teleport_stage(&mut self, teleport_stage: &str, teleport_gate: &str) {
        self._teleport_stage = Some(String::from(teleport_stage));
        self._teleport_gate = Some(String::from(teleport_gate));
    }

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        self.close_game_scene_data();

        log::info!("open_game_scene_data: {:?}", game_scene_data_name);
        let game_resources = ptr_as_mut(self._game_resources);
        let game_scene_data = game_resources.get_game_scene_data(game_scene_data_name);
        self._current_game_scene_data = Some(game_scene_data.clone());

        if let Some(game_scene_data) = self._current_game_scene_data.as_ref() {
            let scene_manager = ptr_as_mut(self._scene_manager);
            let game_scene_data_ref = game_scene_data.borrow();

            // load scene
            scene_manager.create_scene_data(&game_scene_data_ref._scene);

            // terrain
            for (object_name, render_object_create_info) in game_scene_data_ref._terrain.iter() {
                scene_manager.add_terrain_render_object(object_name, render_object_create_info);
            }

            scene_manager.set_start_capture_height_map(true);
        }

        self.set_game_scene_state(GameSceneState::Loading);
    }

    pub fn close_game_scene_data(&mut self) {
        self.clear_game_object_data();
        self.get_scene_manager_mut().close_scene_data();
        self.set_game_scene_state(GameSceneState::None);
    }

    pub fn spawn_game_object_data(&mut self) {
        assert!(self.get_scene_manager().is_load_complete());

        if let Some(game_scene_data) = self._current_game_scene_data.as_ref() {
            let game_scene_data_ref = game_scene_data.borrow();

            // create items
            for (_item_data_name, item_create_info) in game_scene_data_ref._items.iter() {
                self._item_manager.create_item(item_create_info, false);
            }

            // create props
            for (prop_name, prop_create_info) in game_scene_data_ref._props.iter() {
                self._prop_manager.create_prop(prop_name, prop_create_info);
            }

            // create player
            for (character_name, character_create_info) in game_scene_data_ref._player.iter() {
                self._character_manager.create_character(character_name, character_create_info, true);
                self._spawn_point = character_create_info._position;
            }

            // create npc
            for (character_name, character_create_info) in game_scene_data_ref._characters.iter() {
                self._character_manager.create_character(character_name, character_create_info, false);
            }
        }
    }

    pub fn clear_game_object_data(&mut self) {
        self._item_manager.clear_items();
        self._prop_manager.clear_props();
        self._character_manager.clear_characters(true);
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_time_of_day(&mut self, delta_time: f64) {
        self._time_of_day += delta_time as f32 * TIME_OF_DAY_SPEED;
        if 24.0 <= self._time_of_day {
            self._time_of_day = self._time_of_day % 24.0;
            self._date += 1;
        }
        
        let temperature_ratio = 1.0 - (self._time_of_day - 12.0) / 12.0;
        self._temperature = math::lerp(TEMPERATURE_MIN, TEMPERATURE_MAX, temperature_ratio);
        begin_block!("MainLight"); {
            let mut main_light = self.get_scene_manager().get_main_light().borrow_mut();
            let pitch_ratio = (self._time_of_day - 12.0) / 12.0;
            main_light._transform_object.set_pitch(math::lerp(std::f32::consts::PI * 0.5, -std::f32::consts::PI * 0.5, pitch_ratio));
        }
    }

    pub fn update_game_scene_manager(&mut self, delta_time: f64) {
        match self._game_scene_state {
            GameSceneState::None => {

            },
            GameSceneState::Loading => {
                if self.get_scene_manager().is_load_complete() {
                    self.spawn_game_object_data();
                    self.set_game_scene_state(GameSceneState::Playing);
                }
            },
            GameSceneState::Playing => {
                if self._teleport_stage.is_some() {
                    self.close_game_scene_data();

                    let stage_name = self._teleport_stage.as_ref().unwrap().clone();
                    self.open_game_scene_data(stage_name.as_str());
                    self._teleport_stage = None;
                    return;
                }

                if self._teleport_gate.is_some() {
                    let teleport_point = self._prop_manager.get_teleport_point(self._teleport_gate.as_ref().unwrap().as_str());
                    if teleport_point.is_some() && self._character_manager.is_valid_player() {
                        let player = self._character_manager.get_player();
                        player.borrow_mut().set_position(teleport_point.as_ref().unwrap());
                    }
                    self._teleport_gate = None;
                }

                self.update_time_of_day(delta_time);
                self._prop_manager.update_prop_manager(delta_time);
                self._item_manager.update_item_manager(delta_time);
                self._character_manager.update_character_manager(delta_time);
            },
        }
    }
}
