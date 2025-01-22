use nalgebra::Vector3;
use rust_engine_3d::utilities::math::lerp;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use crate::game_module::game_constants::{NPC_ATTACK_TERM_MAX, NPC_ATTACK_TERM_MIN, NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_ROAMING_RADIUS, NPC_ROAMING_TIME, NPC_TRACKING_RANGE_XZ, NPC_TRACKING_RANGE_Y};

#[derive(Default)]
pub struct RoamerBehavior {
    pub _roamer_idle_time: f32,
    pub _roamer_move_direction: Vector3<f32>,
    pub _roamer_spawn_point: Vector3<f32>,
    pub _roamer_target_point: Vector3<f32>,
    pub _roamer_move_time: f32,
    pub _roamer_attack_time: f32,
    pub _behavior_state: BehaviorState,
}

impl BehaviorBase for RoamerBehavior {
    fn initialize_behavior(&mut self, _owner: &mut Character, position: &Vector3<f32>) {
        self._roamer_spawn_point = position.clone();
        self._behavior_state = BehaviorState::None;
    }

    fn is_enemy_in_range(&self, owner: &Character, player: &Character) -> bool {
        if player._is_alive {
            let to_player: Vector3<f32> = player.get_position() - owner.get_position();
            let dist: f32 = (to_player.x * to_player.x + to_player.z * to_player.z).sqrt();
            if dist < NPC_TRACKING_RANGE_XZ && to_player.y.abs() < NPC_TRACKING_RANGE_Y {
                return true;
            }
        }
        false
    }

    fn update_behavior(&mut self, owner: &mut Character, player: &Character, delta_time: f32) {
        match self._behavior_state {
            BehaviorState::None => {
                self.set_behavior(BehaviorState::Idle, owner, player, false);
            },
            BehaviorState::Idle => {
                if self.is_enemy_in_range(owner, player) {
                    self.set_behavior(BehaviorState::Chase, owner, player, false);
                } else if self._roamer_idle_time < 0.0 {
                    self.set_behavior(BehaviorState::Move, owner, player, false);
                }
                self._roamer_idle_time -= delta_time;
            },
            BehaviorState::Move => {
                if self.is_enemy_in_range(owner, player) {
                    self.set_behavior(BehaviorState::Chase, owner, player, false);
                } else {
                    let mut do_idle: bool = false;
                    if 0.0 < self._roamer_move_time {
                        let offset = self._roamer_target_point - owner.get_position();
                        let dist = offset.x * offset.x + offset.z * offset.z;
                        if dist < 1.0 {
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
                }
                self._roamer_move_time -= delta_time;
            },
            BehaviorState::Chase => {
                if player._is_alive {
                    let to_player: Vector3<f32> = player.get_position() - owner.get_position();
                    let dist: f32 = (to_player.x * to_player.x + to_player.z * to_player.z).sqrt();
                    if dist < NPC_TRACKING_RANGE_XZ * 2.0 && to_player.y.abs() < NPC_TRACKING_RANGE_Y {
                        if owner.check_attack_range(ActionAnimationState::Attack, player.get_bounding_box()) {
                            self.set_behavior(BehaviorState::Attack, owner, player, false);
                        } else {
                            // chase
                            owner.set_move(&to_player);
                            owner.set_run(true);
                        }
                    } else {
                        self.set_behavior(BehaviorState::Idle, owner, player, false);
                    }
                } else {
                    self.set_behavior(BehaviorState::Idle, owner, player, false);
                }
            },
            BehaviorState::Attack => {
                // if (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation()) && owner.is_available_move() {
                // }

                if player._is_alive && 0.0 < self._roamer_attack_time {
                    if !owner.is_attack_animation() {
                        owner.set_move_stop();
                        self._roamer_attack_time -= delta_time;
                    }
                } else {
                    self.set_behavior(BehaviorState::Idle, owner, player, false);
                }
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
                    self._roamer_idle_time = lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>());
                },
                BehaviorState::Move => {
                    let move_area = Vector3::new(rand::random::<f32>() - 0.5, 0.0, rand::random::<f32>() - 0.5).normalize() * NPC_ROAMING_RADIUS;
                    self._roamer_target_point = self._roamer_spawn_point + move_area;
                    self._roamer_move_direction = (self._roamer_target_point - owner.get_position()).normalize();
                    self._roamer_move_time = NPC_ROAMING_TIME;
                    owner.set_move(&self._roamer_move_direction);
                    owner.set_run(false);
                },
                BehaviorState::Chase => {
                    // growl
                    owner.get_character_manager().play_audio(&owner._audio_growl);
                },
                BehaviorState::Attack => {
                    let to_player_direction = (player.get_position() - owner.get_position()).normalize();
                    owner.set_move_direction(&to_player_direction);
                    if !NPC_AVAILABLE_MOVING_ATTACK {
                        owner.set_move_stop();
                    }
                    owner.set_action_attack();
                    self._roamer_attack_time = lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>());

                    // growl
                    owner.get_character_manager().play_audio(&owner._audio_growl);
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