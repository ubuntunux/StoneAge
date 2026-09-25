use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{
    AUDIO_QUEST_COMPLETE, AUDIO_ROOSTER, AUDIO_WRAP_UP_THE_DAY, BED_FOR_ARU, CAMERA_DISTANCE_MIN, CAMERA_OFFSET_Y,
    CHARACTER_INTERACTION_DISTANCE, DEFAULT_FADE_TIME, EAT_ITEM_DELAY_TIME, MATERIAL_UI_NONE,
    MAX_BED_RESTRICTION_DISTANCE, SLEEP_TIMER, TARGET_HUNGER_THRESHOLD, TIME_OF_NIGHT,
};

use crate::game_module::game_service_locator::{
    get_game_controller_mut, get_game_scene_manager, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::GameUIManager;

use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    Update,
    GoToSleep,
    Sleep,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioWrapUpTheDaySaveData {
    pub _sleep_timer: f32,
    pub _skip_wakeup: bool,
}

pub struct ScenarioWrapUpTheDay<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,

    _sleep_timer: f32,
    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _audio_bgm: Option<RcRefCell<AudioInstance>>,
    _skip_wakeup: bool,
    _waiting_for_key_release: bool,
    _request_sleep: bool,
    _check_time_for_sleep: f32,
    _ewa_eat_delay: f32,
    _koa_eat_delay: f32,
    _bed_camera_position: Vector3<f32>,
    _camera_target_position: Vector3<f32>,
    _camera_target_velocity: Vector3<f32>,
    _is_camera_target_initialized: bool,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioWrapUpTheDay<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWrapUpTheDay<'a>> {
        newRcRefCell(ScenarioWrapUpTheDay {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _sleep_timer: 0.0,
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _audio_bgm: None,
            _skip_wakeup: false,
            _waiting_for_key_release: true,
            _request_sleep: false,
            _check_time_for_sleep: 0.0,
            _ewa_eat_delay: 0.0,
            _koa_eat_delay: 0.0,
            _bed_camera_position: Vector3::default(),
            _camera_target_position: Vector3::default(),
            _camera_target_velocity: Vector3::default(),
            _is_camera_target_initialized: false,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
                _next_phase_duration: None,
            },
        })
    }

    pub fn set_skip_wakeup(&mut self, skip_wakeup: bool) {
        self._skip_wakeup = skip_wakeup;
    }

    pub fn is_sleeping(&self) -> bool {
        matches!(
            self._scenario_track._scenario_phase,
            ScenarioPhase::GoToSleep | ScenarioPhase::Sleep | ScenarioPhase::End
        )
    }

    pub fn set_sleep_phase(&mut self) {
        self._scenario_track.set_scenario_phase(ScenarioPhase::Sleep, None);
        self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
    }

    pub fn setup_bed_camera(&mut self) {
        let mut pivot = Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
        if let Some(prop_bed) = self._prop_bed_for_aru.as_ref() {
            pivot += prop_bed.borrow().get_position();
        };
        let camera_rotation = Vector3::new(0.4, -1.4, 0.0);
        let camera_rotation_matrix =
            math::make_rotation_matrix(camera_rotation.x, camera_rotation.y, camera_rotation.z);
        self._bed_camera_position = pivot - camera_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MIN + 6.0);
        get_game_controller_mut().set_camera_fixed(true);
        self._is_camera_target_initialized = false;
        self.update_bed_camera(0.0);
    }

    pub fn update_bed_camera(&mut self, delta_time: f32) {
        let goal_target_position = if let Some(player) = self._player.as_ref() {
            player.borrow().get_bounding_box()._center + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
        } else if let Some(prop_bed) = self._prop_bed_for_aru.as_ref() {
            prop_bed.borrow().get_position() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
        } else {
            Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
        };

        if !self._is_camera_target_initialized {
            self._camera_target_position = goal_target_position;
            self._camera_target_velocity = Vector3::zeros();
            self._is_camera_target_initialized = true;
        } else if 0.0 < delta_time {
            let diff = goal_target_position - self._camera_target_position;
            const ACCEL_SPEED: f32 = 25.0;
            const DAMPING: f32 = 8.0;
            self._camera_target_velocity += diff * ACCEL_SPEED * delta_time;
            self._camera_target_velocity *= (-DAMPING * delta_time).exp();
            self._camera_target_position += self._camera_target_velocity * delta_time;
        }

        let diff = self._bed_camera_position - self._camera_target_position;
        let dist = diff.magnitude();
        let camera_rotation = if 0.0001 < dist {
            let back = diff / dist;
            let pitch = back.y.clamp(-1.0, 1.0).asin();
            let yaw = back.x.atan2(back.z);
            Vector3::new(pitch, yaw + std::f32::consts::PI, 0.0)
        } else {
            Vector3::new(0.4, -1.4, 0.0)
        };

        get_game_controller_mut().set_camera_fixed_position_and_rotation(&self._bed_camera_position, &camera_rotation);
    }
}

