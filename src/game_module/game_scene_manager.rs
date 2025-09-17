use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop, AudioManager};
use rust_engine_3d::begin_block;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::collision::CollisionType;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::scene_manager::{RenderObjectCreateInfoMap, SceneObjectID, SceneDataCreateInfo, SceneManager};
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

pub type BlocksMap<'a> = HashMap<SceneObjectID, RcRefCell<RenderObjectData<'a>>>;
type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;
type ItemCreateInfoMap = HashMap<String, ItemCreateInfo>;
type PropCreateInfoMap = HashMap<String, PropCreateInfo>;

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
    pub _game_scene_name: String,
    pub _blocks: BlocksMap<'a>,
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>,
    pub _spawn_point: Vector3<f32>,
    pub _time_of_day: f32,
    pub _temperature: f32,
    pub _date: u32
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
            _game_scene_name: String::new(),
            _blocks: HashMap::new(),
            _character_manager: CharacterManager::create_character_manager(),
            _item_manager: ItemManager::create_item_manager(),
            _prop_manager: PropManager::create_prop_manager(),
            _ambient_sound: None,
            _spawn_point: Vector3::new(0.0, 0.0, 0.0),
            _time_of_day: 10.0,
            _temperature: 30.0,
            _date: 1
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

    pub fn get_blocks(&self) -> &BlocksMap<'a> {
        &self._blocks
    }

    pub fn register_block(&mut self, object: &RcRefCell<RenderObjectData<'a>>) {
        if object.borrow().get_collision_type() != CollisionType::NONE {
            self._blocks.insert(object.borrow().get_object_id(), object.clone());
        }
    }

    pub fn unregister_block(&mut self, object: &RcRefCell<RenderObjectData<'a>>) {
        self._blocks.remove(&object.borrow().get_object_id());
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

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        log::info!("open_game_scene_data: {:?}", game_scene_data_name);
        self._game_scene_name = String::from(game_scene_data_name);
        let game_resources = ptr_as_mut(self._game_resources);
        let scene_manager = ptr_as_mut(self._scene_manager);

        // load scene
        let game_scene_data = game_resources.get_game_scene_data(game_scene_data_name).borrow();
        scene_manager.create_scene_data(&game_scene_data._scene);
        for (_key, object) in scene_manager.get_static_render_object_map().iter() {
            self.register_block(object);
        }

        // terrain
        for (object_name, render_object_create_info) in game_scene_data._terrain.iter() {
            let terrain_object = self.get_scene_manager_mut().add_static_render_object(object_name, render_object_create_info);
            terrain_object.borrow_mut().set_collision_type(CollisionType::NONE);
            terrain_object.borrow_mut().set_render_height_map(true);
        }

        // create items
        for (_item_data_name, item_create_info) in game_scene_data._items.iter() {
            let item = self._item_manager.create_item(item_create_info);
            item.borrow()._render_object.borrow_mut().set_collision_type(CollisionType::NONE);
        }

        // create props
        for (prop_name, prop_create_info) in game_scene_data._props.iter() {
            let prop = self._prop_manager.create_prop(prop_name, prop_create_info);
            self.register_block(&prop.borrow()._render_object);
        }

        // create player
        for (character_name, character_create_info) in game_scene_data._player.iter() {
            self._character_manager.create_character(character_name, character_create_info, true);
            self._spawn_point = character_create_info._position;
        }

        // create npc
        for (character_name, character_create_info) in game_scene_data._characters.iter() {
            self._character_manager.create_character(character_name, character_create_info, false);
        }

        // first update
        self.update_game_scene_manager(0.0);
        self.get_scene_manager_mut().set_start_capture_height_map(true);
        self.get_scene_manager_mut().update_scene_manager(0.0);
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
        self._blocks.clear();
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
        self.update_time_of_day(delta_time);
        self._prop_manager.update_prop_manager(delta_time);
        self._item_manager.update_item_manager(delta_time);
        self._character_manager.update_character_manager(delta_time);
    }
}
