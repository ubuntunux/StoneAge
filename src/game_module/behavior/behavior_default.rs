use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorSaveData, BehaviorState};
use crate::game_module::game_constants::{
    ARRIVAL_DISTANCE_THRESHOLD, GAME_VIEW_MODE, GameViewMode, INTIMACY_ARRIVE_RANGE, INTIMACY_FOLLOW_RANGE,
    INTIMACY_ROAMING_RADIUS, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME,
};
use nalgebra::Vector3;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;
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

            let is_first_update_behavior_state = prev_behavior_state != next_behavior_state && state == State::Update;

            match update_behavior_state {
                BehaviorState::Idle => {
                    match state {
                        State::Begin => {
                            owner.set_move_idle();
                            self._behavior_data.set_behavior_time(lerp(
                                NPC_IDLE_TERM_MIN,
                                NPC_IDLE_TERM_MAX,
                                rand::random::<f32>(),
                            ));
                        }
                        State::Update => {
                            if owner.is_following_intimacy()
                                && let Some(target_ref) = target
                                && target_ref.is_alive()
                                && (target_ref.get_position() - owner.get_position()).norm() > INTIMACY_FOLLOW_RANGE
                            {
                                self.set_next_behavior(BehaviorState::Follow, false);
                            } else if self._behavior_data.is_end_behavior_time() {
                                self.set_next_behavior(BehaviorState::Roaming, false);
                            }
                        }
                        State::End => {}
                    };
                }
                BehaviorState::Roaming => match state {
                    State::Begin => {
                        let mut is_intimate_following = false;
                        if owner.is_following_intimacy()
                            && let Some(target_ref) = target
                            && target_ref.is_alive()
                            && (target_ref.get_position() - owner.get_position()).norm() <= INTIMACY_FOLLOW_RANGE
                        {
                            is_intimate_following = true;
                        }

                        let roam_radius = if is_intimate_following {
                            INTIMACY_ROAMING_RADIUS
                        } else {
                            NPC_ROAMING_RADIUS
                        };

                        let move_area = Vector3::new(
                            (rand::random::<f32>() - 0.5) * 2.0,
                            0.0,
                            if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                                0.0
                            } else {
                                (rand::random::<f32>() - 0.5) * 2.0
                            },
                        ) * roam_radius;

                        let center_point = if is_intimate_following {
                            *target.as_ref().unwrap().get_position()
                        } else {
                            self._behavior_data._spawn_point
                        };

                        self._behavior_data._target_point = center_point + move_area;
                        self._behavior_data._move_direction =
                            math::safe_normalize(&(self._behavior_data._target_point - owner.get_position()));
                        owner.set_move(&self._behavior_data._move_direction);
                        owner.set_run(false);
                        self._behavior_data.set_behavior_time(NPC_ROAMING_TIME);
                    }
                    State::Update => {
                        if owner.is_following_intimacy()
                            && let Some(target_ref) = target
                            && target_ref.is_alive()
                            && (target_ref.get_position() - owner.get_position()).norm() > INTIMACY_FOLLOW_RANGE
                        {
                            // Player moved too far -> chase to close the gap
                            self.set_next_behavior(BehaviorState::Follow, false);
                        } else {
                            let mut do_idle: bool = false;
                            if self._behavior_data.is_end_behavior_time() {
                                do_idle = true;
                            } else {
                                let offset = self._behavior_data._target_point - owner.get_position();
                                let dist = offset.x * offset.x + offset.z * offset.z;
                                if dist < ARRIVAL_DISTANCE_THRESHOLD {
                                    do_idle = true;
                                } else if (owner._controller._is_blocked || owner._controller._is_cliff)
                                    && !owner.is_falling()
                                {
                                    do_idle = true;
                                }
                            }

                            if do_idle {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
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
                    State::Begin => {
                        owner.set_action_wake_up();
                    }
                    State::Update => {
                        if !is_first_update_behavior_state && !owner.is_action(ActionAnimationState::WakeUp) {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Follow => match state {
                    State::Begin => {}
                    State::Update => {
                        if owner.is_following_intimacy()
                            && let Some(target_ref) = target
                            && target_ref.is_alive()
                        {
                            let to_target = target_ref.get_position() - owner.get_position();
                            let dist = (to_target.x * to_target.x + to_target.z * to_target.z).sqrt();
                            if dist <= INTIMACY_ARRIVE_RANGE {
                                // Close enough -> roam around player
                                self.set_next_behavior(BehaviorState::Roaming, false);
                            } else {
                                owner.set_move(&to_target);
                                owner.set_run(true);
                            }
                        } else {
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
