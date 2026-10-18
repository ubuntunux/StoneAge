use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::game_constants::{
    ACTOR_NAME_ALIEN_ALPHA, ACTOR_NAME_ALIEN_BETA, ACTOR_NAME_ARU, ACTOR_NAME_EWA, ACTOR_NAME_KOA,
    ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA, AUDIO_ALIEN_TALK, AUDIO_UFO_EXPERIMENT, AUDIO_UFO_LABORATORY,
    CHARACTER_DATA_NAME_ARU, CHARACTER_INTERACTION_TIME, DEFAULT_FADE_TIME, MATERIAL_EMOJI_GOOD, MATERIAL_UI_NONE,
    TIME_OF_NOON,
};
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{
    set_actors_behavior_none, set_actors_flying_sleep_mode, set_actors_visible,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxItemOption, TextBoxLayerType};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut};
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const PHASE_DURATION_INVESTIGATE: f32 = 2.0;
const PHASE_DURATION_DISCUSSION: f32 = 5.0;
const PHASE_DURATION_REVOLUTION: f32 = 8.0;
const ALIEN_TALK_SECOND_CUE_TIME: f32 = 2.0;
const REVOLUTION_BLINK_END_RATIO: f32 = 0.9;
const REVOLUTION_BLINK_FREQ: f32 = 100.0;
const FLOATING_OFFSET_AMPLITUDE: f32 = 0.5;

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
    _alien_alpha: Option<RcRefCell<Character<'a>>>,
    _alien_beta: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _player: Option<RcRefCell<Character<'a>>>,
    _monkey_ewa: Option<RcRefCell<Character<'a>>>,
    _monkey_koa: Option<RcRefCell<Character<'a>>>,
    _position_timer: f32,
    _position_y: f32,
    _audio_ufo_laboratory: Option<RcRefCell<AudioInstance>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioRevolution<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioRevolution<'a>> {
        newRcRefCell(ScenarioRevolution {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _alien_alpha: None,
            _alien_beta: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _player: None,
            _monkey_ewa: None,
            _monkey_koa: None,
            _position_timer: 0.0,
            _position_y: 0.0,
            _audio_ufo_laboratory: None,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
        })
    }

    pub fn remove_non_player_characters(&mut self) {
        let character_manager = get_character_manager_mut();
        for actor_opt in [
            &self._alien_alpha,
            &self._alien_beta,
            &self._actor_aru,
            &self._actor_ewa,
            &self._actor_koa,
            &self._monkey_ewa,
            &self._monkey_koa,
        ] {
            if let Some(actor) = actor_opt {
                character_manager.remove_character(actor);
            }
        }
        self._alien_alpha = None;
        self._alien_beta = None;
        self._actor_aru = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._monkey_ewa = None;
        self._monkey_koa = None;
    }
}

