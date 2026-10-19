use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    ACTOR_NAME_ALIEN_ALPHA, ACTOR_NAME_ALIEN_BETA, ACTOR_NAME_UFO, AUDIO_UFO_BEAM, AUDIO_UFO_FLYING, DEFAULT_FADE_TIME,
    DEFAULT_GATE_NAME, MATERIAL_UI_NONE, TIME_OF_DAWN,
};
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{set_actors_behavior_none, set_actors_visible};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const DESCEND_ARRIVAL_DIST_SQ: f32 = 0.05;
const DESCEND_MOVE_SPEED: f32 = 4.0;
const GATE_ARRIVAL_DIST: f32 = 1.0;
const PHASE_DURATION_WAIT_AFTER_DESCEND: f32 = 1.0;
const UFO_LONG_SHOT_CAMERA_POS: Vector3<f32> = Vector3::new(35.48, 26.56, -5.02);
const UFO_LONG_SHOT_CAMERA_ROT: Vector3<f32> = Vector3::new(0.76, -0.33, 0.0);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    UfoLongShot,
    DescendAliens,
    WaitAfterDescend,
    MoveToGate,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioUfoSaveData {
    _ground_pos_alien_alpha: Vector3<f32>,
    _ground_pos_alien_beta: Vector3<f32>,
}

pub struct ScenarioUfo<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _alien_alpha: Option<RcRefCell<Character<'a>>>,
    _alien_beta: Option<RcRefCell<Character<'a>>>,
    _prop_gate: Option<RcRefCell<Prop<'a>>>,
    _ground_pos_alien_alpha: Vector3<f32>,
    _ground_pos_alien_beta: Vector3<f32>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _audio_ufo_beam: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioUfo<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioUfo<'a>> {
        newRcRefCell(ScenarioUfo {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _actor_ufo: None,
            _alien_alpha: None,
            _alien_beta: None,
            _prop_gate: None,
            _ground_pos_alien_alpha: Vector3::zeros(),
            _ground_pos_alien_beta: Vector3::zeros(),
            _audio_ufo_flying: None,
            _audio_ufo_beam: None,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::UfoLongShot),
        })
    }

    pub fn update_ufo_movement(&mut self, delta_time: f32) -> bool {
        let target_pos = if self._ground_pos_alien_beta != Vector3::zeros() {
            self._ground_pos_alien_beta
        } else if let Some(beta) = &self._alien_beta {
            *beta.borrow().get_position()
        } else {
            return false;
        };

        if let Some(actor) = &self._actor_ufo {
            if actor.borrow_mut().move_to_target(&target_pos, 0.0, delta_time) {
                actor.borrow_mut().set_move_idle();
                return true;
            }
        }
        false
    }

    pub fn update_descend_actor(actor: &RcRefCell<Character>, target_pos: &Vector3<f32>, delta_time: f64) -> bool {
        let current_pos = *actor.borrow().get_position();
        let to_target = target_pos - current_pos;
        let dist_sq = to_target.magnitude_squared();
        if dist_sq <= DESCEND_ARRIVAL_DIST_SQ {
            let mut actor_mut = actor.borrow_mut();
            actor_mut.set_position(target_pos);
            actor_mut._controller.set_flying_mode(false);
            actor_mut.set_move_idle();
            true
        } else {
            let step = DESCEND_MOVE_SPEED * delta_time as f32;
            let dist = dist_sq.sqrt();
            if dist <= step {
                let mut actor_mut = actor.borrow_mut();
                actor_mut.set_position(target_pos);
                actor_mut._controller.set_flying_mode(false);
                actor_mut.set_move_idle();
                true
            } else {
                let next_pos = current_pos + (to_target / dist) * step;
                actor.borrow_mut().set_position(&next_pos);
                false
            }
        }
    }

    pub fn update_descend_aliens(&mut self, delta_time: f64) -> bool {
        let alpha = self._alien_alpha.clone();
        let beta = self._alien_beta.clone();

        match (alpha, beta) {
            (Some(alpha), Some(beta)) => {
                let arrived_alpha = Self::update_descend_actor(&alpha, &self._ground_pos_alien_alpha, delta_time);
                let arrived_beta = Self::update_descend_actor(&beta, &self._ground_pos_alien_beta, delta_time);
                arrived_alpha && arrived_beta
            }
            (Some(alpha), None) => Self::update_descend_actor(&alpha, &self._ground_pos_alien_alpha, delta_time),
            (None, Some(beta)) => Self::update_descend_actor(&beta, &self._ground_pos_alien_beta, delta_time),
            (None, None) => true,
        }
    }

    pub fn set_player_active(is_active: bool) {
        if let Some(player) = get_game_scene_manager_mut().get_maybe_player() {
            let mut player_mut = player.borrow_mut();
            player_mut.set_visible(is_active);
            player_mut.set_enable_update(is_active);
        }
    }

    pub fn remove_aliens(&mut self) {
        let character_manager = get_character_manager_mut();
        for alien_opt in [&self._alien_alpha, &self._alien_beta] {
            if let Some(alien) = alien_opt {
                alien.borrow_mut().set_visible(false);
                character_manager.remove_character(alien);
            }
        }
        self._alien_alpha = None;
        self._alien_beta = None;
    }

    pub fn update_move_to_gate(&mut self, delta_time: f32) -> bool {
        let gate_pos = if let Some(gate) = &self._prop_gate {
            *gate.borrow().get_position()
        } else {
            self.remove_aliens();
            return true;
        };

        if let Some(alpha) = self._alien_alpha.clone() {
            if alpha.borrow_mut().move_to_target(&gate_pos, GATE_ARRIVAL_DIST, delta_time) {
                alpha.borrow_mut().set_visible(false);
                get_character_manager_mut().remove_character(&alpha);
                self._alien_alpha = None;
            }
        }

        if let Some(beta) = self._alien_beta.clone() {
            if beta.borrow_mut().move_to_target(&gate_pos, GATE_ARRIVAL_DIST, delta_time) {
                beta.borrow_mut().set_visible(false);
                get_character_manager_mut().remove_character(&beta);
                self._alien_beta = None;
            }
        }

        self._alien_alpha.is_none() && self._alien_beta.is_none()
    }
}

