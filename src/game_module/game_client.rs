use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

use crate::application::application::Application;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::{AMBIENT_SOUND, GAME_MUSIC, GAME_SCENE_INTRO, MATERIAL_INTRO_IMAGE};
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::GameUIManager;

pub enum GamePhase {
    None,
    Intro,
    GamePlay
}

pub struct GameClient<'a> {
    pub _engine_core: *const EngineCore<'a>,
    pub _application: *const Application<'a>,
    pub _character_manager: *const CharacterManager<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _game_controller: *const GameController<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _game_phase: GamePhase,
}

impl<'a> GameClient<'a> {
    pub fn create_game_client() -> Box<GameClient<'a>> {
        Box::new(GameClient {
            _engine_core: std::ptr::null(),
            _application: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _game_controller: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _game_phase: GamePhase::None,
        })
    }

    pub fn initialize_game_client(&mut self, engine_core: *const EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_game_client");
        self._engine_core = engine_core;
        self._application = application;
        self._game_controller = application.get_game_controller();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_ui_manager = application.get_game_ui_manager();
    }
    pub fn destroy_game_client(&mut self) {
        ptr_as_mut(self._game_ui_manager).destroy_game_ui_manager();
    }
    pub fn get_engine_core(&self) -> &EngineCore<'a> {
        ptr_as_ref(self._engine_core)
    }
    pub fn get_engine_core_mut(&self) -> &EngineCore<'a> {
        ptr_as_mut(self._engine_core)
    }
    pub fn get_application(&self) -> &Application<'a> {
        ptr_as_ref(self._application)
    }
    pub fn get_application_mut(&self) -> &mut Application<'a> {
        ptr_as_mut(self._application)
    }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager<'a> {
        ptr_as_ref(self._game_scene_manager)
    }
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager<'a> {
        ptr_as_mut(self._game_scene_manager)
    }
    pub fn get_game_resources(&self) -> &GameResources<'a> {
        ptr_as_ref(self._game_resources)
    }
    pub fn get_game_resources_mut(&self) -> &mut GameResources<'a> {
        ptr_as_mut(self._game_resources)
    }
    pub fn get_game_controller(&self) -> &GameController<'a> {
        ptr_as_ref(self._game_controller)
    }
    pub fn get_game_controller_mut(&self) -> &mut GameController<'a> {
        ptr_as_mut(self._game_controller)
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager<'a> {
        ptr_as_ref(self._game_ui_manager)
    }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager<'a> {
        ptr_as_mut(self._game_ui_manager)
    }
    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self.get_game_ui_manager_mut().show_ui(is_game_mode);
    }
    pub fn update_game_mode(&mut self, delta_time: f64) {
        let engine_core = self.get_engine_core();
        let game_scene_manager = self.get_game_scene_manager();
        let scene_manager = game_scene_manager.get_scene_manager();
        let time_data = &engine_core._time_data;
        let mouse_move_data = &engine_core._mouse_move_data;
        let mouse_input_data = &engine_core._mouse_input_data;
        let joystick_input_data = &engine_core._joystick_input_data;
        let keyboard_input_data = &engine_core._keyboard_input_data;
        let mouse_speed_ratio = 1.0;
        let mouse_delta: Vector2<f32> = Vector2::new(
            mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio,
            mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio,
        );

        match self._game_phase {
            GamePhase::None => {
                self._game_phase = GamePhase::Intro;
                self.get_game_ui_manager_mut().set_game_image_material_instance(MATERIAL_INTRO_IMAGE, 1.0);
                ptr_as_mut(self._game_scene_manager).play_bgm(GAME_MUSIC, Some(0.5));
                ptr_as_mut(self._game_scene_manager).play_ambient_sound(AMBIENT_SOUND, None);
            }
            GamePhase::Intro => {
                if self.get_game_ui_manager().is_visible_game_image() {
                    if joystick_input_data.is_any_button_pressed() ||
                        mouse_input_data.is_any_button_pressed() ||
                        keyboard_input_data.is_any_key_pressed() {
                        self.get_game_ui_manager_mut().start_game_image_fadeout(true);
                    }
                } else {
                    self.get_game_scene_manager_mut().open_game_scene_data(GAME_SCENE_INTRO);
                    self._game_phase = GamePhase::GamePlay;
                }
            }
            GamePhase::GamePlay => {
                let game_scene_manager = self.get_game_scene_manager();
                let player = game_scene_manager.get_character_manager().get_player();
                let main_camera = scene_manager.get_main_camera_mut();
                if false == self._game_controller.is_null() {
                    let game_controller = ptr_as_mut(self._game_controller);
                    game_controller.update_game_controller(
                        time_data,
                        &joystick_input_data,
                        &keyboard_input_data,
                        &mouse_move_data,
                        &mouse_input_data,
                        &mouse_delta,
                        main_camera,
                        player,
                    );
                }
                ptr_as_mut(self._game_scene_manager).update_game_scene_manager(delta_time);
            }
        }
    }
}
