use crate::game_module::actors::character::character::Character;
use crate::game_module::actors::character::data::{ActionAnimationState, ActionEvent, MoveAnimationState};
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::{
    AUDIO_ATTACK, AUDIO_EATING, AUDIO_FOOTSTEP, AUDIO_JUMP, AUDIO_QUEST_COMPLETE, AUDIO_ROLL, AUDIO_SNORING,
    BED_FOR_ARU, ITEM_COCONUT, PICKUP_EVENT_TIME,
};
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_game_client_mut, get_game_scene_manager, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut};
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::render_object::AnimationLayer;
use rust_engine_3d::utilities::system::{State, ptr_as_mut, ptr_as_ref};
use strum::IntoEnumIterator;

const WALK_FOOTSTEP_EVENT_TIME_1: f32 = 0.2;
const WALK_FOOTSTEP_EVENT_TIME_2: f32 = 0.9;
const RUN_FOOTSTEP_EVENT_TIME_1: f32 = 0.1;
const RUN_FOOTSTEP_EVENT_TIME_2: f32 = 0.5;
const FOOTSTEP_VOLUME: f32 = 0.5;
const ROLL_SOUND_EVENT_TIME: f32 = 0.2;
const DEAD_ANIMATION_BLEND_TIME: f32 = 0.1;

impl<'a> Character<'a> {
    pub fn post_process_restore_animation(&mut self) {
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());

        // Restore move animation mesh (None defaults to Idle on restore)
        let move_state = match self._animation_state._move_animation_state {
            MoveAnimationState::None => MoveAnimationState::Idle,
            other => other,
        };
        let next_move_speed = self._animation_state._next_move_animation_speed;

        if let Some((mesh, move_speed, move_loop)) = animation_data.get_move_animation_spec(move_state, next_move_speed)
        {
            let animation_info = AnimationPlayArgs {
                _animation_speed: move_speed,
                _animation_loop: move_loop,
                _reset_animation_time: false,
                _force_animation_setting: true,
                ..Default::default()
            };
            render_object.set_animation(mesh, &animation_info, AnimationLayer::BaseLayer);
        } else {
            render_object.set_animation_none(AnimationLayer::BaseLayer);
        }

        // Restore action animation mesh
        let action_state = self._animation_state._action_animation_state;
        let next_action_speed = self._animation_state._next_action_animation_speed;

        if let Some((mesh, action_speed, action_loop, _fade_out)) =
            animation_data.get_action_animation_spec(action_state, next_action_speed)
        {
            let animation_info = AnimationPlayArgs {
                _animation_speed: action_speed,
                _animation_loop: action_loop,
                _reset_animation_time: false,
                _force_animation_setting: true,
                ..Default::default()
            };
            render_object.set_animation(mesh, &animation_info, AnimationLayer::ActionLayer);
        } else {
            render_object.set_animation_none(AnimationLayer::ActionLayer);
        }

