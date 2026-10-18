use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use nalgebra::Vector3;
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_ref};
use std::ffi::c_void;

pub const INTERACTION_OBJECT_UI_OFFSET_Y: f32 = 1.0;
const INTERACTION_KEY_TAG_TAMING: usize = 1;
const INTERACTION_KEY_TAG_FARMING: usize = 2;

#[derive(Clone)]
pub enum InteractionObject<'a> {
    None,
    PropBed(RcRefCell<Prop<'a>>),
    PropPickup(RcRefCell<Prop<'a>>),
    PropGate(RcRefCell<Prop<'a>>),
    PropGathering(RcRefCell<Prop<'a>>),
    PropMonolith(RcRefCell<Prop<'a>>),
    PropTable(RcRefCell<Prop<'a>>),
    PropDeployed(RcRefCell<Prop<'a>>),
    Npc(RcRefCell<Character<'a>>),
    Taming(RcRefCell<Character<'a>>),
    Farming(RcRefCell<Character<'a>>),
}

impl<'a> InteractionObject<'a> {
    pub fn get_key(&self) -> *const c_void {
        match self {
            InteractionObject::None => std::ptr::null(),
            InteractionObject::PropBed(prop)
            | InteractionObject::PropPickup(prop)
            | InteractionObject::PropGate(prop)
            | InteractionObject::PropGathering(prop)
            | InteractionObject::PropMonolith(prop)
            | InteractionObject::PropTable(prop)
            | InteractionObject::PropDeployed(prop) => prop.as_ptr() as *const c_void,
            InteractionObject::Npc(character) => character.as_ptr() as *const c_void,
            InteractionObject::Taming(character) => {
                ((character.as_ptr() as usize) | INTERACTION_KEY_TAG_TAMING) as *const c_void
            }
            InteractionObject::Farming(character) => {
                ((character.as_ptr() as usize) | INTERACTION_KEY_TAG_FARMING) as *const c_void
            }
        }
    }

    pub fn get_character(&self) -> Option<&RcRefCell<Character<'a>>> {
        match self {
            InteractionObject::Npc(character)
            | InteractionObject::Taming(character)
            | InteractionObject::Farming(character) => Some(character),
            _ => None,
        }
    }

    fn position_from_bounding_box(bounding_box: &BoundingBox) -> Vector3<f32> {
        Vector3::new(
            bounding_box._center.x,
            bounding_box._min.y + INTERACTION_OBJECT_UI_OFFSET_Y,
            bounding_box._center.z,
        )
    }

    pub fn get_position(&self) -> Vector3<f32> {
        match self {
            InteractionObject::None => Vector3::zeros(),
            InteractionObject::PropBed(prop)
            | InteractionObject::PropPickup(prop)
            | InteractionObject::PropGate(prop)
            | InteractionObject::PropGathering(prop)
            | InteractionObject::PropMonolith(prop)
            | InteractionObject::PropTable(prop)
            | InteractionObject::PropDeployed(prop) => {
                Self::position_from_bounding_box(ptr_as_ref(prop.as_ptr()).get_bounding_box())
            }
            InteractionObject::Npc(character)
            | InteractionObject::Taming(character)
            | InteractionObject::Farming(character) => {
                Self::position_from_bounding_box(ptr_as_ref(character.as_ptr()).get_bounding_box())
            }
        }
    }
}
