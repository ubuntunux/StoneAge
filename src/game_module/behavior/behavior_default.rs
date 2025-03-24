use nalgebra::Vector3;
use rust_engine_3d::utilities::math::lerp;
use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use crate::game_module::game_constants::{ARRIVAL_DISTANCE_THRESHOLD, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME, NPC_TRACKING_RANGE};

#[derive(Default)]
pub struct BehaviorDefault {
    pub _idle_time: f32,
    pub _move_direction: Vector3<f32>,
    pub _spawn_point: Vector3<f32>,
    pub _target_point: Vector3<f32>,
    pub _move_time: f32,
    pub _behavior_state: BehaviorState,
}

impl BehaviorBase for BehaviorDefault {
    fn initialize_behavior(&mut self, _owner: &mut Character, position: &Vector3<f32>) {
        self._spawn_point = position.clone();
        self._behavior_state = BehaviorState::None;
    }

    fn is_enemy_in_range(&self, owner: &Character, player: &Character) -> bool {
        if player._character_stats._is_alive {
            return owner.check_in_range(player.get_collision(), NPC_TRACKING_RANGE, false);
        }
        false
    }

    fn update_behavior(&mut self, owner: &mut Character, player: &Character, delta_time: f32) {
        match self._behavior_state {
            BehaviorState::None => {
                self.set_behavior(BehaviorState::Idle, owner, player, false);
            },
            BehaviorState::Idle => {
                if self._idle_time < 0.0 {
                    self.set_behavior(BehaviorState::Move, owner, player, false);
                }
                self._idle_time -= delta_time;
            },
            BehaviorState::Move => {
                let mut do_idle: bool = false;
                if 0.0 < self._move_time {
                    let offset = self._target_point - owner.get_position();
                    let dist = offset.x * offset.x + offset.z * offset.z;
                    if dist < ARRIVAL_DISTANCE_THRESHOLD {
                        do_idle = true;
                    } else if owner.is_on_ground() && (owner._controller._is_blocked || owner._controller._is_cliff) {
                        do_idle = true;
                    }
                } else {
                    do_idle = true;
                }

                if do_idle {
                    self.set_behavior(BehaviorState::Idle, owner, player, false);
                }
                self._move_time -= delta_time;
            },
            BehaviorState::Chase => {
            },
            BehaviorState::Attack => {
            }
        }
    }

    fn set_behavior(&mut self, behavior_state: BehaviorState, owner: &mut Character, player: &Character, is_force: bool) {
        if self._behavior_state != behavior_state || is_force {
            self.end_behavior(owner, player);

            self._behavior_state = behavior_state;
            match behavior_state {
                BehaviorState::None => {
                },
                BehaviorState::Idle => {
                    owner.set_move_stop();
                    self._idle_time = lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>());
                },
                BehaviorState::Move => {
                    let move_area = Vector3::new(rand::random::<f32>() - 0.5, 0.0, rand::random::<f32>() - 0.5).normalize() * NPC_ROAMING_RADIUS;
                    self._target_point = self._spawn_point + move_area;
                    self._move_direction = (self._target_point - owner.get_position()).normalize();
                    self._move_time = NPC_ROAMING_TIME;
                    owner.set_move(&self._move_direction);
                    owner.set_run(false);
                },
                BehaviorState::Chase => {
                },
                BehaviorState::Attack => {
                }
            }
        }
    }

    fn end_behavior(&mut self, _owner: &mut Character, _player: &Character) {
        match self._behavior_state {
            BehaviorState::None => {
            },
            BehaviorState::Idle => {
            },
            BehaviorState::Move => {
            },
            BehaviorState::Chase => {
            },
            BehaviorState::Attack => {
            }
        }
    }
}