impl<'a> ScenarioBase<'a> for ScenarioUfo<'a> {
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
        if let Ok(data) = serde_json::from_str::<ScenarioUfoSaveData>(&scenario_save_data._scenario_data) {
            self._ground_pos_alien_alpha = data._ground_pos_alien_alpha;
            self._ground_pos_alien_beta = data._ground_pos_alien_beta;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioUfoSaveData {
            _ground_pos_alien_alpha: self._ground_pos_alien_alpha,
            _ground_pos_alien_beta: self._ground_pos_alien_beta,
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

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        Self::set_player_active(true);
        self.remove_aliens();
        self._prop_gate = None;
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        Self::set_player_active(true);
        self.remove_aliens();
        self._prop_gate = None;
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        self._scenario_create_info.spawn_and_reset_if_matching(game_scene_data_name);

        if game_scene_data_name == Stages::Forest.get_stage_data_name() {
            let game_scene_manager = get_game_scene_manager_mut();
            self._actor_ufo = game_scene_manager.get_actor_by_name(ACTOR_NAME_UFO).cloned();
            if let Some(ufo) = self._actor_ufo.as_ref() {
                ufo.borrow_mut().set_invincibility(true);
            }

            Self::set_player_active(false);

            self._alien_alpha = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_ALPHA).cloned();
            self._alien_beta = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_BETA).cloned();
            self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

            if self._ground_pos_alien_alpha == Vector3::zeros() {
                if let Some(alpha) = &self._alien_alpha {
                    self._ground_pos_alien_alpha = *alpha.borrow().get_position();
                }
            }
            if self._ground_pos_alien_beta == Vector3::zeros() {
                if let Some(beta) = &self._alien_beta {
                    self._ground_pos_alien_beta = *beta.borrow().get_position();
                }
            }

            set_actors_behavior_none(&[&self._alien_alpha, &self._alien_beta], false);
            if self._scenario_track._scenario_phase < ScenarioPhase::DescendAliens {
                set_actors_visible(&[&self._alien_alpha, &self._alien_beta], false);
            }
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();
        let game_ui_manager = get_game_ui_manager_mut();

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

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::UfoLongShot, None);
                }
                ScenarioPhase::UfoLongShot => match state {
                    State::Begin => {
                        game_scene_manager.set_time(TIME_OF_DAWN, 0.0);
                        self._audio_ufo_flying =
                            get_audio_manager_mut().play_audio_bank(AUDIO_UFO_FLYING, AudioLoop::LOOP, Some(1.0));
                        let main_camera = get_scene_manager().get_main_camera_mut();
                        main_camera._transform_object.set_position(&UFO_LONG_SHOT_CAMERA_POS);
                        main_camera._transform_object.set_rotation(&UFO_LONG_SHOT_CAMERA_ROT);
                        Self::set_player_active(false);
                        set_actors_visible(&[&self._alien_alpha, &self._alien_beta], false);
                    }
                    State::Update => {
                        if self.update_ufo_movement(delta_time as f32) {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::DescendAliens, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::DescendAliens => match state {
                    State::Begin => {
                        if let Some(ufo) = &self._actor_ufo {
                            let ufo_pos = *ufo.borrow().get_position();
                            for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                                let mut alien_mut = alien.borrow_mut();
                                alien_mut._controller.set_flying_mode(true);
                                alien_mut.set_behavior_none();
                                alien_mut.set_position(&ufo_pos);
                            }
                        }
                        set_actors_visible(&[&self._alien_alpha, &self._alien_beta], true);
                        self._audio_ufo_beam =
                            get_audio_manager_mut().play_audio_bank(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
                    }
                    State::Update => {
                        if self.update_descend_aliens(delta_time) {
                            self._scenario_track.set_next_scenario_phase(
                                ScenarioPhase::WaitAfterDescend,
                                Some(PHASE_DURATION_WAIT_AFTER_DESCEND),
                            );
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WaitAfterDescend => {
                    if state == State::Update && 1.0 <= self._scenario_track.get_phase_ratio() {
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::MoveToGate, None);
                    }
                }
                ScenarioPhase::MoveToGate => match state {
                    State::Begin => {}
                    State::Update => {
                        if self.update_move_to_gate(delta_time as f32) {
                            if game_ui_manager.is_done_manual_fade_out() {
                                self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                            } else {
                                game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                            }
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {
                    if state == State::Begin {
                        Self::set_player_active(true);
                        if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                            get_audio_manager().stop_audio_instance(audio_instance);
                        }
                        game_ui_manager.set_auto_fade_inout(true);
                        game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioIntro_Revolution);
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
