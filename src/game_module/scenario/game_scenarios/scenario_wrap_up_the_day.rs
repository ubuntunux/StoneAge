use nalgebra::Vector3;
use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    ACTOR_NAME_EWA, ACTOR_NAME_KOA, ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA, AUDIO_QUEST_COMPLETE, AUDIO_ROOSTER,
    BED_FOR_ARU, BED_FOR_EWA, BED_FOR_KOA, DEFAULT_FADE_TIME, MATERIAL_UI_NONE, SLEEP_TIMER,
};
use crate::game_module::game_service_locator::{
    get_game_client_mut, get_game_scene_manager, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::wake_up_npc_on_bed;
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
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
    _sleep_timer: f32,
    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _skip_wakeup: bool,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioWrapUpTheDay<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWrapUpTheDay<'a>> {
        newRcRefCell(ScenarioWrapUpTheDay {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _sleep_timer: 0.0,
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _skip_wakeup: false,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::GoToSleep),
        })
    }
}

fn go_to_sleep(actor: &Option<RcRefCell<Character>>, bed: &Option<RcRefCell<Prop>>, delta_time: f32) {
    if let (Some(actor), Some(bed_ref)) = (actor.as_ref(), bed.as_ref()) {
        if actor.borrow_mut().move_to_target(&bed_ref.borrow().get_position(), 0.0, delta_time) {
            if !actor.borrow().is_move_stop() {
                actor.borrow_mut().set_move_idle();
            }

            if !actor.borrow().is_action(ActionAnimationState::LayingDown)
                && !actor.borrow().is_action(ActionAnimationState::Sleep)
            {
                actor.borrow_mut().set_move_direction(&Vector3::new(0.0, 0.0, -1.0), true);
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

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &str) {
        self._scenario_track.set_phase_from_str(scenario_phase, ScenarioPhase::None);
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

    fn is_available_sleep(&self) -> bool {
        false
    }

    fn is_use_table_storage_as_player_inventory(&self) -> bool {
        false
    }

    fn request_sleep(&mut self) {}

    fn set_sleep_phase(&mut self) {
        self._scenario_track.set_scenario_phase(ScenarioPhase::Sleep, None);
        self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
    }

    fn set_skip_wakeup(&mut self, skip_wakeup: bool) {
        self._skip_wakeup = skip_wakeup;
    }

    fn is_sleeping(&self) -> bool {
        matches!(
            self._scenario_track._scenario_phase,
            ScenarioPhase::GoToSleep | ScenarioPhase::Sleep | ScenarioPhase::End
        )
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        get_game_client_mut().set_is_available_interaction(true);
        let game_ui_manager = get_game_ui_manager_mut();
        game_ui_manager.get_item_bar_widget_mut().set_show_table_storage_items(false);
        if game_ui_manager.is_opened_daily_settlement() {
            game_ui_manager.close_daily_settlement();
        }
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {}

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager
            .get_actor_by_name(ACTOR_NAME_MONKEY_EWA)
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name(ACTOR_NAME_EWA).cloned());
        self._actor_koa = game_scene_manager
            .get_actor_by_name(ACTOR_NAME_MONKEY_KOA)
            .cloned()
            .or_else(|| game_scene_manager.get_actor_by_name(ACTOR_NAME_KOA).cloned());
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_EWA).cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_KOA).cloned();
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

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToSleep, None);
                }
                ScenarioPhase::GoToSleep => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                        let item_manager = get_game_scene_manager().get_item_manager_mut();
                        for actor in [&self._player, &self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                            let mut actor_mut = actor.borrow_mut();
                            actor_mut.set_action_none();
                            item_manager.detach_item(&mut actor_mut);
                        }
                    }
                    State::Update => {
                        go_to_sleep(&self._player, &self._prop_bed_for_aru, delta_time as f32);
                        go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa, delta_time as f32);
                        go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa, delta_time as f32);

                        let aru_is_sleeping = self
                            ._player
                            .as_ref()
                            .is_some_and(|actor| actor.borrow().is_action(ActionAnimationState::Sleep));
                        if aru_is_sleeping {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
                        }
                    }
                    State::End => {}
                },
                ScenarioPhase::Sleep => match state {
                    State::Begin => {
                        self._sleep_timer = 0.0;
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() && self._sleep_timer < SLEEP_TIMER {
                            if self._sleep_timer == 0.0 {
                                for (actor_opt, bed_opt) in [
                                    (&self._actor_ewa, &self._prop_bed_for_ewa),
                                    (&self._actor_koa, &self._prop_bed_for_koa),
                                ] {
                                    if let Some(actor) = actor_opt {
                                        if let Some(bed) = bed_opt {
                                            actor.borrow_mut().set_position(bed.borrow().get_position());
                                        }
                                        actor.borrow_mut().set_action_sleep();
                                    }
                                }
                            }

                            self._sleep_timer += delta_time as f32;
                            if SLEEP_TIMER <= self._sleep_timer {
                                if self._skip_wakeup {
                                    self._skip_wakeup = false;
                                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                                } else {
                                    get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);
                                    game_ui_manager.set_auto_fade_inout(true);
                                    game_scene_manager.set_next_time_of_day();
                                }
                            }
                        } else if game_ui_manager.is_done_game_image_progress() {
                            if let Some(actor) = &self._player {
                                actor.borrow_mut().set_action_wake_up();
                            }
                            wake_up_npc_on_bed(&self._actor_ewa, None);
                            wake_up_npc_on_bed(&self._actor_koa, None);
                            if let Some(scenario) =
                                game_scene_manager.get_game_scenario(ScenarioType::ScenarioIntro_Intro)
                            {
                                scenario.borrow_mut().complete_sub_quest_sleep();
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
