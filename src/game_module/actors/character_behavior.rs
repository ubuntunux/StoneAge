use nalgebra::Vector3;
use rand;
use rust_engine_3d::utilities::math::lerp;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterDataType, MoveDirections};
use crate::game_module::game_constants::{NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, NPC_ROAMING_TERM, NPC_TRACKING_RANGE_X, NPC_TRACKING_RANGE_Y, NPC_AVAILABLE_MOVING_ATTACK, NPC_IDLE_TERM_MAX, NPC_IDLE_TERM_MIN, NPC_IDLE_PLAY_MIN, NPC_IDLE_PLAY_MAX};

pub trait BehaviorBase {
    fn update_behavior<'a>(&mut self, character: &mut Character, player: &Character<'a>, delta_time: f32);
}

pub struct RoamerBehavior {
    pub _roamer_idle_time: f32,
    pub _roamer_idle_play_time: f32,
    pub _roamer_move_time: f32,
    pub _roamer_move_direction: MoveDirections,
    pub _roamer_attack_time: f32,
}

impl Default for RoamerBehavior {
    fn default() -> Self {
        Self {
            _roamer_idle_time: generate_idle_time(),
            _roamer_idle_play_time: 0.0,
            _roamer_move_time: 0.0,
            _roamer_move_direction: MoveDirections::NONE,
            _roamer_attack_time: generate_attack_time(),
        }
    }
}

// implements
fn generate_idle_time() -> f32 {
    lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>())
}

fn generate_idle_play_time() -> f32 {
    lerp(NPC_IDLE_PLAY_MIN, NPC_IDLE_PLAY_MAX, rand::random::<f32>())
}

fn generate_attack_time() -> f32 {
    lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>())
}

pub fn create_character_behavior(character_type: CharacterDataType) -> Box<dyn BehaviorBase> {
    match character_type {
        CharacterDataType::Roamer => Box::new(RoamerBehavior {
            ..Default::default()
        }),
    }
}

impl BehaviorBase for RoamerBehavior {
    fn update_behavior<'a>(&mut self, owner: &mut Character, player: &Character<'a>, delta_time: f32) {
        // update attack
        if 0.0 < self._roamer_attack_time {
            self._roamer_attack_time -= delta_time;
        } else {
            if owner.check_attack_range(ActionAnimationState::Attack, player.get_bounding_box()) {
                let to_player_direction =
                    if owner.get_position().x <= player.get_position().x {
                        MoveDirections::RIGHT
                    } else {
                        MoveDirections::LEFT
                    };
                owner.set_move_direction(to_player_direction);
                if !NPC_AVAILABLE_MOVING_ATTACK {
                    owner.set_move_stop();
                }
                owner.set_action_attack();
                self._roamer_attack_time = generate_attack_time();
            }
        }

        // update roaming & tracking
        if (NPC_AVAILABLE_MOVING_ATTACK || !owner.is_attack_animation()) && owner.is_available_move() {
            let is_blocked = owner._controller._is_blocked;
            let is_cliff = owner._controller._is_cliff;
            let to_player: Vector3<f32> = player.get_position() - owner.get_position();
            if to_player.x.abs() < NPC_TRACKING_RANGE_X && to_player.y.abs() < NPC_TRACKING_RANGE_Y {
                let move_direction = if 0.0 < to_player.x {
                    MoveDirections::RIGHT
                } else {
                    MoveDirections::LEFT
                };

                if owner.get_bounding_box()._size.x * 0.5 < to_player.x.abs() {
                    // tracking player
                    owner.set_move(move_direction);
                    owner.set_run(true);
                } else {
                    // player in attack range
                    owner.set_move_idle();
                    owner.set_move_direction(move_direction);
                }
            } else {
                // idle
                self._roamer_idle_time -= delta_time;
                if self._roamer_idle_time <= 0.0 {
                    owner.set_move_stop();
                    self._roamer_idle_play_time = generate_idle_play_time();
                    self._roamer_idle_time = self._roamer_idle_play_time + generate_idle_time();
                }

                if self._roamer_idle_play_time < 0.0 {
                    // idle
                    owner.set_move_stop();
                    self._roamer_idle_play_time -= delta_time;
                } else if owner.is_on_ground() {
                    // roaming
                    self._roamer_move_time -= delta_time;
                    if self._roamer_move_time <= 0.0 || is_blocked || is_cliff {
                        self._roamer_move_direction =
                            if MoveDirections::LEFT == self._roamer_move_direction {
                                MoveDirections::RIGHT
                            } else {
                                MoveDirections::LEFT
                            };
                        owner.set_move(self._roamer_move_direction);
                        owner.set_run(false);
                        self._roamer_move_time = NPC_ROAMING_TERM;
                    }
                }
            }
        }
    }
}