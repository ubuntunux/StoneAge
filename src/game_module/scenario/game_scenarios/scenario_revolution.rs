use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::game_constants::{
    AUDIO_ALIEN_TALK, AUDIO_UFO_EXPERIMENT, AUDIO_UFO_LABORATORY, CHARACTER_INTERACTION_TIME, DEFAULT_FADE_TIME,
    MATERIAL_EMOJI_GOOD, MATERIAL_UI_NONE, TIME_OF_NOON,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioType};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxLayerType};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut, ptr_as_ref};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioPhase {
    None,
    Begin,
    Investigate,
    Discussion,
    Revolution,
    End,
}

fn update_actor_position(actor: &mut Character, position_y: f32) {
    actor.set_position(&Vector3::new(
        actor.get_position().x,
        position_y,
        actor.get_position().z,
    ));
}

pub struct ScenarioRevolution<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _alien_alpha: Option<RcRefCell<Character<'a>>>,
    _alien_beta: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _monkey_aru: Option<RcRefCell<Character<'a>>>,
    _monkey_ewa: Option<RcRefCell<Character<'a>>>,
    _monkey_koa: Option<RcRefCell<Character<'a>>>,
    _position_timer: f32,
    _position_y: f32,
    _audio_alien_talk: Option<RcRefCell<AudioInstance>>,
    _audio_ufo_laboratory: Option<RcRefCell<AudioInstance>>,
    _audio_ufo_experiment: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioRevolution<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioRevolution<'a>> {
        newRcRefCell(ScenarioRevolution {
            _is_load_completed: false,
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _alien_alpha: None,
            _alien_beta: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _monkey_aru: None,
            _monkey_ewa: None,
            _monkey_koa: None,
            _position_timer: 0.0,
            _position_y: 0.0,
            _audio_alien_talk: None,
            _audio_ufo_laboratory: None,
            _audio_ufo_experiment: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
                _next_phase_duration: None,
            },
        })
    }
}

