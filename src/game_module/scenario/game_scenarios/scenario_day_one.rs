use std::str::FromStr;
use nalgebra::Vector3;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{AUDIO_UFO_BEAM, AUDIO_UFO_FLYING, CAMERA_DISTANCE_MAX, CAMERA_OFFSET_Y, TIME_OF_EARLY_MORNING};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

pub static mut SKIP_SCENARIO: bool = false;

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    ReleaseFamily,
    UfoGone,
    DropMonolith,
    ReallyUfoGone,
    CloseUpShot,
    Awake,
    End,
}

pub struct ScenarioDayOne<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _around_start_position: Vector3<f32>,
    _around_end_position: Vector3<f32>,
    _around_start_rotation: Vector3<f32>,
    _around_end_rotation: Vector3<f32>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _prop_monolith: Option<RcRefCell<Prop<'a>>>,
    _monolith_start_position: Vector3<f32>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
}

impl<'a> ScenarioDayOne<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioDayOne<'a>> {
        newRcRefCell(ScenarioDayOne {
            _is_load_completed: false,
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: Vector3::new(0.4, 0.0, 0.0),
            _around_end_rotation: Vector3::new(0.35, 0.0, 0.0),
            _actor_ufo: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _prop_monolith: None,
            _monolith_start_position: Vector3::zeros(),
            _audio_ufo_flying: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            }
        })
    }

    pub fn update_release_actor(&mut self, actor: RcRefCell<Character>, target: RcRefCell<Prop>, delta_time: f64) -> bool {
        let pos_before = actor.borrow().get_position().clone();
        let target_position = target.borrow().get_position().clone();
        let to_target = target_position - pos_before;
        let move_dist = 4.0f32 * delta_time as f32;
        let moved_pos = pos_before + math::safe_normalize(&to_target) * move_dist;
        let range = 0.1f32;
        if math::check_arrival_with_radius(&pos_before, &moved_pos, &target_position, range, false) {
            actor.borrow_mut().set_position(&target_position);
            return true;
        }

        actor.borrow_mut().set_position(&moved_pos);
        false
    }

    pub fn update_release_family(&mut self, delta_time: f64) -> bool {
        let arrived_aru = self.update_release_actor(self._actor_aru.as_ref().unwrap().clone(), self._prop_bed_for_aru.as_ref().unwrap().clone(), delta_time);
        let arrived_ewa = self.update_release_actor(self._actor_ewa.as_ref().unwrap().clone(), self._prop_bed_for_ewa.as_ref().unwrap().clone(), delta_time);
        let arrived_koa = self.update_release_actor(self._actor_koa.as_ref().unwrap().clone(), self._prop_bed_for_koa.as_ref().unwrap().clone(), delta_time);
        arrived_aru && arrived_ewa && arrived_koa
    }

    pub fn drop_monolith(&mut self, delta_time: f64) -> bool {
        let pos_before = self._prop_monolith.as_ref().unwrap().borrow().get_position().clone();
        let to_target = self._monolith_start_position - pos_before;
        let move_dist = 10.0f32 * delta_time as f32;
        let moved_pos = pos_before + math::safe_normalize(&to_target) * move_dist;
        let range = 0.1f32;
        if math::check_arrival_with_radius(&pos_before, &moved_pos, &self._monolith_start_position, range, false) {
            self._prop_monolith.as_ref().unwrap().borrow_mut().set_position(&moved_pos);
            return true;
        }

        self._prop_monolith.as_ref().unwrap().borrow_mut().set_position(&moved_pos);
        false
    }
}

