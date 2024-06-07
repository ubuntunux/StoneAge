use rand;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterDataType, MoveDirections};

pub trait BehaviorBase {
    fn update_behavior<'a>(&mut self, character: &mut Character, delta_time: f32, is_blocked: bool, player: &Character<'a>);
}

pub struct BehaviorProperty {
    pub _behavior_move_time: f32,
    pub _behavior_move_direction: MoveDirections,
    pub _behavior_attack_time: f32,
}

impl Default for BehaviorProperty {
    fn default() -> Self {
        Self {
            _behavior_move_time: 0.0,
            _behavior_move_direction: MoveDirections::NONE,
            _behavior_attack_time: generate_attack_time()
        }
    }
}

pub struct HomoSapiensBehavior {
    pub _property: BehaviorProperty
}

pub struct TyrannosaurusBehavior {
    pub _property: BehaviorProperty
}

// implements
fn generate_attack_time() -> f32 {
    1.0 + rand::random::<f32>() * 2.0
}

pub fn create_character_behavior(character_type: CharacterDataType) -> Box<dyn BehaviorBase> {
    match character_type {
        CharacterDataType::HomoSapiens => Box::new(HomoSapiensBehavior {
            _property: BehaviorProperty::default()
        }),
        CharacterDataType::Tyrannosaurus => Box::new(TyrannosaurusBehavior {
            _property: BehaviorProperty::default()
        }),
    }
}

impl BehaviorBase for HomoSapiensBehavior {
    fn update_behavior<'a>(&mut self, _character: &mut Character, _delta_time: f32, _is_blocked: bool, _player: &Character<'a>) {}
}

impl BehaviorBase for TyrannosaurusBehavior {
    fn update_behavior<'a>(&mut self, character: &mut Character, delta_time: f32, is_blocked: bool, player: &Character<'a>) {
        if 0.0 < self._property._behavior_attack_time {
            self._property._behavior_attack_time -= delta_time;

            if self._property._behavior_attack_time <= 0.0 {
                character.set_action_attack();
                self._property._behavior_attack_time = generate_attack_time();
            }
        }

        if character.is_available_move() && !character.is_action(ActionAnimationState::Attack) {
            self._property._behavior_move_time += delta_time;
            if 2.0 <= self._property._behavior_move_time || is_blocked {
                self._property._behavior_move_direction =
                    if MoveDirections::LEFT == self._property._behavior_move_direction {
                        MoveDirections::RIGHT
                    } else {
                        MoveDirections::LEFT
                    };
                self._property._behavior_move_time = 0.0;
            }

            character.set_move(self._property._behavior_move_direction);
        }
    }
}