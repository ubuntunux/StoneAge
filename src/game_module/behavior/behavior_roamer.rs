use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorSaveData, BehaviorState};
use crate::game_module::behavior::behavior_common::{
    IntimacyFollowResult, begin_eating, begin_idle, begin_interaction, begin_roaming, begin_wake_up,
    is_player_too_far_for_intimacy, should_roaming_go_idle, update_eating_should_idle, update_interaction_should_idle,
    update_intimacy_follow, update_wake_up_should_idle,
};
use crate::game_module::game_constants::{
    NPC_ATTACK_RANGE, NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN, NPC_AVAILABLE_MOVING_ATTACK, NPC_TRACKING_RANGE,
};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;
use rust_engine_3d::utilities::system::State;
use strum::IntoEnumIterator;

pub struct BehaviorRoamer<'a> {
    pub _behavior_data: BehaviorData<'a>,
    pub _attack_time: f32,
}

impl<'a> Default for BehaviorRoamer<'a> {
    fn default() -> Self {
        Self {
            _behavior_data: BehaviorData::default(),
            _attack_time: 0.0,
        }
    }
}

impl<'a> BehaviorRoamer<'a> {
    /// Player is excluded from enemy detection for tamed or intimacy-following characters.
    fn is_enemy_in_range(&self, owner: &Character, target: Option<&Character>) -> bool {
        if let Some(target) = target.as_ref()
            && target.is_alive()
            && !(target._is_player && (owner.is_tamed() || owner.is_following_intimacy()))
        {
            return owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false);
        }
        false
    }
}

impl<'a> BehaviorBase<'a> for BehaviorRoamer<'a> {
    fn initialize_behavior(&mut self, position: &Vector3<f32>) {
        self._behavior_data.initialize_behavior_data(position);
        self.set_next_behavior(BehaviorState::Idle, true);
    }

    fn get_behavior_data(&self) -> &BehaviorData<'a> {
        &self._behavior_data
    }

    fn get_behavior_data_mut(&mut self) -> &mut BehaviorData<'a> {
        &mut self._behavior_data
    }

    fn update_behavior(&mut self, owner: &mut Character<'a>, target: Option<&Character<'a>>, delta_time: f32) {
        let prev_behavior_state = self._behavior_data.get_behavior_state();
        let next_behavior_state = self._behavior_data.get_next_behavior_state();
        let is_force = self._behavior_data.is_force_behavior_state_changed_and_reset();

        for state in State::iter() {
            if !is_force && prev_behavior_state == next_behavior_state && (state == State::End || state == State::Begin)
            {
                continue;
            }

            let update_behavior_state: BehaviorState = match state {
                State::End => prev_behavior_state,
                State::Begin => {
                    self._behavior_data.set_behavior_state(next_behavior_state);
                    next_behavior_state
                }
                State::Update => next_behavior_state,
            };

            let is_first_update = prev_behavior_state != next_behavior_state && state == State::Update;

            match update_behavior_state {
                BehaviorState::Idle => match state {
                    State::Begin => begin_idle(&mut self._behavior_data, owner),
                    State::Update => {
                        if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if self._behavior_data.is_end_behavior_time() {
                            self.set_next_behavior(BehaviorState::Roaming, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Eating => match state {
                    State::Begin => begin_eating(&mut self._behavior_data, owner),
                    State::Update => {
                        if update_eating_should_idle(is_first_update, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Interaction => match state {
                    State::Begin => begin_interaction(&mut self._behavior_data, owner),
                    State::Update => {
                        if update_interaction_should_idle(&self._behavior_data, owner, target) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Roaming => match state {
                    State::Begin => begin_roaming(&mut self._behavior_data, owner, target),
                    State::Update => {
                        if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if should_roaming_go_idle(&self._behavior_data, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Chase => match state {
                    State::Begin => {}
                    State::Update => {
                        let mut do_idle = true;
                        if let Some(target_ref) = target
                            && target_ref.is_alive()
                        {
                            if owner.is_following_intimacy() && target_ref._is_player {
                                match update_intimacy_follow(owner, target) {
                                    IntimacyFollowResult::Arrived => {
                                        self.set_next_behavior(BehaviorState::Roaming, false);
                                    }
                                    IntimacyFollowResult::Moving | IntimacyFollowResult::NotFollowing => {}
                                }
                                do_idle = false;
                            } else if owner.check_in_range(target_ref.get_collision(), NPC_TRACKING_RANGE, false) {
                                if owner.check_in_range(target_ref.get_collision(), NPC_ATTACK_RANGE, false) {
                                    self.set_next_behavior(BehaviorState::Attack, false);
                                } else {
                                    let to_target = target_ref.get_position() - owner.get_position();
                                    owner.set_move(&to_target);
                                    owner.set_run(true);
                                }
                                do_idle = false;
                            }
                        }
                        if do_idle {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Attack => match state {
                    State::Begin => {
                        let to_dir =
                            math::safe_normalize(&(target.as_ref().unwrap().get_position() - owner.get_position()));
                        owner.set_move_direction(&to_dir, false);
                        if !NPC_AVAILABLE_MOVING_ATTACK {
                            owner.set_move_idle();
                        }

                        if owner.is_available_attack() {
                            owner.set_action_attack();
                        }

                        self._attack_time = lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>());
                        get_audio_manager_mut().play_audio_resource_data(
                            &owner._character_data.borrow()._audio_data._audio_growl,
                            AudioLoop::ONCE,
                            None,
                        );
                    }
                    State::Update => {
                        if let Some(target_ref) = target
                            && target_ref.is_alive()
                            && 0.0 < self._attack_time
                        {
                            if owner.is_attack_animation() {
                                if !owner.is_available_move()
                                    || (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation())
                                {
                                    owner.set_move_idle();
                                }
                            } else {
                                owner.set_move_idle();
                                self._attack_time -= delta_time;
                            }
                        } else if let Some(target_ref) = target
                            && target_ref.is_alive()
                            && owner.check_in_range(target_ref.get_collision(), NPC_TRACKING_RANGE, false)
                        {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Dead => {
                    if state == State::Begin {
                        owner.set_action_dead();
                    }
                }
                BehaviorState::WakeUp => match state {
                    State::Begin => begin_wake_up(owner),
                    State::Update => {
                        if update_wake_up_should_idle(is_first_update, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                _ => {}
            }

            if state == State::Update {
                self._behavior_data.update_behavior_time(delta_time);
            }
        }
    }

    fn get_behavior_save_data(&self) -> BehaviorSaveData {
        self._behavior_data.get_behavior_save_data()
    }

    fn load_behavior_save_data(&mut self, save_data: &BehaviorSaveData) {
        self._behavior_data.load_behavior_save_data(save_data);
    }
}
