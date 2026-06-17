use std::str::FromStr;
use nalgebra::{Vector3};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::game_constants::{AUDIO_UFO_BEAM, AUDIO_UFO_FLYING, DEFAULT_FADE_TIME, MATERIAL_UI_NONE, TIME_OF_DAWN};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, Stages};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    AppearUfo,
    UfoLongShot,
    BeAbducted,
    End,
}

pub struct ScenarioUfo<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _audio_ufo_beam: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
}

impl<'a> ScenarioUfo<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioUfo<'a>> {
        newRcRefCell(ScenarioUfo {
            _is_load_completed: false,
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_ufo: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _audio_ufo_flying: None,
            _audio_ufo_beam: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            }
        })
    }

    pub fn update_ufo_movement(&mut self) -> bool {
        let actor = self._actor_ufo.as_ref().unwrap();
        let target = self._actor_koa.as_ref().unwrap();
        let to_target = target.borrow().get_position() - actor.borrow().get_position();
        if math::make_vector_xz(&to_target).magnitude_squared() < 1.0 {
            actor.borrow_mut().set_move_idle();
            true
        } else {
            actor.borrow_mut().set_move(&math::safe_normalize(&to_target));
            false
        }
    }

    pub fn update_be_abducted_actor(&mut self, actor: RcRefCell<Character>, delta_time: f64) -> bool {
        let target = self._actor_ufo.as_ref().unwrap();
        let to_target = target.borrow().get_position() - actor.borrow().get_position();
        if 1.0 < to_target.magnitude_squared() {
            let mut pos = actor.borrow().get_position().clone();
            let speed = 4.0f32;
            pos += math::safe_normalize(&to_target) * speed * delta_time as f32;
            actor.borrow_mut().set_position(&pos);
            return false;
        }
        true
    }

    pub fn update_be_abducted(&mut self, delta_time: f64) -> bool {
        let arrived_aru = self.update_be_abducted_actor(self._actor_aru.as_ref().unwrap().clone(), delta_time);
        let arrived_ewa = self.update_be_abducted_actor(self._actor_ewa.as_ref().unwrap().clone(), delta_time);
        let arrived_koa = self.update_be_abducted_actor(self._actor_koa.as_ref().unwrap().clone(), delta_time);
        arrived_aru && arrived_ewa && arrived_koa
    }
}

impl<'a> ScenarioBase<'a> for ScenarioUfo<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_load_completed(&self) -> bool {
        self._is_load_completed
    }

    fn is_play_scenario_mode(&self) -> bool {
        true
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self._is_load_completed = false;
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if game_scene_data_name == Stages::Home.get_stage_data_name() {
            self._actor_ufo = Some(game_scene_manager.get_actor("ufo").unwrap().clone());
            self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
            self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
            self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
        }

        self._is_load_completed = true;
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>) {
        let next_scenario_phase = ScenarioPhase::from_str(next_scenario_phase).expect("scenario error");
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::AppearUfo => {
                self._audio_ufo_flying = game_scene_manager.get_scene_manager().play_audio_options(
                    AUDIO_UFO_FLYING,
                    AudioLoop::LOOP,
                    Some(1.0),
                );
            }
            ScenarioPhase::UfoLongShot => {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
                main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));

                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
            }
            ScenarioPhase::BeAbducted => {
                self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
                self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
                self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
                self._audio_ufo_beam = game_scene_manager.get_scene_manager().play_audio_options(
                    AUDIO_UFO_BEAM,
                    AudioLoop::ONCE,
                    Some(1.0),
                );
            }
            ScenarioPhase::End => {
                if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                    game_scene_manager.get_scene_manager().stop_audio_instance(&audio_instance)
                }

                game_ui_manager.set_auto_fade_inout(true);
                game_scene_manager.open_game_scenario(ScenarioType::ScenarioRevolution);
            }
            _ => {}
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => ()
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        let _phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        let current_scenario_phase = self._scenario_track._scenario_phase;
        match current_scenario_phase {
            ScenarioPhase::Begin => {
                game_scene_manager.set_time_of_day(TIME_OF_DAWN, 0.0);
                self.set_scenario_phase(ScenarioPhase::AppearUfo.to_string().as_str(), Some(3.0));
            },
            ScenarioPhase::AppearUfo => {
                self.update_ufo_movement();
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::UfoLongShot.to_string().as_str(), None);
                }
            },
            ScenarioPhase::UfoLongShot => {
                let arrived = self.update_ufo_movement();
                if arrived {
                    self.set_scenario_phase(ScenarioPhase::BeAbducted.to_string().as_str(), None);
                }
            },
            ScenarioPhase::BeAbducted => {
                let complete = self.update_be_abducted(delta_time);
                if complete {
                    if game_ui_manager.is_done_manual_fade_out() {
                        self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
                    } else {
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                }
            }
            ScenarioPhase::End => {}
        }

        self._scenario_track.update_scenario_track(current_scenario_phase, delta_time as f32);
    }
}
