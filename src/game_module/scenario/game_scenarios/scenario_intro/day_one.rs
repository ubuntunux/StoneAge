use crate::game_module::actors::character::{Character, CharacterCreateInfo};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{
    ACTOR_NAME_ALIEN_ALPHA, ACTOR_NAME_ALIEN_BETA, ACTOR_NAME_ARU, ACTOR_NAME_EWA, ACTOR_NAME_KOA,
    ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA, ACTOR_NAME_UFO, AUDIO_ROOSTER, AUDIO_UFO_BEAM, AUDIO_UFO_FLYING,
    BED_FOR_ARU, BED_FOR_EWA, BED_FOR_KOA, CAMERA_DISTANCE_MAX, CAMERA_OFFSET_Y, CHARACTER_DATA_NAME_EWA,
    CHARACTER_DATA_NAME_KOA, DEFAULT_GATE_NAME, PROP_NAME_MONOLITH, TIME_OF_EARLY_MORNING, TIME_OF_NIGHT,
};
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{get_character_manager_mut, get_game_scene_manager_mut};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{set_actors_behavior_none, set_actors_visible};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const AROUND_START_ROT: Vector3<f32> = Vector3::new(0.4, 0.0, 0.0);
const AROUND_END_ROT: Vector3<f32> = Vector3::new(0.35, 0.0, 0.0);
const INITIAL_CAMERA_POS: Vector3<f32> = Vector3::new(35.48, 26.56, -5.02);
const INITIAL_CAMERA_ROT: Vector3<f32> = Vector3::new(0.76, -0.33, 0.0);
const AROUND_CAMERA_EXTRA_DIST: f32 = 2.0;
const ABDUCTION_ARRIVAL_DIST_SQ: f32 = 1.0;
const ABDUCTION_MOVE_SPEED: f32 = 4.0;
const MONOLITH_DROP_SPEED: f32 = 10.0;
const ARRIVAL_RADIUS: f32 = 0.1;
const MONOLITH_INITIAL_SCALE: f32 = 0.1;
const ALIEN_BETA_MOVE_DELAY: f32 = 0.5;
const PHASE_DURATION_ABDUCT_ALIENS: f32 = 5.0;
const PHASE_DURATION_DROP_MONOLITH: f32 = 3.0;
const PHASE_DURATION_UFO_GONE: f32 = 3.0;
const PHASE_DURATION_CLOSE_UP: f32 = 5.0;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    MoveAliens,
    AbductAliens,
    DropMonolith,
    ReallyUfoGone,
    CloseUpShot,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioDayOneSaveData {
    _load_pos_alien_alpha: Vector3<f32>,
    _load_pos_alien_beta: Vector3<f32>,
}

