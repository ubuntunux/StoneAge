use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_ref};
use std::ffi::c_void;

#[derive(Clone)]
pub enum InteractionObject<'a> {
    None,
    PropBed(RcRefCell<Prop<'a>>),
    PropPickup(RcRefCell<Prop<'a>>),
    PropGate(RcRefCell<Prop<'a>>),
    PropGathering(RcRefCell<Prop<'a>>),
    PropMonolith(RcRefCell<Prop<'a>>),
    PropTable(RcRefCell<Prop<'a>>),
    Npc(RcRefCell<Character<'a>>),
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
            | InteractionObject::PropTable(prop) => prop.as_ptr() as *const c_void,
            InteractionObject::Npc(character) => character.as_ptr() as *const c_void,
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        match self {
            InteractionObject::None => Vector3::new(0.0, 0.0, 0.0),
            InteractionObject::PropBed(prop)
            | InteractionObject::PropPickup(prop)
            | InteractionObject::PropGate(prop)
            | InteractionObject::PropGathering(prop)
            | InteractionObject::PropMonolith(prop)
            | InteractionObject::PropTable(prop) => {
                let bounding_box = ptr_as_ref(prop.as_ptr()).get_bounding_box();
                Vector3::new(
                    bounding_box._center.x,
                    bounding_box._min.y + 1.0,
                    bounding_box._center.z,
                )
            }
            InteractionObject::Npc(character) => {
                let bounding_box = ptr_as_ref(character.as_ptr()).get_bounding_box();
                Vector3::new(
                    bounding_box._center.x,
                    bounding_box._min.y + 1.0,
                    bounding_box._center.z,
                )
            }
        }
    }
}
