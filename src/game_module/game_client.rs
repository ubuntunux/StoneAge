use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

use crate::application::application::Application;
use crate::game_module::character::character_manager::CharacterManager;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::game_resource::GameResources;

pub struct GameClient {
    pub _audio_manager: *const AudioManager,
    pub _effect_manager: *const EffectManager,
    pub _application: *const Application,
    pub _character_manager: *const CharacterManager,
    pub _game_scene_manager: *const GameSceneManager,
    pub _game_resources: *const GameResources,
    pub _game_controller: *const GameController,
    pub _game_ui_manager: *const GameUIManager,
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _application: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _game_controller: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
        })
    }

    pub fn initialize_game_client(&mut self, application: &Application) {
        log::info!("initialize_game_client");
        self._application = application;
        self._character_manager = application.get_character_manager();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_ui_manager = application.get_game_ui_manager();
        self._audio_manager = application.get_audio_manager();
        self._effect_manager = application.get_effect_manager();
    }
    pub fn destroy_game_client(&mut self) {
        ptr_as_mut(self._game_ui_manager).destroy_game_ui_manager();
    }
    pub fn get_audio_manager(&self) -> &AudioManager {
        ptr_as_ref(self._audio_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_effect_manager(&self) -> &EffectManager {
        ptr_as_ref(self._effect_manager)
    }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager {
        ptr_as_mut(self._effect_manager)
    }
    pub fn get_application(&self) -> &Application {
        ptr_as_ref(self._application)
    }
    pub fn get_application_mut(&self) -> &mut Application {
        ptr_as_mut(self._application)
    }
    pub fn get_character_manager(&self) -> &CharacterManager { ptr_as_ref(self._character_manager) }
    pub fn get_character_manager_mut(&self) -> &mut CharacterManager { ptr_as_mut(self._character_manager) }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager { ptr_as_ref(self._game_scene_manager) }
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager { ptr_as_mut(self._game_scene_manager)}
    pub fn get_game_resources(&self) -> &GameResources { ptr_as_ref(self._game_resources) }
    pub fn get_game_resources_mut(&self) -> &mut GameResources {
        ptr_as_mut(self._game_resources)
    }
    pub fn get_game_controller(&self) -> &GameController { ptr_as_ref(self._game_controller) }
    pub fn get_game_controller_mut(&self) -> &mut GameController { ptr_as_mut(self._game_controller) }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager) }
    pub fn start_game(&mut self) {
        log::info!("start_game");
        self.get_game_ui_manager_mut().build_game_ui();
        self.get_game_scene_manager_mut()
            .open_game_scene_data("intro_stage");
    }

    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        let game_ui_layout_mut = ptr_as_mut(self.get_game_ui_manager().game_ui_layout());
        game_ui_layout_mut
            .get_ui_component_mut()
            .set_visible(is_game_mode);
    }

    pub fn update_game_event(&mut self) {
        let application = ptr_as_ref(self._application);
        let engine_core = application.get_engine_core();
        let game_scene_manager = application.get_game_scene_manager();
        let scene_manager = game_scene_manager.get_scene_manager();
        let time_data = &engine_core._time_data;
        let mouse_move_data = &engine_core._mouse_move_data;
        let mouse_input_data = &engine_core._mouse_input_data;
        let keyboard_input_data = &engine_core._keyboard_input_data;
        let mouse_speed_ratio = engine_core._window_size.y as f32 / 1080.0;
        let mouse_delta: Vector2<f32> = Vector2::new(
            mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio,
            mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio,
        );
        let _scroll_delta = &mouse_move_data._scroll_delta;
        let main_camera = scene_manager.get_main_camera_mut();
        ptr_as_mut(self._game_controller).update_game_event(
            time_data,
            &keyboard_input_data,
            &mouse_move_data,
            &mouse_input_data,
            &mouse_delta,
            main_camera,
        );
    }

    pub fn update_game_client(&mut self, delta_time: f32) {
        ptr_as_mut(self._game_controller).update_game_controller(delta_time);
        ptr_as_mut(self._game_ui_manager).update_game_ui(delta_time);
    }
}