pub struct ScenarioDayOne<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _around_start_position: Vector3<f32>,
    _around_end_position: Vector3<f32>,
    _around_start_rotation: Vector3<f32>,
    _around_end_rotation: Vector3<f32>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _alien_alpha: Option<RcRefCell<Character<'a>>>,
    _alien_beta: Option<RcRefCell<Character<'a>>>,
    _load_pos_alien_alpha: Vector3<f32>,
    _load_pos_alien_beta: Vector3<f32>,
    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _prop_monolith: Option<RcRefCell<Prop<'a>>>,
    _prop_gate: Option<RcRefCell<Prop<'a>>>,
    _monolith_start_position: Vector3<f32>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioDayOne<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioDayOne<'a>> {
        newRcRefCell(ScenarioDayOne {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: AROUND_START_ROT,
            _around_end_rotation: AROUND_END_ROT,
            _actor_ufo: None,
            _alien_alpha: None,
            _alien_beta: None,
            _load_pos_alien_alpha: Vector3::zeros(),
            _load_pos_alien_beta: Vector3::zeros(),
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _prop_monolith: None,
            _prop_gate: None,
            _monolith_start_position: Vector3::zeros(),
            _audio_ufo_flying: None,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
        })
    }

    pub fn update_abduct_actor(actor: &RcRefCell<Character>, ufo_pos: &Vector3<f32>, delta_time: f64) -> bool {
        let current_pos = *actor.borrow().get_position();
        let to_target = ufo_pos - current_pos;
        let dist_sq = to_target.magnitude_squared();
        if dist_sq <= ABDUCTION_ARRIVAL_DIST_SQ {
            actor.borrow().set_visible(false);
            true
        } else {
            let step = ABDUCTION_MOVE_SPEED * delta_time as f32;
            let dist = dist_sq.sqrt();
            if dist <= step {
                actor.borrow().set_visible(false);
                true
            } else {
                let next_pos = current_pos + (to_target / dist) * step;
                actor.borrow_mut().set_position(&next_pos);
                false
            }
        }
    }

    pub fn update_abduct_aliens(&mut self, delta_time: f64) -> bool {
        let Some(ufo) = &self._actor_ufo else {
            return true;
        };
        let ufo_pos = *ufo.borrow().get_position();

        let alpha = self._alien_alpha.clone();
        let beta = self._alien_beta.clone();

        match (alpha, beta) {
            (Some(alpha), Some(beta)) => {
                let arrived_alpha = Self::update_abduct_actor(&alpha, &ufo_pos, delta_time);
                let arrived_beta = Self::update_abduct_actor(&beta, &ufo_pos, delta_time);
                arrived_alpha && arrived_beta
            }
            (Some(alpha), None) => Self::update_abduct_actor(&alpha, &ufo_pos, delta_time),
            (None, Some(beta)) => Self::update_abduct_actor(&beta, &ufo_pos, delta_time),
            (None, None) => true,
        }
    }

    pub fn update_move_aliens(&mut self, delta_time: f32) -> bool {
        let alpha_arrived = if let Some(alpha) = &self._alien_alpha {
            if self._load_pos_alien_alpha != Vector3::zeros() {
                let arrived =
                    alpha.borrow_mut().move_to_target(&self._load_pos_alien_alpha, ARRIVAL_RADIUS, delta_time);
                if arrived {
                    alpha.borrow_mut().set_move_idle();
                }
                arrived
            } else {
                true
            }
        } else {
            true
        };

        let beta_arrived = if let Some(beta) = &self._alien_beta {
            if self._load_pos_alien_beta != Vector3::zeros() {
                if ALIEN_BETA_MOVE_DELAY <= self._scenario_track.get_phase_time() {
                    let arrived =
                        beta.borrow_mut().move_to_target(&self._load_pos_alien_beta, ARRIVAL_RADIUS, delta_time);
                    if arrived {
                        beta.borrow_mut().set_move_idle();
                    }
                    arrived
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            true
        };

        alpha_arrived && beta_arrived
    }

    pub fn get_alien_pickup_position(&self) -> Vector3<f32> {
        if self._load_pos_alien_alpha != Vector3::zeros() && self._load_pos_alien_beta != Vector3::zeros() {
            (self._load_pos_alien_alpha + self._load_pos_alien_beta) * 0.5
        } else if self._load_pos_alien_alpha != Vector3::zeros() {
            self._load_pos_alien_alpha
        } else if self._load_pos_alien_beta != Vector3::zeros() {
            self._load_pos_alien_beta
        } else if let (Some(alpha), Some(beta)) = (&self._alien_alpha, &self._alien_beta) {
            (*alpha.borrow().get_position() + *beta.borrow().get_position()) * 0.5
        } else if let Some(target) = self._alien_beta.as_ref().or(self._alien_alpha.as_ref()) {
            *target.borrow().get_position()
        } else {
            self._around_start_position
        }
    }

    pub fn set_ufo_pickup_position(&mut self) {
        if let Some(ufo) = &self._actor_ufo {
            let pickup_pos = self.get_alien_pickup_position();
            let mut ufo_position = *ufo.borrow().get_position();
            ufo_position.x = pickup_pos.x;
            ufo_position.z = pickup_pos.z;
            ufo.borrow_mut().set_position(&ufo_position);
        }
    }

    pub fn drop_monolith(&mut self, delta_time: f64) -> bool {
        if let Some(prop) = &self._prop_monolith {
            let pos_before = *prop.borrow().get_position();
            let to_target = self._monolith_start_position - pos_before;
            let move_dist = MONOLITH_DROP_SPEED * delta_time as f32;
            let moved_pos = pos_before + math::safe_normalize(&to_target) * move_dist;
            if math::check_arrival_with_radius(
                &pos_before,
                &moved_pos,
                &self._monolith_start_position,
                ARRIVAL_RADIUS,
                false,
            ) {
                prop.borrow_mut()._prop_stats._scale = Vector3::new(1.0, 1.0, 1.0);
                prop.borrow_mut().set_position(&moved_pos);
                return true;
            }

            let scale = 1.0f32.min(prop.borrow_mut()._prop_stats._scale.x + delta_time as f32);
            prop.borrow_mut()._prop_stats._scale = Vector3::new(scale, scale, scale);
            prop.borrow_mut().set_position(&moved_pos);
            false
        } else {
            false
        }
    }

    pub fn set_player_active(is_active: bool) {
        let game_scene_manager = get_game_scene_manager_mut();
        if let Some(player) = game_scene_manager.get_maybe_player() {
            let mut player_mut = player.borrow_mut();
            player_mut.set_visible(is_active);
            player_mut.set_enable_update(is_active);
        }
        if let Some(aru) = game_scene_manager.get_actor_by_name(ACTOR_NAME_ARU) {
            let mut aru_mut = aru.borrow_mut();
            aru_mut.set_visible(is_active);
            aru_mut.set_enable_update(is_active);
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioDayOne<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &str) {
        self._scenario_track.set_phase_from_str(scenario_phase, ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(data) = serde_json::from_str::<ScenarioDayOneSaveData>(&scenario_save_data._scenario_data) {
            self._load_pos_alien_alpha = data._load_pos_alien_alpha;
            self._load_pos_alien_beta = data._load_pos_alien_beta;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioDayOneSaveData {
            _load_pos_alien_alpha: self._load_pos_alien_alpha,
            _load_pos_alien_beta: self._load_pos_alien_beta,
        };
        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: self._scenario_track.save_scenario_track_data(),
            _scenario_data: serde_json::to_string(&save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        self._scenario_track._scenario_phase != ScenarioPhase::End
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        Self::set_player_active(true);
        self._prop_gate = None;
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self._actor_ufo = None;
        self._alien_alpha = None;
        self._alien_beta = None;
        self._player = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_bed_for_aru = None;
        self._prop_bed_for_ewa = None;
        self._prop_bed_for_koa = None;
        self._prop_monolith = None;
        self._prop_gate = None;
        if self._scenario_track._scenario_phase == ScenarioPhase::End {
            Self::set_player_active(true);
        }
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        self._scenario_create_info.spawn_and_reset_if_matching(game_scene_data_name);

        let game_scene_manager = get_game_scene_manager_mut();

        if game_scene_data_name == Stages::Home.get_stage_data_name() {
            Self::set_player_active(true);
            self._player = game_scene_manager.get_maybe_player().clone();

            self._actor_ewa = game_scene_manager
                .get_actor_by_name(ACTOR_NAME_EWA)
                .cloned()
                .or_else(|| game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_EWA).cloned());
            if self._actor_ewa.is_none() {
                let create_info = CharacterCreateInfo {
                    _character_data_name: CHARACTER_DATA_NAME_EWA.to_string(),
                    ..Default::default()
                };
                self._actor_ewa = Some(game_scene_manager.get_character_manager_mut().create_character(
                    ACTOR_NAME_EWA,
                    &create_info,
                    false,
                ));
            }

            self._actor_koa = game_scene_manager
                .get_actor_by_name(ACTOR_NAME_KOA)
                .cloned()
                .or_else(|| game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_KOA).cloned());
            if self._actor_koa.is_none() {
                let create_info = CharacterCreateInfo {
                    _character_data_name: CHARACTER_DATA_NAME_KOA.to_string(),
                    ..Default::default()
                };
                self._actor_koa = Some(game_scene_manager.get_character_manager_mut().create_character(
                    ACTOR_NAME_KOA,
                    &create_info,
                    false,
                ));
            }

            self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
            self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_EWA).cloned();
            self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_KOA).cloned();

            if let Some(actor) = &self._player {
                actor.borrow_mut()._controller.set_flying_mode(false);
                if let Some(bed) = &self._prop_bed_for_aru {
                    actor.borrow_mut().set_position(bed.borrow().get_position());
                }
                actor.borrow_mut().set_action_wake_up();
            }

            if let Some(actor) = &self._actor_ewa {
                actor.borrow_mut()._controller.set_flying_mode(false);
                if let Some(bed) = &self._prop_bed_for_ewa {
                    actor.borrow_mut().set_position(bed.borrow().get_position());
                }
                actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
            }

            if let Some(actor) = &self._actor_koa {
                actor.borrow_mut()._controller.set_flying_mode(false);
                if let Some(bed) = &self._prop_bed_for_koa {
                    actor.borrow_mut().set_position(bed.borrow().get_position());
                }
                actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
            }

            get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);

            self._around_start_rotation = AROUND_START_ROT;
            self._around_end_rotation = AROUND_END_ROT;

            let pivot = if let Some(actor) = &self._player {
                *actor.borrow().get_center() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
            } else {
                Vector3::zeros()
            };
            let start_rotation_matrix = math::make_rotation_matrix(
                self._around_start_rotation.x,
                self._around_start_rotation.y,
                self._around_start_rotation.z,
            );
            self._around_start_position =
                pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + AROUND_CAMERA_EXTRA_DIST);

            let main_camera = get_scene_manager().get_main_camera_mut();
            main_camera._transform_object.set_position(&self._around_start_position);
            main_camera._transform_object.set_rotation(&self._around_start_rotation);
        } else {
            self._actor_ufo = game_scene_manager.get_actor_by_name(ACTOR_NAME_UFO).cloned();
            self._alien_alpha = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_ALPHA).cloned();
            self._alien_beta = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_BETA).cloned();
            self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();
            self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

            if self._load_pos_alien_alpha == Vector3::zeros() {
                if let Some(alpha) = &self._alien_alpha {
                    self._load_pos_alien_alpha = *alpha.borrow().get_position();
                }
            }
            if self._load_pos_alien_beta == Vector3::zeros() {
                if let Some(beta) = &self._alien_beta {
                    self._load_pos_alien_beta = *beta.borrow().get_position();
                }
            }

            if self._scenario_track._scenario_phase <= ScenarioPhase::MoveAliens {
                if let Some(gate) = &self._prop_gate {
                    let gate_pos = *gate.borrow().get_position();
                    for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                        alien.borrow_mut().set_position(&gate_pos);
                        alien.borrow_mut()._controller.set_flying_mode(false);
                        alien.borrow_mut().set_move_idle();
                    }
                }
            }

            set_actors_behavior_none(&[&self._alien_alpha, &self._alien_beta], false);
            set_actors_visible(&[&self._alien_alpha, &self._alien_beta], true);

            if self._scenario_track._scenario_phase < ScenarioPhase::DropMonolith {
                self.set_ufo_pickup_position();
            }

            Self::set_player_active(false);
            if let Some(aru) = game_scene_manager.get_actor_by_name(ACTOR_NAME_ARU).cloned() {
                get_character_manager_mut().remove_character(&aru);
            }

            let main_camera = get_scene_manager().get_main_camera_mut();
            main_camera._transform_object.set_position(&INITIAL_CAMERA_POS);
            main_camera._transform_object.set_rotation(&INITIAL_CAMERA_ROT);

            let pivot =
                self._prop_monolith.as_ref().map(|prop| *prop.borrow().get_position()).unwrap_or_else(Vector3::zeros);
            let start_rotation_matrix = math::make_rotation_matrix(
                self._around_start_rotation.x,
                self._around_start_rotation.y,
                self._around_start_rotation.z,
            );
            self._around_start_position =
                pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + AROUND_CAMERA_EXTRA_DIST);

            if let Some(prop) = &self._prop_monolith {
                self._monolith_start_position = *prop.borrow().get_position();
                prop.borrow()._render_object.borrow_mut().set_visible(false);

                if let Some(ufo) = &self._actor_ufo {
                    prop.borrow_mut()._prop_stats._scale = Vector3::repeat(MONOLITH_INITIAL_SCALE);
                    prop.borrow_mut().set_position(&Vector3::new(
                        self._monolith_start_position.x,
                        ufo.borrow().get_position().y,
                        self._monolith_start_position.z,
                    ));
                }
            }
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();

        if self._scenario_track._scenario_phase < ScenarioPhase::CloseUpShot {
            Self::set_player_active(false);
        }

        let prev_scenario_phase = self._scenario_track._scenario_phase;
        let next_scenario_phase = self._scenario_track._next_scenario_phase;
        let next_phase_duration = self._scenario_track._next_phase_duration;

        for state in State::iter() {
            let Some(update_scenario_phase) = self._scenario_track.resolve_state_phase(
                state,
                prev_scenario_phase,
                next_scenario_phase,
                next_phase_duration,
            ) else {
                continue;
            };

            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    if state == State::Begin {
                        game_scene_manager.set_time(TIME_OF_NIGHT, 0.0);
                        Self::set_player_active(false);
                        self.set_ufo_pickup_position();
                        if self._audio_ufo_flying.is_none() {
                            self._audio_ufo_flying =
                                get_audio_manager_mut().play_audio_bank(AUDIO_UFO_FLYING, AudioLoop::LOOP, Some(1.0));
                        }
                        if let Some(gate) = &self._prop_gate {
                            let gate_pos = *gate.borrow().get_position();
                            for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                                alien.borrow_mut().set_position(&gate_pos);
                                alien.borrow_mut()._controller.set_flying_mode(false);
                                alien.borrow_mut().set_behavior_none();
                                alien.borrow_mut().set_move_idle();
                            }
                        }
                    } else if state == State::Update {
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::MoveAliens, None);
                    }
                }
                ScenarioPhase::MoveAliens => match state {
                    State::Begin => {
                        for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                            alien.borrow_mut()._controller.set_flying_mode(false);
                            alien.borrow_mut().set_behavior_none();
                        }
                    }
                    State::Update => {
                        if self.update_move_aliens(delta_time as f32) {
                            self._scenario_track.set_next_scenario_phase(
                                ScenarioPhase::AbductAliens,
                                Some(PHASE_DURATION_ABDUCT_ALIENS),
                            );
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::AbductAliens => match state {
                    State::Begin => {
                        Self::set_player_active(false);
                        self.set_ufo_pickup_position();

                        for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                            alien.borrow_mut()._controller.set_flying_mode(true);
                            alien.borrow_mut().set_behavior_none();
                        }

                        if self._audio_ufo_flying.is_none() {
                            self._audio_ufo_flying =
                                get_audio_manager_mut().play_audio_bank(AUDIO_UFO_FLYING, AudioLoop::LOOP, Some(1.0));
                        }
                        get_audio_manager_mut().play_audio_bank(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
                    }
                    State::Update => {
                        if self.update_abduct_aliens(delta_time) {
                            self._scenario_track.set_next_scenario_phase(
                                ScenarioPhase::DropMonolith,
                                Some(PHASE_DURATION_DROP_MONOLITH),
                            );
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::DropMonolith => {
                    if state == State::Update {
                        let mut drop_completed = false;
                        let ufo_at_target = if let (Some(ufo), Some(_prop)) = (&self._actor_ufo, &self._prop_monolith) {
                            ufo.borrow_mut().move_to_target(&self._monolith_start_position, 0.0, delta_time as f32)
                        } else {
                            false
                        };

                        if ufo_at_target {
                            let is_visible = self
                                ._prop_monolith
                                .as_ref()
                                .is_some_and(|prop| prop.borrow()._render_object.borrow().is_visible());
                            if !is_visible {
                                if let Some(prop) = &self._prop_monolith {
                                    prop.borrow()._render_object.borrow_mut().set_visible(true);
                                    if let Some(ufo) = &self._actor_ufo {
                                        prop.borrow_mut().set_position(ufo.borrow().get_position());
                                    }
                                }
                                get_audio_manager_mut().play_audio_bank(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
                            }
                            drop_completed = self.drop_monolith(delta_time);
                        }

                        if drop_completed {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::ReallyUfoGone, Some(PHASE_DURATION_UFO_GONE));
                        }
                    }
                }
                ScenarioPhase::ReallyUfoGone => match state {
                    State::Update => {
                        if let Some(ufo) = &self._actor_ufo {
                            ufo.borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
                        }
                        if 1.0 <= phase_ratio {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::CloseUpShot, Some(PHASE_DURATION_CLOSE_UP));
                        }
                    }
                    State::End => {
                        if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                            get_audio_manager().stop_audio_instance(audio_instance);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::CloseUpShot => match state {
                    State::Begin => {
                        game_scene_manager.set_time(TIME_OF_EARLY_MORNING, 0.0);
                        Self::set_player_active(false);
                        if let Some(ufo) = &self._actor_ufo {
                            get_character_manager_mut().remove_character(ufo);
                        }
                        for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                            get_character_manager_mut().remove_character(alien);
                        }
                        get_game_scene_manager_mut().request_open_game_scene(Stages::Home.get_stage_data_name());
                    }
                    State::Update => {
                        let main_camera = get_scene_manager().get_main_camera_mut();
                        let pivot = if let Some(actor) = &self._player {
                            *actor.borrow().get_center() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
                        } else {
                            Vector3::zeros()
                        };
                        self._around_start_position = pivot
                            - main_camera._transform_object.get_front()
                                * (CAMERA_DISTANCE_MAX + AROUND_CAMERA_EXTRA_DIST);
                        self._around_end_position =
                            pivot - main_camera._transform_object.get_front() * CAMERA_DISTANCE_MAX;

                        let progress = phase_ratio.powf(2.0);
                        let position = self._around_start_position.lerp(&self._around_end_position, progress);
                        let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                        main_camera._transform_object.set_position(&position);
                        main_camera._transform_object.set_rotation(&rotation);

                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {
                    if state == State::Begin {
                        Self::set_player_active(true);
                        get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWoodenClub);
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
