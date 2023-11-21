use nalgebra::Vector3;
use crate::game_module::character::character::{Character, CharacterBase};

pub struct Player {
    pub _behavior: Box<Character>

}

impl CharacterBase for Player {
    fn set_move_idle(&mut self) {
        self._behavior.set_move_idle();
    }

    fn set_move_walk(&mut self, is_left: bool) {
        self._behavior.set_move_walk(is_left);
    }

    fn set_move_jump(&mut self) {
        self._behavior.set_move_jump();
    }

    fn set_action_attack(&mut self) {
        self._behavior.set_action_attack();
    }

    fn get_position(&self) -> &Vector3<f32> {
        &self._behavior.get_character_controller()._position
    }

    fn update_character(&mut self, delta_time: f32) {
        self._behavior.update_character(delta_time);
    }
}

impl Player {
    pub fn create_player(behavior: Box<Character>) -> Player {
        Player {
            _behavior: behavior
        }
    }
}