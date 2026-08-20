use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::*;
use crate::game_module::game_service_locator::{
    get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::scenario::game_scenarios::scenario_wrap_up_the_day::ScenarioWrapUpTheDay;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxLayerType};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, RcRefCell, State};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const USE_STORY_BOARDS: bool = false;
const PHASE_TIME_SLEEP: f32 = 5.0;
const PHASE_TIME_HUNGRY: f32 = 3.0;

pub const STORY_BOARDS: [&str; 2] = [
    "ui/story_board/story_board_intro_00",
    "ui/story_board/story_board_intro_01",
];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    StoryBoard,
    Morning,
    WakeUp,
    AssembleFamily,
    IamHungry,
    WrapUpTheDay,
    Sleeping,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioIntroSaveData {
    pub _scenario_create_info: ScenarioDataCreateInfo,
    pub _scenario_phase: String,
    pub _next_scenario_phase: String,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
    pub _story_board_phase: usize,
    pub _wakeup_delay_aru: f32,
    pub _wakeup_delay_ewa: f32,
    pub _wakeup_delay_koa: f32,
}

pub struct ScenarioIntro<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,

    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_gate: Option<RcRefCell<Prop<'a>>>,
    _prop_gate_stage01: Option<RcRefCell<Prop<'a>>>,
    _prop_tree: Option<RcRefCell<Prop<'a>>>,
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _wakeup_delay_aru: f32,
    _wakeup_delay_ewa: f32,
    _wakeup_delay_koa: f32,
    _camera_direction: Vector3<f32>,
    _camera_distance: f32,
    _around_start_position: Vector3<f32>,
    _around_end_position: Vector3<f32>,
    _around_start_rotation: Vector3<f32>,
    _around_end_rotation: Vector3<f32>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
    _story_board_phase: usize,
}

