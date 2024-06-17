use rand;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterDataType, MoveDirections};
use crate::game_module::game_constants::{NPC_ATTACK_TERM, NPC_ROAMING_TERM};

pub trait BehaviorBase {
    fn update_behavior<'a>(&mut self, character: &mut Character, player: &Character<'a>, delta_time: f32);
}

pub struct RoamerBehavior {
    pub _roamer_move_time: f32,
    pub _roamer_move_direction: MoveDirections,
    pub _roamer_attack_time: f32,
}

impl Default for RoamerBehavior {
    fn default() -> Self {
        Self {
            _roamer_move_time: 0.0,
            _roamer_move_direction: MoveDirections::NONE,
            _roamer_attack_time: generate_attack_time()
        }
    }
}

// implements
fn generate_attack_time() -> f32 {
    rand::random::<f32>() * NPC_ATTACK_TERM
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
            let to_player_direction =
                if owner.get_position().x <= player.get_position().x {
                    MoveDirections::RIGHT
                } else {
                    MoveDirections::LEFT
                };
            let bounding_box = owner.get_bounding_box();
            let player_bounding_box = player.get_bounding_box();
            if bounding_box.collide_bound_box_xy(&player_bounding_box._min, &player_bounding_box._max) {
                owner._controller.set_move_direction(to_player_direction);
                owner.set_action_attack();
                self._roamer_attack_time = generate_attack_time();
            }
        }

        // update roaming
        if !owner.is_action(ActionAnimationState::Attack) && owner.is_available_move() {
            let is_blocked = owner._controller._is_blocked;
            let is_cliff = owner._controller._is_cliff;
            self._roamer_move_time -= delta_time;
            if self._roamer_move_time <= 0.0 || is_blocked || is_cliff {
                self._roamer_move_direction =
                    if MoveDirections::LEFT == self._roamer_move_direction {
                        MoveDirections::RIGHT
                    } else {
                        MoveDirections::LEFT
                    };
                self._roamer_move_time = NPC_ROAMING_TERM;
            }

            owner.set_move(self._roamer_move_direction);
        }
    }
}