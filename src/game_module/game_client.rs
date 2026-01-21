use crate::application::application::Application;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::{
    AMBIENT_SOUND, CAMERA_DISTANCE_MAX, GAME_MUSIC, MATERIAL_INTRO_IMAGE, SCENARIO_INTRO,
    SLEEP_TIMER, STORY_BOARD_FADE_TIME, STORY_IMAGE_NONE, GAME_MODE_2D
};
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, GameSceneState};
use crate::game_module::game_ui_manager::{EditorUIManager, GameUIManager};
use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum GamePhase {
    Start,
    Loading,
    GamePlay,
    PlayGameScenario,
    Teleport,
    Sleep,
    OpenToolbox,
}

pub struct GameClient<'a> {
    pub _engine_core: *const EngineCore<'a>,
    pub _application: *const Application<'a>,
    pub _character_manager: *const CharacterManager<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _game_controller: *const GameController<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _editor_ui_manager: *const EditorUIManager<'a>,
    pub _game_phase: GamePhase,
    pub _need_sleep_mode: bool,
    pub _need_toolbox_mode: bool,
    pub _sleep_timer: f32,
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
            _editor_ui_manager: std::ptr::null(),
            _game_phase: GamePhase::Start,
            _need_sleep_mode: false,
            _need_toolbox_mode: false,
            _sleep_timer: 0.0,
        })
    }

    pub fn initialize_game_client(
        &mut self,
        engine_core: *const EngineCore<'a>,
        application: &Application<'a>,
    ) {
        log::info!("initialize_game_client");
        self._engine_core = engine_core;
        self._application = application;
        self._game_controller = application.get_game_controller();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_ui_manager = application.get_game_ui_manager();
        self._editor_ui_manager = application.get_editor_ui_manager();
        self._need_sleep_mode = false;
        self._sleep_timer = 0.0;
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
    pub fn get_editor_ui_manager(&self) -> &EditorUIManager<'a> {
        ptr_as_ref(self._editor_ui_manager)
    }
    pub fn get_editor_ui_manager_mut(&self) -> &mut EditorUIManager<'a> {
        ptr_as_mut(self._editor_ui_manager)
    }
    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self.get_editor_ui_manager_mut().show_editor_ui(!is_game_mode);
        self.get_game_ui_manager_mut().show_ui(is_game_mode);
        if is_game_mode {
            if self.get_game_scene_manager().get_character_manager().is_valid_player() {
                let main_camera = self.get_game_controller().get_main_camera();
                let player = self.get_game_scene_manager().get_character_manager().get_player();
                let mut player_position = main_camera.get_camera_position() + CAMERA_DISTANCE_MAX * main_camera.get_camera_front();
                if GAME_MODE_2D {
                    player_position.z = player.borrow().get_position().z;
                }
                player.borrow_mut().set_position(&player_position);
            }
        }
    }
    pub fn set_need_sleep_mode(&mut self, need_sleep_mode: bool) {
        self._need_sleep_mode = need_sleep_mode;
    }
    pub fn need_sleep_mode(&self) -> bool {
        self._need_sleep_mode
    }
    pub fn set_need_toolbox_mode(&mut self, need_toolbox_mode: bool) {
        self._need_toolbox_mode = need_toolbox_mode;
    }
    pub fn need_toolbox_mode(&self) -> bool {
        self._need_toolbox_mode
    }
    pub fn reset_sleep_timer(&mut self) {
        self._sleep_timer = 0.0;
    }
    pub fn set_game_phase(&mut self, game_phase: GamePhase) {
        if self._game_phase != game_phase {
            log::debug!("set_game_phase: {:?}", game_phase);
            self.update_game_mode_end();
            self._game_phase = game_phase;
            self.update_game_mode_begin();
        }
    }
    pub fn update_game_mode_begin(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let character_manager = ptr_as_mut(game_scene_manager._character_manager.as_ref());
        let game_ui_manager = ptr_as_mut(self._game_ui_manager);
        match self._game_phase {
            GamePhase::Teleport => {
                if character_manager.is_valid_player() {
                    character_manager.get_player().borrow_mut().set_action_none();
                    character_manager.get_player().borrow_mut().set_move_stop();
                }
                game_ui_manager.set_image_manual_fade_inout(STORY_IMAGE_NONE, STORY_BOARD_FADE_TIME);
            }
            GamePhase::Sleep => {
                self.reset_sleep_timer();
                game_ui_manager.set_image_manual_fade_inout(STORY_IMAGE_NONE, STORY_BOARD_FADE_TIME);
                self.set_need_sleep_mode(false);
            }
            GamePhase::OpenToolbox => {
                game_ui_manager.open_toolbox();
                self.set_need_toolbox_mode(false);
            }
            _ => (),
        }
    }

    pub fn update_game_mode_end(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let character_manager = ptr_as_mut(game_scene_manager._character_manager.as_ref());
        let game_ui_manager = ptr_as_mut(self._game_ui_manager);
        let game_controller = ptr_as_mut(self._game_controller);
        match self._game_phase {
            GamePhase::Teleport => {
                game_ui_manager.set_auto_fade_inout(true);
                if character_manager.is_valid_player() {
                    character_manager.get_player().borrow_mut().set_action_none();
                    character_manager.get_player().borrow_mut().set_move_stop();
                }
            }
            GamePhase::Sleep => {
                character_manager.get_player().borrow_mut().set_action_stand_up();
            }
            GamePhase::PlayGameScenario => {
                game_controller.set_game_camera_auto_blend_mode(true);
            }
            GamePhase::OpenToolbox => {
                game_ui_manager.close_toolbox();
            }
            _ => (),
        }
    }

    pub fn update_game_mode(&mut self, delta_time: f64) {
        let engine_core = ptr_as_ref(self._engine_core);
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let scene_manager = ptr_as_mut(game_scene_manager._scene_manager);
        let character_manager = ptr_as_mut(game_scene_manager._character_manager.as_ref());
        let game_ui_manager = ptr_as_mut(self._game_ui_manager);
        let game_controller = ptr_as_mut(self._game_controller);

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
        let any_key_pressed = joystick_input_data.is_any_button_pressed()
            || mouse_input_data.is_any_button_pressed()
            || keyboard_input_data.is_any_key_pressed();
        let any_key_hold = joystick_input_data.is_any_button_hold()
            || mouse_input_data.is_any_button_hold()
            || keyboard_input_data.is_any_key_hold();

        if game_ui_manager.is_done_game_image_progress() {
            game_ui_manager.set_game_image_fade_speed(if any_key_hold { 5.0 } else { 1.0 });
        }

        match self._game_phase {
            GamePhase::Start => {
                game_ui_manager.set_image_auto_fade_inout(MATERIAL_INTRO_IMAGE, 0.0);
                game_scene_manager.play_bgm(GAME_MUSIC, Some(0.5));
                game_scene_manager.play_ambient_sound(AMBIENT_SOUND, None);
                self.set_game_phase(GamePhase::Loading);
            }
            GamePhase::Loading => {
                if game_scene_manager.is_game_scene_state(GameSceneState::None) {
                    game_scene_manager.open_scenario_data(SCENARIO_INTRO);
                } else if game_scene_manager.is_game_scene_state(GameSceneState::PlayGame) {
                    game_controller.set_game_camera_goal_transform(
                        1.0,
                        scene_manager.get_main_camera()._transform_object.get_pitch(),
                        scene_manager.get_main_camera()._transform_object.get_yaw()
                    );
                    self.set_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::GamePlay => {
                // reset
                // if keyboard_input_data.get_key_pressed(KeyCode::Enter) {
                //     game_scene_manager.clear_game_object_data();
                //     game_scene_manager.spawn_game_object_data();
                // }

                if game_scene_manager.is_game_scene_state(GameSceneState::PlayGame) {
                    if character_manager.is_valid_player() {
                        if game_scene_manager.has_scenario() {
                            self.set_game_phase(GamePhase::PlayGameScenario);
                        } else if game_scene_manager.is_teleport_mode() {
                            self.set_game_phase(GamePhase::Teleport);
                        } else if self.need_sleep_mode() {
                            self.set_game_phase(GamePhase::Sleep);
                        } else if self.need_toolbox_mode() {
                            self.set_game_phase(GamePhase::OpenToolbox);
                        } else {
                            if game_controller.is_game_camera_auto_blend_mode() {
                                game_controller.update_game_camera_auto_blend(
                                    scene_manager.get_main_camera_mut(),
                                    character_manager.get_player(),
                                    time_data._delta_time as f32
                                );
                            } else {
                                game_controller.update_game_controller(
                                    time_data,
                                    &joystick_input_data,
                                    &keyboard_input_data,
                                    &mouse_move_data,
                                    &mouse_input_data,
                                    &mouse_delta,
                                    scene_manager.get_main_camera_mut(),
                                    character_manager.get_player(),
                                );
                            }
                        }
                    }
                }
            }
            GamePhase::PlayGameScenario => {
                game_scene_manager.update_game_scenario(game_ui_manager, any_key_hold, any_key_pressed, delta_time);
                if game_scene_manager.has_scenario() == false {
                    self.set_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::Teleport => {
                if game_ui_manager.is_done_manual_fade_out() {
                    game_scene_manager.update_teleport(character_manager);
                    if game_scene_manager.is_teleport_mode() == false {
                        self.set_game_phase(GamePhase::GamePlay);
                    }
                }
            }
            GamePhase::Sleep => {
                if game_ui_manager.is_done_manual_fade_out() && self._sleep_timer < SLEEP_TIMER {
                    self._sleep_timer += delta_time as f32;
                    if SLEEP_TIMER <= self._sleep_timer {
                        game_ui_manager.set_auto_fade_inout(true);
                        game_scene_manager.set_next_time_of_day();
                    }
                } else if game_ui_manager.is_done_game_image_progress() {
                    self.set_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::OpenToolbox => {
                if game_ui_manager.is_opened_toolbox() {
                    game_ui_manager.update_toolbox_widget(
                        time_data,
                        &joystick_input_data,
                        &keyboard_input_data,
                        &mouse_move_data,
                        &mouse_input_data,
                        &mouse_delta,
                        character_manager.get_player(),
                    );
                } else {
                    self.set_game_phase(GamePhase::GamePlay);
                }
            }
        }
        game_scene_manager.update_game_scene_manager(delta_time);
    }
}
