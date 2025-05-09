use nalgebra::Vector3;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use crate::game_module::behavior::behavior_base::BehaviorBase;
use crate::game_module::actors::character_controller::CharacterController;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterData, MoveAnimationState};
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::actors::weapons::Weapon;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Default)]
pub struct CharacterStats {
    pub _is_alive: bool,
    pub _hp: i32,
    pub _stamina: f32,
    pub _invincibility: bool
}

#[derive(Default)]
pub struct CharacterAnimationState {
    pub _move_animation_state: MoveAnimationState,
    pub _move_animation_state_prev: MoveAnimationState,
    pub _attack_event: ActionAnimationState,
    pub _action_animation_state: ActionAnimationState,
    pub _action_animation_state_prev: ActionAnimationState
}

pub struct Character<'a> {
    pub _character_manager: *const CharacterManager<'a>,
    pub _character_id: u64,
    pub _is_player: bool,
    pub _character_data_name: String,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _character_stats: Box<CharacterStats>,
    pub _controller: Box<CharacterController>,
    pub _behavior: Box<dyn BehaviorBase>,
    pub _animation_state: Box<CharacterAnimationState>,
    pub _weapon: Option<Box<Weapon<'a>>>
}