fn clamp_actor_position_from_bed(actor: &Option<RcRefCell<Character>>, bed_pos: &Vector3<f32>, max_distance: f32) {
    if let Some(actor_ref) = actor.as_ref() {
        let mut actor_mut = actor_ref.borrow_mut();
        let current_pos = *actor_mut.get_position();
        let mut diff = current_pos - bed_pos;
        diff.y = 0.0;
        let dist_xz = diff.magnitude();
        if max_distance < dist_xz && 0.0001 < dist_xz {
            let clamped_diff = (diff / dist_xz) * max_distance;
            let mut new_pos = *bed_pos + clamped_diff;
            new_pos.y = current_pos.y;
            actor_mut.set_position(&new_pos);
        }
    }
}

fn set_actor_table_position(
    scene_manager: &SceneManager,
    actor: &Option<RcRefCell<Character>>,
    table: &Option<RcRefCell<Prop>>,
    direction: &Vector3<f32>,
) {
    if let (Some(actor_ref), Some(table_ref)) = (actor.as_ref(), table.as_ref()) {
        let mut pos = table_ref.borrow().get_position() - math::safe_normalize(direction) * 2.0;
        pos.y = scene_manager.get_height_bilinear(&pos, 0);
        actor_ref.borrow_mut().set_position(&pos);
        actor_ref.borrow_mut().look_at(table_ref.borrow().get_position());
        actor_ref.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
    }
}

fn dance_in_place(actor: &Option<RcRefCell<Character>>, player: &Option<RcRefCell<Character>>) {
    if let (Some(actor_ref), Some(player_ref)) = (actor.as_ref(), player.as_ref()) {
        actor_ref.borrow_mut().look_at(player_ref.borrow().get_position());
        actor_ref.borrow_mut().set_next_behavior(BehaviorState::Dance, true);
    }
}

fn stand_in_place(actor: &Option<RcRefCell<Character>>) {
    if let Some(actor_ref) = actor.as_ref() {
        actor_ref.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
    }
}

fn is_in_table_interaction_range(actor: &Character, table: &Option<RcRefCell<Prop>>) -> bool {
    if let Some(table_prop) = table.as_ref() {
        actor.check_in_range(
            table_prop.borrow().get_collision(),
            CHARACTER_INTERACTION_DISTANCE * 2.0,
            false,
        )
    } else {
        false
    }
}

