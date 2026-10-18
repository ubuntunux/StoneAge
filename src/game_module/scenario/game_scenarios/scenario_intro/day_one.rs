use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    ACTOR_NAME_EWA, ACTOR_NAME_KOA, ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA, ACTOR_NAME_UFO, AUDIO_UFO_BEAM,
    AUDIO_UFO_FLYING, BED_FOR_ARU, BED_FOR_EWA, BED_FOR_KOA, CAMERA_DISTANCE_MAX, CAMERA_OFFSET_Y, PROP_NAME_MONOLITH,
    TIME_OF_EARLY_MORNING,
};
use crate::game_module::game_service_locator::{get_character_manager_mut, get_game_scene_manager_mut};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{set_actors_flying_sleep_mode, wake_up_npc_on_bed};
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
const INITIAL_CAMERA_POS: Vector3<f32> = Vector3::new(13.48, 26.56, -5.02);
const INITIAL_CAMERA_ROT: Vector3<f32> = Vector3::new(0.76, 0.33, 0.0);
const AROUND_CAMERA_EXTRA_DIST: f32 = 6.0;
const RELEASE_ACTOR_SPEED: f32 = 4.0;
const MONOLITH_DROP_SPEED: f32 = 10.0;
const ARRIVAL_RADIUS: f32 = 0.1;
const MONOLITH_INITIAL_SCALE: f32 = 0.1;
const PHASE_DURATION_RELEASE_FAMILY: f32 = 6.0;
const PHASE_DURATION_DROP_MONOLITH: f32 = 3.0;
const PHASE_DURATION_UFO_GONE: f32 = 3.0;
const PHASE_DURATION_CLOSE_UP: f32 = 3.0;
const PHASE_DURATION_AWAKE: f32 = 5.0;

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    ReleaseFamily,
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
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _prop_monolith: None,
            _monolith_start_position: Vector3::zeros(),
            _audio_ufo_flying: None,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
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
        let move_dist = RELEASE_ACTOR_SPEED * delta_time as f32;
        let moved_pos = pos_before + math::safe_normalize(&to_target) * move_dist;
        if math::check_arrival_with_radius(&pos_before, &moved_pos, &target_position, ARRIVAL_RADIUS, false) {
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
}

impl<'a> ScenarioBase<'a> for ScenarioDayOne<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        self._scenario_track.set_phase_from_str(scenario_phase.as_str(), ScenarioPhase::None);
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

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {}

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        self._scenario_create_info.spawn_and_reset_if_matching(game_scene_data_name);

        let game_scene_manager = get_game_scene_manager_mut();
        for monkey_name in [ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA] {
            if let Some(actor) = game_scene_manager.get_actor_by_name(monkey_name).cloned() {
                get_character_manager_mut().remove_character(&actor);
            }
        }

        self._actor_ufo = game_scene_manager.get_actor_by_name(ACTOR_NAME_UFO).cloned();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager.get_actor_by_name(ACTOR_NAME_EWA).cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name(ACTOR_NAME_KOA).cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_EWA).cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_KOA).cloned();
        self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();

        if let Some(ufo) = &self._actor_ufo {
            let ufo_pos = *ufo.borrow().get_position();
            set_actors_flying_sleep_mode(
                &[&self._player, &self._actor_ewa, &self._actor_koa],
                true,
                Some(&ufo_pos),
            );
        }

        let main_camera = get_scene_manager().get_main_camera_mut();
        main_camera._transform_object.set_position(&INITIAL_CAMERA_POS);
        main_camera._transform_object.set_rotation(&INITIAL_CAMERA_ROT);

        let pivot =
            self._prop_bed_for_aru.as_ref().map(|prop| *prop.borrow().get_position()).unwrap_or_else(Vector3::zeros);
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

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();

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
                        if let Some(ufo) = &self._actor_ufo {
                            let mut ufo_position = *ufo.borrow().get_position();
                            ufo_position.x = self._around_start_position.x;
                            ufo_position.z = self._around_start_position.z;
                            ufo.borrow_mut().set_position(&ufo_position);
                        }
                    } else if state == State::Update {
                        game_scene_manager.set_time(TIME_OF_EARLY_MORNING, 0.0);
                        self._scenario_track
                            .set_next_scenario_phase(ScenarioPhase::ReleaseFamily, Some(PHASE_DURATION_RELEASE_FAMILY));
                    }
                }
                ScenarioPhase::ReleaseFamily => match state {
                    State::Begin => {
                        if let (Some(ufo), Some(bed_aru)) = (&self._actor_ufo, &self._prop_bed_for_aru) {
                            let prop_bed_position = *bed_aru.borrow().get_position();
                            let mut ufo_position = *ufo.borrow().get_position();
                            ufo_position.x = prop_bed_position.x;
                            ufo_position.z = prop_bed_position.z;
                            ufo.borrow_mut().set_position(&ufo_position);
                            for actor in [&self._player, &self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                                actor.borrow_mut().set_position(&ufo_position);
                            }
                        }

                        self._audio_ufo_flying =
                            get_audio_manager_mut().play_audio_bank(AUDIO_UFO_FLYING, AudioLoop::LOOP, Some(1.0));
                        get_audio_manager_mut().play_audio_bank(AUDIO_UFO_BEAM, AudioLoop::ONCE, Some(1.0));
                    }
                    State::Update => {
                        if self.update_release_family(delta_time) {
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
                        let main_camera = get_scene_manager().get_main_camera_mut();
                        main_camera._transform_object.set_position(&self._around_start_position);
                        main_camera._transform_object.set_rotation(&self._around_start_rotation);
                        if let Some(actor) = &self._player {
                            actor.borrow_mut().set_action_sleep();
                        }
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Awake, Some(PHASE_DURATION_AWAKE));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Awake => match state {
                    State::Begin => {
                        if let Some(ufo) = &self._actor_ufo {
                            get_character_manager_mut().remove_character(ufo);
                        }

                        if let Some(actor) = &self._player {
                            actor.borrow_mut()._controller.set_flying_mode(false);
                            if let Some(bed) = &self._prop_bed_for_aru {
                                actor.borrow_mut().set_position(bed.borrow().get_position());
                            }
                            actor.borrow_mut().set_action_wake_up();
                        }

                        wake_up_npc_on_bed(&self._actor_ewa, Some(&self._prop_bed_for_ewa));
                        wake_up_npc_on_bed(&self._actor_koa, Some(&self._prop_bed_for_koa));
                    }
                    State::Update => {
                        let main_camera = get_scene_manager().get_main_camera_mut();
                        let pivot = self
                            ._player
                            .as_ref()
                            .map(|actor| *actor.borrow().get_center() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0))
                            .unwrap_or_else(Vector3::zeros);
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
