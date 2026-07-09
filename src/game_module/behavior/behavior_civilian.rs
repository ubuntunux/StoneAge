use nalgebra::Vector3;
use strum::IntoEnumIterator;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{State};
use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorState};
use crate::game_module::game_constants::{GameViewMode, ARRIVAL_DISTANCE_THRESHOLD, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME};
use crate::game_module::actors::character_data::ActionAnimationState;

pub struct BehaviorCivilian<'a> {
    pub _behavior_data: BehaviorData<'a>
}

impl<'a> Default for BehaviorCivilian<'a> {
    fn default() -> Self {
        Self {
            _behavior_data: BehaviorData::default()
        }
    }
}

impl<'a> BehaviorBase<'a> for BehaviorCivilian<'a> {
    fn initialize_behavior(&mut self, owner: &Character<'a>, position: &Vector3<f32>) {
        self._behavior_data.initialize_behavior_data(owner, position);
    }

    fn set_next_behavior(&mut self, next_behavior_state: BehaviorState, is_force: bool) {
        self._behavior_data.set_next_behavior_state(next_behavior_state, is_force);
    }

    fn update_behavior(&mut self, owner: &mut Character<'a>, target: Option<&Character<'a>>, delta_time: f32) {
        let prev_behavior_state = self._behavior_data.get_behavior_state();
        let next_behavior_state = self._behavior_data.get_next_behavior_state();
        let is_force = self._behavior_data.is_force_behavior_state_changed();

        for state in State::iter() {
            if is_force == false && prev_behavior_state == next_behavior_state && (state == State::End || state == State::Begin) {
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

            match update_behavior_state {
                BehaviorState::Idle => {
                    match state {
                        State::Begin => {
                            if owner.get_stats().is_hungry() {
                                owner.set_action_hungry();
                                owner.set_sit_down();
                            } else {
                                owner.set_action_none();
                                owner.set_move_idle();
                            }
                            self._behavior_data.set_behavior_time(math::lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>()));
                        }
                        State::Update => {
                            if owner.get_attached_item_data_type().is_eatable() {
                                self.set_next_behavior(BehaviorState::Eating, false);
                            } else {
                                if owner.get_stats().is_hungry() == false {
                                    if self._behavior_data.is_end_behavior_time() {
                                        self.set_next_behavior(BehaviorState::Roaming, false);
                                    }
                                }
                            }
                        }
                        State::End => {},
                    };
                }
                BehaviorState::Eating => {
                    match state {
                        State::Begin => {
                            owner.set_action_eating();
                            if owner.is_move_stop() == false {
                                owner.set_move_idle();
                            }
                            self._behavior_data.set_behavior_time(NPC_IDLE_TERM_MIN);
                        }
                        State::Update => {
                            if owner.is_action(ActionAnimationState::Eating) == false {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
                        }
                        State::End => {},
                    };
                }
                BehaviorState::Roaming => {
                    match state {
                        State::Begin => {
                            let move_area = math::safe_normalize(&Vector3::new(
                                rand::random::<f32>() - 0.5,
                                0.0,
                                if GAME_VIEW_MODE == GameViewMode::GameViewMode2D { 0.0 } else { rand::random::<f32>() - 0.5 },
                            )) * NPC_ROAMING_RADIUS;
                            self._behavior_data._target_point = self._behavior_data._spawn_point + move_area;
                            self._behavior_data._move_direction = math::make_normalize_xz(&(self._behavior_data._target_point - owner.get_position()));
                            owner.set_move(&self._behavior_data._move_direction);
                            owner.set_run(false);
                            self._behavior_data.set_behavior_time(NPC_ROAMING_TIME);
                        }
                        State::Update => {
                            if owner.get_attached_item_data_type().is_eatable() {
                                self.set_next_behavior(BehaviorState::Eating, false);
                            } else {
                                let mut do_idle: bool = false;
                                if self._behavior_data.is_end_behavior_time() {
                                    do_idle = true;
                                } else {
                                    let offset = self._behavior_data._target_point - owner.get_position();
                                    let dist = offset.x * offset.x + offset.z * offset.z;
                                    if dist < ARRIVAL_DISTANCE_THRESHOLD {
                                        do_idle = true;
                                    } else if (owner._controller._is_blocked || owner._controller._is_cliff) && !owner.is_falling() {
                                        do_idle = true;
                                    }
                                }

                                if do_idle {
                                    self.set_next_behavior(BehaviorState::Idle, false);
                                }
                            }
                        }
                        State::End => {},
                    };
                }
                BehaviorState::Interaction => {
                    match state {
                        State::Begin => {
                            if owner.is_move_stop() == false {
                                owner.set_move_idle();
                            }
                            self._behavior_data.set_behavior_time(CHARACTER_INTERACTION_TIME);
                        }
                        State::Update => {
                            if self._behavior_data.is_end_behavior_time() {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            } else {
                                if let Some(target_actor) = target.as_ref() {
                                    owner.look_at(&target_actor.get_position());
                                }
                            }
                        }
                        State::End => {},
                    };
                }
                BehaviorState::WakeUp => {
                    match state {
                        State::Begin => {}
                        State::Update => {
                            if owner.is_action(ActionAnimationState::WakeUp) == false {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
                        }
                        State::End => {},
                    };
                }
                _ => {}
            }

            if state == State::Update {
                self._behavior_data.update_behavior_time(delta_time);
            }
        }
    }
}