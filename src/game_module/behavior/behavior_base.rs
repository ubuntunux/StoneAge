use nalgebra::Vector3;
use rand;
use rust_engine_3d::utilities::math::lerp;

use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::CharacterDataType;
use crate::game_module::behavior::roamer_behavior::RoamerBehavior;
use crate::game_module::game_constants::{
    NPC_ATTACK_TERM_MIN,
    NPC_ATTACK_TERM_MAX,
    NPC_IDLE_TERM_MAX,
    NPC_IDLE_TERM_MIN,
    NPC_IDLE_PLAY_MIN,
    NPC_IDLE_PLAY_MAX
};

pub fn create_character_behavior(character_type: CharacterDataType) -> Box<dyn BehaviorBase> {
    match character_type {
        CharacterDataType::Roamer => Box::new(RoamerBehavior {
            ..Default::default()
        }),
    }
}

pub trait BehaviorBase {
    fn initialize_behavior<'a>(&mut self, character: &mut Character, position: &Vector3<f32>);
    fn update_behavior<'a>(&mut self, character: &mut Character, player: &Character<'a>, delta_time: f32);
    fn generate_idle_time(&self) -> f32 {
        lerp(NPC_IDLE_TERM_MIN, NPC_IDLE_TERM_MAX, rand::random::<f32>())
    }

    fn generate_idle_play_time(&self) -> f32 {
        lerp(NPC_IDLE_PLAY_MIN, NPC_IDLE_PLAY_MAX, rand::random::<f32>())
    }

    fn generate_attack_time(&self) -> f32 {
        lerp(NPC_ATTACK_TERM_MIN, NPC_ATTACK_TERM_MAX, rand::random::<f32>())
    }
}