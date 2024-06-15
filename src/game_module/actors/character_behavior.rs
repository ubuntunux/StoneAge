use rand;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterDataType, MoveDirections};
use crate::game_module::game_constants::EAT_FOOD_DISTANCE;

pub trait BehaviorBase {
    fn update_behavior<'a>(&mut self, character: &mut Character, delta_time: f32, is_blocked: bool, player: &Character<'a>);
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
    1.0 + rand::random::<f32>() * 2.0
}

pub fn create_character_behavior(character_type: CharacterDataType) -> Box<dyn BehaviorBase> {
    match character_type {
        CharacterDataType::Roamer => Box::new(RoamerBehavior {
            ..Default::default()
        }),
    }
}

impl BehaviorBase for RoamerBehavior {
    fn update_behavior<'a>(&mut self, character: &mut Character, delta_time: f32, is_blocked: bool, player: &Character<'a>) {
        if 0.0 < self._roamer_attack_time {
            self._roamer_attack_time -= delta_time;
        }

        if self._roamer_attack_time <= 0.0 {
            let dist = (character.get_position() - player.get_position()).norm();
            if dist <= EAT_FOOD_DISTANCE {
                character.set_action_attack();
                self._roamer_attack_time = generate_attack_time();
            }
        }

        if !character.is_action(ActionAnimationState::Attack) && character.is_available_move() {
            self._roamer_move_time += delta_time;
            if 2.0 <= self._roamer_move_time || is_blocked {
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