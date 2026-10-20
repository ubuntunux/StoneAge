use crate::game_module::actors::character::Character;
use crate::game_module::actors::character::{ActionAnimationState, MoveAnimationState};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::{
    ACTOR_NAME_EWA, ACTOR_NAME_KOA, ACTOR_NAME_MONKEY_EWA, ACTOR_NAME_MONKEY_KOA, AUDIO_QUEST_COMPLETE, AUDIO_ROOSTER,
    BED_FOR_ARU, BED_FOR_EWA, BED_FOR_KOA, CHARACTER_INTERACTION_DISTANCE, DEFAULT_FADE_TIME, MATERIAL_UI_NONE,
    PROP_NAME_TABLE, SLEEP_TIMER, TARGET_HUNGER_THRESHOLD,
};
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_input::GameInput;
use crate::game_module::game_service_locator::{
    get_game_client_mut, get_game_controller_mut, get_game_scene_manager, get_game_scene_manager_mut,
    get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{set_actors_behavior_none, wake_up_npc_on_bed};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_core, get_scene_manager};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const TABLE_SEAT_INITIAL_DIST: f32 = 1.0;
const TABLE_SEAT_EATING_DIST: f32 = 2.0;
const TABLE_SEAT_ARRIVAL_DIST: f32 = 0.3;

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
    Begin,
    ResourceSettlement,
    Update,
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
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _audio_bgm: Option<RcRefCell<AudioInstance>>,
    _skip_wakeup: bool,
    _request_sleep: bool,
    _check_time_for_sleep: f32,
    _camera_target_position: Vector3<f32>,
    _camera_target_velocity: Vector3<f32>,
    _is_camera_target_initialized: bool,
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
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _audio_bgm: None,
            _skip_wakeup: false,
            _request_sleep: false,
            _check_time_for_sleep: 0.0,
            _camera_target_position: Vector3::default(),
            _camera_target_velocity: Vector3::default(),
            _is_camera_target_initialized: false,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
        })
    }

    pub fn snap_game_camera_to_player(&self) {
        if let Some(player) = self._player.as_ref() {
            let mut player_mut = player.borrow_mut();
            player_mut.update_transform();
            player_mut.update_render_object();

            let game_controller = get_game_controller_mut();
            game_controller.set_camera_fixed(false);
            game_controller.set_game_camera_auto_blend_mode(false);
            game_controller.apply_game_camera_transform(get_scene_manager().get_main_camera_mut(), &mut player_mut);
        }
    }
}
fn set_actor_table_position(
    scene_manager: &SceneManager,
    actor: &Option<RcRefCell<Character>>,
    table: &Option<RcRefCell<Prop>>,
    direction: &Vector3<f32>,
) {
    if let (Some(actor_ref), Some(table_ref)) = (actor.as_ref(), table.as_ref()) {
        let mut pos = table_ref.borrow().get_position() - math::safe_normalize(direction) * TABLE_SEAT_INITIAL_DIST;
        pos.y = scene_manager.get_height_bilinear(&pos, 0);
        let mut actor_mut = actor_ref.borrow_mut();
        actor_mut.set_position(&pos);
        actor_mut.look_at(table_ref.borrow().get_position());
        actor_mut.set_next_behavior(BehaviorState::Idle, true);
    }
}

fn sync_actor_dance_state(
    actor: &Option<RcRefCell<Character>>,
    player: &Option<RcRefCell<Character>>,
    should_dance: bool,
) {
    let Some(actor_ref) = actor.as_ref() else {
        return;
    };
    let is_dancing = actor_ref.borrow().is_action(ActionAnimationState::Dance);
    if should_dance && !is_dancing {
        if let Some(player_ref) = player.as_ref() {
            actor_ref.borrow_mut().look_at(player_ref.borrow().get_position());
        }
        actor_ref.borrow_mut().set_next_behavior(BehaviorState::Dance, true);
    } else if !should_dance && is_dancing {
        actor_ref.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
    }
}

fn is_in_table_interaction_range(actor: &Character, table: &Option<RcRefCell<Prop>>) -> bool {
    table.as_ref().is_some_and(|table_prop| {
        actor.check_in_range(
            table_prop.borrow().get_collision(),
            CHARACTER_INTERACTION_DISTANCE * 2.0,
            false,
        )
    })
}

