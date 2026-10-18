use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorState};
use crate::game_module::behavior::behavior_common::{
    begin_idle, begin_roaming, is_enemy_in_range, is_player_too_far_for_intimacy, should_roaming_go_idle, step_attack,
    step_chase, step_dance, step_eating, step_hunger, step_interaction, step_wake_up, update_idle_interacting,
};
use rust_engine_3d::utilities::system::State;

#[derive(Default)]
pub struct BehaviorRoamer<'a> {
    pub _behavior_data: BehaviorData<'a>,
}

impl<'a> BehaviorBase<'a> for BehaviorRoamer<'a> {
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
        delta_time: f32,
    ) {
        match behavior_state {
            BehaviorState::Idle => match state {
                State::Begin => begin_idle(&mut self._behavior_data, owner),
                State::Update => {
                    if owner.get_attached_item_data_type().is_eatable() {
                        self.set_next_behavior(BehaviorState::Eating, true);
                    } else if owner.is_interacting() {
                        update_idle_interacting(owner, target);
                    } else if owner.get_stats().is_hungry() {
                        self.set_next_behavior(BehaviorState::Hunger, true);
                    } else if is_enemy_in_range(owner, target) || is_player_too_far_for_intimacy(owner, target) {
                        self.set_next_behavior(BehaviorState::Chase, false);
                    } else if self._behavior_data.is_end_behavior_time() {
                        self.set_next_behavior(BehaviorState::Roaming, false);
                    }
                }
                State::End => {}
            },
            BehaviorState::Hunger => step_hunger(&mut self._behavior_data, state, owner),
            BehaviorState::Eating => step_eating(&mut self._behavior_data, state, is_first_update, owner, target),
            BehaviorState::Interaction => step_interaction(&mut self._behavior_data, state, owner, target),
            BehaviorState::Roaming => match state {
                State::Begin => begin_roaming(&mut self._behavior_data, owner, target),
                State::Update => {
                    if owner.get_attached_item_data_type().is_eatable() {
                        self.set_next_behavior(BehaviorState::Eating, false);
                    } else if owner.is_interacting() {
                        self.set_next_behavior(BehaviorState::Interaction, true);
                    } else if owner.get_stats().is_hungry() {
                        self.set_next_behavior(BehaviorState::Hunger, true);
                    } else if is_enemy_in_range(owner, target) || is_player_too_far_for_intimacy(owner, target) {
                        self.set_next_behavior(BehaviorState::Chase, false);
                    } else if should_roaming_go_idle(&self._behavior_data, owner) {
                        self.set_next_behavior(BehaviorState::Idle, false);
                    }
                }
                State::End => {}
            },
            BehaviorState::Chase => step_chase(&mut self._behavior_data, state, owner, target),
            BehaviorState::Attack => step_attack(&mut self._behavior_data, state, owner, target, delta_time),
            BehaviorState::Dead => {
                if state == State::Begin {
                    owner.set_action_dead();
                }
            }
            BehaviorState::WakeUp => step_wake_up(&mut self._behavior_data, state, is_first_update, owner, false),
            BehaviorState::Dance => step_dance(&mut self._behavior_data, state, owner),
            _ => {}
        }
    }
}
