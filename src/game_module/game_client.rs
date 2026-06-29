use std::cmp::PartialEq;
use crate::application::application::Application;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::{GameViewMode, AMBIENT_SOUND, CAMERA_DISTANCE_MAX, GAME_MUSIC, MATERIAL_INTRO_IMAGE, MATERIAL_UI_NONE, GAME_VIEW_MODE, MATERIAL_WORLDMAP_FADE_TIME, DEFAULT_GATE_NAME, DEFAULT_FADE_TIME, DEFAULT_BGM_VOLUME};
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, GameSceneState};
use crate::game_module::game_ui_manager::{EditorUIManager, GameUIManager};
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use crate::game_module::scenario::game_scenarios::scenario_day_one;
use crate::game_module::scenario::scenario::{ScenarioType};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum GamePhase {
    Start,
    Loading,
    GameMenu,
    GamePlay,
    PlayGameScenario,
    Teleport,
    OpenToolbox,
    WorldMapOpen,
    WorldMapUpdate,
    WorldMapClose,
    ExitGame,
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
    pub _next_game_phase: GamePhase
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
            _next_game_phase: GamePhase::Start
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
        self._next_game_phase = self._game_phase;
    }
    pub fn destroy_game_client(&mut self) {
        ptr_as_mut(self._game_ui_manager).destroy_game_ui_manager();
    }
    pub fn is_game_over(&self) -> bool {
        self.is_game_phase(GamePhase::ExitGame)
    }
    pub fn exit_game(&mut self) {
        self.set_next_game_phase(GamePhase::ExitGame);
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
        self.get_game_ui_manager_mut().show_game_ui(is_game_mode);
        if is_game_mode {
            if self.get_game_scene_manager().get_character_manager().is_valid_player() {
                let main_camera = self.get_game_controller().get_main_camera();
                let player = self.get_game_scene_manager().get_character_manager().get_player();
                let mut player_position = main_camera.get_camera_position() + CAMERA_DISTANCE_MAX * main_camera.get_camera_front();
                if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                    player_position.z = player.borrow().get_position().z;
                }
                player.borrow_mut().set_position(&player_position);
                player.borrow_mut().set_on_ground(player_position.y, &Vector3::new(0.0, 1.0, 0.0));
            }
        }
    }
    pub fn is_game_phase(&self, game_phase: GamePhase) -> bool {
        self._game_phase == game_phase
    }
    pub fn set_next_game_phase(&mut self, next_game_phase: GamePhase) {
        self._next_game_phase = next_game_phase;
    }
    fn set_game_phase(&mut self, game_phase: GamePhase) {
        if self._game_phase != game_phase {
            self.update_game_mode_end();
            self._game_phase = game_phase;
            self.update_game_mode_begin();
        }
    }
    pub fn load_game(&mut self) {
        //self.get_game_scene_manager_mut().load_game_scene();
    }
    pub fn save_game(&mut self) {
        //self.get_game_scene_manager_mut().save_game_scene();
    }
    fn update_game_mode_begin(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let character_manager = ptr_as_mut(game_scene_manager._character_manager.as_ref());
        let game_ui_manager = ptr_as_mut(self._game_ui_manager);

        match self._game_phase {
            GamePhase::GameMenu => {
                game_ui_manager.set_cross_hair_visible(true);
                game_ui_manager.open_game_menu();
            }
            GamePhase::GamePlay => {
                game_ui_manager.show_game_ui(true);
            }
            GamePhase::Loading => {
                const SKIP_SCENARIO: bool = true;
                if SKIP_SCENARIO {
                    game_scene_manager.open_game_scenario(ScenarioType::ScenarioDayOne);
                    unsafe { scenario_day_one::SKIP_SCENARIO = true; }
                    game_ui_manager.set_image_auto_fade_inout(MATERIAL_UI_NONE, MATERIAL_WORLDMAP_FADE_TIME);
                } else {
                    game_scene_manager.open_game_scenario(ScenarioType::ScenarioIntro);
                }
            }
            GamePhase::PlayGameScenario => {
                game_ui_manager.show_game_ui(false);
            }
            GamePhase::Teleport => {
                if character_manager.is_valid_player() {
                    character_manager.get_player().borrow_mut().set_action_none();
                    character_manager.get_player().borrow_mut().set_move_idle();
                }
                game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
            }
            GamePhase::OpenToolbox => {
                game_ui_manager.open_toolbox();
            }
            GamePhase::WorldMapOpen => {
                game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, MATERIAL_WORLDMAP_FADE_TIME);
            }
            GamePhase::WorldMapClose => {
                game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, MATERIAL_WORLDMAP_FADE_TIME);
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
                    character_manager.get_player().borrow_mut().set_move_idle();
                }
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

        if self._next_game_phase != self._game_phase {
            self.set_game_phase(self._next_game_phase);
        }

        match self._game_phase {
            GamePhase::Start => {
                game_ui_manager.set_image_auto_fade_inout(MATERIAL_INTRO_IMAGE, 0.0);
                game_scene_manager.play_bgm(GAME_MUSIC, DEFAULT_BGM_VOLUME);
                game_scene_manager.play_ambient_sound(AMBIENT_SOUND, None);
                self.set_next_game_phase(GamePhase::Loading);
            }
            GamePhase::Loading => {
                if game_scene_manager.is_game_scene_state(GameSceneState::LoadCompleted) {
                    game_controller.set_game_camera_goal_transform(
                        1.0,
                        scene_manager.get_main_camera()._transform_object.get_pitch(),
                        scene_manager.get_main_camera()._transform_object.get_yaw()
                    );
                    self.set_next_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::GameMenu => {
                game_ui_manager.update_game_menu_widget(joystick_input_data, keyboard_input_data);

                if game_ui_manager.is_opened_game_menu() == false {
                    game_ui_manager.set_cross_hair_visible(false);
                    self.set_next_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::GamePlay => {
                if game_scene_manager.is_game_scene_state(GameSceneState::LoadCompleted) {
                    if character_manager.is_valid_player() {
                        if game_scene_manager.is_play_scenario_mode() {
                            self.set_next_game_phase(GamePhase::PlayGameScenario);
                        } else if game_scene_manager.is_teleport_mode() {
                            self.set_next_game_phase(GamePhase::Teleport);
                        } else {
                            if game_controller.is_game_camera_auto_blend_mode() {
                                game_controller.update_game_camera_auto_blend(
                                    scene_manager.get_main_camera_mut(),
                                    character_manager.get_player(),
                                    time_data._delta_time_with_scale as f32
                                );
                            } else {
                                game_controller.update_game_controller(
                                    time_data,
                                    joystick_input_data,
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
                if game_scene_manager.is_play_scenario_mode() == false {
                    self.set_next_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::Teleport => {
                if game_ui_manager.is_done_manual_fade_out() {
                    game_scene_manager.update_teleport(character_manager);
                    if game_scene_manager.is_teleport_mode() == false {
                        self.set_next_game_phase(GamePhase::GamePlay);
                    }
                }
            }
            GamePhase::OpenToolbox => {
                if game_ui_manager.is_opened_toolbox() {
                    game_ui_manager.update_toolbox_widget(
                        time_data,
                        joystick_input_data,
                        keyboard_input_data,
                        &mouse_move_data,
                        &mouse_input_data,
                        &mouse_delta,
                        character_manager.get_player(),
                    );
                } else {
                    self.set_next_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::WorldMapOpen => {
                if game_ui_manager.is_done_manual_fade_out() {
                    game_ui_manager.set_text_box_visible(false);
                    game_ui_manager.set_cross_hair_visible(true);
                    game_ui_manager.set_auto_fade_inout(true);
                    game_ui_manager.open_world_map();
                    game_ui_manager.set_selected_world_map_stage(self.get_game_scene_manager().get_current_game_scene_data_name());
                    self.set_next_game_phase(GamePhase::WorldMapUpdate);
                }
            }
            GamePhase::WorldMapUpdate => {
                game_ui_manager.update_world_map_widget(joystick_input_data, keyboard_input_data);

                if game_scene_manager.is_teleport_mode() {
                    self.set_next_game_phase(GamePhase::WorldMapClose);
                } else if game_ui_manager.is_requested_close_world_map() {
                    self.get_game_scene_manager_mut().set_teleport_stage(
                        self.get_game_scene_manager().get_current_game_scene_data_name(),
                        DEFAULT_GATE_NAME,
                    );
                    self.set_next_game_phase(GamePhase::WorldMapClose);
                }
            }
            GamePhase::WorldMapClose => {
                if game_ui_manager.is_done_manual_fade_out() || game_ui_manager.is_done_game_image_progress() {
                    game_ui_manager.set_text_box_visible(true);
                    game_ui_manager.set_cross_hair_visible(false);
                    game_ui_manager.set_auto_fade_inout(true);
                    game_ui_manager.unset_selected_world_map_stage();
                    game_ui_manager.close_world_map();
                    game_scene_manager.update_teleport(character_manager);
                    self.set_next_game_phase(GamePhase::GamePlay);
                }
            }
            GamePhase::ExitGame => {
            }
        }
        game_scene_manager.update_game_scenario(any_key_hold, any_key_pressed, delta_time);
        game_scene_manager.update_game_scene_manager(delta_time);
    }
}
