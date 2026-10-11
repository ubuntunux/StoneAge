use crate::game_module::actors::character::{ActionAnimationState, Character, MoveAnimationState};
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorSaveData, BehaviorState};
use crate::game_module::behavior::behavior_common::{
    IntimacyFollowResult, begin_attack, begin_eating, begin_idle, begin_interaction, begin_roaming, begin_wake_up,
    is_player_too_far_for_intimacy, should_roaming_go_idle, update_attack, update_chase, update_eating_should_idle,
    update_interaction_should_idle, update_intimacy_follow, update_wake_up_should_idle,
};
use crate::game_module::game_constants::{CIVILIAN_DEAD_TIME, NPC_TRACKING_RANGE};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::State;
use strum::IntoEnumIterator;

pub struct BehaviorCivilian<'a> {
    pub _behavior_data: BehaviorData<'a>,
    pub _attack_time: f32,
    pub _is_in_attack_range: bool,
}

impl<'a> Default for BehaviorCivilian<'a> {
    fn default() -> Self {
        Self {
            _behavior_data: BehaviorData::default(),
            _attack_time: 0.0,
            _is_in_attack_range: false,
        }
    }
}

impl<'a> BehaviorCivilian<'a> {
    /// Civilian is on player's side; targets non-player, non-civilian wild enemies (e.g. Roamers).
    fn is_enemy_in_range(&self, owner: &Character, target: Option<&Character>) -> bool {
        if let Some(target) = target.as_ref()
            && target.is_alive()
            && !target._is_player
            && !target.is_civilian()
        {
            return owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false);
        }
        false
    }
}

impl<'a> BehaviorBase<'a> for BehaviorCivilian<'a> {
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
                    State::Begin => {
                        owner.set_action_none();
                        begin_idle(&mut self._behavior_data, owner);
                    }
                    State::Update => {
                        if owner.get_attached_item_data_type().is_eatable() {
                            self.set_next_behavior(BehaviorState::Eating, true);
                        } else if owner.is_interacting() {
                            if !owner.is_move_state(MoveAnimationState::SitDownLoop) {
                                owner.set_move_idle();
                            }
                            if let Some(target_actor) = target {
                                owner.look_at(target_actor.get_position());
                            }
                        } else if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if owner.get_stats().is_hungry() {
                            self.set_next_behavior(BehaviorState::Hunger, true);
                        } else if self._behavior_data.is_end_behavior_time() {
                            self.set_next_behavior(BehaviorState::Roaming, true);
                        } else if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Follow, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Hunger => match state {
                    State::Begin => {
                        owner.set_action_hungry();
                        owner.set_sit_down();
                    }
                    State::Update => {
                        if !owner.get_stats().is_hungry() {
                            self.set_next_behavior(BehaviorState::Idle, true);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Eating => match state {
                    State::Begin => begin_eating(&mut self._behavior_data, owner, target),
                    State::Update => {
                        if update_eating_should_idle(is_first_update, owner) {
                            if owner.get_stats().is_hungry() {
                                self.set_next_behavior(BehaviorState::Hunger, true);
                            } else {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Roaming => match state {
                    State::Begin => begin_roaming(&mut self._behavior_data, owner, target),
                    State::Update => {
                        if owner.get_attached_item_data_type().is_eatable() {
                            self.set_next_behavior(BehaviorState::Eating, false);
                        } else if owner.is_interacting() {
                            self.set_next_behavior(BehaviorState::Interaction, true);
                        } else if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if owner.get_stats().is_hungry() {
                            self.set_next_behavior(BehaviorState::Hunger, true);
                        } else if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Follow, false);
                        } else if should_roaming_go_idle(&self._behavior_data, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Interaction => match state {
                    State::Begin => begin_interaction(&mut self._behavior_data, owner, target),
                    State::Update => {
                        if owner.get_attached_item_data_type().is_eatable() {
                            self.set_next_behavior(BehaviorState::Eating, true);
                        } else if update_interaction_should_idle(&self._behavior_data, owner, target) {
                            if owner.get_stats().is_hungry() {
                                self.set_next_behavior(BehaviorState::Hunger, true);
                            } else {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Chase => match state {
                    State::Begin => {}
                    State::Update => {
                        let next_state = update_chase(owner, target);
                        if next_state != BehaviorState::Chase {
                            self.set_next_behavior(next_state, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Attack => match state {
                    State::Begin => {
                        let next_state =
                            begin_attack(owner, target, &mut self._attack_time, &mut self._is_in_attack_range);
                        if next_state != BehaviorState::Attack {
                            self.set_next_behavior(next_state, false);
                        }
                    }
                    State::Update => {
                        let next_state = update_attack(
                            owner,
                            target,
                            &mut self._attack_time,
                            &mut self._is_in_attack_range,
                            delta_time,
                        );
                        if next_state != BehaviorState::Attack {
                            self.set_next_behavior(next_state, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Dead => match state {
                    State::Begin => {
                        owner.set_action_dead();
                        self._behavior_data.set_behavior_time(CIVILIAN_DEAD_TIME);
                    }
                    State::Update => {
                        if self._behavior_data.is_end_behavior_time() {
                            self.set_next_behavior(BehaviorState::WakeUp, true);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::WakeUp => match state {
                    State::Begin => {
                        begin_wake_up(owner);
                        // Civilian 전용: 부활 처리
                        owner._character_stats._is_alive = true;
                        let max_hp = owner._character_stats.get_max_hp();
                        owner._character_stats.set_hp(max_hp);
                    }
                    State::Update => {
                        if update_wake_up_should_idle(is_first_update, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Follow => match state {
                    State::Begin => {}
                    State::Update => match update_intimacy_follow(owner, target) {
                        IntimacyFollowResult::Arrived => self.set_next_behavior(BehaviorState::Roaming, false),
                        IntimacyFollowResult::Moving => {}
                        IntimacyFollowResult::NotFollowing => self.set_next_behavior(BehaviorState::Idle, false),
                    },
                    State::End => {}
                },
                BehaviorState::Dance => match state {
                    State::Begin => {
                        owner.set_action_dance();
                    }
                    State::Update => {
                        if !owner.is_action(ActionAnimationState::Dance) {
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
