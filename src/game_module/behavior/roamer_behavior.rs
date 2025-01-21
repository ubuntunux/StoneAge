use nalgebra::Vector3;
use rust_engine_3d::utilities::math::lerp;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState};
use crate::game_module::game_constants::{
    NPC_ATTACK_TERM_MAX,
    NPC_ATTACK_TERM_MIN,
    NPC_AVAILABLE_MOVING_ATTACK,
    NPC_IDLE_TERM_MAX,
    NPC_IDLE_TERM_MIN,
    NPC_ROAMING_RADIUS,
    NPC_ROAMING_TIME,
    NPC_TRACKING_RANGE_X,
    NPC_TRACKING_RANGE_Y
};

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

    fn is_enemy_in_range(&self, _owner: &Character, _player: &Character) -> bool {
        false
    }

    fn update_behavior(&mut self, owner: &mut Character, player: &Character, delta_time: f32) {
        match self._behavior_state {
            BehaviorState::None => {
                self.set_behavior(BehaviorState::Idle, owner, player, false);
            },
            BehaviorState::Idle => {
                if self._roamer_idle_time < 0.0 {
                    self.set_behavior(BehaviorState::Move, owner, player, false);
                }
                self._roamer_idle_time -= delta_time;
            },
            BehaviorState::Move => {
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
                self._roamer_move_time -= delta_time;
            },
            BehaviorState::Chase => {
                if (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation()) && owner.is_available_move() {
                    let to_player: Vector3<f32> = player.get_position() - owner.get_position();
                    if player._is_alive && (to_player.x.abs() < NPC_TRACKING_RANGE_X && to_player.y.abs() < NPC_TRACKING_RANGE_Y) {
                        // tracking
                        if owner.get_bounding_box()._size.x * 0.5 < to_player.x.abs() {
                            // tracking player
                            owner.set_move(&to_player);
                            owner.set_run(true);
                        } else {
                            // player in attack range
                            owner.set_move_idle();
                            owner.set_move_direction(&to_player);
                        }
                    }
                }
            },
            BehaviorState::Attack => {
                if player._is_alive {
                    if 0.0 < self._roamer_attack_time {
                        self._roamer_attack_time -= delta_time;
                    } else if owner.check_attack_range(ActionAnimationState::Attack, player.get_bounding_box()) {
                        let to_player_direction = (player.get_position() - owner.get_position()).normalize();
                        owner.set_move_direction(&to_player_direction);
                        if !NPC_AVAILABLE_MOVING_ATTACK {
                            owner.set_move_stop();
                        }
                        owner.set_action_attack();
                        self._roamer_attack_time = lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>());
                    }
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
                    let to_player: Vector3<f32> = player.get_position() - owner.get_position();
                    if to_player.x.abs() < NPC_TRACKING_RANGE_X * 2.0 {
                        owner.get_character_manager().play_audio(&owner._audio_growl);
                    }
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