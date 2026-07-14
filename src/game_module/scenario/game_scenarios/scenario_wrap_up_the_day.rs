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
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut, ptr_as_ref};
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

pub struct ScenarioWrapUpTheDay<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
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
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWrapUpTheDay<'a>> {
        newRcRefCell(ScenarioWrapUpTheDay {
            _is_load_completed: false,
            _scenario_type: scenario_type,
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
    let mut pos = table.as_ref().unwrap().borrow().get_position() - math::safe_normalize(direction) * 2.0;
    pos.y = scene_manager.get_height_bilinear(&pos, 0);
    actor.as_ref().unwrap().borrow_mut().set_position(&pos);
    actor.as_ref().unwrap().borrow_mut().look_at(table.as_ref().unwrap().borrow().get_position());
    actor.as_ref().unwrap().borrow_mut().set_action_dance();
}

fn go_to_sleep(actor: &Option<RcRefCell<Character>>, bed: &Option<RcRefCell<Prop>>) {
    if let Some(actor) = actor.as_ref() {
        let radius = bed.as_ref().unwrap().borrow().get_collision()._bounding_box._mag_xz * 0.5;
        let (direction, dist) = math::make_normalize_xz_with_norm(
            &(bed.as_ref().unwrap().borrow().get_position() - actor.borrow().get_position()),
        );
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
        self._scenario_track._scenario_phase = ScenarioPhase::from_str(scenario_phase.as_str()).unwrap();
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

    fn destroy_game_scenario(&mut self) {}

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self._is_load_completed = false;
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_aru") {
            self._actor_aru = Some(actor.clone());
        } else if let Some(actor) = game_scene_manager.get_actor_by_name("aru") {
            self._actor_aru = Some(actor.clone());
        }

        if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_ewa") {
            self._actor_ewa = Some(actor.clone());
        } else if let Some(actor) = game_scene_manager.get_actor_by_name("ewa") {
            self._actor_ewa = Some(actor.clone());
        }

        if let Some(actor) = game_scene_manager.get_actor_by_name("monkey_koa") {
            self._actor_koa = Some(actor.clone());
        } else if let Some(actor) = game_scene_manager.get_actor_by_name("koa") {
            self._actor_koa = Some(actor.clone());
        }

        self._prop_table = Some(game_scene_manager.get_prop_manager().get_prop_by_name("table").unwrap().clone());
        self._prop_bed_for_aru =
            Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").unwrap().clone());
        self._prop_bed_for_ewa =
            Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").unwrap().clone());
        self._prop_bed_for_koa =
            Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").unwrap().clone());

        self._is_load_completed = true;
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
                        self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior_none();
                        self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior_none();
                        self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior_none();
                        self._actor_aru.as_ref().unwrap().borrow_mut().set_action_none();
                        self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_none();
                        self._actor_koa.as_ref().unwrap().borrow_mut().set_action_none();
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

                        if (self._audio_bgm.is_some()
                            && !game_scene_manager
                                .get_audio_manager()
                                .is_playing_audio_instance(self._audio_bgm.as_ref().unwrap()))
                            || 1.0 <= phase_ratio
                        {
                            game_scene_manager
                                .get_audio_manager_mut()
                                .stop_audio_instance(self._audio_bgm.as_ref().unwrap());
                            game_scene_manager.get_audio_manager_mut().play_audio_bank(
                                AUDIO_QUEST_COMPLETE,
                                AudioLoop::ONCE,
                                None,
                            );
                            self._actor_aru.as_ref().unwrap().borrow_mut().set_action_none();
                            self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_none();
                            self._actor_koa.as_ref().unwrap().borrow_mut().set_action_none();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToSleep, Some(10.0));
                        }
                    }
                }
                ScenarioPhase::GoToSleep => {
                    if state == State::Update && 3.0 < phase_time {
                        go_to_sleep(&self._actor_aru, &self._prop_bed_for_aru);
                        go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa);
                        go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa);

                        if self._actor_aru.as_ref().unwrap().borrow().is_action(ActionAnimationState::Sleep) {
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
                                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_wake_up();
                                self._actor_ewa
                                    .as_ref()
                                    .unwrap()
                                    .borrow_mut()
                                    .set_next_behavior(BehaviorState::WakeUp, true);
                                self._actor_koa
                                    .as_ref()
                                    .unwrap()
                                    .borrow_mut()
                                    .set_next_behavior(BehaviorState::WakeUp, true);
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
