use nalgebra::Vector3;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::lerp;
use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use crate::game_module::game_constants::{GameViewMode, ARRIVAL_DISTANCE_THRESHOLD, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME};
use crate::game_module::actors::character_data::ActionAnimationState;

#[derive(Default)]
pub struct BehaviorCivilian {
    pub _behavior_time: f32,
    pub _move_direction: Vector3<f32>,
    pub _spawn_point: Vector3<f32>,
    pub _target_point: Vector3<f32>,
    pub _behavior_state: BehaviorState,
    pub _next_behavior_state: BehaviorState,
}

impl BehaviorBase for BehaviorCivilian {
    fn initialize_behavior(&mut self, _owner: &mut Character, position: &Vector3<f32>) {
        self._spawn_point = position.clone();
        self._behavior_state = BehaviorState::Idle;
    }

    fn update_behavior(
        &mut self,
        owner: &mut Character,
        target: Option<&Character>,
        delta_time: f32,
    ) {
        match self._behavior_state {
            BehaviorState::Idle => {
                if owner.get_attached_item_data_type().is_eatable() {
                    self.set_behavior(BehaviorState::Eating, owner, target, false);
                } else {
                    if owner.get_stats().is_hungry() == false {
                        if self._behavior_time < 0.0 {
                            self.set_behavior(BehaviorState::Roaming, owner, target, false);
                        }
                        self._behavior_time -= delta_time;
                    }
                }
            }
            BehaviorState::Eating => {
                if owner.is_action(ActionAnimationState::Eating) == false {
                    self.set_behavior(BehaviorState::Idle, owner, target, false);
                }
            }
            BehaviorState::Roaming => {
                if owner.get_attached_item_data_type().is_eatable() {
                    self.set_behavior(BehaviorState::Eating, owner, target, false);
                } else {
                    let mut do_idle: bool = false;
                    if 0.0 < self._behavior_time {
                        let offset = self._target_point - owner.get_position();
                        let dist = offset.x * offset.x + offset.z * offset.z;
                        if dist < ARRIVAL_DISTANCE_THRESHOLD {
                            do_idle = true;
                        } else if (owner._controller._is_blocked || owner._controller._is_cliff) && !owner.is_falling() {
                            do_idle = true;
                        }
                    } else {
                        do_idle = true;
                    }

                    if do_idle {
                        self.set_behavior(BehaviorState::Idle, owner, target, false);
                    }
                    self._behavior_time -= delta_time;
                }
            }
            BehaviorState::Interaction => {
                if 0.0 < self._behavior_time {
                    owner.look_at(&target.unwrap().get_position());
                } else {
                    self.set_behavior(BehaviorState::Idle, owner, target, false);
                }
                self._behavior_time -= delta_time;
            }
            BehaviorState::WakeUp => {
                if owner.is_action(ActionAnimationState::WakeUp) == false {
                    self.set_behavior(BehaviorState::Idle, owner, target, false);
                }
            }
            _ => (),
        }
    }

    fn set_behavior(
        &mut self,
        behavior_state: BehaviorState,
        owner: &mut Character,
        target: Option<&Character>,
        is_force: bool,
    ) {
        if self._behavior_state != behavior_state || is_force {
            self.end_behavior(owner, target);

            self._behavior_state = behavior_state;
            match behavior_state {
                BehaviorState::Idle => {
                    if owner.get_stats().is_hungry() {
                        owner.set_action_hungry();
                        owner.set_sit_down();
                    } else {
                        owner.set_action_none();
                        owner.set_move_idle();
                    }
                    self._behavior_time = lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>());
                }
                BehaviorState::Eating => {
                    owner.set_action_eating();
                    if owner.is_move_stop() == false {
                        owner.set_move_idle();
                    }
                    self._behavior_time = NPC_IDLE_TERM_MIN;
                }
                BehaviorState::Roaming => {
                    let move_area = math::safe_normalize(&Vector3::new(
                        rand::random::<f32>() - 0.5,
                        0.0,
                        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D { 0.0 } else { rand::random::<f32>() - 0.5 },
                    )) * NPC_ROAMING_RADIUS;
                    self._target_point = self._spawn_point + move_area;
                    self._move_direction = math::make_normalize_xz(&(self._target_point - owner.get_position()));
                    owner.set_move(&self._move_direction);
                    owner.set_run(false);
                    self._behavior_time = NPC_ROAMING_TIME;
                }
                BehaviorState::Interaction => {
                    if owner.is_move_stop() == false {
                        owner.set_move_idle();
                    }
                    self._behavior_time = CHARACTER_INTERACTION_TIME;
                }
                BehaviorState::Sleep => {
                    owner.set_action_sleep();
                }
                BehaviorState::WakeUp => {
                    owner.set_action_wake_up();
                }
                _ => (),
            }
        }
    }

    fn end_behavior(&mut self, _owner: &mut Character, _target: Option<&Character>) {
        match self._behavior_state {
            _ => (),
        }
    }
}