fn update_actor_table_eating<'a>(
    actor_opt: &Option<RcRefCell<Character<'a>>>,
    table_opt: &Option<RcRefCell<Prop<'a>>>,
    seat_direction: &Vector3<f32>,
    eat_delay: &mut f32,
    has_eatable_item: bool,
    game_ui_manager: &mut GameUIManager<'a>,
) {
    let Some(actor) = actor_opt.as_ref() else {
        return;
    };

    if actor.borrow().is_action(ActionAnimationState::Dance) {
        return;
    }

    let is_hungry = actor.borrow().get_hunger() > TARGET_HUNGER_THRESHOLD;

    if is_hungry {
        if has_eatable_item {
            let is_in_range = is_in_table_interaction_range(&actor.borrow(), table_opt);

            if !is_in_range {
                if let Some(table_prop) = table_opt.as_ref() {
                    let scene_manager = get_scene_manager();
                    let mut seat_pos = table_prop.borrow().get_position() - math::safe_normalize(seat_direction) * 2.0;
                    seat_pos.y = scene_manager.get_height_bilinear(&seat_pos, 0);

                    let (direction, dist) =
                        math::make_normalize_xz_with_norm(&(seat_pos - actor.borrow().get_position()));
                    if dist > 0.3 {
                        actor.borrow_mut().set_move(&direction);
                    } else {
                        if !actor.borrow().is_move_stop() {
                            actor.borrow_mut().set_move_idle();
                        }
                        actor.borrow_mut().look_at(table_prop.borrow().get_position());
                    }
                }
            } else {
                if !actor.borrow().is_move_stop() {
                    actor.borrow_mut().set_move_idle();
                }
                if let Some(table_prop) = table_opt.as_ref() {
                    actor.borrow_mut().look_at(table_prop.borrow().get_position());
                }

                let is_ready_to_eat = {
                    let actor_ref = actor.borrow();
                    *eat_delay <= 0.0
                        && actor_ref.get_attached_item().is_none()
                        && !actor_ref.is_action(ActionAnimationState::Eating)
                };

                if is_ready_to_eat {
                    if let Some(item_data_name) = game_ui_manager.pop_eatable_table_storage_item() {
                        let item_manager = get_game_scene_manager().get_item_manager_mut();
                        let mut actor_ref = actor.borrow_mut();
                        item_manager.attach_item(&mut actor_ref, item_data_name.as_str());
                        actor_ref.set_next_behavior(BehaviorState::Eating, true);
                        *eat_delay = EAT_ITEM_DELAY_TIME;
                    }
                }
            }
        } else {
            if !actor.borrow().is_move_stop() {
                actor.borrow_mut().set_move_idle();
            }

            let mut actor_ref = actor.borrow_mut();
            if actor_ref.get_attached_item().is_none()
                && !actor_ref.is_action(ActionAnimationState::Eating)
                && !actor_ref.is_action(ActionAnimationState::Hungry)
            {
                actor_ref.set_action_hungry();
                actor_ref.set_sit_down();
            }
        }
    }
}

