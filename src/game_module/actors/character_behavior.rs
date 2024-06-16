use rand;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterDataType, MoveDirections};
use crate::game_module::game_constants::{NPC_ATTACK_TERM, NPC_ROAMING_TERM, NPC_ATTACK_DISTANCE};

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
    fn update_behavior<'a>(&mut self, character: &mut Character, player: &Character<'a>, delta_time: f32) {
        // updat attack
        if 0.0 < self._roamer_attack_time {
            self._roamer_attack_time -= delta_time;
        } else {
            let attack_range: f32 = character.get_bounding_box()._size.x * 0.5;
            let dist = (character.get_position() - player.get_position()).norm();
            if dist <= NPC_ATTACK_DISTANCE.max(attack_range) {
                character.set_action_attack();
                self._roamer_attack_time = generate_attack_time();
            }
        }

        // update roaming
        if !character.is_action(ActionAnimationState::Attack) && character.is_available_move() {
            let is_blocked = character._controller._is_blocked;
            let is_cliff = character._controller._is_cliff;
            self._roamer_move_time += delta_time;
            if NPC_ROAMING_TERM <= self._roamer_move_time || is_blocked || is_cliff {
                self._roamer_move_direction =
                    if MoveDirections::LEFT == self._roamer_move_direction {
                        MoveDirections::RIGHT
                    } else {
                        MoveDirections::LEFT
                    };
                self._roamer_move_time = 0.0;
            }

            character.set_move(self._roamer_move_direction);
        }
    }
}