impl<'a> ScenarioIntro<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioIntro<'a>> {
        newRcRefCell(ScenarioIntro {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_gate: None,
            _prop_gate_stage01: None,
            _prop_tree: None,
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _wakeup_delay_aru: 2.0,
            _wakeup_delay_ewa: 3.5,
            _wakeup_delay_koa: 4.0,
            _camera_direction: Default::default(),
            _camera_distance: 0.0,
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: Vector3::new(0.4, 0.0, 0.0),
            _around_end_rotation: Vector3::new(0.35, 0.0, 0.0),
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
                _next_phase_duration: None,
            },
            _story_board_phase: 0,
        })
    }

    pub fn get_story_board_phase(&self) -> usize {
        self._story_board_phase
    }
    pub fn clear_story_board_phase(&mut self) {
        self._story_board_phase = 0;
    }
    pub fn next_story_board_phase(&mut self) {
        self._story_board_phase += 1;
    }

    pub fn update_assemble(&self, actor: &RcRefCell<Character<'a>>, target: &RcRefCell<Character<'a>>) -> bool {
        let radius = target.borrow().get_collision()._bounding_box._mag_xz + 0.5;
        let (direction, dist) =
            math::make_normalize_xz_with_norm(&(target.borrow().get_position() - actor.borrow().get_position()));
        if radius < dist {
            actor.borrow_mut().set_move(&direction);
            return false;
        }
        true
    }

    pub fn emoji_hungry(&self, game_ui_manager: &mut GameUIManager<'a>, actor: &RcRefCell<Character<'a>>) {
        let contents = vec![TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY))];
        game_ui_manager.add_text_box_item(
            TextBoxLayerType::InteractionLayer,
            ActorWrapper::Character(actor.clone()),
            &contents,
            Some(CHARACTER_INTERACTION_TIME),
        );
        actor.borrow_mut().set_move_idle();
    }

    pub fn create_wrap_up_the_day_text_box(&self) {
        if let Some(prop) = self._prop_table.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Wrap up the day.\""))];
            get_game_ui_manager_mut().add_text_box_item(TextBoxLayerType::GamePlayLayer, wrapper, &contents, None);
        }
    }

    pub fn remove_wrap_up_the_day_text_box(&self) {
        if let Some(prop) = self._prop_table.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn clear_all(&mut self) {
        self.remove_wrap_up_the_day_text_box();

        self._player = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_gate = None;
        self._prop_gate_stage01 = None;
        self._prop_tree = None;
        self._prop_bed_for_aru = None;
        self._prop_bed_for_ewa = None;
        self._prop_bed_for_koa = None;
    }
}

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        self._scenario_track._scenario_phase =
            ScenarioPhase::from_str(scenario_phase.as_str()).unwrap_or(ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = self._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(data) = serde_json::from_str::<ScenarioIntroSaveData>(&scenario_save_data._scenario_data) {
            self._story_board_phase = data._story_board_phase;
            self._wakeup_delay_aru = data._wakeup_delay_aru;
            self._wakeup_delay_ewa = data._wakeup_delay_ewa;
            self._wakeup_delay_koa = data._wakeup_delay_koa;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioIntroSaveData {
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_phase: self._scenario_track._scenario_phase.to_string(),
            _next_scenario_phase: self._scenario_track._next_scenario_phase.to_string(),
            _phase_duration: self._scenario_track._phase_duration,
            _next_phase_duration: self._scenario_track._next_phase_duration,
            _phase_time: self._scenario_track._phase_time,
            _story_board_phase: self._story_board_phase,
            _wakeup_delay_aru: self._wakeup_delay_aru,
            _wakeup_delay_ewa: self._wakeup_delay_ewa,
            _wakeup_delay_koa: self._wakeup_delay_koa,
        };

        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: self._scenario_track.save_scenario_track_data(),
            _scenario_data: serde_json::to_string(&save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::WrapUpTheDay => false,
            _ => true,
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        self.clear_all();
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.clear_all();
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager_mut();
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }

        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager.get_actor_by_name("monkey_ewa").cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name("monkey_koa").cloned();
        self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name("table").cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
        self._prop_tree = game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00").cloned();
        self._prop_gate_stage01 = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin => {
                let mut pivot = Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                if let Some(actor) = self._player.as_ref() {
                    pivot += *actor.borrow().get_center();
                };
                let start_rotation_matrix = math::make_rotation_matrix(
                    self._around_start_rotation.x,
                    self._around_start_rotation.y,
                    self._around_start_rotation.z,
                );
                self._around_start_position =
                    pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

                let end_rotation_matrix = math::make_rotation_matrix(
                    self._around_end_rotation.x,
                    self._around_end_rotation.y,
                    self._around_end_rotation.z,
                );
                self._around_end_position = pivot - end_rotation_matrix.column(2).xyz() * CAMERA_DISTANCE_MIN;

                let main_camera = get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);

                self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
            }
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();
        let game_ui_manager = get_game_ui_manager_mut();

        if TIME_OF_MORNING <= game_scene_manager.get_time_of_day() {
            game_scene_manager.set_time_of_day_speed(1.0);
        }

        let prev_scenario_phase = self._scenario_track._scenario_phase;
        let next_scenario_phase = self._scenario_track._next_scenario_phase;
        let next_phase_duration = self._scenario_track._next_phase_duration;

        for state in State::iter() {
            if prev_scenario_phase == next_scenario_phase && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_scenario_phase: ScenarioPhase = match state {
                State::End => prev_scenario_phase,
                State::Begin => {
                    self._scenario_track.set_scenario_phase(next_scenario_phase, next_phase_duration);
                    next_scenario_phase
                }
                State::Update => next_scenario_phase,
            };

            let phase_time = self._scenario_track.get_phase_time();
            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
                }
                ScenarioPhase::StoryBoard => match state {
                    State::Begin => {
                        game_scene_manager.set_time(TIME_OF_DAWN, 0.0);
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_behavior_none();
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_behavior_none();
                        }
                        if let Some(actor) = &self._player {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                    }
                    State::Update => {
                        let story_board_phase = self.get_story_board_phase();
                        if !USE_STORY_BOARDS || STORY_BOARDS.len() < story_board_phase {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Morning, Some(PHASE_TIME_SLEEP));
                        } else {
                            if story_board_phase == 0
                                || game_ui_manager.is_done_game_image_progress() && any_key_pressed
                            {
                                if story_board_phase < STORY_BOARDS.len() {
                                    game_ui_manager
                                        .set_image_auto_fade_inout(STORY_BOARDS[story_board_phase], DEFAULT_FADE_TIME);
                                } else {
                                    game_ui_manager.set_image_auto_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                                }
                                self.next_story_board_phase();
                            }
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Morning => match state {
                    State::Begin => {
                        if let Some(actor) = &self._player {
                            actor.borrow_mut().set_action_sleep();
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_action_sleep();
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_action_sleep();
                        }
                    }
                    State::Update => {
                        let main_camera = get_scene_manager().get_main_camera_mut();
                        let progress = 1.0 - (phase_ratio * -5.0).exp2();
                        let position = self._around_start_position.lerp(&self._around_end_position, progress);
                        let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                        main_camera._transform_object.set_position(&position);
                        main_camera._transform_object.set_rotation(&rotation);

                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WakeUp, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WakeUp => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);
                    }
                    State::Update => {
                        let time_of_day_ratio = phase_time * 0.2;
                        if time_of_day_ratio < 1.0 {
                            game_scene_manager
                                .set_time(math::lerp(TIME_OF_DAWN, TIME_OF_EARLY_MORNING, time_of_day_ratio), 0.0);
                        }

                        let prev_wakeup_delay_aru = self._wakeup_delay_aru;
                        let prev_wakeup_delay_ewa = self._wakeup_delay_ewa;
                        let prev_wakeup_delay_koa = self._wakeup_delay_koa;
                        self._wakeup_delay_aru -= delta_time as f32;
                        self._wakeup_delay_ewa -= delta_time as f32;
                        self._wakeup_delay_koa -= delta_time as f32;

                        if 0.0 <= prev_wakeup_delay_aru
                            && self._wakeup_delay_aru < 0.0
                            && let Some(actor) = &self._player
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        if 0.0 <= prev_wakeup_delay_ewa
                            && self._wakeup_delay_ewa < 0.0
                            && let Some(actor) = &self._actor_ewa
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        if 0.0 <= prev_wakeup_delay_koa
                            && self._wakeup_delay_koa < 0.0
                            && let Some(actor) = &self._actor_koa
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        let aru_none = self
                            ._player
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let ewa_none = self
                            ._actor_ewa
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let koa_none = self
                            ._actor_koa
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));

                        if self._wakeup_delay_koa < 0.0
                            && self._wakeup_delay_ewa < 0.0
                            && self._wakeup_delay_aru < 0.0
                            && aru_none
                            && ewa_none
                            && koa_none
                        {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::AssembleFamily, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::AssembleFamily => {
                    if state == State::Update {
                        let mut done = true;

                        if let (Some(actor_aru), Some(actor_ewa), Some(actor_koa)) =
                            (&self._player, &self._actor_ewa, &self._actor_koa)
                        {
                            if self.update_assemble(actor_ewa, actor_aru) {
                                self.emoji_hungry(game_ui_manager, actor_ewa);
                            } else {
                                done = false;
                            }

                            if self.update_assemble(actor_koa, actor_aru) {
                                self.emoji_hungry(game_ui_manager, actor_koa);
                            } else {
                                done = false;
                            }
                        } else {
                            done = false;
                        }

                        if done {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::IamHungry, Some(PHASE_TIME_HUNGRY));
                        }
                    }
                }
                ScenarioPhase::IamHungry => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_STOMACH_GROWLING, AudioLoop::ONCE, None);

                        if let (Some(actor_aru), Some(actor_ewa), Some(actor_koa)) =
                            (&self._player, &self._actor_ewa, &self._actor_koa)
                        {
                            actor_aru.borrow_mut().look_at(actor_koa.borrow().get_position());
                            actor_ewa.borrow_mut().look_at(actor_aru.borrow().get_position());
                            actor_ewa.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                            actor_ewa.borrow_mut().set_action_hungry();
                            actor_koa.borrow_mut().look_at(actor_aru.borrow().get_position());
                            actor_koa.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                            actor_koa.borrow_mut().set_action_hungry();
                        }
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WrapUpTheDay => {
                    if state == State::Begin {
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                        self.create_wrap_up_the_day_text_box();
                    }
                }
                ScenarioPhase::Sleeping => match state {
                    State::Begin => {
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                    }
                    State::Update => {
                        if !game_scene_manager.has_game_scenario(ScenarioType::ScenarioWrapUpTheDay) {
                            self.clear_all();
                            game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioIntro_Ufo);
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {}
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }

        if self._scenario_track._scenario_phase == ScenarioPhase::WrapUpTheDay
            && let Some(scenario_wrap_up_the_day) =
                game_scene_manager.get_game_scenario(ScenarioType::ScenarioWrapUpTheDay).as_ref()
        {
            ptr_as_mut(scenario_wrap_up_the_day.as_ptr() as *const ScenarioWrapUpTheDay).set_skip_wakeup(true);
            self.remove_wrap_up_the_day_text_box();
            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleeping, None);
        }
    }
}