fn go_to_sleep(actor: &Option<RcRefCell<Character>>, bed: &Option<RcRefCell<Prop>>) {
    if let (Some(actor), Some(bed_ref)) = (actor.as_ref(), bed.as_ref()) {
        let radius = bed_ref.borrow().get_collision()._bounding_box._mag_xz * 0.5;
        let (direction, dist) =
            math::make_normalize_xz_with_norm(&(bed_ref.borrow().get_position() - actor.borrow().get_position()));
        if radius < dist {
            actor.borrow_mut().set_move(&direction);
        } else {
            if !actor.borrow().is_move_stop() {
                actor.borrow_mut().set_move_idle();
            }

            if !actor.borrow().is_action(ActionAnimationState::LayingDown)
                && !actor.borrow().is_action(ActionAnimationState::Sleep)
            {
                actor.borrow_mut().set_action_laying_down();
            }
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioWrapUpTheDay<'a> {
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
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(data) = serde_json::from_str::<ScenarioWrapUpTheDaySaveData>(&scenario_save_data._scenario_data) {
            self._sleep_timer = data._sleep_timer;
            self._skip_wakeup = data._skip_wakeup;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioWrapUpTheDaySaveData {
            _sleep_timer: self._sleep_timer,
            _skip_wakeup: self._skip_wakeup,
        };
        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: self._scenario_track.save_scenario_track_data(),
            _scenario_data: serde_json::to_string(&save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        true
    }

    fn is_allow_player_control(&self) -> bool {
        matches!(self._scenario_track._scenario_phase, ScenarioPhase::Update)
    }

    fn is_available_sleep(&self) -> bool {
        matches!(self._scenario_track._scenario_phase, ScenarioPhase::Update)
    }

    fn request_sleep(&mut self) {
        if self.is_available_sleep() && !self._waiting_for_key_release {
            self._request_sleep = true;
        }
    }

    fn on_interaction_released(&mut self) {
        if self._waiting_for_key_release {
            self._waiting_for_key_release = false;
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {}

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager
            .get_actor_by_name("monkey_ewa")
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name("ewa").cloned());
        self._actor_koa = game_scene_manager
            .get_actor_by_name("monkey_koa")
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name("koa").cloned());
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name("table").cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
        self.setup_bed_camera();
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();
        let game_ui_manager = get_game_ui_manager_mut();

        self.update_bed_camera(delta_time as f32);

        if let Some(prop_table) = self._prop_table.as_ref() {
            let table_pos = *prop_table.borrow().get_position();
            clamp_actor_position_from_bed(&self._player, &table_pos, MAX_BED_RESTRICTION_DISTANCE);
            clamp_actor_position_from_bed(&self._actor_ewa, &table_pos, MAX_BED_RESTRICTION_DISTANCE);
            clamp_actor_position_from_bed(&self._actor_koa, &table_pos, MAX_BED_RESTRICTION_DISTANCE);
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

            let _phase_time = self._scenario_track.get_phase_time();
            let _phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._check_time_for_sleep = get_game_scene_manager().get_time_of_day();
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => match state {
                    State::Begin => {
                        if let Some(actor) = &self._player {
                            actor.borrow_mut().set_behavior_none();
                            actor.borrow_mut().set_action_none();
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_behavior_none();
                            actor.borrow_mut().set_action_none();
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_behavior_none();
                            actor.borrow_mut().set_action_none();
                        }
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() {
                            set_actor_table_position(
                                get_scene_manager(),
                                &self._actor_ewa,
                                &self._prop_table,
                                &Vector3::new(0.0, 0.0, 1.0),
                            );
                            set_actor_table_position(
                                get_scene_manager(),
                                &self._actor_koa,
                                &self._prop_table,
                                &Vector3::new(-1.0, 0.0, 0.0),
                            );
                            game_ui_manager.set_auto_fade_inout(true);
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Update, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Update => {
                    if state == State::Begin {
                        // set time of day
                        if get_game_scene_manager().get_time_of_day() < TIME_OF_NIGHT {
                            get_game_scene_manager_mut().set_time_of_day(TIME_OF_NIGHT);
                        }

                        // set camera
                        self.setup_bed_camera();
                    } else if state == State::Update {
                        let player_is_dancing =
                            self._player.as_ref().is_some_and(|p| p.borrow().is_action(ActionAnimationState::Dance));

                        if player_is_dancing {
                            if self._audio_bgm.is_none() {
                                self._audio_bgm = get_audio_manager_mut().play_audio_bank(
                                    AUDIO_WRAP_UP_THE_DAY,
                                    AudioLoop::SOME(99),
                                    None,
                                );
                            }

                            let ewa_dancing = self
                                ._actor_ewa
                                .as_ref()
                                .is_some_and(|a| a.borrow().is_action(ActionAnimationState::Dance));
                            if !ewa_dancing {
                                dance_in_place(&self._actor_ewa, &self._player);
                            }

                            let koa_dancing = self
                                ._actor_koa
                                .as_ref()
                                .is_some_and(|a| a.borrow().is_action(ActionAnimationState::Dance));
                            if !koa_dancing {
                                dance_in_place(&self._actor_koa, &self._player);
                            }
                        } else {
                            if let Some(audio_bgm) = &self._audio_bgm {
                                get_audio_manager_mut().stop_audio_instance(audio_bgm);
                                self._audio_bgm = None;
                            }

                            let ewa_dancing = self
                                ._actor_ewa
                                .as_ref()
                                .is_some_and(|a| a.borrow().is_action(ActionAnimationState::Dance));
                            if ewa_dancing {
                                stand_in_place(&self._actor_ewa);
                            }

                            let koa_dancing = self
                                ._actor_koa
                                .as_ref()
                                .is_some_and(|a| a.borrow().is_action(ActionAnimationState::Dance));
                            if koa_dancing {
                                stand_in_place(&self._actor_koa);
                            }

                            if self._ewa_eat_delay > 0.0 {
                                self._ewa_eat_delay = (self._ewa_eat_delay - delta_time as f32).max(0.0);
                            }
                            if self._koa_eat_delay > 0.0 {
                                self._koa_eat_delay = (self._koa_eat_delay - delta_time as f32).max(0.0);
                            }

                            let has_eatable_item = game_ui_manager.has_eatable_table_storage_item();

                            update_actor_table_eating(
                                &self._actor_ewa,
                                &self._prop_table,
                                &Vector3::new(0.0, 0.0, 1.0),
                                &mut self._ewa_eat_delay,
                                has_eatable_item,
                                game_ui_manager,
                            );

                            update_actor_table_eating(
                                &self._actor_koa,
                                &self._prop_table,
                                &Vector3::new(-1.0, 0.0, 0.0),
                                &mut self._koa_eat_delay,
                                has_eatable_item,
                                game_ui_manager,
                            );
                        }

                        let prev_time_of_day = self._check_time_for_sleep;
                        self._check_time_for_sleep = get_game_scene_manager().get_time_of_day();

                        if self._request_sleep || self._check_time_for_sleep < prev_time_of_day {
                            self._request_sleep = false;
                            if let Some(audio_bgm) = &self._audio_bgm {
                                get_audio_manager_mut().stop_audio_instance(audio_bgm);
                            }
                            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                            let item_manager = get_game_scene_manager().get_item_manager_mut();
                            if let Some(actor) = &self._player {
                                actor.borrow_mut().set_action_none();
                            }
                            if let Some(actor) = &self._actor_ewa {
                                actor.borrow_mut().set_action_none();
                                item_manager.detach_item(&mut actor.borrow_mut());
                            }
                            if let Some(actor) = &self._actor_koa {
                                actor.borrow_mut().set_action_none();
                                item_manager.detach_item(&mut actor.borrow_mut());
                            }
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToSleep, None);
                        }
                    }
                }
                ScenarioPhase::GoToSleep => {
                    if state == State::Update {
                        go_to_sleep(&self._player, &self._prop_bed_for_aru);
                        go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa);
                        go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa);

                        let aru_is_sleeping = self
                            ._player
                            .as_ref()
                            .is_some_and(|actor| actor.borrow().is_action(ActionAnimationState::Sleep));
                        if aru_is_sleeping {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
                        }
                    }
                }
                ScenarioPhase::Sleep => match state {
                    State::Begin => {
                        self._sleep_timer = 0.0;
                        self.setup_bed_camera();
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() && self._sleep_timer < SLEEP_TIMER {
                            if self._sleep_timer == 0.0 {
                                if let Some(actor) = &self._actor_ewa {
                                    if let Some(bed) = &self._prop_bed_for_ewa {
                                        actor.borrow_mut().set_position(bed.borrow().get_position());
                                    }
                                    actor.borrow_mut().set_action_sleep();
                                }
                                if let Some(actor) = &self._actor_koa {
                                    if let Some(bed) = &self._prop_bed_for_koa {
                                        actor.borrow_mut().set_position(bed.borrow().get_position());
                                    }
                                    actor.borrow_mut().set_action_sleep();
                                }
                            }

                            self._sleep_timer += delta_time as f32;
                            if SLEEP_TIMER <= self._sleep_timer {
                                game_ui_manager.set_auto_fade_inout(true);
                                game_scene_manager.set_next_time_of_day();
                            }
                        } else if game_ui_manager.is_done_game_image_progress() {
                            if self._skip_wakeup {
                                self._skip_wakeup = false;
                            } else {
                                get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);
                                if let Some(actor) = &self._player {
                                    actor.borrow_mut().set_action_wake_up();
                                }
                                if let Some(actor) = &self._actor_ewa {
                                    actor.borrow_mut()._controller.set_flying_mode(false);
                                    actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
                                }
                                if let Some(actor) = &self._actor_koa {
                                    actor.borrow_mut()._controller.set_flying_mode(false);
                                    actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
                                }
                            }
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                        } else {
                            go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa);
                            go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {
                    if state == State::Begin {
                        get_game_controller_mut().set_camera_fixed(false);
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
