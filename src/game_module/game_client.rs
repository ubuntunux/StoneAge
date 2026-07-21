use crate::application::application::Application;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::{
    AMBIENT_SOUND, CAMERA_DISTANCE_MAX, DEFAULT_BGM_VOLUME, DEFAULT_FADE_TIME, DEFAULT_GAME_SAVE_DATA,
    DEFAULT_GATE_NAME, GAME_MUSIC, GAME_VIEW_MODE, GameViewMode, MATERIAL_INTRO_IMAGE, MATERIAL_UI_NONE,
    MATERIAL_WORLDMAP_FADE_TIME,
};
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, GameSceneState};
use crate::game_module::game_ui_manager::{EditorUIManager, GameUIManager};
use crate::game_module::save_data::save_data::GameSaveData;
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::utilities::system::{BoxRefCell, State, newBoxRefCell, ptr_as_mut, ptr_as_ref};
use std::cmp::PartialEq;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum GamePhase {
    None,
    Start,
    TitleScreen,
    BeginLoading,
    LoadingProgress,
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
    pub _next_game_phase: GamePhase,
    pub _game_save_data_name: String,
    pub _game_save_data: BoxRefCell<GameSaveData>,
    pub _request_load_game_save_data: bool,
    pub _request_new_game: bool,
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
            _game_phase: GamePhase::None,
            _next_game_phase: GamePhase::Start,
            _game_save_data_name: DEFAULT_GAME_SAVE_DATA.to_string(),
            _game_save_data: newBoxRefCell(GameSaveData::default()),
            _request_load_game_save_data: false,
            _request_new_game: false,
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
        self._editor_ui_manager = application.get_editor_ui_manager();
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
        if is_game_mode && self.get_game_scene_manager().get_character_manager().is_valid_player() {
            let main_camera = self.get_game_controller().get_main_camera();
            let player = self.get_game_scene_manager().get_character_manager().get_player();
            let mut player_position =
                main_camera.get_camera_position() + CAMERA_DISTANCE_MAX * main_camera.get_camera_front();
            if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                player_position.z = player.borrow().get_position().z;
            }
            player.borrow_mut().set_position(&player_position);
            player.borrow_mut().set_on_ground(player_position.y, &Vector3::new(0.0, 1.0, 0.0));
        }
    }
    pub fn is_game_phase(&self, game_phase: GamePhase) -> bool {
        self._game_phase == game_phase
    }
    pub fn is_next_game_phase(&self, next_game_phase: GamePhase) -> bool {
        self._next_game_phase == next_game_phase
    }
    pub fn set_next_game_phase(&mut self, next_game_phase: GamePhase) {
        self._next_game_phase = next_game_phase;
    }
    pub fn get_game_save_data(&self) -> &BoxRefCell<GameSaveData> {
        &self._game_save_data
    }
    pub fn request_load_game(&mut self, game_save_data_name: &str) {
        self._request_load_game_save_data = true;
        self._game_save_data_name = game_save_data_name.to_string();
        self.set_next_game_phase(GamePhase::BeginLoading);
    }
    pub fn request_new_game(&mut self) {
        self._request_new_game = true;
        self._game_save_data_name = DEFAULT_GAME_SAVE_DATA.to_string();
        self.set_next_game_phase(GamePhase::BeginLoading);
    }
    fn new_game(&mut self) {
        self._game_save_data = newBoxRefCell(GameSaveData::default());
        self.get_game_scene_manager_mut().close_game_scene_data();
        self.get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioIntro_Intro);
    }
    fn load_game(&mut self) {
        let game_save_data =
            self.get_game_resources_mut().get_game_save_data(self._game_save_data_name.as_str()).clone();
        self._game_save_data = newBoxRefCell(game_save_data.borrow().clone());
        self.get_game_scene_manager_mut().load_game_save_data(&mut self._game_save_data.borrow_mut());
    }
    pub fn save_game(&self, save_file: bool) {
        self.get_game_scene_manager().update_game_save_data(&mut self._game_save_data.borrow_mut());
        if save_file {
            self.get_game_resources_mut()
                .save_game_save_data(self._game_save_data_name.as_str(), &self._game_save_data.borrow());
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

        let game_phase = self._game_phase;
        let next_game_phase = self._next_game_phase;

        for state in State::iter() {
            if game_phase == next_game_phase && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_game_phase: GamePhase = match state {
                State::End => game_phase,
                State::Begin => {
                    self._game_phase = next_game_phase;
                    next_game_phase
                }
                State::Update => next_game_phase,
            };

            match update_game_phase {
                GamePhase::None => {}
                GamePhase::Start => {
                    if state == State::Update {
                        game_ui_manager.set_image_auto_fade_inout(MATERIAL_INTRO_IMAGE, 0.0);
                        game_scene_manager.play_bgm(GAME_MUSIC, DEFAULT_BGM_VOLUME);
                        game_scene_manager.play_ambient_sound(AMBIENT_SOUND, None);
                        self.set_next_game_phase(GamePhase::TitleScreen);
                    }
                }
                GamePhase::TitleScreen => {
                    if state == State::Update && any_key_pressed {
                        self.set_next_game_phase(GamePhase::BeginLoading);
                    }
                }
                GamePhase::BeginLoading => match state {
                    State::Begin => {
                        game_ui_manager.show_game_ui(false);
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() {
                            if self._request_new_game {
                                self.new_game();
                                self._request_new_game = false;
                                self.set_next_game_phase(GamePhase::LoadingProgress);
                            } else if self._request_load_game_save_data {
                                self.load_game();
                                self._request_load_game_save_data = false;
                                self.set_next_game_phase(GamePhase::LoadingProgress);
                            } else {
                                self.request_new_game();
                            }
                        }
                    }
                    State::End => {}
                },
                GamePhase::LoadingProgress => {
                    if state == State::Update && game_scene_manager.is_game_scene_state(GameSceneState::LoadCompleted) {
                        game_ui_manager.set_auto_fade_inout(true);
                        game_controller.set_game_camera_goal_transform(
                            1.0,
                            scene_manager.get_main_camera()._transform_object.get_pitch(),
                            scene_manager.get_main_camera()._transform_object.get_yaw(),
                        );
                        self.set_next_game_phase(GamePhase::GamePlay);
                    }
                }
                GamePhase::GameMenu => match state {
                    State::Begin => {
                        game_ui_manager.set_cross_hair_visible(true);
                        game_ui_manager.open_game_menu();
                    }
                    State::Update => {
                        if !game_ui_manager.is_opened_game_menu() {
                            self.set_next_game_phase(GamePhase::GamePlay);
                        }
                        game_ui_manager.update_game_menu_widget(joystick_input_data, keyboard_input_data);
                    }
                    State::End => {
                        game_ui_manager.set_cross_hair_visible(false);
                    }
                },
                GamePhase::GamePlay => match state {
                    State::Begin => {
                        game_ui_manager.show_game_ui(true);
                    }
                    State::Update => {
                        if game_scene_manager.is_game_scene_state(GameSceneState::LoadCompleted)
                            && character_manager.is_valid_player()
                        {
                            if game_scene_manager.is_play_scenario_mode() {
                                self.set_next_game_phase(GamePhase::PlayGameScenario);
                            } else if game_scene_manager.is_teleport_mode() {
                                self.set_next_game_phase(GamePhase::Teleport);
                            } else {
                                if game_controller.is_game_camera_auto_blend_mode() {
                                    game_controller.update_game_camera_auto_blend(
                                        scene_manager.get_main_camera_mut(),
                                        character_manager.get_player(),
                                        time_data._delta_time_with_scale as f32,
                                    );
                                } else {
                                    game_controller.update_game_controller(
                                        time_data,
                                        joystick_input_data,
                                        keyboard_input_data,
                                        mouse_move_data,
                                        mouse_input_data,
                                        &mouse_delta,
                                        scene_manager.get_main_camera_mut(),
                                        character_manager.get_player(),
                                    );
                                }
                            }
                        }
                    }
                    State::End => {}
                },
                GamePhase::PlayGameScenario => match state {
                    State::Begin => {
                        game_ui_manager.show_game_ui(false);
                    }
                    State::Update => {
                        if !game_scene_manager.is_play_scenario_mode() {
                            self.set_next_game_phase(GamePhase::GamePlay);
                        }
                    }
                    State::End => {
                        game_controller.set_game_camera_auto_blend_mode(true);
                    }
                },
                GamePhase::Teleport => match state {
                    State::Begin => {
                        if character_manager.is_valid_player() {
                            character_manager.get_player().borrow_mut().set_action_none();
                            character_manager.get_player().borrow_mut().set_move_idle();
                        }
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() {
                            game_scene_manager.update_teleport(character_manager);
                            if !game_scene_manager.is_teleport_mode() {
                                self.set_next_game_phase(GamePhase::GamePlay);
                            }
                        }
                    }
                    State::End => {
                        game_ui_manager.set_auto_fade_inout(true);
                        if character_manager.is_valid_player() {
                            character_manager.get_player().borrow_mut().set_action_none();
                            character_manager.get_player().borrow_mut().set_move_idle();
                        }
                    }
                },
                GamePhase::OpenToolbox => match state {
                    State::Begin => {
                        game_ui_manager.open_toolbox();
                    }
                    State::Update => {
                        if game_ui_manager.is_opened_toolbox() {
                            game_ui_manager.update_toolbox_widget(
                                time_data,
                                joystick_input_data,
                                keyboard_input_data,
                                mouse_move_data,
                                mouse_input_data,
                                &mouse_delta,
                                character_manager.get_player(),
                            );
                        } else {
                            self.set_next_game_phase(GamePhase::GamePlay);
                        }
                    }
                    State::End => {
                        game_ui_manager.close_toolbox();
                    }
                },
                GamePhase::WorldMapOpen => match state {
                    State::Begin => {
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, MATERIAL_WORLDMAP_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() {
                            game_ui_manager.set_text_box_visible(false);
                            game_ui_manager.set_cross_hair_visible(true);
                            game_ui_manager.set_auto_fade_inout(true);
                            game_ui_manager.open_world_map();
                            game_ui_manager.set_selected_world_map_stage(
                                self.get_game_scene_manager().get_current_game_scene_data_name(),
                            );
                            self.set_next_game_phase(GamePhase::WorldMapUpdate);
                        }
                    }
                    State::End => {}
                },
                GamePhase::WorldMapUpdate => match state {
                    State::Begin => {}
                    State::Update => {
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
                    State::End => {}
                },
                GamePhase::WorldMapClose => {
                    if state == State::Update {
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, MATERIAL_WORLDMAP_FADE_TIME);
                        if game_ui_manager.is_done_manual_fade_out() || game_ui_manager.is_done_game_image_progress() {
                            game_ui_manager.set_text_box_visible(true);
                            game_ui_manager.set_cross_hair_visible(false);
                            game_ui_manager.unset_selected_world_map_stage();
                            game_ui_manager.close_world_map();
                            // note: Pay attention to the order of operations. is_teleport_stage -> update_teleport
                            if !game_scene_manager.is_teleport_stage() {
                                game_ui_manager.set_auto_fade_inout(true);
                            }
                            game_scene_manager.update_teleport(character_manager);
                            self.set_next_game_phase(GamePhase::GamePlay);
                        }
                    }
                }
                GamePhase::ExitGame => {}
            }
        }

        game_scene_manager.update_game_scene_manager(any_key_hold, any_key_pressed, delta_time);
    }
}
