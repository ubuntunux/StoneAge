use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{
    AUDIO_UFO_BEAM, AUDIO_UFO_FLYING, CAMERA_DISTANCE_MAX, CAMERA_OFFSET_Y, TIME_OF_EARLY_MORNING,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    ReleaseFamily,
    UfoGone,
    DropMonolith,
    ReallyUfoGone,
    CloseUpShot,
    Awake,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioDayOneSaveData {}

pub struct ScenarioDayOne<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _game_scene_manager: *const GameSceneManager<'a>,
    _around_start_position: Vector3<f32>,
    _around_end_position: Vector3<f32>,
    _around_start_rotation: Vector3<f32>,
    _around_end_rotation: Vector3<f32>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _prop_monolith: Option<RcRefCell<Prop<'a>>>,
    _monolith_start_position: Vector3<f32>,
    _audio_ufo_flying: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioDayOne<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioDayOne<'a>> {
        newRcRefCell(ScenarioDayOne {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _game_scene_manager: game_scene_manager,
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: Vector3::new(0.4, 0.0, 0.0),
            _around_end_rotation: Vector3::new(0.35, 0.0, 0.0),
            _actor_ufo: None,
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _prop_monolith: None,
            _monolith_start_position: Vector3::zeros(),
            _audio_ufo_flying: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_duration: None,
                _next_phase_duration: None,
                _phase_time: 0.0,
            },
        })
    }

    pub fn update_release_actor(
        &mut self,
        actor: RcRefCell<Character>,
        target: RcRefCell<Prop>,
        delta_time: f64,
    ) -> bool {
        let pos_before = *actor.borrow().get_position();
        let target_position = *target.borrow().get_position();
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
        let player = self._player.clone();
        let bed_aru = self._prop_bed_for_aru.clone();
        let actor_ewa = self._actor_ewa.clone();
        let bed_ewa = self._prop_bed_for_ewa.clone();
        let actor_koa = self._actor_koa.clone();
        let bed_koa = self._prop_bed_for_koa.clone();

        if let (Some(player), Some(bed_aru), Some(actor_ewa), Some(bed_ewa), Some(actor_koa), Some(bed_koa)) =
            (player, bed_aru, actor_ewa, bed_ewa, actor_koa, bed_koa)
        {
            let arrived_aru = self.update_release_actor(player, bed_aru, delta_time);
            let arrived_ewa = self.update_release_actor(actor_ewa, bed_ewa, delta_time);
            let arrived_koa = self.update_release_actor(actor_koa, bed_koa, delta_time);
            arrived_aru && arrived_ewa && arrived_koa
        } else {
            false
        }
    }

    pub fn drop_monolith(&mut self, delta_time: f64) -> bool {
        if let Some(prop) = &self._prop_monolith {
            let pos_before = *prop.borrow().get_position();
            let to_target = self._monolith_start_position - pos_before;
            let move_dist = 10.0f32 * delta_time as f32;
            let moved_pos = pos_before + math::safe_normalize(&to_target) * move_dist;
            let range = 0.1f32;
            if math::check_arrival_with_radius(&pos_before, &moved_pos, &self._monolith_start_position, range, false) {
                prop.borrow_mut().set_position(&moved_pos);
                return true;
            }

            prop.borrow_mut().set_position(&moved_pos);
            false
        } else {
            false
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

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        self._scenario_track._scenario_phase =
            ScenarioPhase::from_str(scenario_phase.as_str()).unwrap_or(ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioDayOneSaveData {};
        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: Default::default(),
            _scenario_data: serde_json::to_string(&save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        self._scenario_track._scenario_phase != ScenarioPhase::End
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {}

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }
        // if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_aru").cloned() {
        //     game_scene_manager.get_character_manager_mut().remove_character(&actor);
        // }
        if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_ewa").cloned() {
            game_scene_manager.get_character_manager_mut().remove_character(&actor);
        }
        if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_koa").cloned() {
            game_scene_manager.get_character_manager_mut().remove_character(&actor);
        }

        self._actor_ufo = game_scene_manager.get_actor_by_name("ufo").cloned();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager.get_actor_by_name("ewa").cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name("koa").cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
        self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name("monolith").cloned();

        if let Some(ufo) = &self._actor_ufo {
            if let Some(actor) = &self._player {
                actor.borrow_mut()._controller.set_flying_mode(true);
                actor.borrow_mut().set_behavior_none();
                actor.borrow_mut().set_action_sleep_no_snoring();
                actor.borrow_mut().set_position(ufo.borrow().get_position());
            }

            if let Some(actor) = &self._actor_ewa {
                actor.borrow_mut()._controller.set_flying_mode(true);
                actor.borrow_mut().set_behavior_none();
                actor.borrow_mut().set_action_sleep_no_snoring();
                actor.borrow_mut().set_position(ufo.borrow().get_position());
            }

            if let Some(actor) = &self._actor_koa {
                actor.borrow_mut()._controller.set_flying_mode(true);
                actor.borrow_mut().set_behavior_none();
                actor.borrow_mut().set_action_sleep_no_snoring();
                actor.borrow_mut().set_position(ufo.borrow().get_position());
            }
        }

        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
        main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
        main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));

        let pivot = if let Some(actor) = &self._player {
            *actor.borrow().get_center()
        } else {
            Vector3::zeros()
        };
        let start_rotation_matrix = math::make_rotation_matrix(
            self._around_start_rotation.x,
            self._around_start_rotation.y,
            self._around_start_rotation.z,
        );
        self._around_start_position = pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

        if let Some(prop) = &self._prop_monolith {
            self._monolith_start_position = *prop.borrow().get_position();
            prop.borrow()._render_object.borrow_mut().set_visible(false);
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

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

            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    if state == State::Update {
                        game_scene_manager.set_time(TIME_OF_EARLY_MORNING, 0.0);
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::ReleaseFamily, Some(6.0));
                    }
                }
                ScenarioPhase::ReleaseFamily => match state {
                    State::Begin => {
                        self._audio_ufo_flying = game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_FLYING,
                            AudioLoop::LOOP,
                            Some(1.0),
                        );
                        game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_BEAM,
                            AudioLoop::ONCE,
                            Some(1.0),
                        );
                    }
                    State::Update => {
                        let complete = self.update_release_family(delta_time);
                        if complete {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::UfoGone, Some(3.0));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::UfoGone => {
                    if state == State::Update {
                        if let Some(ufo) = &self._actor_ufo {
                            ufo.borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
                        }
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::DropMonolith, None);
                        }
                    }
                }
                ScenarioPhase::DropMonolith => {
                    if state == State::Update {
                        let radius = 0.5;
                        let mut drop_completed: bool = false;
                        let ufo_at_target = if let (Some(ufo), Some(_prop)) = (&self._actor_ufo, &self._prop_monolith) {
                            ufo.borrow_mut().move_to_target(&self._monolith_start_position, radius)
                        } else {
                            false
                        };

                        if ufo_at_target {
                            let mut is_visible = false;
                            if let Some(prop) = &self._prop_monolith {
                                is_visible = prop.borrow()._render_object.borrow().is_visible();
                            }
                            if !is_visible {
                                if let Some(prop) = &self._prop_monolith {
                                    prop.borrow()._render_object.borrow_mut().set_visible(true);
                                    if let Some(ufo) = &self._actor_ufo {
                                        prop.borrow_mut().set_position(ufo.borrow().get_position());
                                    }
                                }
                                game_scene_manager.get_scene_manager().play_audio_options(
                                    AUDIO_UFO_BEAM,
                                    AudioLoop::ONCE,
                                    Some(1.0),
                                );
                            }
                            drop_completed = self.drop_monolith(delta_time);
                        }

                        if drop_completed {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::ReallyUfoGone, Some(3.0));
                        }
                    }
                }
                ScenarioPhase::ReallyUfoGone => match state {
                    State::Update => {
                        if let Some(ufo) = &self._actor_ufo {
                            ufo.borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
                        }
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::CloseUpShot, Some(3.0));
                        }
                    }
                    State::End => {
                        if let Some(audio_instance) = self._audio_ufo_flying.as_ref() {
                            game_scene_manager.get_scene_manager().stop_audio_instance(audio_instance)
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::CloseUpShot => match state {
                    State::Begin => {
                        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                        main_camera._transform_object.set_position(&self._around_start_position);
                        main_camera._transform_object.set_rotation(&self._around_start_rotation);
                        if let Some(actor) = &self._player {
                            actor.borrow_mut().set_action_sleep();
                        }
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Awake, Some(5.0));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Awake => match state {
                    State::Begin => {
                        if let Some(ufo) = &self._actor_ufo {
                            game_scene_manager.get_character_manager_mut().remove_character(ufo);
                        }

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
                    }
                    State::Update => {
                        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                        let pivot = if let Some(actor) = &self._player {
                            *actor.borrow().get_center() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0)
                        } else {
                            Vector3::zeros()
                        };
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
                ScenarioPhase::End => {}
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
