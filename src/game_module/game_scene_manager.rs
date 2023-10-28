use std::collections::HashMap;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::scene_manager::{ProjectSceneManagerBase, SceneManager};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;
use crate::application::application::Application;
use crate::game_module::character::character::{Character, CharacterCreateInfo};
use crate::game_module::character::character_manager::CharacterManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;

type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene_data_name: String,
    pub _characters: CharacterCreateInfoMap,
    pub _start_point: Vector3<f32>,
}

pub struct GameSceneManager {
    pub _effect_manager: *const EffectManager,
    pub _scene_manager: *const SceneManager,
    pub _game_resources: *const GameResources,
    pub _character_manager: *const CharacterManager,
    pub _game_scene_name: String,
}

impl ProjectSceneManagerBase for GameSceneManager {}

impl GameSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager> {
        Box::new(GameSceneManager {
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _game_scene_name: String::new(),
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application,
        engine_core: &EngineCore,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._character_manager = application.get_character_manager();
        self._game_resources = application.get_game_resources();
        engine_core.get_scene_manager_mut().initialize_scene_manager(
            self as *const dyn ProjectSceneManagerBase,
            engine_core.get_renderer_context(),
            engine_core.get_effect_manager(),
            engine_core.get_engine_resources(),
            window_size,
        )
    }

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        log::info!("open_game_scene_data: {:?}", game_scene_data_name);
        self._game_scene_name = String::from(game_scene_data_name);
        let game_resources = ptr_as_ref(self._game_resources);


        log::info!("1: {:?}", self._game_resources);

        if false == game_resources.has_game_scene_data(game_scene_data_name) {
            // TODO
        }

        log::info!("2");

        // load scene
        let game_scene_data = game_resources.get_game_scene_data(game_scene_data_name).borrow();
        let scene_data_name = &game_scene_data._scene_data_name;
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);

        log::info!("3");

        // character
        for (character_name, character_create_info) in game_scene_data._characters.iter() {
            let is_player = true;
            log::info!("4");
            ptr_as_mut(self._character_manager).create_character(character_name, character_create_info, is_player);
        }
        log::info!("5");
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_game_scene_manager(
        &mut self,
        engine_core: &EngineCore,
        delta_time: f64,
    ) {
        {
            let is_left = engine_core._keyboard_input_data.get_key_hold(VirtualKeyCode::Left);
            let is_right = engine_core._keyboard_input_data.get_key_hold(VirtualKeyCode::Right);

            if is_left || is_right {
                let character_manager = ptr_as_ref(self._character_manager);
                let pickle = character_manager.get_player();
                let mut object = pickle.borrow_mut();
                object._controller._position.x += delta_time as f32 * if is_left { 0.2 } else { -0.2 };
                object.update_character(delta_time as f32);
            }
        }

        self.get_scene_manager_mut()
            .update_scene_manager(engine_core, delta_time);
    }
}