fn update_actor_table_eating<'a>(
    actor_opt: &Option<RcRefCell<Character<'a>>>,
    table_opt: &Option<RcRefCell<Prop<'a>>>,
    seat_direction: &Vector3<f32>,
    has_eatable_item: bool,
) {
    let Some(actor) = actor_opt.as_ref() else {
        return;
    };

    if actor.borrow().is_action(ActionAnimationState::Dance) {
        return;
    }

    let is_hungry = actor.borrow().get_hunger() > TARGET_HUNGER_THRESHOLD;
    let should_eat = has_eatable_item || is_hungry;

    if !should_eat {
        return;
    }

    if has_eatable_item {
        if actor.borrow().is_action(ActionAnimationState::Hungry) {
            actor.borrow_mut().set_action_none();
        }

        let is_in_range = is_in_table_interaction_range(&actor.borrow(), table_opt);

        if !is_in_range {
            if let Some(table_prop) = table_opt.as_ref() {
                let scene_manager = get_scene_manager();
                let mut seat_pos =
                    table_prop.borrow().get_position() - math::safe_normalize(seat_direction) * TABLE_SEAT_EATING_DIST;
                seat_pos.y = scene_manager.get_height_bilinear(&seat_pos, 0);

                let (direction, dist) = math::make_normalize_xz_with_norm(&(seat_pos - actor.borrow().get_position()));
                if dist > TABLE_SEAT_ARRIVAL_DIST {
                    let mut actor_ref = actor.borrow_mut();
                    actor_ref.set_behavior_none();
                    actor_ref.set_move(&direction);
                } else {
                    if !actor.borrow().is_move_stop() {
                        actor.borrow_mut().set_move_idle();
                    }
                    actor.borrow_mut().look_at(table_prop.borrow().get_position());
                }
            }
        } else {
            if !actor.borrow().is_move_stop() {
                actor.borrow_mut().set_move_idle();
            }
            if let Some(table_prop) = table_opt.as_ref() {
                actor.borrow_mut().look_at(table_prop.borrow().get_position());
            }

            let is_ready_to_eat = {
                let actor_ref = actor.borrow();
                actor_ref.get_attached_item().is_none()
                    && !actor_ref.is_action(ActionAnimationState::Eating)
                    && actor_ref._animation_state._next_action_animation_state != Some(ActionAnimationState::Eating)
                    && actor_ref._behavior.get_behavior_data().get_next_behavior_state() != BehaviorState::Eating
            };

            if is_ready_to_eat && let Some(table_prop) = table_opt.as_ref() {
                let item_manager = get_game_scene_manager().get_item_manager_mut();
                if let Some(item_data_name) = item_manager.consume_item_on_table(table_prop) {
                    let mut actor_ref = actor.borrow_mut();
                    actor_ref.apply_food_item_stats(item_data_name.as_str());

                    if actor_ref.is_player() {
                        item_manager.attach_item(&mut actor_ref, item_data_name.as_str());
                        actor_ref.set_action_eating();
                    } else {
                        actor_ref.set_next_behavior(BehaviorState::Eating, true);
                    }
                }
            }
        }
    } else if is_hungry {
        if !actor.borrow().is_move_stop() {
            actor.borrow_mut().set_move_idle();
        }

        let mut actor_ref = actor.borrow_mut();
        if actor_ref.get_attached_item().is_none()
            && !actor_ref.is_action(ActionAnimationState::Eating)
            && !actor_ref.is_action(ActionAnimationState::Hungry)
        {
            actor_ref.set_action_hungry();
            actor_ref.set_sit_down();
        }
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
        !matches!(
            self._scenario_track._scenario_phase,
            ScenarioPhase::ResourceSettlement | ScenarioPhase::Update
        )
    }

    fn is_allow_player_control(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Update => {
                true
            }
            _ => false,
        }
    }

    fn is_available_sleep(&self) -> bool {
        matches!(self._scenario_track._scenario_phase, ScenarioPhase::Update)
    }

    fn is_use_table_storage_as_player_inventory(&self) -> bool {
        matches!(
            self._scenario_track._scenario_phase,
            ScenarioPhase::Update | ScenarioPhase::GoToSleep
        )
    }

    fn request_sleep(&mut self) {
        if self.is_available_sleep() {
            self._request_sleep = true;
        }
    }

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
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_TABLE).cloned();
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
                    self._check_time_for_sleep = get_game_scene_manager().get_time_of_day();
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => match state {
                    State::Begin => {
                        set_actors_behavior_none(&[&self._player, &self._actor_ewa, &self._actor_koa], true);
                        game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    }
                    State::Update => {
                        if game_ui_manager.is_done_manual_fade_out() {
                            let scene_manager = get_scene_manager();
                            set_actor_table_position(
                                scene_manager,
                                &self._player,
                                &self._prop_table,
                                &Vector3::new(0.0, 0.0, -1.0),
                            );
                            set_actor_table_position(
                                scene_manager,
                                &self._actor_ewa,
                                &self._prop_table,
                                &Vector3::new(1.0, 0.0, -1.0),
                            );
                            set_actor_table_position(
                                scene_manager,
                                &self._actor_koa,
                                &self._prop_table,
                                &Vector3::new(1.0, 0.0, 0.0),
                            );
                            self.snap_game_camera_to_player();
                            game_ui_manager.set_auto_fade_inout(true);
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::ResourceSettlement, None);
                            get_game_client_mut().set_next_game_phase(GamePhase::ResourceSettlement);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::ResourceSettlement => match state {
                    State::Begin => {}
                    State::Update => {
                        if !game_ui_manager.is_opened_daily_settlement() {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Update, None);
                        }
                    }
                    State::End => {}
                },
                ScenarioPhase::Update => match state {
                    State::Begin => {}
                    State::Update => {
                        let engine_core = get_engine_core();
                        let is_moving =
                            GameInput::is_moving(&engine_core._keyboard_input_data, &engine_core._joystick_input_data);
                        let is_any_input = GameInput::is_any_input(
                            &engine_core._keyboard_input_data,
                            &engine_core._joystick_input_data,
                            &engine_core._mouse_input_data,
                        );

                        if let Some(player) = &self._player {
                            let mut player_ref = player.borrow_mut();
                            if player_ref.is_move_state(MoveAnimationState::SitDownLoop) && is_moving {
                                player_ref.set_move_idle();
                            }
                        }

                        let player_is_acting = self._player.as_ref().is_some_and(|p| {
                            let p_ref = p.borrow();
                            p_ref.is_action(ActionAnimationState::Dance)
                                || p_ref.is_action(ActionAnimationState::Hungry)
                        });

                        if player_is_acting
                            && is_any_input
                            && let Some(player) = &self._player
                        {
                            let mut player_ref = player.borrow_mut();
                            player_ref.set_action_none();
                            if player_ref.is_move_state(MoveAnimationState::SitDownLoop) {
                                player_ref.set_move_idle();
                            }
                        }

                        let player_is_dancing =
                            self._player.as_ref().is_some_and(|p| p.borrow().is_action(ActionAnimationState::Dance));

                        sync_actor_dance_state(&self._actor_ewa, &self._player, player_is_dancing);
                        sync_actor_dance_state(&self._actor_koa, &self._player, player_is_dancing);

                        if !player_is_dancing {
                            let has_food_for_ewa = self._prop_table.as_ref().is_some_and(|table| {
                                get_game_scene_manager().get_item_manager().has_item_on_table(table)
                            });
                            update_actor_table_eating(
                                &self._actor_ewa,
                                &self._prop_table,
                                &Vector3::new(0.0, 0.0, 1.0),
                                has_food_for_ewa,
                            );

                            let has_food_for_koa = self._prop_table.as_ref().is_some_and(|table| {
                                get_game_scene_manager().get_item_manager().has_item_on_table(table)
                            });
                            update_actor_table_eating(
                                &self._actor_koa,
                                &self._prop_table,
                                &Vector3::new(-1.0, 0.0, 0.0),
                                has_food_for_koa,
                            );
                        }

                        let prev_time_of_day = self._check_time_for_sleep;
                        self._check_time_for_sleep = get_game_scene_manager().get_time_of_day();

                        if self._request_sleep || self._check_time_for_sleep < prev_time_of_day {
                            self._request_sleep = false;
                            if let Some(audio_bgm) = &self._audio_bgm {
                                get_audio_manager_mut().stop_audio_instance(audio_bgm);
                            }
                            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                            let item_manager = get_game_scene_manager().get_item_manager_mut();
                            for actor in [&self._player, &self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                                let mut actor_mut = actor.borrow_mut();
                                actor_mut.set_action_none();
                                item_manager.detach_item(&mut actor_mut);
                            }
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToSleep, None);
                        }
                    }
                    State::End => {}
                },
                ScenarioPhase::GoToSleep => {
                    if state == State::Update {
                        go_to_sleep(&self._player, &self._prop_bed_for_aru);
                        go_to_sleep(&self._actor_ewa, &self._prop_bed_for_ewa);
                        go_to_sleep(&self._actor_koa, &self._prop_bed_for_koa);

                        let aru_is_sleeping = self
                            ._player
                            .as_ref()
                            .is_some_and(|actor| actor.borrow().is_action(ActionAnimationState::Sleep));
                        if aru_is_sleeping {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleep, None);
                        }
                    }
                }
                ScenarioPhase::Sleep => match state {
                    State::Begin => {
                        self._sleep_timer = 0.0;
                        if let Some(actor) = self._player.as_ref() {
                            actor.borrow_mut().set_action_sleep_no_snoring();
                        }
                        if let Some(actor) = self._actor_ewa.as_ref() {
                            actor.borrow_mut().set_action_sleep_no_snoring();
                        }
                        if let Some(actor) = self._actor_koa.as_ref() {
                            actor.borrow_mut().set_action_sleep_no_snoring();
                        }
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
                                    game_ui_manager.set_auto_fade_inout(true);
                                    game_scene_manager.set_next_time_of_day();
                                }
                            }
                        } else if !self._skip_wakeup && game_ui_manager.is_done_game_image_progress() {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);
                            if let Some(actor) = &self._player {
                                actor.borrow_mut().set_action_wake_up();
                            }
                            wake_up_npc_on_bed(&self._actor_ewa, None);
                            wake_up_npc_on_bed(&self._actor_koa, None);
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
