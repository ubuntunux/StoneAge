use crate::application::application::Application;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_manager::{
    CharacterCreateInfo, CharacterID, CharacterManager, CharacterSaveData,
};
use crate::game_module::actors::items::{ItemCreateInfo, ItemManager, ItemSaveData};
use crate::game_module::actors::props::{PropCreateInfo, PropManager, PropSaveData};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{
    CHARACTER_DATA_NAME_MONKEY_ARU, GAME_VIEW_MODE, GameViewMode, TEMPERATURE_MAX, TEMPERATURE_MIN, TIME_OF_DAWN,
    TIME_OF_DAY_SPEED, TIME_OF_MORNING,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::save_data::save_data::GameSaveData;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioType, create_scenario};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop, AudioManager};
use rust_engine_3d::begin_block;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::scene_manager::{SceneDataCreateInfo, SceneManager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum_macros::{Display, EnumString};

pub type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;
pub type CharacterSaveDataMap = HashMap<String, CharacterSaveData>;
pub type ItemCreateInfoMap = HashMap<String, ItemCreateInfo>;
pub type ItemSaveDataMap = HashMap<String, ItemSaveData>;
pub type PropCreateInfoMap = HashMap<String, PropCreateInfo>;
pub type PropSaveDataMap = HashMap<String, PropSaveData>;
pub type ScenarioMap<'a> = HashMap<ScenarioType, RcRefCell<dyn ScenarioBase<'a> + 'a>>;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy)]
pub enum Stages {
    None,
    Home,
    Forest,
    Cave,
    WorldMap,
    Ufo,
}

impl Stages {
    pub fn get_stage_display_name(&self) -> &str {
        match *self {
            Stages::None => "",
            Stages::Home => "HOME",
            Stages::Forest => "FOREST",
            Stages::Cave => "CAVE",
            Stages::WorldMap => "WORLD MAP",
            Stages::Ufo => "UFO",
        }
    }

    pub fn get_stage_data_name(&self) -> &str {
        match *self {
            Stages::None => "",
            Stages::Home => "game_scenes/intro_stage",
            Stages::Forest => "game_scenes/stage_01",
            Stages::Cave => "game_scenes/stage_cave",
            Stages::WorldMap => "game_scenes/world_map",
            Stages::Ufo => "game_scenes/stage_ufo",
        }
    }

    pub fn find_stage_value(stage_data_name: &str) -> Stages {
        if stage_data_name == Stages::None.get_stage_data_name() {
            return Stages::None;
        } else if stage_data_name == Stages::Home.get_stage_data_name() {
            return Stages::Home;
        } else if stage_data_name == Stages::Forest.get_stage_data_name() {
            return Stages::Forest;
        } else if stage_data_name == Stages::Cave.get_stage_data_name() {
            return Stages::Cave;
        } else if stage_data_name == Stages::WorldMap.get_stage_data_name() {
            return Stages::WorldMap;
        } else if stage_data_name == Stages::Ufo.get_stage_data_name() {
            return Stages::Ufo;
        }

        assert!(false, "not implemented: {}", stage_data_name);
        Stages::None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameSceneState {
    None,
    Loading,
    LoadCompleted,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _characters: CharacterCreateInfoMap,
    pub _items: ItemCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _props: PropCreateInfoMap,
    pub _scene: SceneDataCreateInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneSaveData {
    pub _characters: CharacterSaveDataMap,
    pub _items: ItemSaveDataMap,
    pub _props: PropSaveDataMap,
}

pub struct GameSceneManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _effect_manager: *const EffectManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _character_manager: Box<CharacterManager<'a>>,
    pub _item_manager: Box<ItemManager<'a>>,
    pub _prop_manager: Box<PropManager<'a>>,
    pub _reservation_scenarios: Vec<ScenarioType>,
    pub _scenarios: ScenarioMap<'a>,
    pub _completed_game_scenarios: HashSet<ScenarioType>,
    pub _is_play_scenario_mode: bool,
    pub _current_game_scene_data_name: String,
    pub _current_game_scene_data: Option<RcRefCell<GameSceneDataCreateInfo>>,
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>,
    pub _teleport_stage: Option<String>,
    pub _teleport_gate: Option<String>,
    pub _teleport_spawn_point: Option<String>,
    pub _is_sleep_mode: bool,
    pub _time_of_day: f32,
    pub _time_of_day_speed: f32,
    pub _temperature: f32,
    pub _date: u32,
    pub _game_scene_state: GameSceneState,
    pub _next_game_scene_state: GameSceneState,
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

    pub fn get_scene_manager_ptr(&self) -> *const SceneManager<'a> {
        self._scene_manager
    }

    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }

    pub fn get_game_ui_manager(&self) -> &GameUIManager<'a> {
        ptr_as_ref(self._game_ui_manager)
    }

    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager<'a> {
        ptr_as_mut(self._game_ui_manager)
    }

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        self._character_manager.as_ref()
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager.as_ref())
    }

    pub fn get_actor(&self, character_id: CharacterID) -> Option<&RcRefCell<Character<'a>>> {
        self.get_character_manager().get_character(character_id)
    }

    pub fn get_actor_by_name(&self, actor_name: &str) -> Option<&RcRefCell<Character<'a>>> {
        self.get_character_manager().get_character_by_name(actor_name)
    }

    pub fn get_maybe_player(&self) -> &Option<RcRefCell<Character<'a>>> {
        self.get_character_manager().get_maybe_player()
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
            _game_client: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _current_game_scene_data_name: Default::default(),
            _current_game_scene_data: None,
            _character_manager: CharacterManager::create_character_manager(),
            _item_manager: ItemManager::create_item_manager(),
            _prop_manager: PropManager::create_prop_manager(),
            _reservation_scenarios: Default::default(),
            _scenarios: Default::default(),
            _completed_game_scenarios: Default::default(),
            _is_play_scenario_mode: false,
            _ambient_sound: None,
            _teleport_stage: None,
            _teleport_gate: None,
            _teleport_spawn_point: None,
            _is_sleep_mode: false,
            _time_of_day: TIME_OF_MORNING,
            _time_of_day_speed: 1.0,
            _temperature: 30.0,
            _date: 1,
            _game_scene_state: GameSceneState::None,
            _next_game_scene_state: GameSceneState::None,
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application<'a>,
        engine_core: &EngineCore<'a>,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._game_client = application.get_game_client();
        self._audio_manager = engine_core.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._game_ui_manager = application.get_game_ui_manager();
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

    pub fn get_game_client(&self) -> &GameClient<'a> {
        ptr_as_ref(self._game_client)
    }

    pub fn get_game_client_mut(&self) -> &mut GameClient<'a> {
        ptr_as_mut(self._game_client)
    }

    pub fn get_audio_manager(&self) -> &AudioManager<'a> {
        ptr_as_ref(self._audio_manager)
    }

    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
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

    pub fn get_time_of_day(&self) -> f32 {
        self._time_of_day
    }

    pub fn temperature(&self) -> f32 {
        self._temperature
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self._temperature = temperature;
    }

    pub fn get_date(&self) -> u32 {
        self._date
    }

    pub fn get_current_game_scene_data_name(&self) -> &String {
        &self._current_game_scene_data_name
    }

    pub fn load_game_save_data(&mut self, game_save_data: &GameSaveData) {
        self.clear_all_game_scenario();

        // loading
        self.open_game_scene_data(&game_save_data._last_game_scene_data_name);
        self.set_temperature(game_save_data._temperature);
        self._date = game_save_data._date;
        self.set_time_of_day(game_save_data._time_of_day);

        self._completed_game_scenarios = game_save_data._completed_game_scenarios.clone();
        for game_scenario_create_info in game_save_data._game_scenarios.iter() {
            let opened_scenario = self.open_game_scenario_data(
                game_scenario_create_info._scenario_type,
                &game_scenario_create_info._scenario_create_info,
                false,
            );
            opened_scenario.borrow_mut().load_scenario_save_data(&game_scenario_create_info);
        }

        self.get_game_ui_manager_mut().clear_inventory_items();
        for create_infos in game_save_data._inventory_item_create_info_list.values() {
            for item_create_info in create_infos.iter() {
                self.get_game_ui_manager_mut()
                    .add_item(item_create_info._item_data_name.as_str(), item_create_info._item_count);
            }
        }
    }

    pub fn update_game_save_data(&self, game_save_data: &mut GameSaveData) {
        if self.get_character_manager().is_valid_player() {
            game_save_data._player =
                Some(self.get_character_manager().get_player().as_ref().borrow().get_character_save_data())
        }
        game_save_data._time_of_day = self.get_time_of_day();
        game_save_data._temperature = self.temperature();
        game_save_data._date = self.get_date();
        game_save_data._last_game_scene_data_name = self.get_current_game_scene_data_name().clone();
        game_save_data._inventory_item_create_info_list = self.get_game_ui_manager().get_inventory_item_create_infos();
        game_save_data._selected_inventory_item_index = self.get_game_ui_manager().get_selected_inventory_item_index();

        game_save_data._game_scenes.insert(
            self.get_current_game_scene_data_name().clone(),
            self.get_game_scene_save_data(),
        );
        game_save_data._completed_game_scenarios = self._completed_game_scenarios.clone();
        game_save_data._game_scenarios =
            self._scenarios.values().map(|scenario| scenario.borrow().get_scenario_save_data()).collect();
    }

    pub fn get_game_scene_save_data(&self) -> GameSceneSaveData {
        GameSceneSaveData {
            _characters: self.get_character_manager().get_characters_save_data(),
            _items: self._item_manager.get_items_save_data(),
            _props: self._prop_manager.get_props_save_data(),
        }
    }

    // update teleport
    pub fn is_teleport_mode(&self) -> bool {
        self._teleport_stage.is_some() || self._teleport_gate.is_some() || self._teleport_spawn_point.is_some()
    }
    pub fn is_teleport_stage(&self) -> bool {
        if let Some(teleport_stage) = self._teleport_stage.as_ref() {
            return self.get_current_game_scene_data_name() != teleport_stage;
        }
        false
    }
    pub fn set_teleport_stage(&mut self, teleport_stage: &str, teleport_gate: &str) {
        self._teleport_stage = Some(String::from(teleport_stage));
        self._teleport_gate = Some(String::from(teleport_gate));
    }
    pub fn set_teleport_spawn_point(&mut self, teleport_stage: &str, spawn_point_name: &str) {
        self._teleport_stage = Some(String::from(teleport_stage));
        self._teleport_spawn_point = Some(String::from(spawn_point_name));
    }
    pub fn update_teleport(&mut self, character_manager: &CharacterManager<'a>) {
        // teleport
        if self._teleport_stage.is_some() {
            let game_scene_data_name = ptr_as_ref(self._teleport_stage.as_ref().unwrap().as_str());
            if self.get_current_game_scene_data_name() != game_scene_data_name {
                self.goto_game_scene(game_scene_data_name);
                return;
            }
            self._teleport_stage = None;
        }

        // goto gate
        if self.is_game_scene_state(GameSceneState::LoadCompleted)
            && self._teleport_stage.is_none()
            && self._teleport_gate.is_some()
        {
            if character_manager.is_valid_player()
                && let Some(teleport_point) =
                    self.get_prop_manager().get_teleport_point(self._teleport_gate.as_ref().unwrap().as_str())
            {
                if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                    character_manager.get_player().borrow_mut().set_position_xy(&teleport_point);
                } else {
                    let height_map_data = self.get_scene_manager().get_height_map_data();
                    let ground_height = height_map_data.get_height_bilinear(&teleport_point, 0);
                    let ground_normal = height_map_data.get_normal_bilinear(&teleport_point);
                    character_manager.get_player().borrow_mut().set_position(&teleport_point);
                    character_manager.get_player().borrow_mut().set_on_ground(ground_height, &ground_normal);
                }
            }
            self._teleport_gate = None;
        }

        // goto spawn point prop (e.g., bed_for_aru)
        if self.is_game_scene_state(GameSceneState::LoadCompleted)
            && self._teleport_stage.is_none()
            && self._teleport_spawn_point.is_some()
        {
            if character_manager.is_valid_player() {
                let spawn_point_name = self._teleport_spawn_point.as_ref().unwrap().clone();
                if let Some(prop) = self.get_prop_manager().get_prop_by_name(&spawn_point_name) {
                    let spawn_position = prop.borrow().get_position().clone();
                    let player = character_manager.get_player();
                    let rotation = player.borrow().get_rotation().clone();
                    let scale = player.borrow().get_scale().clone();
                    // respawn
                    player.borrow_mut().respawn_character(&spawn_position, &rotation, &scale);
                }
            }
            self._teleport_spawn_point = None;
        }
    }

    // scenario
    pub fn has_game_scenario(&self, scenario_type: ScenarioType) -> bool {
        self._scenarios.contains_key(&scenario_type)
    }

    pub fn get_game_scenario(&self, scenario_type: ScenarioType) -> Option<&RcRefCell<dyn ScenarioBase<'a> + 'a>> {
        self._scenarios.get(&scenario_type)
    }

    pub fn request_open_game_scenario(&mut self, scenario_type: ScenarioType) {
        self._reservation_scenarios.push(scenario_type);
    }

    fn open_game_scenario_data(
        &mut self,
        scenario_type: ScenarioType,
        scenario_data_create_info: &ScenarioDataCreateInfo,
        open_game_scene: bool,
    ) -> &RcRefCell<dyn ScenarioBase<'a> + 'a> {
        self._scenarios.insert(
            scenario_type,
            create_scenario(self, self._game_resources, scenario_type, &scenario_data_create_info),
        );

        if open_game_scene {
            let game_scene_data_name = scenario_data_create_info.get_game_scene_data_name();
            if !game_scene_data_name.is_empty() && *self.get_current_game_scene_data_name() != game_scene_data_name {
                self.get_game_client().save_game(false);
                self.open_game_scene_data(game_scene_data_name.as_str());
            }
        }
        self._scenarios.get(&scenario_type).unwrap()
    }

    fn clear_all_game_scenario(&mut self) {
        if self.has_scenarios() {
            self._scenarios.values_mut().for_each(|scenario_refcell| {
                let mut scenario = scenario_refcell.borrow_mut();
                scenario.destroy_game_scenario();
            });
            self._scenarios.clear();
        }
        self._is_play_scenario_mode = true;
    }

    // game scene
    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        // clear all
        self.close_game_scene_data();

        // open game scene data
        let game_scene_data = self.get_game_resources().get_game_scene_data(game_scene_data_name);
        self._current_game_scene_data = Some(game_scene_data.clone());
        self._current_game_scene_data_name = String::from(game_scene_data_name);
        if let Some(game_scene_data) = self._current_game_scene_data.as_ref() {
            let scene_manager = ptr_as_mut(self._scene_manager);
            scene_manager.create_scene_data(&game_scene_data.borrow()._scene);
        }

        self.set_next_game_scene_state(GameSceneState::Loading);
    }

    pub fn close_game_scene_data(&mut self) {
        if self.has_scenarios() {
            self._scenarios.values_mut().for_each(|scenario| {
                scenario.borrow_mut().on_close_game_scene(self._current_game_scene_data_name.as_str());
            });
        }
        self.clear_game_object_data();
        self.get_scene_manager_mut().close_scene_data();

        self._game_scene_state = GameSceneState::None;
        self._next_game_scene_state = GameSceneState::None;
        self._current_game_scene_data = None;
        self._current_game_scene_data_name = String::new();
    }

    pub fn goto_game_scene(&mut self, game_scene_data_name: &str) {
        self.get_game_client().save_game(false);
        self.open_game_scene_data(game_scene_data_name);
    }

    pub fn spawn_game_scene_save_data_objects(&mut self, game_scene_save_data: &GameSceneSaveData) {
        self._character_manager.load_characters_save_data(&game_scene_save_data._characters);
        self._item_manager.load_items_save_data(&game_scene_save_data._items);
        self._prop_manager.load_props_save_data(&game_scene_save_data._props);
        let game_scene_manager = ptr_as_ref(self as *const GameSceneManager);
        self._character_manager.post_process_after_characters_loading(game_scene_manager);
    }

    pub fn spawn_game_scene_objects(&mut self, game_scene_data: &GameSceneDataCreateInfo) {
        // items
        for (item_data_name, item_create_info) in game_scene_data._items.iter() {
            self._item_manager.create_item(item_data_name, item_create_info, None);
        }

        // props
        for (prop_name, prop_create_info) in game_scene_data._props.iter() {
            self._prop_manager.create_prop(prop_name, prop_create_info);
        }

        // npc
        self._character_manager.create_characters(&game_scene_data._characters);
    }

    pub fn spawn_game_scenario_objects(&mut self, scenario_create_info: &ScenarioDataCreateInfo) {
        // cameras
        let main_camera = self.get_scene_manager().get_main_camera_mut();
        for (_camera_name, camera_create_info) in scenario_create_info._scene._cameras.iter() {
            main_camera._transform_object.set_position(&camera_create_info.position);
            main_camera._transform_object.set_rotation(&camera_create_info.rotation);
        }

        // items
        for (item_data_name, item_create_info) in scenario_create_info._items.iter() {
            self._item_manager.create_item(item_data_name, item_create_info, None);
        }

        // props
        for (prop_name, prop_create_info) in scenario_create_info._props.iter() {
            self._prop_manager.create_prop(prop_name, prop_create_info);
        }

        // player
        for (_character_name, character_create_info) in scenario_create_info._player.iter() {
            if let Some(character) = self._character_manager.get_maybe_player() {
                character.borrow_mut().update_characters_save_data(character_create_info);
            }
        }

        // npc
        for (character_name, character_create_info) in scenario_create_info._characters.iter() {
            if let Some(character) = self._character_manager.get_character_by_name(character_name) {
                character.borrow_mut().update_characters_save_data(character_create_info);
            } else {
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

    pub fn set_next_time_of_day(&mut self) {
        if TIME_OF_DAWN <= self._time_of_day {
            self._date += 1;
        }
        self._time_of_day = TIME_OF_DAWN;
    }

    pub fn set_time_of_day_speed(&mut self, speed: f32) {
        self._time_of_day_speed = speed;
    }

    pub fn set_time_of_day(&mut self, time_of_day: f32) {
        self._time_of_day = time_of_day;
        self.update_time_of_day(0.0);
        self.get_scene_manager_mut().reset_render_light_probe_time();
    }
    pub fn set_time(&mut self, time: f32, minute: f32) {
        self.set_time_of_day(time + minute / 60.0f32)
    }

    pub fn update_time_of_day(&mut self, delta_time: f64) {
        self._time_of_day += self._time_of_day_speed * TIME_OF_DAY_SPEED * delta_time as f32;

        if 24.0 <= self._time_of_day {
            self._time_of_day %= 24.0;
            self._date += 1;
        }

        let temperature_ratio = 1.0 - (self._time_of_day - 12.0) / 12.0;
        self._temperature = math::lerp(TEMPERATURE_MIN, TEMPERATURE_MAX, temperature_ratio);
        begin_block!("MainLight");
        {
            let mut main_light = self.get_scene_manager().get_main_light().borrow_mut();
            let pitch_ratio = (self._time_of_day - 12.0) / 12.0;
            main_light._transform_object.set_pitch(math::lerp(
                std::f32::consts::PI * 0.5,
                -std::f32::consts::PI * 0.5,
                pitch_ratio,
            ));
        }
    }

    pub fn is_completed_or_progress_game_scenario(&self, game_scenario: ScenarioType) -> bool {
        self._completed_game_scenarios.contains(&game_scenario) || self.has_game_scenario(game_scenario)
    }

    pub fn has_scenario(&self, game_scenario: ScenarioType) -> bool {
        self._scenarios.contains_key(&game_scenario)
    }

    pub fn has_scenarios(&self) -> bool {
        !self._scenarios.is_empty()
    }

    pub fn is_play_scenario_mode(&self) -> bool {
        self._is_play_scenario_mode
    }

    pub fn get_game_scene_state(&self) -> GameSceneState {
        self._game_scene_state
    }

    pub fn set_next_game_scene_state(&mut self, state: GameSceneState) {
        self._next_game_scene_state = state;
    }

    pub fn is_game_scene_state(&self, state: GameSceneState) -> bool {
        self._game_scene_state == state
    }

    pub fn update_game_scenarios(&mut self, any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let game_resources = ptr_as_ref(self._game_resources);
        let current_game_scene_data_name = self._current_game_scene_data_name.clone();
        if !self._reservation_scenarios.is_empty() {
            let current_game_scene_state = self._game_scene_state.clone();
            for scenario_type in self._reservation_scenarios.clone().iter() {
                let scenario_data_create_info =
                    game_resources.get_scenario_data(scenario_type.get_scenario_data_name());
                let scenario = self.open_game_scenario_data(*scenario_type, &scenario_data_create_info.borrow(), true);
                if current_game_scene_state == GameSceneState::LoadCompleted {
                    let game_scene_data_name = scenario_data_create_info.borrow().get_game_scene_data_name();
                    if game_scene_data_name.is_empty()
                        || current_game_scene_data_name == scenario_data_create_info.borrow().get_game_scene_data_name()
                    {
                        scenario
                            .borrow_mut()
                            .on_open_game_scene(scenario_data_create_info.borrow().get_game_scene_data_name().as_str());
                    }
                }
            }
            self._reservation_scenarios.clear();
        }

        self._is_play_scenario_mode = false;
        if self.has_scenarios() {
            self._scenarios.retain(|_scenario_type, scenario_refcell| {
                let mut scenario = scenario_refcell.borrow_mut();
                scenario.update_game_scenario(any_key_hold, any_key_pressed, delta_time);

                if scenario.is_play_scenario_mode() {
                    self._is_play_scenario_mode = true;
                }

                if scenario.is_end_of_scenario() {
                    self._completed_game_scenarios.insert(scenario.get_scenario_type());
                    scenario.destroy_game_scenario();
                    false
                } else {
                    true
                }
            });
        }
    }

    pub fn update_game_scene_manager(&mut self, any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        self._game_scene_state = self._next_game_scene_state;
        match self._game_scene_state {
            GameSceneState::Loading => {
                if self.get_scene_manager().is_load_complete() {
                    let game_save_data = ptr_as_ref(self._game_client).get_game_save_data().borrow();
                    if let Some(game_scene_save_data) =
                        game_save_data._game_scenes.get(&self._current_game_scene_data_name).as_ref()
                    {
                        self.spawn_game_scene_save_data_objects(game_scene_save_data);
                    } else if let Some(game_scene_data) = self._current_game_scene_data.clone() {
                        self.spawn_game_scene_objects(&game_scene_data.borrow());
                    }

                    // player
                    if let Some(player_save_data) = game_save_data._player.as_ref() {
                        self._character_manager.load_character_save_data(
                            player_save_data.0.as_str(),
                            &player_save_data.1,
                            true,
                        );
                    } else {
                        let character_create_info = CharacterCreateInfo {
                            _character_data_name: CHARACTER_DATA_NAME_MONKEY_ARU.to_string(),
                            ..Default::default()
                        };
                        self._character_manager.create_character("player", &character_create_info, true);
                    };

                    // post process after loading
                    let game_scene_manager = ptr_as_ref(self as *const GameSceneManager);
                    self._character_manager.post_process_after_characters_loading(game_scene_manager);
                    self._item_manager.post_process_after_item_loading();
                    self._prop_manager.post_process_after_prop_loading();
                    self.get_game_ui_manager_mut().select_item(game_save_data._selected_inventory_item_index);

                    for scenario in self._scenarios.values() {
                        scenario.borrow_mut().on_open_game_scene(self._current_game_scene_data_name.as_str());
                    }

                    self.set_next_game_scene_state(GameSceneState::LoadCompleted);
                }
            }
            GameSceneState::None | GameSceneState::LoadCompleted => {
                self.update_time_of_day(delta_time);
                self._prop_manager.update_prop_manager(delta_time);
                self._item_manager.update_item_manager(delta_time);
                self._character_manager.update_character_manager(delta_time);

                self.update_game_scenarios(any_key_hold, any_key_pressed, delta_time);
            }
        }
    }
}
