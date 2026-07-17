use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::{
    AUDIO_UFO_BEAM, AUDIO_UFO_FLYING, DEFAULT_FADE_TIME, MATERIAL_UI_NONE, TIME_OF_DAWN,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, Stages};
use crate::game_module::scenario::scenario::{GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use serde::{Deserialize, Serialize};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    AppearUfo,
    UfoLongShot,
    BeAbducted,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioUfoSaveData {
}

pub struct ScenarioUfo<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _audio_ufo_beam: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioUfo<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioUfo<'a>> {
        newRcRefCell(ScenarioUfo {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _game_scene_manager: game_scene_manager,
            _actor_ufo: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _audio_ufo_flying: None,
            _audio_ufo_beam: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
                _next_phase_duration: None,
            },
        })
    }

    pub fn update_ufo_movement(&mut self) -> bool {
        if let (Some(actor), Some(target)) = (&self._actor_ufo, &self._actor_koa) {
            let to_target = target.borrow().get_position() - actor.borrow().get_position();
            if math::make_vector_xz(&to_target).magnitude_squared() < 1.0 {
                actor.borrow_mut().set_move_idle();
                true
            } else {
                actor.borrow_mut().set_move(&math::safe_normalize(&to_target));
                false
            }
        } else {
            false
        }
    }

    pub fn update_be_abducted_actor(&mut self, actor: RcRefCell<Character>, delta_time: f64) -> bool {
        if let Some(target) = &self._actor_ufo {
            let to_target = target.borrow().get_position() - actor.borrow().get_position();
            if 1.0 < to_target.magnitude_squared() {
                let mut pos = *actor.borrow().get_position();
                let speed = 4.0f32;
                pos += math::safe_normalize(&to_target) * speed * delta_time as f32;
                actor.borrow_mut().set_position(&pos);
                return false;
            }
            true
        } else {
            false
        }
    }

    pub fn update_be_abducted(&mut self, delta_time: f64) -> bool {
        let aru = self._actor_aru.clone();
        let ewa = self._actor_ewa.clone();
        let koa = self._actor_koa.clone();

        if let (Some(aru), Some(ewa), Some(koa)) = (aru, ewa, koa) {
            let arrived_aru = self.update_be_abducted_actor(aru, delta_time);
            let arrived_ewa = self.update_be_abducted_actor(ewa, delta_time);
            let arrived_koa = self.update_be_abducted_actor(koa, delta_time);
            arrived_aru && arrived_ewa && arrived_koa
        } else {
            false
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioUfo<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        self._scenario_track._scenario_phase = ScenarioPhase::from_str(scenario_phase.as_str()).unwrap_or(ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(_data) = serde_json::from_str::<ScenarioUfoSaveData>(&scenario_save_data._scenario_data) {
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioUfoSaveData {
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

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }

        if game_scene_data_name == Stages::Home.get_stage_data_name() {
            self._actor_ufo = game_scene_manager.get_actor_by_name("ufo").cloned();
            self._actor_aru = game_scene_manager.get_actor_by_name("monkey_aru").cloned();
            self._actor_ewa = game_scene_manager.get_actor_by_name("monkey_ewa").cloned();
            self._actor_koa = game_scene_manager.get_actor_by_name("monkey_koa").cloned();
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let game_ui_manager = ptr_as_mut(game_scene_manager._game_ui_manager);

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
            let phase_ratio = self._scenario_track.get_phase_ratio();
            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    if state == State::Update {
                        game_scene_manager.set_time(TIME_OF_DAWN, 0.0);
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::AppearUfo, Some(3.0));
                    }
                }
                ScenarioPhase::AppearUfo => match state {
                    State::Begin => {
                        self._audio_ufo_flying = game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_FLYING,
                            AudioLoop::LOOP,
                            Some(1.0),
                        );
                    }
                    State::Update => {
                        self.update_ufo_movement();
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::UfoLongShot, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::UfoLongShot => match state {
                    State::Begin => {
                        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                        main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
                        main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));
                        if let Some(actor) = &self._actor_aru {
                            actor.borrow_mut().set_action_sleep_no_snoring();
                        }
                    }
                    State::Update => {
                        if self.update_ufo_movement() {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::BeAbducted, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::BeAbducted => match state {
                    State::Begin => {
                        if let Some(actor) = &self._actor_aru {
                            actor.borrow_mut()._controller.set_flying_mode(true);
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut()._controller.set_flying_mode(true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut()._controller.set_flying_mode(true);
                        }
                        self._audio_ufo_beam = game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_BEAM,
                            AudioLoop::ONCE,
                            Some(1.0),
                        );
                    }
                    State::Update => {
                        if self.update_be_abducted(delta_time) {
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
                        if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                            game_scene_manager.get_scene_manager().stop_audio_instance(audio_instance)
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
