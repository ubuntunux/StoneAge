use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{
    AUDIO_QUEST_COMPLETE, AUDIO_ROOSTER, AUDIO_WRAP_UP_THE_DAY, DEFAULT_BGM_VOLUME, DEFAULT_FADE_TIME, GAME_MUSIC,
    MATERIAL_UI_NONE, SLEEP_TIMER,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const TABLE_SCENE_CAMERA_POSITION: [f32; 3] = [23.27, 3.64, 19.15];
const TABLE_SCENE_CAMERA_ROTATION: [f32; 3] = [0.06, -3.13, 0.0];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    Performance,
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
    _game_scene_manager: *const GameSceneManager<'a>,
    _sleep_timer: f32,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _audio_bgm: Option<RcRefCell<AudioInstance>>,
    _skip_wakeup: bool,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioWrapUpTheDay<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWrapUpTheDay<'a>> {
        newRcRefCell(ScenarioWrapUpTheDay {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _game_scene_manager: game_scene_manager,
            _sleep_timer: 0.0,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _audio_bgm: None,
            _skip_wakeup: false,
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
}

fn dance_around_the_table(
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
        actor_ref.borrow_mut().set_action_dance();
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

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {}

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        self._actor_aru = game_scene_manager
            .get_actor_by_name("monkey_aru")
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name("aru").cloned());
        self._actor_ewa = game_scene_manager
            .get_actor_by_name("monkey_ewa")
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name("ewa").cloned());
        self._actor_koa = game_scene_manager
            .get_actor_by_name("monkey_koa")
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name("koa").cloned());
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name("table").cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
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

            let phase_time = self._scenario_track.get_phase_time();
            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    if state == State::Update {
                        if let Some(actor) = &self._actor_aru {
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
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::Performance, Some(10.0));
                    }
                }
                ScenarioPhase::Performance => {
                    if state == State::Update {
                        if game_ui_manager.is_done_manual_fade_out() {
                            game_scene_manager.stop_bgm();
                            self._audio_bgm = game_scene_manager.get_audio_manager_mut().play_audio_bank(
                                AUDIO_WRAP_UP_THE_DAY,
                                AudioLoop::SOME(4),
                                None,
                            );

                            let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                            main_camera._transform_object.set_position(&Vector3::from(TABLE_SCENE_CAMERA_POSITION));
                            main_camera._transform_object.set_rotation(&Vector3::from(TABLE_SCENE_CAMERA_ROTATION));

                            dance_around_the_table(
                                game_scene_manager.get_scene_manager(),
                                &self._actor_aru,
                                &self._prop_table,
                                &Vector3::new(1.0, 0.0, 0.0),
                            );
                            dance_around_the_table(
                                game_scene_manager.get_scene_manager(),
                                &self._actor_ewa,
                                &self._prop_table,
                                &Vector3::new(0.0, 0.0, 1.0),
                            );
                            dance_around_the_table(
                                game_scene_manager.get_scene_manager(),
                                &self._actor_koa,
                                &self._prop_table,
                                &Vector3::new(-1.0, 0.0, 0.0),
                            );
                            game_ui_manager.set_auto_fade_inout(true);
                        }

                        let is_playing = if let Some(audio_bgm) = &self._audio_bgm {
                            game_scene_manager.get_audio_manager().is_playing_audio_instance(audio_bgm)
                        } else {
                            false
                        };

                        if (self._audio_bgm.is_some() && !is_playing) || 1.0 <= phase_ratio {
                            if let Some(audio_bgm) = &self._audio_bgm {
                                game_scene_manager.get_audio_manager_mut().stop_audio_instance(audio_bgm);
                            }
                            game_scene_manager.get_audio_manager_mut().play_audio_bank(
                                AUDIO_QUEST_COMPLETE,
                                AudioLoop::ONCE,
                                None,
                            );
                            if let Some(actor) = &self._actor_aru {
                                actor.borrow_mut().set_action_none();
                            }
                            if let Some(actor) = &self._actor_ewa {
                                actor.borrow_mut().set_action_none();
                            }
                            if let Some(actor) = &self._actor_koa {
                                actor.borrow_mut().set_action_none();
                            }
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToSleep, Some(10.0));
                        }
                    }
                }
                ScenarioPhase::GoToSleep => {
                    if state == State::Update && 3.0 < phase_time {
                        go_to_sleep(&self._actor_aru, &self._prop_bed_for_aru);
                        go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa);
                        go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa);

                        let aru_is_sleeping = self
                            ._actor_aru
                            .as_ref()
                            .map_or(false, |actor| actor.borrow().is_action(ActionAnimationState::Sleep));
                        if aru_is_sleeping {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
                        }
                    }
                }
                ScenarioPhase::Sleep => match state {
                    State::Begin => {
                        self._sleep_timer = 0.0;
                        game_scene_manager.play_bgm(GAME_MUSIC, DEFAULT_BGM_VOLUME);
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() && self._sleep_timer < SLEEP_TIMER {
                            self._sleep_timer += delta_time as f32;
                            if SLEEP_TIMER <= self._sleep_timer {
                                game_ui_manager.set_auto_fade_inout(true);
                                game_scene_manager.set_next_time_of_day();
                            }
                        } else if game_ui_manager.is_done_game_image_progress() {
                            if self._skip_wakeup {
                                self._skip_wakeup = false;
                            } else {
                                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_ROOSTER);
                                if let Some(actor) = &self._actor_aru {
                                    actor.borrow_mut().set_action_wake_up();
                                }
                                if let Some(actor) = &self._actor_ewa {
                                    actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
                                }
                                if let Some(actor) = &self._actor_koa {
                                    actor.borrow_mut().set_next_behavior(BehaviorState::WakeUp, true);
                                }
                            }
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
