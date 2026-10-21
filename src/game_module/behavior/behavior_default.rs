use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorState};
use crate::game_module::behavior::behavior_common::{
    begin_idle, begin_roaming, is_player_too_far_for_intimacy, should_roaming_go_idle, step_dance, step_eating,
    step_follow, step_hunger, step_interaction, step_wake_up, update_hunger_action, update_idle_interacting,
};
use rust_engine_3d::utilities::system::State;

#[derive(Default)]
pub struct BehaviorDefault<'a> {
    pub _behavior_data: BehaviorData<'a>,
}

impl<'a> BehaviorBase<'a> for BehaviorDefault<'a> {
    fn get_behavior_data(&self) -> &BehaviorData<'a> {
        &self._behavior_data
    }

    fn get_behavior_data_mut(&mut self) -> &mut BehaviorData<'a> {
        &mut self._behavior_data
    }

    fn step_behavior_state(
        &mut self,
        state: State,
        behavior_state: BehaviorState,
        is_first_update: bool,
        owner: &mut Character<'a>,
        target: Option<&Character<'a>>,
        _delta_time: f32,
    ) {
        match behavior_state {
            BehaviorState::Idle => match state {
                State::Begin => begin_idle(&mut self._behavior_data, owner),
                State::Update => {
                    update_hunger_action(owner);
                    if owner.get_attached_item_data_type().is_eatable() {
                        self.set_next_behavior(BehaviorState::Eating, true);
                    } else if owner.is_interacting() {
                        update_idle_interacting(owner, target);
                    } else if is_player_too_far_for_intimacy(owner, target) {
                        self.set_next_behavior(BehaviorState::Follow, false);
                    } else if self._behavior_data.is_end_behavior_time() {
                        self.set_next_behavior(BehaviorState::Roaming, false);
                    }
                }
                State::End => {}
            },
            BehaviorState::Hunger => step_hunger(&mut self._behavior_data, state, owner),
            BehaviorState::Eating => step_eating(&mut self._behavior_data, state, is_first_update, owner, target),
            BehaviorState::Roaming => match state {
                State::Begin => begin_roaming(&mut self._behavior_data, owner, target),
                State::Update => {
                    update_hunger_action(owner);
                    if owner.get_attached_item_data_type().is_eatable() {
                        self.set_next_behavior(BehaviorState::Eating, false);
                    } else if owner.is_interacting() {
                        self.set_next_behavior(BehaviorState::Idle, true);
                    } else if is_player_too_far_for_intimacy(owner, target) {
                        self.set_next_behavior(BehaviorState::Follow, false);
                    } else if should_roaming_go_idle(&self._behavior_data, owner) {
                        self.set_next_behavior(BehaviorState::Idle, false);
                    }
                }
                State::End => {}
            },
            BehaviorState::Interaction => step_interaction(&mut self._behavior_data, state, owner, target),
            BehaviorState::Dead => {
                if state == State::Begin {
                    owner.set_action_dead();
                    owner.set_move_idle();
                }
            }
            BehaviorState::PassedOut => {
                if state == State::Begin {
                    owner.set_action_passed_out();
                    owner.set_move_idle();
                }
            }
            BehaviorState::WakeUp => step_wake_up(&mut self._behavior_data, state, is_first_update, owner, false),
            BehaviorState::Follow => step_follow(&mut self._behavior_data, state, owner, target),
            BehaviorState::Dance => step_dance(&mut self._behavior_data, state, owner),
            _ => {}
        }
    }
}
