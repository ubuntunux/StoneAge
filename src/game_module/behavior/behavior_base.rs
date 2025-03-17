use nalgebra::Vector3;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::CharacterDataType;
use crate::game_module::behavior::behavior_default::BehaviorDefault;
use crate::game_module::behavior::behavior_roamer::BehaviorRoamer;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy, Default)]
pub enum BehaviorState {
    #[default]
    None,
    Idle,
    Move,
    Chase,
    Attack
}

pub fn create_character_behavior(character_type: CharacterDataType) -> Box<dyn BehaviorBase> {
    match character_type {
        CharacterDataType::None => Box::new(BehaviorDefault {
            ..Default::default()
        }),
        CharacterDataType::Roamer => Box::new(BehaviorRoamer {
            ..Default::default()
        }),
    }
}

pub trait BehaviorBase {
    fn initialize_behavior(&mut self, owner: &mut Character, position: &Vector3<f32>);
    fn is_enemy_in_range(&self, owner: &Character, player: &Character) -> bool;
    fn update_behavior(&mut self, owner: &mut Character, player: &Character, delta_time: f32);
    fn set_behavior(&mut self, behavior_state: BehaviorState, owner: &mut Character, player: &Character, is_force: bool);
    fn end_behavior(&mut self, owner: &mut Character, player: &Character);
}