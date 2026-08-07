use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorSaveData, BehaviorState};
use crate::game_module::behavior::behavior_common::{
    IntimacyFollowResult, begin_idle, begin_roaming, begin_wake_up, is_player_too_far_for_intimacy,
    should_roaming_go_idle, update_intimacy_follow, update_wake_up_should_idle,
};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::State;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct BehaviorDefault<'a> {
    pub _behavior_data: BehaviorData<'a>,
}

impl<'a> BehaviorBase<'a> for BehaviorDefault<'a> {
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
                        if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Follow, false);
                        } else if self._behavior_data.is_end_behavior_time() {
                            self.set_next_behavior(BehaviorState::Roaming, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Roaming => match state {
                    State::Begin => begin_roaming(&mut self._behavior_data, owner, target),
                    State::Update => {
                        if is_player_too_far_for_intimacy(owner, target) {
                            self.set_next_behavior(BehaviorState::Follow, false);
                        } else if should_roaming_go_idle(&self._behavior_data, owner) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Dead => match state {
                    State::Begin => {
                        owner.set_action_dead();
                        owner.set_move_idle();
                    }
                    State::Update => {}
                    State::End => {}
                },
                BehaviorState::WakeUp => match state {
                    State::Begin => begin_wake_up(owner),
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