impl<'a> ScenarioBase<'a> for ScenarioRevolution<'a> {
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
        self._alien_alpha = Some(game_scene_manager.get_actor_by_name("alien_alpha").unwrap().clone());
        self._alien_beta = Some(game_scene_manager.get_actor_by_name("alien_beta").unwrap().clone());
        self._monkey_aru = Some(game_scene_manager.get_actor_by_name("monkey_aru").unwrap().clone());
        self._monkey_ewa = Some(game_scene_manager.get_actor_by_name("monkey_ewa").unwrap().clone());
        self._monkey_koa = Some(game_scene_manager.get_actor_by_name("monkey_koa").unwrap().clone());
        self._actor_aru = Some(game_scene_manager.get_actor_by_name("aru").unwrap().clone());
        self._actor_ewa = Some(game_scene_manager.get_actor_by_name("ewa").unwrap().clone());
        self._actor_koa = Some(game_scene_manager.get_actor_by_name("koa").unwrap().clone());

        self._alien_alpha.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._alien_beta.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._monkey_aru.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._monkey_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._monkey_ewa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._monkey_ewa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._monkey_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._monkey_koa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._monkey_koa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._monkey_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);

        self._actor_aru.as_ref().unwrap().borrow_mut()._render_object.borrow_mut().set_visible(false);
        self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_ewa.as_ref().unwrap().borrow_mut()._render_object.borrow_mut().set_visible(false);
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_koa.as_ref().unwrap().borrow_mut()._render_object.borrow_mut().set_visible(false);
        self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior_none();
        self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep_no_snoring();
        self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);

        self._position_y = self._monkey_aru.as_ref().unwrap().borrow().get_position().y;

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
                        game_scene_manager.set_time(TIME_OF_NOON, 0.0);
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::Investigate, Some(2.0));
                    }
                }
                ScenarioPhase::Investigate => match state {
                    State::Begin => {
                        self._audio_ufo_laboratory = game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_LABORATORY,
                            AudioLoop::LOOP,
                            Some(0.2),
                        );
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Discussion, Some(5.0));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Discussion => match state {
                    State::Begin => {
                        let contents = vec![TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_GOOD))];
                        game_ui_manager.add_text_box_item(
                            TextBoxLayerType::InteractionLayer,
                            ActorWrapper::Character(self._alien_alpha.as_ref().unwrap().clone()),
                            &contents,
                            Some(CHARACTER_INTERACTION_TIME),
                        );
                        game_ui_manager.add_text_box_item(
                            TextBoxLayerType::InteractionLayer,
                            ActorWrapper::Character(self._alien_beta.as_ref().unwrap().clone()),
                            &contents,
                            Some(CHARACTER_INTERACTION_TIME),
                        );
                        self._alien_alpha
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .look_at(self._alien_beta.as_ref().unwrap().borrow().get_position());
                        self._alien_beta
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .look_at(self._alien_alpha.as_ref().unwrap().borrow().get_position());
                    }
                    State::Update => {
                        if phase_time == 0.0 {
                            game_scene_manager.get_scene_manager().play_audio_options(
                                AUDIO_ALIEN_TALK,
                                AudioLoop::ONCE,
                                Some(1.0),
                            );
                        } else if phase_time <= 2.0 && 2.0 < (phase_time + delta_time as f32) {
                            game_scene_manager.get_scene_manager().play_audio_options(
                                AUDIO_ALIEN_TALK,
                                AudioLoop::ONCE,
                                Some(1.0),
                            );
                        }

                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Revolution, Some(8.0));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Revolution => match state {
                    State::Begin => {
                        self._alien_alpha
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .look_at(self._actor_aru.as_ref().unwrap().borrow().get_position());
                        self._alien_beta
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .look_at(self._actor_aru.as_ref().unwrap().borrow().get_position());
                        game_scene_manager.get_scene_manager().play_audio_options(
                            AUDIO_UFO_EXPERIMENT,
                            AudioLoop::ONCE,
                            Some(1.0),
                        );
                    }
                    State::Update => {
                        let visible_monkey = if phase_ratio < 0.9 {
                            ((phase_ratio * phase_ratio * 100.0) as i32 % 2) == 0
                        } else {
                            false
                        };
                        self._monkey_aru
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(visible_monkey);
                        self._monkey_ewa
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(visible_monkey);
                        self._monkey_koa
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(visible_monkey);
                        self._actor_aru
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(!visible_monkey);
                        self._actor_ewa
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(!visible_monkey);
                        self._actor_koa
                            .as_ref()
                            .unwrap()
                            .borrow()
                            ._render_object
                            .borrow_mut()
                            .set_visible(!visible_monkey);
                        if 1.0 <= phase_ratio {
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
                        if let Some(audio_instance) = self._audio_ufo_laboratory.as_ref() {
                            game_scene_manager.get_scene_manager().stop_audio_instance(audio_instance);
                        }
                        self._audio_ufo_laboratory = None;
                        game_ui_manager.set_auto_fade_inout(true);
                        game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioDayOne, false);
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }

        if self._monkey_aru.is_some() {
            self._position_timer += delta_time as f32;
            let position_y = self._position_y + (self._position_timer.sin() * 0.5 + 0.5);
            update_actor_position(&mut self._monkey_aru.as_ref().unwrap().borrow_mut(), position_y);
            update_actor_position(&mut self._monkey_ewa.as_ref().unwrap().borrow_mut(), position_y);
            update_actor_position(&mut self._monkey_koa.as_ref().unwrap().borrow_mut(), position_y);
            update_actor_position(&mut self._actor_aru.as_ref().unwrap().borrow_mut(), position_y);
            update_actor_position(&mut self._actor_ewa.as_ref().unwrap().borrow_mut(), position_y);
            update_actor_position(&mut self._actor_koa.as_ref().unwrap().borrow_mut(), position_y);
        }
    }
}