        self.update_animation_layers();
    }

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._animation_state._move_animation_state
    }

    pub fn is_move_stop(&self) -> bool {
        matches!(
            self._animation_state._move_animation_state,
            MoveAnimationState::None | MoveAnimationState::Idle | MoveAnimationState::SitDownLoop
        )
    }

    pub fn is_idle_action(&self) -> bool {
        self.is_action(ActionAnimationState::None) || self.is_action(ActionAnimationState::Hungry)
    }

    pub fn is_additive_animation_for_action(&self) -> bool {
        matches!(
            self._animation_state._action_animation_state,
            ActionAnimationState::Attack
                | ActionAnimationState::PowerAttack
                | ActionAnimationState::Hit
                | ActionAnimationState::Eating
                | ActionAnimationState::Hungry
                | ActionAnimationState::Pickup
        ) && matches!(
            self._animation_state._move_animation_state,
            MoveAnimationState::Jump
                | MoveAnimationState::Run
                | MoveAnimationState::RunningJump
                | MoveAnimationState::SitDownLoop
                | MoveAnimationState::Walk
        )
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._animation_state._action_animation_state
    }

    pub fn is_attack_animation(&self) -> bool {
        matches!(
            self._animation_state._action_animation_state,
            ActionAnimationState::Attack | ActionAnimationState::PowerAttack | ActionAnimationState::Kick
        )
    }

    pub fn get_animation_play_info(&self, layer: AnimationLayer) -> &AnimationPlayInfo {
        ptr_as_ref(self._render_object.as_ptr()).get_animation_play_info(layer)
    }

    pub fn set_action_none(&mut self) {
        self.set_next_action_animation(ActionAnimationState::None, 1.0);
    }

    pub fn set_action_dance(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Dance, 1.0);
    }

    pub fn set_action_passed_out(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::PassedOut, 1.0);
    }

    pub fn set_action_wake_up(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::WakeUp, 1.0);
    }

    pub fn set_action_laying_down(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::LayingDown, 2.0);
    }

    pub fn set_action_sleep(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Sleep, 1.0);
    }

    pub fn set_action_sleep_no_snoring(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::SleepNoSnoring, 1.0);
    }

    pub fn set_action_hungry(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Hungry, 1.0);
    }

    pub fn set_action_eating(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Eating, 1.0);
    }

    pub fn set_action_hit(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Hit, 1.0);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Dead, 1.0);
    }

    pub fn set_next_move_animation(&mut self, move_animation_state: MoveAnimationState, animation_speed: f32) {
        self._animation_state._next_move_animation_state = move_animation_state;
        self._animation_state._next_move_animation_speed = animation_speed;
    }

    pub fn set_next_action_animation(&mut self, action_animation_state: ActionAnimationState, animation_speed: f32) {
        self._animation_state._next_action_animation_state = Some(action_animation_state);
        self._animation_state._next_action_animation_speed = animation_speed;
    }

    pub fn stop_animations(&mut self, apply_immediately: bool) {
        self.set_next_action_animation(ActionAnimationState::None, 1.0);
        self.set_next_move_animation(MoveAnimationState::None, 1.0);
        self.set_run(false);
        self.set_move_speed(0.0);
        if apply_immediately {
            self.update_action_keyframe_event(0.0);
            self.update_move_keyframe_event();
        }
    }

    pub fn update_animation_layers(&self) {
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        render_object.clear_animation_layers(AnimationLayer::ActionLayer);

        if self.is_additive_animation_for_action() {
            render_object.set_animation_layers(
                self._character_data.borrow()._animation_data._upper_animation_layer.as_ptr(),
                AnimationLayer::ActionLayer,
            );
        }
    }

    pub fn update_move_keyframe_event(&mut self) {
        let current_move_animation_state = self._animation_state._move_animation_state;
        let next_move_animation_state = self._animation_state._next_move_animation_state;
        let next_move_animation_speed = self._animation_state._next_move_animation_speed;
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());

        for state in State::iter() {
            if current_move_animation_state == next_move_animation_state
                && (state == State::End || state == State::Begin)
            {
                continue;
            }

            let update_move_animation_state: MoveAnimationState = match state {
                State::End => current_move_animation_state,
                State::Begin => {
                    self._animation_state._move_animation_state = next_move_animation_state;
                    next_move_animation_state
                }
                State::Update => next_move_animation_state,
            };

            if state == State::Begin {
                if let Some((mesh, speed, loop_anim)) =
                    animation_data.get_move_animation_spec(update_move_animation_state, next_move_animation_speed)
                {
                    let animation_info = AnimationPlayArgs {
                        _animation_speed: speed,
                        _animation_loop: loop_anim,
                        ..Default::default()
                    };
                    render_object.set_animation(mesh, &animation_info, AnimationLayer::BaseLayer);
                } else {
                    render_object.set_animation_none(AnimationLayer::BaseLayer);
                }
            }

            match update_move_animation_state {
                MoveAnimationState::Walk => {
                    if state == State::Update {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player
                            && (animation_play_info.check_animation_event_time(WALK_FOOTSTEP_EVENT_TIME_1)
                                || animation_play_info.check_animation_event_time(WALK_FOOTSTEP_EVENT_TIME_2))
                        {
                            get_audio_manager_mut().play_audio_bank(
                                AUDIO_FOOTSTEP,
                                AudioLoop::ONCE,
                                Some(FOOTSTEP_VOLUME),
                            );
                        }
                    }
                }
                MoveAnimationState::Run => {
                    if state == State::Update {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player
                            && (animation_play_info.check_animation_event_time(RUN_FOOTSTEP_EVENT_TIME_1)
                                || animation_play_info.check_animation_event_time(RUN_FOOTSTEP_EVENT_TIME_2))
                        {
                            get_audio_manager_mut().play_audio_bank(
                                AUDIO_FOOTSTEP,
                                AudioLoop::ONCE,
                                Some(FOOTSTEP_VOLUME),
                            );
                        }
                    }
                }
                MoveAnimationState::Jump | MoveAnimationState::RunningJump => {
                    if state == State::Begin {
                        get_audio_manager_mut().play_audio_bank(AUDIO_JUMP, AudioLoop::ONCE, None);
                    }
                }
                MoveAnimationState::Roll => match state {
                    State::Begin => {
                        self.set_invincibility(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player && animation_play_info.check_animation_event_time(ROLL_SOUND_EVENT_TIME) {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ROLL, AudioLoop::ONCE, None);
                        } else if animation_play_info._is_animation_end {
                            self.set_move_idle();
                        }
                    }
                    State::End => {
                        self._controller.set_roll_delay();
                        self.set_invincibility(false);
                    }
                },
                _ => {}
            }
        }
        self.update_animation_layers();
    }

    pub fn update_action_keyframe_event(&mut self, delta_time: f32) {
        self._animation_state.set_action_event(ActionEvent::None);

        let current_action_animation_state = self._animation_state._action_animation_state;
        let next_action_animation_state = self._animation_state._next_action_animation_state;
        self._animation_state._next_action_animation_state = None;
        let next_action_animation_speed = self._animation_state._next_action_animation_speed;
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let item_manager = get_game_scene_manager().get_item_manager_mut();

        for state in State::iter() {
            if next_action_animation_state.is_none() && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_action_animation_state: ActionAnimationState = match state {
                State::End => current_action_animation_state,
                State::Begin => {
                    if let Some(next_action) = next_action_animation_state {
                        self._animation_state._action_animation_state = next_action;
                        next_action
                    } else {
                        current_action_animation_state
                    }
                }
                State::Update => self._animation_state._action_animation_state,
            };

            if state == State::Begin {
                if let Some((mesh, speed, loop_anim, fade_out_time)) =
                    animation_data.get_action_animation_spec(update_action_animation_state, next_action_animation_speed)
                {
                    let is_down_state = matches!(
                        update_action_animation_state,
                        ActionAnimationState::Dead | ActionAnimationState::PassedOut
                    );
                    let mut animation_info = AnimationPlayArgs {
                        _animation_loop: loop_anim,
                        _force_animation_setting: !is_down_state,
                        _animation_fade_out_time: fade_out_time,
                        _animation_speed: speed,
                        ..Default::default()
                    };
                    if is_down_state {
                        animation_info._animation_blend_time = DEAD_ANIMATION_BLEND_TIME;
                    }
                    render_object.set_animation(mesh, &animation_info, AnimationLayer::ActionLayer);
                } else {
                    render_object.set_animation_none(AnimationLayer::ActionLayer);
                }
            }

            match update_action_animation_state {
                ActionAnimationState::None => {}
                ActionAnimationState::Attack => match state {
                    State::Begin => {
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(character_data._stat_data._attack_event_time)
                        {
                            self._animation_state.set_action_event(ActionEvent::Attack);
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::Dance | ActionAnimationState::WakeUp => match state {
                    State::Begin => {
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::Dead | ActionAnimationState::PassedOut => match state {
                    State::Begin => {
                        if update_action_animation_state == ActionAnimationState::PassedOut {
                            get_audio_manager_mut().play_audio_resource_data(
                                &self._character_data.borrow()._audio_data._audio_dead,
                                AudioLoop::ONCE,
                                None,
                            );
                        }
                        self.set_weapon_visible(false);
                        self.set_invincibility(true);
                        self.set_is_down_loop(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_invincibility(false);
                            self.set_is_down_loop(true);

                            if self._is_player {
                                let game_scene_manager = get_game_scene_manager_mut();
                                if !game_scene_manager.is_teleport_mode() {
                                    game_scene_manager
                                        .set_teleport_spawn_point(Stages::Home.get_stage_data_name(), BED_FOR_ARU);
                                    let next_phase = if update_action_animation_state == ActionAnimationState::Dead {
                                        GamePhase::Respawn
                                    } else {
                                        GamePhase::PassOut
                                    };
                                    get_game_client_mut().set_next_game_phase(next_phase);
                                }
                            }
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                        self.set_invincibility(false);
                        self.set_is_down_loop(false);
                    }
                },
                ActionAnimationState::Hit | ActionAnimationState::Hungry => {
                    if state == State::Update {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                }
                ActionAnimationState::Kick => {
                    if state == State::Update {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(character_data._stat_data._kick_event_time) {
                            self._animation_state.set_action_event(ActionEvent::Kick);
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        } else if self.is_on_ground() {
                            self.set_move_idle();
                        }
                    }
                }
                ActionAnimationState::LayingDown => match state {
                    State::Begin => {
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_sleep();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::Pickup => match state {
                    State::Begin => {
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(PICKUP_EVENT_TIME) {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                            self._animation_state.set_action_event(ActionEvent::Pickup);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::PowerAttack => match state {
                    State::Begin => {
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info
                            .check_animation_event_time(character_data._stat_data._power_attack_event_time)
                        {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                            self._animation_state.set_action_event(ActionEvent::PowerAttack);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::Sleep | ActionAnimationState::SleepNoSnoring => match state {
                    State::Begin => {
                        if self._is_player && update_action_animation_state == ActionAnimationState::Sleep {
                            if let Some(audio_instance) = self._audio_snoring.as_ref() {
                                get_audio_manager().stop_audio_instance(audio_instance)
                            }
                            self._audio_snoring =
                                get_audio_manager_mut().play_audio_bank(AUDIO_SNORING, AudioLoop::LOOP, Some(1.0));
                        }
                        self.set_weapon_visible(false);
                    }
                    State::End => {
                        if self._is_player
                            && let Some(audio_instance) = self._audio_snoring.as_ref()
                        {
                            get_audio_manager().stop_audio_instance(audio_instance)
                        }
                        self.set_weapon_visible(true);
                    }
                    _ => {}
                },
                ActionAnimationState::Eating => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_EATING, AudioLoop::ONCE, None);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        if !self._is_player
                            && let Some(attached_item) = self.get_attached_item().clone()
                        {
                            let item_data_name = attached_item.borrow()._item_data_name.clone();
                            self.apply_food_item_stats(item_data_name.as_str());
                            item_manager.detach_item(self);
                        }
                    }
                },
                ActionAnimationState::FishingBegin => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let (anim_play_time, animation_length, is_anim_end) = {
                            let anim_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                            (
                                anim_info._animation_play_time,
                                anim_info.get_animation_length(),
                                anim_info._is_animation_end,
                            )
                        };

                        if self._fishing_state._is_fishing_button_held {
                            self.update_fishing(delta_time);
                            render_object.get_animation_play_info_mut(AnimationLayer::ActionLayer)._animation_speed =
                                if 0.0 < animation_length {
                                    (1.0 - (anim_play_time / (animation_length * 0.5)).min(1.0)).powf(2.0)
                                } else {
                                    1.0
                                };
                        } else {
                            render_object.get_animation_play_info_mut(AnimationLayer::ActionLayer)._animation_speed =
                                1.0;
                        }

                        if is_anim_end {
                            if self.is_fishing_spot() {
                                self.set_next_action_animation(ActionAnimationState::FishingLoop, 1.0);
                            } else {
                                self.set_action_none();
                            }
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::FishingLoop => match state {
                    State::Begin => {
                        self.set_weapon_visible(true);
                        self.start_fishing_minigame();
                    }
                    State::Update => {
                        self.update_fishing_minigame(delta_time);
                    }
                    _ => {}
                },
                ActionAnimationState::FishingEnd => match state {
                    State::Begin => {
                        get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        self.set_weapon_visible(true);

                        if self._is_player && self._fishing_state._minigame_success == Some(true) {
                            let is_perfect = self._fishing_state._is_perfect_fishing;
                            item_manager.pick_item(ITEM_COCONUT, 1);
                            get_game_ui_manager_mut().show_fishing_popup(ITEM_COCONUT, is_perfect);
                            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                        }
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
            }
        }
        self.update_animation_layers();
    }
}
