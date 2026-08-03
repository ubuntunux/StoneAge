use super::character::Character;
use crate::game_module::actors::character::ActionEvent;
use crate::game_module::actors::character::data::{ActionAnimationState, MoveAnimationState};
use crate::game_module::actors::props::Prop;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;

#[derive(Clone)]
pub enum ActorWrapper<'a> {
    Prop(RcRefCell<Prop<'a>>),
    Character(RcRefCell<Character<'a>>),
    RenderObject(RcRefCell<RenderObjectData<'a>>),
}

impl<'a> ActorWrapper<'a> {
    pub fn get_key(&self) -> *const c_void {
        match self {
            ActorWrapper::Prop(prop) => prop.as_ptr() as *const c_void,
            ActorWrapper::Character(character) => character.as_ptr() as *const c_void,
            ActorWrapper::RenderObject(render_object) => render_object.as_ptr() as *const c_void,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct CharacterStatsSaveData {
    pub _is_alive: bool,
    pub _hp: i32,
    pub _max_hp: i32,
    pub _max_hp_data: i32,
    pub _stamina_recovery_delay_time: f32,
    pub _prev_stamina: f32,
    pub _stamina: f32,
    pub _max_stamina: f32,
    pub _max_stamina_data: f32,
    pub _hunger: f32,
    pub _tired: f32,
    pub _happiness: f32,
    pub _invincibility: bool,
    pub _is_stat_displayed: bool,
}

#[derive(Default)]
pub struct CharacterStats {
    pub _is_alive: bool,
    pub _hp: i32,
    pub _max_hp: i32,
    pub _max_hp_data: i32,
    pub _stamina_recovery_delay_time: f32,
    pub _prev_stamina: f32,
    pub _stamina: f32,
    pub _max_stamina: f32,
    pub _max_stamina_data: f32,
    pub _hunger: f32,
    pub _tired: f32,
    pub _happiness: f32,
    pub _invincibility: bool,
    pub _is_stat_displayed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct CharacterAnimationState {
    pub _move_animation_state: MoveAnimationState,
    pub _prev_move_animation_state: MoveAnimationState,
    pub _next_move_animation_state: MoveAnimationState,
    pub _next_move_animation_speed: f32,
    pub _action_event: ActionEvent,
    pub _action_animation_state: ActionAnimationState,
    pub _next_action_animation_state: Option<ActionAnimationState>,
    pub _next_action_animation_speed: f32,
}