impl<'a> ScenarioBase<'a> for ScenarioRevolution<'a> {
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

    fn destroy_game_scenario(&mut self) {
        self.remove_non_player_characters();
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.remove_non_player_characters();
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        self._scenario_create_info.spawn_and_reset_if_matching(game_scene_data_name);

        let game_scene_manager = get_game_scene_manager_mut();
        self._alien_alpha = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_ALPHA).cloned();
        self._alien_beta = game_scene_manager.get_actor_by_name(ACTOR_NAME_ALIEN_BETA).cloned();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._monkey_ewa = game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_EWA).cloned();
        self._monkey_koa = game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_KOA).cloned();
        self._actor_aru = game_scene_manager.get_actor_by_name(ACTOR_NAME_ARU).cloned();
        self._actor_ewa = game_scene_manager.get_actor_by_name(ACTOR_NAME_EWA).cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name(ACTOR_NAME_KOA).cloned();

        set_actors_behavior_none(&[&self._alien_alpha, &self._alien_beta], false);
        set_actors_flying_sleep_mode(&[&self._player], false, None);
        set_actors_flying_sleep_mode(
            &[
                &self._monkey_ewa,
                &self._monkey_koa,
                &self._actor_aru,
                &self._actor_ewa,
                &self._actor_koa,
            ],
            true,
            None,
        );
        set_actors_visible(&[&self._actor_aru, &self._actor_ewa, &self._actor_koa], false);

        if let Some(actor) = &self._player {
            self._position_y = actor.borrow().get_position().y;
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

            let phase_time = self._scenario_track.get_phase_time();
            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    if state == State::Update {
                        game_scene_manager.set_time(TIME_OF_NOON, 0.0);
                        self._scenario_track
                            .set_next_scenario_phase(ScenarioPhase::Investigate, Some(PHASE_DURATION_INVESTIGATE));
                    }
                }
                ScenarioPhase::Investigate => match state {
                    State::Begin => {
                        self._audio_ufo_laboratory =
                            get_audio_manager_mut().play_audio_bank(AUDIO_UFO_LABORATORY, AudioLoop::LOOP, Some(0.2));
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Discussion, Some(PHASE_DURATION_DISCUSSION));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Discussion => match state {
                    State::Begin => {
                        let contents = vec![TextBoxContent::MaterialInstance(
                            String::from(MATERIAL_EMOJI_GOOD),
                            None,
                        )];
                        for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                            game_ui_manager.add_text_box_item(
                                ActorWrapper::Character(alien.clone()),
                                &contents,
                                &TextBoxItemOption {
                                    _layer_type: TextBoxLayerType::InteractionLayer,
                                    _duration: Some(CHARACTER_INTERACTION_TIME),
                                    ..Default::default()
                                },
                            );
                        }
                        if let (Some(alpha), Some(beta)) = (&self._alien_alpha, &self._alien_beta) {
                            alpha.borrow_mut().look_at(beta.borrow().get_position());
                            beta.borrow_mut().look_at(alpha.borrow().get_position());
                        }
                    }
                    State::Update => {
                        if phase_time == 0.0
                            || (phase_time <= ALIEN_TALK_SECOND_CUE_TIME
                                && ALIEN_TALK_SECOND_CUE_TIME < (phase_time + delta_time as f32))
                        {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ALIEN_TALK, AudioLoop::ONCE, Some(1.0));
                        }

                        if 1.0 <= phase_ratio {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Revolution, Some(PHASE_DURATION_REVOLUTION));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Revolution => match state {
                    State::Begin => {
                        if let Some(aru) = &self._actor_aru {
                            let aru_pos = *aru.borrow().get_position();
                            for alien in [&self._alien_alpha, &self._alien_beta].into_iter().flatten() {
                                alien.borrow_mut().look_at(&aru_pos);
                            }
                        }
                        get_audio_manager_mut().play_audio_bank(AUDIO_UFO_EXPERIMENT, AudioLoop::ONCE, Some(1.0));
                    }
                    State::Update => {
                        let visible_monkey = if phase_ratio < REVOLUTION_BLINK_END_RATIO {
                            ((phase_ratio * phase_ratio * REVOLUTION_BLINK_FREQ) as i32 % 2) == 0
                        } else {
                            false
                        };
                        set_actors_visible(&[&self._player, &self._monkey_ewa, &self._monkey_koa], visible_monkey);
                        set_actors_visible(&[&self._actor_aru, &self._actor_ewa, &self._actor_koa], !visible_monkey);
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
                        if let Some(player) = self._player.as_ref() {
                            game_scene_manager
                                .get_character_manager_mut()
                                .change_character_data(player, CHARACTER_DATA_NAME_ARU);
                        }

                        self.remove_non_player_characters();

                        if let Some(audio_instance) = self._audio_ufo_laboratory.as_ref() {
                            get_audio_manager().stop_audio_instance(audio_instance);
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

        if self._player.is_some() {
            self._position_timer += delta_time as f32;
            let position_y =
                self._position_y + (self._position_timer.sin() * FLOATING_OFFSET_AMPLITUDE + FLOATING_OFFSET_AMPLITUDE);
            for actor in [
                &self._player,
                &self._monkey_ewa,
                &self._monkey_koa,
                &self._actor_aru,
                &self._actor_ewa,
                &self._actor_koa,
            ]
            .into_iter()
            .flatten()
            {
                update_actor_position(&mut actor.borrow_mut(), position_y);
            }
        }
    }
}
