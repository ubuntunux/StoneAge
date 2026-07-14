use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorData, BehaviorState};
use crate::game_module::game_constants::{
    ARRIVAL_DISTANCE_THRESHOLD, GAME_VIEW_MODE, GameViewMode, NPC_ATTACK_RANGE,
    NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN, NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX,
    NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME, NPC_TRACKING_RANGE,
};
use nalgebra::Vector3;
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
    fn is_enemy_in_range(&self, owner: &Character, target: Option<&Character>) -> bool {
        if let Some(target) = target.as_ref()
            && target.is_alive()
        {
            return owner.check_in_range(target.get_collision(), NPC_TRACKING_RANGE, false);
        }
        false
    }
}

impl<'a> BehaviorBase<'a> for BehaviorRoamer<'a> {
    fn initialize_behavior(&mut self, position: &Vector3<f32>) {
        self._behavior_data.initialize_behavior_data(position);
    }

    fn set_next_behavior(&mut self, next_behavior_state: BehaviorState, is_force: bool) {
        self._behavior_data.set_next_behavior_state(next_behavior_state, is_force);
    }

    fn update_behavior(
        &mut self,
        owner: &mut Character<'a>,
        target: Option<&Character<'a>>,
        delta_time: f32,
    ) {
        let prev_behavior_state = self._behavior_data.get_behavior_state();
        let next_behavior_state = self._behavior_data.get_next_behavior_state();
        let is_force = self._behavior_data.is_force_behavior_state_changed();

        for state in State::iter() {
            if !is_force
                && prev_behavior_state == next_behavior_state
                && (state == State::End || state == State::Begin)
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

            match update_behavior_state {
                BehaviorState::Idle => match state {
                    State::Begin => {
                        owner.set_move_idle();
                        self._behavior_data.set_behavior_time(lerp(
                            NPC_IDLE_TERM_MIN,
                            NPC_IDLE_TERM_MAX,
                            rand::random::<f32>(),
                        ));
                    }
                    State::Update => {
                        if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else if self._behavior_data.is_end_behavior_time() {
                            self.set_next_behavior(BehaviorState::Roaming, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Roaming => match state {
                    State::Begin => {
                        let move_area = math::safe_normalize(&Vector3::new(
                            rand::random::<f32>() - 0.5,
                            0.0,
                            if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                                0.0
                            } else {
                                rand::random::<f32>() - 0.5
                            },
                        )) * NPC_ROAMING_RADIUS;
                        self._behavior_data._target_point =
                            self._behavior_data._spawn_point + move_area;
                        self._behavior_data._move_direction = math::safe_normalize(
                            &(self._behavior_data._target_point - owner.get_position()),
                        );
                        self._behavior_data.set_behavior_time(NPC_ROAMING_TIME);
                        owner.set_move(&self._behavior_data._move_direction);
                        owner.set_run(false);
                    }
                    State::Update => {
                        if self.is_enemy_in_range(owner, target) {
                            self.set_next_behavior(BehaviorState::Chase, false);
                        } else {
                            let mut do_idle: bool = false;
                            if self._behavior_data.is_end_behavior_time() {
                                do_idle = true;
                            } else {
                                let offset =
                                    self._behavior_data._target_point - owner.get_position();
                                let dist = offset.x * offset.x + offset.z * offset.z;
                                if dist < ARRIVAL_DISTANCE_THRESHOLD {
                                    do_idle = true;
                                } else if (owner._controller._is_blocked
                                    || owner._controller._is_cliff)
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
                BehaviorState::Chase => match state {
                    State::Begin => {}
                    State::Update => {
                        let mut do_idle: bool = true;
                        if let Some(target_ref) = target
                            && target_ref.is_alive()
                            && owner.check_in_range(
                                target_ref.get_collision(),
                                NPC_TRACKING_RANGE,
                                false,
                            )
                        {
                            if owner.check_in_range(
                                target_ref.get_collision(),
                                NPC_ATTACK_RANGE,
                                false,
                            ) {
                                self.set_next_behavior(BehaviorState::Attack, false);
                            } else {
                                let to_target: Vector3<f32> =
                                    target_ref.get_position() - owner.get_position();
                                owner.set_move(&to_target);
                                owner.set_run(true);
                            }
                            do_idle = false;
                        }

                        if do_idle {
                            self.set_next_behavior(BehaviorState::Idle, false);
                        }
                    }
                    State::End => {}
                },
                BehaviorState::Attack => {
                    match state {
                        State::Begin => {
                            let to_target_direction = math::safe_normalize(
                                &(target.as_ref().unwrap().get_position() - owner.get_position()),
                            );
                            owner.set_move_direction(&to_target_direction, false);
                            if !NPC_AVAILABLE_MOVING_ATTACK {
                                owner.set_move_idle();
                            }
                            owner.set_action_attack();
                            self._attack_time = lerp(
                                NPC_ATTACK_TERM_MIN,
                                NPC_ATTACK_TERM_MAX,
                                rand::random::<f32>(),
                            );

                            // growl
                            owner.get_character_manager().get_scene_manager().play_audio(
                                &owner._character_data.borrow()._audio_data._audio_growl,
                            );
                        }
                        State::Update => {
                            let mut do_idle: bool = true;
                            if let Some(target_ref) = target
                                && target_ref.is_alive()
                                && 0.0 < self._attack_time
                            {
                                if owner.is_attack_animation() {
                                    if !owner.is_available_move()
                                        || (NPC_AVAILABLE_MOVING_ATTACK
                                            || !owner.is_attack_animation())
                                    {
                                        owner.set_move_idle();
                                    }
                                } else {
                                    owner.set_move_idle();
                                    self._attack_time -= delta_time;
                                }
                                do_idle = false;
                            }

                            if do_idle {
                                self.set_next_behavior(BehaviorState::Idle, false);
                            }
                        }
                        State::End => {}
                    }
                }
                _ => {}
            }

            if state == State::Update {
                self._behavior_data.update_behavior_time(delta_time);
            }
        }
    }
}
