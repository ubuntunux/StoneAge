use crate::game_module::actors::character::Character;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    AUDIO_HIT, EFFECT_HIT, HIT_BLINK_INTENSITY, HIT_BLINK_SPEED, HIT_BLINK_TIME, OBJECT_SHAKE_INTENSITY,
    OBJECT_SHAKE_SPEED_X, OBJECT_SHAKE_SPEED_Z,
};
use nalgebra::{Vector3, Vector4};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager_mut};
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::renderer::push_constants::PushConstantParameter;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use std::ffi::c_void;
use uuid::Uuid;

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

pub fn resolve_actor_id(configured_id: Uuid, parsed_uuid: Option<Uuid>) -> Uuid {
    if configured_id.is_nil() {
        parsed_uuid.unwrap_or_else(Uuid::new_v4)
    } else {
        if let Some(uuid) = parsed_uuid {
            debug_assert_eq!(configured_id, uuid);
        }
        configured_id
    }
}

pub fn calculate_hit_shake_position(base_position: &Vector3<f32>, hit_blink_time: f32) -> Vector3<f32> {
    if 0.0 < hit_blink_time {
        let shake_ratio = hit_blink_time / HIT_BLINK_TIME;
        let shake_amount = shake_ratio * OBJECT_SHAKE_INTENSITY;
        let shake_x = (hit_blink_time * OBJECT_SHAKE_SPEED_X).sin() * shake_amount;
        let shake_z = (hit_blink_time * OBJECT_SHAKE_SPEED_Z).cos() * shake_amount;
        base_position + Vector3::new(shake_x, 0.0, shake_z)
    } else {
        *base_position
    }
}

pub fn update_actor_hit_blink_color(
    render_object: &RcRefCell<RenderObjectData>,
    hit_blink_time: &mut f32,
    delta_time: f32,
) {
    *hit_blink_time = (*hit_blink_time - delta_time).max(0.0);

    let blink_phase = 0.5 - (*hit_blink_time * HIT_BLINK_SPEED).cos() * 0.5;
    let hit_color = Vector4::new(
        1.0 + blink_phase * HIT_BLINK_INTENSITY,
        1.0 + blink_phase * HIT_BLINK_INTENSITY,
        1.0 + blink_phase * HIT_BLINK_INTENSITY,
        1.0,
    );

    render_object.borrow_mut().set_push_constant_parameter("_color", &PushConstantParameter::Float4(hit_color));
}

pub fn play_hit_effect_and_audio(effect_position: &Vector3<f32>) {
    let effect_create_info = EffectCreateInfo {
        _effect_position: *effect_position,
        _effect_data_name: String::from(EFFECT_HIT),
        ..Default::default()
    };
    get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
    get_audio_manager_mut().play_audio_bank(AUDIO_HIT, AudioLoop::ONCE, None);
}
