use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::MoveDirections;

pub struct CharacterBehavior {
    pub _behavior_move_time: f32,
    pub _behavior_move_direction: MoveDirections,
}

impl CharacterBehavior {
    pub fn create_character_behavior() -> CharacterBehavior {
        CharacterBehavior {
            _behavior_move_time: 0.0,
            _behavior_move_direction: MoveDirections::NONE,
        }
    }

    pub fn update_behavior(&mut self, character: &mut Character, delta_time: f32, is_blocked: bool) {
        if character.is_available_move() {
            self._behavior_move_time += delta_time;
            if 2.0 <= self._behavior_move_time || is_blocked {
                self._behavior_move_direction =
                    if MoveDirections::LEFT == self._behavior_move_direction {
                        MoveDirections::RIGHT
                    } else {
                        MoveDirections::LEFT
                    };
                self._behavior_move_time = 0.0;
            }

            let is_running = false;
            character.set_move(self._behavior_move_direction, is_running);
        }
    }
}