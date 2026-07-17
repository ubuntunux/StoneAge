use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::game_constants::{
    AUDIO_ALIEN_TALK, AUDIO_UFO_EXPERIMENT, AUDIO_UFO_LABORATORY, CHARACTER_INTERACTION_TIME, DEFAULT_FADE_TIME,
    MATERIAL_EMOJI_GOOD, MATERIAL_UI_NONE, TIME_OF_NOON,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxLayerType};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioRevolutionSaveData {
    pub _position_timer: f32,
    pub _position_y: f32,
}

fn update_actor_position(actor: &mut Character, position_y: f32) {
    actor.set_position(&Vector3::new(
        actor.get_position().x,
        position_y,
        actor.get_position().z,
    ));
}

pub struct ScenarioRevolution<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
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
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioRevolution<'a>> {
        newRcRefCell(ScenarioRevolution {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
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
        self._scenario_track._scenario_phase =
            ScenarioPhase::from_str(scenario_phase.as_str()).unwrap_or(ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(data) = serde_json::from_str::<ScenarioRevolutionSaveData>(&scenario_save_data._scenario_data) {
            self._position_timer = data._position_timer;
            self._position_y = data._position_y;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioRevolutionSaveData {
            _position_timer: self._position_timer,
            _position_y: self._position_y,
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

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }

        self._alien_alpha = game_scene_manager.get_actor_by_name("alien_alpha").cloned();
        self._alien_beta = game_scene_manager.get_actor_by_name("alien_beta").cloned();
        self._monkey_aru = game_scene_manager.get_actor_by_name("monkey_aru").cloned();
        self._monkey_ewa = game_scene_manager.get_actor_by_name("monkey_ewa").cloned();
        self._monkey_koa = game_scene_manager.get_actor_by_name("monkey_koa").cloned();
        self._actor_aru = game_scene_manager.get_actor_by_name("aru").cloned();
        self._actor_ewa = game_scene_manager.get_actor_by_name("ewa").cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name("koa").cloned();

        if let Some(actor) = &self._alien_alpha {
            actor.borrow_mut().set_behavior_none();
        }
        if let Some(actor) = &self._alien_beta {
            actor.borrow_mut().set_behavior_none();
        }
        if let Some(actor) = &self._monkey_aru {
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }
        if let Some(actor) = &self._monkey_ewa {
            actor.borrow_mut().set_behavior_none();
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }
        if let Some(actor) = &self._monkey_koa {
            actor.borrow_mut().set_behavior_none();
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }

        if let Some(actor) = &self._actor_aru {
            actor.borrow_mut()._render_object.borrow_mut().set_visible(false);
            actor.borrow_mut().set_behavior_none();
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }
        if let Some(actor) = &self._actor_ewa {
            actor.borrow_mut()._render_object.borrow_mut().set_visible(false);
            actor.borrow_mut().set_behavior_none();
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }
        if let Some(actor) = &self._actor_koa {
            actor.borrow_mut()._render_object.borrow_mut().set_visible(false);
            actor.borrow_mut().set_behavior_none();
            actor.borrow_mut().set_action_sleep_no_snoring();
            actor.borrow_mut()._controller.set_flying_mode(true);
        }

        if let Some(actor) = &self._monkey_aru {
            self._position_y = actor.borrow().get_position().y;
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
                        if let Some(alpha) = &self._alien_alpha {
                            game_ui_manager.add_text_box_item(
                                TextBoxLayerType::InteractionLayer,
                                ActorWrapper::Character(alpha.clone()),
                                &contents,
                                Some(CHARACTER_INTERACTION_TIME),
                            );
                        }
                        if let Some(beta) = &self._alien_beta {
                            game_ui_manager.add_text_box_item(
                                TextBoxLayerType::InteractionLayer,
                                ActorWrapper::Character(beta.clone()),
                                &contents,
                                Some(CHARACTER_INTERACTION_TIME),
                            );
                        }
                        if let (Some(alpha), Some(beta)) = (&self._alien_alpha, &self._alien_beta) {
                            alpha.borrow_mut().look_at(beta.borrow().get_position());
                            beta.borrow_mut().look_at(alpha.borrow().get_position());
                        }
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
                        if let Some(aru) = &self._actor_aru {
                            let aru_pos = aru.borrow().get_position().clone();
                            if let Some(alpha) = &self._alien_alpha {
                                alpha.borrow_mut().look_at(&aru_pos);
                            }
                            if let Some(beta) = &self._alien_beta {
                                beta.borrow_mut().look_at(&aru_pos);
                            }
                        }
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
                        if let Some(m) = &self._monkey_aru {
                            m.borrow()._render_object.borrow_mut().set_visible(visible_monkey);
                        }
                        if let Some(m) = &self._monkey_ewa {
                            m.borrow()._render_object.borrow_mut().set_visible(visible_monkey);
                        }
                        if let Some(m) = &self._monkey_koa {
                            m.borrow()._render_object.borrow_mut().set_visible(visible_monkey);
                        }
                        if let Some(a) = &self._actor_aru {
                            a.borrow()._render_object.borrow_mut().set_visible(!visible_monkey);
                        }
                        if let Some(a) = &self._actor_ewa {
                            a.borrow()._render_object.borrow_mut().set_visible(!visible_monkey);
                        }
                        if let Some(a) = &self._actor_koa {
                            a.borrow()._render_object.borrow_mut().set_visible(!visible_monkey);
                        }
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
                        game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioIntro_DayOne);
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
            if let Some(actor) = &self._monkey_aru {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
            if let Some(actor) = &self._monkey_ewa {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
            if let Some(actor) = &self._monkey_koa {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
            if let Some(actor) = &self._actor_aru {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
            if let Some(actor) = &self._actor_ewa {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
            if let Some(actor) = &self._actor_koa {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
        }
    }
}