impl<'a> ScenarioBase<'a> for ScenarioDayOne<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_load_completed(&self) -> bool {
        self._is_load_completed
    }

    fn is_play_scenario_mode(&self) -> bool {
        self._scenario_track._scenario_phase != ScenarioPhase::End
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self._is_load_completed = false;
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        self._actor_ufo = Some(game_scene_manager.get_actor_by_name("ufo").unwrap().clone());
        self._actor_aru = Some(game_scene_manager.get_actor_by_name("aru").unwrap().clone());
        self._actor_ewa = Some(game_scene_manager.get_actor_by_name("ewa").unwrap().clone());
        self._actor_koa = Some(game_scene_manager.get_actor_by_name("koa").unwrap().clone());
        self._prop_bed_for_aru = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").unwrap().clone());
        self._prop_bed_for_ewa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").unwrap().clone());
        self._prop_bed_for_koa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").unwrap().clone());
        self._prop_monolith = Some(game_scene_manager.get_prop_manager().get_prop_by_name("monolith").unwrap().clone());

        self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_aru.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_koa.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
        main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
        main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));

        let pivot = self._actor_aru.as_ref().unwrap().borrow().get_center().clone();
        let start_rotation_matrix = math::make_rotation_matrix(self._around_start_rotation.x, self._around_start_rotation.y, self._around_start_rotation.z);
        self._around_start_position = pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

        self._monolith_start_position = self._prop_monolith.as_ref().unwrap().borrow().get_position().clone();
        self._prop_monolith.as_ref().unwrap().borrow()._render_object.borrow_mut().set_visible(false);

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
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::ReleaseFamily => {
                self._audio_ufo_flying = game_scene_manager.get_scene_manager().play_audio_options(AUDIO_UFO_FLYING, AudioLoop::LOOP, Some(1.0));
                game_scene_manager.get_scene_manager().play_audio_options(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
            }
            ScenarioPhase::CloseUpShot => {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep();
            }
            ScenarioPhase::Awake => {
                game_scene_manager.get_character_manager_mut().remove_character(self._actor_ufo.as_ref().unwrap());

                self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_position(&self._prop_bed_for_aru.as_ref().unwrap().borrow().get_position());
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_wake_up();

                self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_position(&self._prop_bed_for_ewa.as_ref().unwrap().borrow().get_position());
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::WakeUp, None, true);

                self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_position(&self._prop_bed_for_koa.as_ref().unwrap().borrow().get_position());
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::WakeUp, None, true);
            }
            ScenarioPhase::End => {
            }
            _ => {}
        }
    }

    fn update_game_scenario_end(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        match self._scenario_track._scenario_phase {
            ScenarioPhase::ReallyUfoGone => {
                if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                    game_scene_manager.get_scene_manager().stop_audio_instance(&audio_instance)
                }
            }
            _ => ()
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        let _phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        let current_scenario_phase = self._scenario_track._scenario_phase;
        match current_scenario_phase {
            ScenarioPhase::Begin => {
                if unsafe { SKIP_SCENARIO } {
                    self.set_scenario_phase(ScenarioPhase::Awake.to_string().as_str(), Some(6.0));
                } else {
                    game_scene_manager.set_time(TIME_OF_EARLY_MORNING, 0.0);
                    self.set_scenario_phase(ScenarioPhase::ReleaseFamily.to_string().as_str(), Some(6.0));
                }
            },
            ScenarioPhase::ReleaseFamily => {
                let complete = self.update_release_family(delta_time);
                if complete {
                    self.set_scenario_phase(ScenarioPhase::UfoGone.to_string().as_str(), Some(3.0));
                }
            },
            ScenarioPhase::UfoGone => {
                self._actor_ufo.as_ref().unwrap().borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::DropMonolith.to_string().as_str(), None);
                }
            },
            ScenarioPhase::DropMonolith => {
                let radius = 0.5;
                let mut drop_completed: bool = false;
                if self._actor_ufo.as_ref().unwrap().borrow_mut().move_to_target(&self._monolith_start_position, radius) {
                    if self._prop_monolith.as_ref().unwrap().borrow()._render_object.borrow().is_visible() == false {
                        self._prop_monolith.as_ref().unwrap().borrow()._render_object.borrow_mut().set_visible(true);
                        self._prop_monolith.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());
                        game_scene_manager.get_scene_manager().play_audio_options(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
                    }
                    drop_completed = self.drop_monolith(delta_time);
                }

                if drop_completed {
                    self.set_scenario_phase(ScenarioPhase::ReallyUfoGone.to_string().as_str(), Some(3.0));
                }
            },
            ScenarioPhase::ReallyUfoGone => {
                self._actor_ufo.as_ref().unwrap().borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::CloseUpShot.to_string().as_str(), Some(3.0));
                }
            },
            ScenarioPhase::CloseUpShot => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::Awake.to_string().as_str(), Some(5.0));
                }
            }
            ScenarioPhase::Awake => {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                let pivot = self._actor_aru.as_ref().unwrap().borrow().get_center().clone() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                self._around_end_position = pivot - main_camera._transform_object.get_front() * CAMERA_DISTANCE_MAX;

                let progress = phase_ratio.powf(2.0);
                let position = self._around_start_position.lerp(&self._around_end_position, progress);
                let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                main_camera._transform_object.set_position(&position);
                main_camera._transform_object.set_rotation(&rotation);

                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
                }
            }
            _ => {}
        }

        self._scenario_track.update_scenario_track(current_scenario_phase, delta_time as f32);
    }
}
