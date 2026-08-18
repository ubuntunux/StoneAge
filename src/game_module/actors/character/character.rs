use crate::game_module::actors::character::controller::CharacterController;
use crate::game_module::actors::character::data::*;
use crate::game_module::actors::character::manager::{CharacterCreateInfo, CharacterID, CharacterSaveData};
use crate::game_module::actors::character::stats::*;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::{Item, ItemCreateInfo, ItemID};
use rust_engine_3d::audio::audio_manager::AudioInstance;

use crate::game_module::actors::items::ItemDataType;
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState, create_character_behavior};
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_character_manager, get_character_manager_mut, get_game_client_mut, get_game_scene_manager,
    get_game_scene_manager_mut, get_game_ui_manager_mut, get_item_manager,
};
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::{Vector3, Vector4};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{
    get_audio_manager, get_audio_manager_mut, get_scene_manager, get_scene_manager_mut,
};
use rust_engine_3d::renderer::push_constants::PushConstantParameter;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::make_rotation_matrix;
use rust_engine_3d::utilities::system::{RcRefCell, State, format_name_with_uuid, ptr_as_mut, ptr_as_ref};
use std::ffi::c_void;
use strum::IntoEnumIterator;

pub struct Character<'a> {
    pub _character_name: String,
    pub _character_id: CharacterID,
    pub _is_player: bool,
    pub _character_data_name: String,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _character_stats: Box<CharacterStats>,
    pub _controller: Box<CharacterController<'a>>,
    pub _behavior: Box<dyn BehaviorBase<'a> + 'a>,
    pub _animation_state: Box<CharacterAnimationState>,
    pub _attached_item: Option<RcRefCell<Item<'a>>>,
    pub _attached_item_id: Option<ItemID>,
    pub _audio_snoring: Option<RcRefCell<AudioInstance>>,
    pub _fishing_state: Box<CharacterFishingState>,
    pub _dead_time: f32,
}

impl CharacterAnimationState {
    pub fn is_attack_event(&self) -> bool {
        self._action_event == ActionEvent::Attack
            || self._action_event == ActionEvent::PowerAttack
            || self._action_event == ActionEvent::Kick
    }
    pub fn is_action_event(&self, action_event: ActionEvent) -> bool {
        self._action_event == action_event
    }
    pub fn get_action_event(&self) -> ActionEvent {
        self._action_event
    }
    pub fn set_action_event(&mut self, action_event: ActionEvent) {
        self._action_event = action_event;
    }
}

impl CharacterStats {
    pub fn create_character_stats() -> CharacterStats {
        CharacterStats {
            _is_alive: true,
            _is_tamed: false,
            _is_dead_loop: false,
            _corpse_hit_count: MAX_CORPSE_HIT_COUNT,
            _hp: 100,
            _max_hp: 100,
            _max_hp_data: 100,
            _stamina_recovery_delay_time: 0.0,
            _prev_stamina: MAX_STAMINA,
            _stamina: MAX_STAMINA,
            _max_stamina: MAX_STAMINA,
            _max_stamina_data: MAX_STAMINA,
            _hunger: 0.0,
            _tired: 0.0,
            _happiness: 1.0,
            _intimacy: 0.0,
            _invincibility: false,
            _is_stat_displayed: false,
            _hit_blink_time: 0.0,
        }
    }

    pub fn initialize_character_stats(&mut self, character_data: &CharacterData) {
        self._is_alive = true;
        self._hp = character_data._stat_data._max_hp;
        self._max_hp = character_data._stat_data._max_hp;
        self._max_hp_data = character_data._stat_data._max_hp;
        self._stamina_recovery_delay_time = 0.0;
        self._prev_stamina = MAX_STAMINA;
        self._stamina = MAX_STAMINA;
        self._max_stamina = MAX_STAMINA;
        self._max_stamina_data = MAX_STAMINA;
        self._hunger = 0.0;
        self._invincibility = false;
    }
}

impl CharacterStats {
    pub fn get_hp(&self) -> i32 {
        self._hp
    }
    pub fn set_hp(&mut self, hp: i32) {
        self._hp = self._max_hp.min(0.max(hp));
    }
    pub fn add_hp(&mut self, hp: i32) {
        self.set_hp(self.get_hp() + hp);
    }
    pub fn get_max_hp(&self) -> i32 {
        self._max_hp
    }
    pub fn set_max_hp(&mut self, hp: i32) {
        self._max_hp = self._max_hp_data.min(0.max(hp));
    }
    pub fn add_max_hp(&mut self, hp: i32) {
        self.set_max_hp(self.get_max_hp() + hp);
    }
    pub fn get_max_hp_data(&self) -> i32 {
        self._max_hp_data
    }
    pub fn get_hunger_level(&self) -> f32 {
        1f32.min(((MAX_HUNGER - self._hunger) * 10.0).ceil() / 10.0)
    }
    pub fn is_hungry(&self) -> bool {
        HUNGER_WARNING_THRESHOLD <= self._hunger
    }
    pub fn get_hunger(&self) -> f32 {
        self._hunger
    }
    pub fn set_hunger(&mut self, hunger: f32) {
        self._hunger = MAX_HUNGER.min(0f32.max(hunger));
        let hunger_level = self.get_hunger_level();
        self.set_max_hp((self._max_hp_data as f32 * hunger_level).ceil() as i32);
        if self._max_hp < self._hp {
            self.set_hp(self._max_hp);
        }

        self.set_max_stamina((self._max_stamina_data * hunger_level).ceil());
        if self._max_stamina < self._stamina {
            self.set_stamina(self._max_stamina);
        }
    }
    pub fn add_hunger(&mut self, hunger: f32) {
        self.set_hunger(self.get_hunger() + hunger);
    }
    pub fn get_tired(&self) -> f32 {
        self._tired
    }
    pub fn set_tired(&mut self, tired: f32) {
        self._tired = tired;
    }
    pub fn get_happiness(&self) -> f32 {
        self._happiness
    }
    pub fn set_happiness(&mut self, happiness: f32) {
        self._happiness = happiness;
    }
    pub fn get_intimacy(&self) -> f32 {
        self._intimacy
    }
    pub fn set_intimacy(&mut self, intimacy: f32) {
        self._intimacy = intimacy;
    }
    pub fn add_intimacy(&mut self, intimacy: f32) {
        self._intimacy += intimacy;
    }

    pub fn get_stamina(&self) -> f32 {
        self._stamina
    }

    pub fn set_stamina(&mut self, stamina: f32) {
        self._stamina = self._max_stamina.min(0f32.max(stamina));
    }

    pub fn add_stamina(&mut self, stamina: f32) {
        self.set_stamina(self.get_stamina() + stamina);
    }

    pub fn get_max_stamina(&self) -> f32 {
        self._max_stamina
    }

    pub fn set_max_stamina(&mut self, stamina: f32) {
        self._max_stamina = self._max_stamina_data.min(0f32.max(stamina));
    }

    pub fn add_max_stamina(&mut self, stamina: f32) {
        self.set_max_stamina(self.get_max_stamina() + stamina);
    }

    pub fn get_max_stamina_data(&self) -> f32 {
        self._max_stamina_data
    }

    pub fn set_invincibility(&mut self, invincibility: bool) {
        self._invincibility = invincibility;
    }

    pub fn get_is_stat_displayed(&self) -> bool {
        self._is_stat_displayed
    }

    pub fn set_is_stat_displayed(&mut self, is_stat_displayed: bool) {
        self._is_stat_displayed = is_stat_displayed
    }

    pub fn update_hp<'a>(&mut self, _owner: &Character<'a>, _delta_time: f32) {
        if self._max_hp < self._hp {
            self._hp = self._max_hp;
        }
    }

    pub fn update_stamina<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        if self._max_stamina < self._stamina {
            self._stamina = self._max_stamina;
        }

        if self._prev_stamina != self._stamina {
            if self._stamina < self._prev_stamina {
                self._stamina_recovery_delay_time = STAMINA_RECOVERY_DELAY_TIME;
            }
            self._prev_stamina = self._stamina;
        }

        if owner.is_move_state(MoveAnimationState::Run) {
            self._stamina -= STAMINA_RUN * delta_time;
            if self._stamina < 0.0 {
                self._stamina = 0.0;
            }
        } else if owner.is_idle_action()
            && (owner.is_move_stop()
                || owner.is_move_state(MoveAnimationState::SitDownLoop)
                || owner.is_move_state(MoveAnimationState::Walk))
        {
            if self._stamina < 0.0 {
                self._stamina = 0.0;
            }

            if self._stamina_recovery_delay_time <= 0.0 {
                self._stamina += STAMINA_RECOVERY * delta_time;
                if self._max_stamina < self._stamina {
                    self._stamina = self._max_stamina;
                }
            } else {
                self._stamina_recovery_delay_time -= delta_time;
            }
        }
    }

    pub fn update_stat<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        if owner._is_player && self._is_alive {
            self.update_hp(owner, delta_time);
            self.update_stamina(owner, delta_time);
        }
    }

    pub fn get_character_stats_save_data(&self) -> CharacterStatsSaveData {
        CharacterStatsSaveData {
            _is_alive: self._is_alive,
            _is_tamed: self._is_tamed,
            _is_dead_loop: self._is_dead_loop,
            _corpse_hit_count: self._corpse_hit_count,
            _hp: self._hp,
            _max_hp: self._max_hp,
            _max_hp_data: self._max_hp_data,
            _stamina_recovery_delay_time: self._stamina_recovery_delay_time,
            _prev_stamina: self._prev_stamina,
            _stamina: self._stamina,
            _max_stamina: self._max_stamina,
            _max_stamina_data: self._max_stamina_data,
            _hunger: self._hunger,
            _tired: self._tired,
            _happiness: self._happiness,
            _intimacy: self._intimacy,
            _invincibility: self._invincibility,
            _is_stat_displayed: self._is_stat_displayed,
        }
    }

    pub fn load_character_stats_save_data(&mut self, save_data: &CharacterStatsSaveData) {
        self._is_alive = save_data._is_alive;
        self._is_tamed = save_data._is_tamed;
        self._is_dead_loop = save_data._is_dead_loop;
        self._corpse_hit_count = save_data._corpse_hit_count;
        self._hp = save_data._hp;
        self._max_hp = save_data._max_hp;
        self._max_hp_data = save_data._max_hp_data;
        self._stamina_recovery_delay_time = save_data._stamina_recovery_delay_time;
        self._prev_stamina = save_data._prev_stamina;
        self._stamina = save_data._stamina;
        self._max_stamina = save_data._max_stamina;
        self._max_stamina_data = save_data._max_stamina_data;
        self._hunger = save_data._hunger;
        self._tired = save_data._tired;
        self._happiness = save_data._happiness;
        self._intimacy = save_data._intimacy;
        self._invincibility = save_data._invincibility;
        self._is_stat_displayed = save_data._is_stat_displayed;
    }
}

impl<'a> Character<'a> {
    pub fn create_character_instance(
        character_name: &str,
        character_id: CharacterID,
        is_player: bool,
        character_data_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
    ) -> Character<'a> {
        let mut character = Character {
            _character_name: String::from(character_name),
            _character_id: character_id,
            _is_player: is_player,
            _character_data_name: String::from(character_data_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_stats: Box::new(CharacterStats::default()),
            _animation_state: Box::new(CharacterAnimationState::default()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _behavior: create_character_behavior(character_data.borrow()._character_type),
            _attached_item: None,
            _attached_item_id: None,
            _audio_snoring: None,
            _fishing_state: Box::new(CharacterFishingState::default()),
            _dead_time: 0.0,
        };

        character.initialize_character(position, rotation, scale);
        character
    }

    pub fn initialize_character(&mut self, position: &Vector3<f32>, rotation: &Vector3<f32>, scale: &Vector3<f32>) {
        self._character_stats.initialize_character_stats(&self._character_data.borrow());
        self._controller.initialize_controller(position, rotation, scale);
        self._behavior.initialize_behavior(position);

        self.set_move_idle();
        self.set_action_none();
        self.initialize_transform(position, rotation, scale);
    }

    pub fn initialize_transform(&mut self, position: &Vector3<f32>, rotation: &Vector3<f32>, scale: &Vector3<f32>) {
        self._controller._position = *position;
        self._controller._position.y = self
            ._controller
            ._position
            .y
            .max(get_scene_manager().get_height_map_data().get_height_bilinear(position, 0));
        self._controller._rotation = *rotation;
        self._controller._scale = *scale;
        let direction: Vector3<f32> = make_rotation_matrix(
            self._controller._rotation.x,
            self._controller._rotation.y,
            self._controller._rotation.z,
        )
        .column(2)
        .xyz();
        self._controller.set_move_direction(&direction);
        self.update_transform();
        self.update_render_object();
    }

    pub fn destroy_character(&mut self) {
        self.stop_animations(true);
        self._character_stats.set_is_stat_displayed(false);
        get_game_ui_manager_mut().remove_text_box_item((self as *const Self) as *const c_void);
    }

    pub fn get_debug_info(&self) -> String {
        let position = self.get_position();
        format!(
            "Behavior: {:?}({:.1})\nAnimation: {:?}/{:?}\nHP: {:?}/{:?}\nIs hunger({:?}): {:.1}\nHappiness: {:?}\nIntimacy: {:?}\nPosition: [{:.1}, {:.1}, {:.1}]",
            self._behavior.get_behavior_state(),
            self._behavior.get_behavior_data().get_behavior_time(),
            self._animation_state._action_animation_state,
            self._animation_state._move_animation_state,
            self._character_stats._hp,
            self._character_stats._max_hp,
            self._character_stats.is_hungry(),
            self._character_stats.get_hunger(),
            self._character_stats._happiness,
            self._character_stats._intimacy,
            position.x,
            position.y,
            position.z
        )
    }

    pub fn respawn_character(&mut self, position: &Vector3<f32>, rotation: &Vector3<f32>, scale: &Vector3<f32>) {
        self.initialize_character(position, rotation, scale);
        self.set_action_wake_up();
    }

    pub fn change_character_model(&mut self, render_object: &RcRefCell<RenderObjectData<'a>>) {
        self._render_object = render_object.clone();
        self._render_object.borrow_mut().update_render_object_data(0.0);
    }

    pub fn update_characters_save_data(&mut self, character_create_info: &CharacterCreateInfo) {
        self.initialize_transform(
            &character_create_info._position,
            &character_create_info._rotation,
            &character_create_info._scale,
        )
    }

    pub fn load_character_save_data(&mut self, character_save_data: &CharacterSaveData) {
        self.update_characters_save_data(&character_save_data._character_create_info);
        self._controller.load_controller_save_data(&character_save_data._character_controller_save_data);
        self._render_object.borrow_mut().load_render_object_save_data(&character_save_data._render_object_save_data);
        self._character_stats.load_character_stats_save_data(&character_save_data._character_stats);
        self._behavior.load_behavior_save_data(&character_save_data._behavior);
        *self._animation_state = character_save_data._animation_state.clone();
        self._attached_item_id = character_save_data._attached_item;
    }

    pub fn get_character_save_data(&self) -> (String, CharacterSaveData) {
        (
            format_name_with_uuid(self._character_name.as_str(), self.get_character_id()),
            CharacterSaveData {
                _character_create_info: CharacterCreateInfo {
                    _character_id: self.get_character_id(),
                    _character_data_name: self._character_data_name.clone(),
                    _position: *self.get_position(),
                    _rotation: *self.get_rotation(),
                    _scale: *self.get_scale(),
                },
                _character_controller_save_data: self._controller.get_controller_save_data(),
                _render_object_save_data: self._render_object.borrow().get_render_object_save_data(),
                _character_stats: self._character_stats.get_character_stats_save_data(),
                _behavior: self._behavior.get_behavior_save_data(),
                _animation_state: *self._animation_state.clone(),
                _attached_item: self._attached_item.as_ref().map(|item| item.borrow().get_item_id()),
            },
        )
    }

    pub fn post_process_after_character_loading(&mut self) {
        if let Some(item_id) = self._attached_item_id.take()
            && let Some(item) = get_item_manager().get_item(item_id)
        {
            self.attach_item(item.clone());
        }

        self.post_process_restore_animation();

        if !self._is_player && self.is_alive() {
            let maybe_player = get_character_manager().get_maybe_player();
            let player_ref = maybe_player.as_ref().map(|p| p.borrow());
            let target = player_ref.as_deref();
            self._behavior.update_behavior(ptr_as_mut(self), target, 0.0);
        }
    }

    pub fn post_process_restore_animation(&mut self) {
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());

        // Restore move animation mesh
        let move_state = self._animation_state._move_animation_state;
        let next_move_speed = self._animation_state._next_move_animation_speed;

        let (move_mesh, move_speed, move_loop) = match move_state {
            MoveAnimationState::None | MoveAnimationState::Idle => (
                Some(&animation_data._idle_animation),
                animation_data._idle_animation_speed * next_move_speed,
                true,
            ),
            MoveAnimationState::Walk => (
                Some(&animation_data._walk_animation),
                animation_data._walk_animation_speed * next_move_speed,
                true,
            ),
            MoveAnimationState::Run => (
                Some(&animation_data._run_animation),
                animation_data._run_animation_speed * next_move_speed,
                true,
            ),
            MoveAnimationState::Jump => (
                Some(&animation_data._jump_animation),
                animation_data._jump_animation_speed * next_move_speed,
                false,
            ),
            MoveAnimationState::Roll => (
                Some(&animation_data._roll_animation),
                animation_data._roll_animation_speed * next_move_speed,
                false,
            ),
            MoveAnimationState::RunningJump => (
                Some(&animation_data._running_jump_animation),
                animation_data._running_jump_animation_speed * next_move_speed,
                false,
            ),
            MoveAnimationState::SitDownLoop => (Some(&animation_data._sit_down_loop_animation), next_move_speed, true),
        };

        if let Some(mesh) = move_mesh {
            let animation_info = AnimationPlayArgs {
                _animation_speed: move_speed,
                _animation_loop: move_loop,
                _reset_animation_time: false,
                _force_animation_setting: true,
                ..Default::default()
            };
            render_object.set_animation(mesh, &animation_info, AnimationLayer::BaseLayer);
        } else {
            render_object.set_animation_none(AnimationLayer::BaseLayer);
        }

        // Restore action animation mesh
        let action_state = self._animation_state._action_animation_state;
        let next_action_speed = self._animation_state._next_action_animation_speed;

        let (action_mesh, action_speed, action_loop) = match action_state {
            ActionAnimationState::None => (None, 1.0, false),
            ActionAnimationState::Attack => (
                Some(&animation_data._attack_animation),
                animation_data._attack_animation_speed * next_action_speed,
                false,
            ),
            ActionAnimationState::Dance => (Some(&animation_data._dance_animation), next_action_speed, true),
            ActionAnimationState::Dead => (
                Some(&animation_data._dead_animation),
                animation_data._dead_animation_speed * next_action_speed,
                false,
            ),
            ActionAnimationState::Eating => (Some(&animation_data._eating_animation), next_action_speed, false),
            ActionAnimationState::Hit => (
                Some(&animation_data._hit_animation),
                animation_data._hit_animation_speed * next_action_speed,
                false,
            ),
            ActionAnimationState::Hungry => (Some(&animation_data._hungry_animation), next_action_speed, true),
            ActionAnimationState::Kick => (
                Some(&animation_data._kick_animation),
                animation_data._kick_animation_speed * next_action_speed,
                false,
            ),
            ActionAnimationState::LayingDown => {
                (Some(&animation_data._laying_down_animation), next_action_speed, false)
            }
            ActionAnimationState::Pickup => (Some(&animation_data._pickup_animation), next_action_speed, false),
            ActionAnimationState::PowerAttack => (
                Some(&animation_data._power_attack_animation),
                animation_data._power_attack_animation_speed * next_action_speed,
                false,
            ),
            ActionAnimationState::Sleep | ActionAnimationState::SleepNoSnoring => {
                (Some(&animation_data._sleep_animation), next_action_speed, true)
            }
            ActionAnimationState::WakeUp => (Some(&animation_data._wake_up_animation), next_action_speed, false),
            ActionAnimationState::FishingBegin => {
                (Some(&animation_data._fishing_begin_animation), next_action_speed, false)
            }
            ActionAnimationState::FishingLoop => {
                (Some(&animation_data._fishing_loop_animation), next_action_speed, true)
            }
            ActionAnimationState::FishingEnd => {
                (Some(&animation_data._fishing_end_animation), next_action_speed, false)
            }
        };

        if let Some(mesh) = action_mesh {
            let animation_info = AnimationPlayArgs {
                _animation_speed: action_speed,
                _animation_loop: action_loop,
                _reset_animation_time: false,
                _force_animation_setting: true,
                ..Default::default()
            };
            render_object.set_animation(mesh, &animation_info, AnimationLayer::ActionLayer);
        } else {
            render_object.set_animation_none(AnimationLayer::ActionLayer);
        }

        self.update_animation_layers();
    }

    pub fn attach_item(&mut self, attach_item: RcRefCell<Item<'a>>) {
        self._attached_item = Some(attach_item);
    }

    pub fn get_attached_item(&self) -> &Option<RcRefCell<Item<'a>>> {
        &self._attached_item
    }

    pub fn get_attached_item_data_type(&self) -> ItemDataType {
        if let Some(attached_item) = self._attached_item.as_ref() {
            return attached_item.borrow().get_item_data_type();
        }
        ItemDataType::None
    }

    pub fn detach_item(&mut self) {
        self._attached_item = None;
    }

    pub fn set_weapon_visible(&self, visible: bool) {
        if let Some(attached_item) = self._attached_item.as_ref() {
            attached_item.borrow()._render_object.borrow_mut().set_visible(visible);
        }
    }

    pub fn get_character_name(&self) -> &String {
        &self._character_name
    }

    pub fn get_character_id(&self) -> CharacterID {
        self._character_id
    }

    pub fn get_character_data(&self) -> &CharacterData {
        ptr_as_ref(self._character_data.as_ptr())
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &ptr_as_ref(self._render_object.as_ptr())._bounding_box
    }

    pub fn get_entity_id(&self) -> rust_engine_3d::ecs::EntityId {
        ptr_as_ref(self._render_object.as_ptr()).get_entity_id()
    }

    pub fn get_transform(&self) -> &TransformObjectData {
        ptr_as_ref(self._render_object.as_ptr()).get_transform_object_data()
    }

    pub fn get_collision(&self) -> &CollisionData {
        &ptr_as_ref(self._render_object.as_ptr())._collision
    }

    pub fn get_stats(&self) -> &CharacterStats {
        self._character_stats.as_ref()
    }

    pub fn get_stats_mut(&mut self) -> &mut CharacterStats {
        self._character_stats.as_mut()
    }

    pub fn is_player(&self) -> bool {
        self._is_player
    }

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._animation_state._move_animation_state
    }

    pub fn is_move_stop(&self) -> bool {
        self.is_move_state(MoveAnimationState::None)
            || self.is_move_state(MoveAnimationState::Idle)
            || self.is_move_state(MoveAnimationState::SitDownLoop)
    }

    pub fn is_alive(&self) -> bool {
        self._character_stats._is_alive
    }

    pub fn is_tamed(&self) -> bool {
        self._character_stats._is_tamed
    }

    pub fn is_civilian(&self) -> bool {
        self._character_data.borrow()._character_type == CharacterDataType::Civilian
    }

    pub fn is_corpse(&self) -> bool {
        !self.is_alive() && !self.is_tamed() && !self.is_civilian() && self._character_stats._is_dead_loop
    }

    pub fn get_intimacy(&self) -> f32 {
        self._character_stats.get_intimacy()
    }

    pub fn set_intimacy(&mut self, intimacy: f32) {
        self._character_stats.set_intimacy(intimacy);
    }

    pub fn add_intimacy(&mut self, intimacy: f32) {
        self._character_stats.add_intimacy(intimacy);
    }

    pub fn is_following_intimacy(&self) -> bool {
        self.is_alive() && self._character_stats.get_intimacy() >= INTIMACY_FOLLOW_THRESHOLD
    }

    pub fn set_tamed(&mut self, is_tamed: bool) {
        self._character_stats._is_tamed = is_tamed;
    }

    pub fn get_corpse_hit_count(&self) -> i32 {
        self._character_stats._corpse_hit_count
    }

    pub fn tame(&mut self) {
        self._character_stats._is_tamed = true;
        self._character_stats._is_alive = true;
        self._character_stats._corpse_hit_count = MAX_CORPSE_HIT_COUNT;
        let max_hp = self._character_stats.get_max_hp();
        self._character_stats.set_hp(max_hp);
        self.set_next_behavior(BehaviorState::WakeUp, true);
    }

    pub fn is_on_ground(&self) -> bool {
        self._controller.is_on_ground()
    }

    pub fn is_falling(&self) -> bool {
        self._controller.is_falling()
    }

    pub fn is_in_roll_delay(&self) -> bool {
        self._controller.is_in_roll_delay()
    }

    pub fn is_jump(&self) -> bool {
        self._controller.is_jump()
    }

    pub fn get_nearest_interaction_object(&self) -> &InteractionObject<'a> {
        self._controller.get_nearest_interaction_object()
    }
    pub fn is_in_interaction_range(&self) -> bool {
        self._controller.is_in_interaction_range()
    }
    pub fn add_interaction_object(&mut self, object: InteractionObject<'a>) {
        self._controller.add_interaction_object(object);
    }

    pub fn is_idle_action(&self) -> bool {
        self.is_action(ActionAnimationState::None) || self.is_action(ActionAnimationState::Hungry)
    }

    pub fn is_additive_animation_for_action(&self) -> bool {
        if (self.is_action(ActionAnimationState::Attack)
            || self.is_action(ActionAnimationState::PowerAttack)
            || self.is_action(ActionAnimationState::Hit)
            || self.is_action(ActionAnimationState::Eating)
            || self.is_action(ActionAnimationState::Hungry)
            || self.is_action(ActionAnimationState::Pickup))
            && (self.is_move_state(MoveAnimationState::Jump)
                || self.is_move_state(MoveAnimationState::Run)
                || self.is_move_state(MoveAnimationState::RunningJump)
                || self.is_move_state(MoveAnimationState::SitDownLoop)
                || self.is_move_state(MoveAnimationState::Walk))
        {
            return true;
        }
        false
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._animation_state._action_animation_state
    }

    pub fn is_attack_animation(&self) -> bool {
        self.is_action(ActionAnimationState::Attack)
            || self.is_action(ActionAnimationState::PowerAttack)
            || self.is_action(ActionAnimationState::Kick)
    }

    pub fn is_available_attack(&self) -> bool {
        let action_animation_play_info = self.get_animation_play_info(AnimationLayer::ActionLayer);
        if self.is_available_move() {
            if self.is_idle_action() || self.is_action(ActionAnimationState::Hit) {
                return true;
            } else if self.is_action(ActionAnimationState::Attack) {
                let attackable_time = self.get_character_data()._stat_data._attack_event_time + ATTACK_DELAY;
                return attackable_time < action_animation_play_info._animation_play_time;
            }
        } else {
            if self.is_action(ActionAnimationState::Kick) {
                let attackable_time = self.get_character_data()._stat_data._kick_event_time + KICK_DELAY;
                return attackable_time < action_animation_play_info._animation_play_time;
            }
        }
        false
    }

    pub fn is_available_move(&self) -> bool {
        self.is_alive()
            && !self.is_move_state(MoveAnimationState::Roll)
            && (!self.is_on_ground() || !self.is_action(ActionAnimationState::Kick))
            && !self.is_action(ActionAnimationState::LayingDown)
            && !self.is_action(ActionAnimationState::Sleep)
            && !self.is_action(ActionAnimationState::SleepNoSnoring)
            && !self.is_action(ActionAnimationState::WakeUp)
            && !self.is_action(ActionAnimationState::FishingBegin)
            && !self.is_action(ActionAnimationState::FishingLoop)
            && !self.is_action(ActionAnimationState::FishingEnd)
    }

    pub fn is_available_jump(&self) -> bool {
        !self.is_jump() && !self.is_falling() && self.is_available_move()
    }

    pub fn is_available_roll(&self) -> bool {
        if self._is_player && (self._character_stats._stamina < STAMINA_ROLL || self.is_in_roll_delay()) {
            return false;
        }
        !self.is_falling() && self.is_available_attack() && !self.is_move_state(MoveAnimationState::Roll)
    }

    pub fn is_speed_running(&self) -> bool {
        self.is_move_state(MoveAnimationState::Run) || self.is_move_state(MoveAnimationState::RunningJump)
    }

    pub fn get_animation_play_info(&self, layer: AnimationLayer) -> &AnimationPlayInfo {
        &ptr_as_ref(self._render_object.as_ptr())._animation_play_infos[layer as usize]
    }

    pub fn get_attack_range(&self, attack_event: ActionAnimationState) -> f32 {
        match attack_event {
            ActionAnimationState::Attack => self.get_character_data()._stat_data._attack_range,
            ActionAnimationState::PowerAttack => self.get_character_data()._stat_data._power_attack_range,
            ActionAnimationState::Kick => self.get_character_data()._stat_data._kick_range,
            _ => panic!("check_attack_range not implemented: {:?}", attack_event),
        }
    }

    pub fn check_in_range(&self, target_collision: &CollisionData, check_range: f32, check_direction: bool) -> bool {
        let collision = self.get_collision();
        let height_diff = (target_collision._bounding_box._min.y - collision._bounding_box._min.y).abs();
        if collision._bounding_box._extents.y < height_diff {
            return false;
        }

        let to_target = target_collision._bounding_box._center - collision._bounding_box._center;
        let (to_target_dir, distance) = math::make_normalize_xz_with_norm(&to_target);
        let d0 = collision._bounding_box._orientation.column(0).dot(&to_target_dir).abs();
        let r0 = math::lerp(
            collision._bounding_box._extents.z,
            collision._bounding_box._extents.x,
            d0,
        );
        let d1 = target_collision._bounding_box._orientation.column(0).dot(&to_target_dir).abs();
        let r1 = math::lerp(
            target_collision._bounding_box._extents.z,
            target_collision._bounding_box._extents.x,
            d1,
        );
        distance <= (r0 + check_range + r1)
            && (!check_direction || self.get_transform().get_front().dot(&to_target_dir) < 0.0)
    }

    pub fn check_in_range_xy(&self, target_collision: &CollisionData, check_range: f32, check_direction: bool) -> bool {
        let collision = self.get_collision();
        let height_diff = (target_collision._bounding_box._min.y - collision._bounding_box._min.y).abs();
        if collision._bounding_box._extents.y < height_diff {
            return false;
        }

        let to_target = target_collision._bounding_box._center - collision._bounding_box._center;
        let to_target = Vector3::new(to_target.x, 0.0, 0.0);
        let (to_target_dir, distance) = math::make_normalize_xz_with_norm(&to_target);
        let d0 = collision._bounding_box._orientation.column(0).dot(&to_target_dir).abs();
        let r0 = math::lerp(
            collision._bounding_box._extents.z,
            collision._bounding_box._extents.x,
            d0,
        );
        let d1 = target_collision._bounding_box._orientation.column(0).dot(&to_target_dir).abs();
        let r1 = math::lerp(
            target_collision._bounding_box._extents.z,
            target_collision._bounding_box._extents.x,
            d1,
        );
        distance <= (r0 + check_range + r1)
            && (!check_direction || self.get_transform().get_front().dot(&to_target_dir) < 0.0)
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        self._controller.get_rotation()
    }
    pub fn get_face_direction(&self) -> &Vector3<f32> {
        self._controller.get_face_direction()
    }
    pub fn get_scale(&self) -> &Vector3<f32> {
        self._controller.get_scale()
    }
    pub fn look_at(&mut self, target_position: &Vector3<f32>) {
        let direction = math::make_normalize_xz(&(target_position - self.get_position()));
        self._controller.set_move_direction(&direction);
        if !self.is_move_stop() {
            self.set_move_idle();
        }
    }

    pub fn get_prev_position(&self) -> &Vector3<f32> {
        &self._controller._prev_position
    }
    pub fn get_position(&self) -> &Vector3<f32> {
        &self._controller._position
    }
    pub fn get_velocity(&self) -> &Vector3<f32> {
        &self._controller._velocity
    }
    pub fn get_final_velocity(&self) -> &Vector3<f32> {
        &self._controller._final_velocity
    }
    pub fn get_center(&self) -> &Vector3<f32> {
        self.get_bounding_box().get_center()
    }

    pub fn check_arrival_with_radius(&self, target_position: &Vector3<f32>, radius: f32, ignore_y_axis: bool) -> bool {
        self._controller.check_arrival_with_radius(target_position, radius, ignore_y_axis)
    }

    pub fn get_power(&self, attack_event: ActionEvent) -> i32 {
        match attack_event {
            ActionEvent::Attack => self.get_character_data()._stat_data._attack_damage,
            ActionEvent::PowerAttack => self.get_character_data()._stat_data._power_attack_damage,
            ActionEvent::Kick => self.get_character_data()._stat_data._kick_damage,
            _ => panic!("get_power not implemented: {:?}", attack_event),
        }
    }

    pub fn set_damage(&mut self, damage: i32) {
        if 0 < damage {
            if self.is_alive() {
                let hp = self._character_stats.get_hp() - damage;
                self._character_stats.set_hp(hp);
                if hp <= 0 {
                    get_audio_manager_mut().play_audio_resource_data(
                        &self._character_data.borrow()._audio_data._audio_dead,
                        AudioLoop::ONCE,
                        None,
                    );
                    self.set_dead();
                } else {
                    get_audio_manager_mut().play_audio_resource_data(
                        &self._character_data.borrow()._audio_data._audio_pain,
                        AudioLoop::ONCE,
                        None,
                    );
                    if self._is_player && !self.is_move_state(MoveAnimationState::Roll) {
                        self.set_action_hit();
                    }
                }
            } else if !self.is_tamed() && 0 < self._character_stats._corpse_hit_count {
                self._character_stats._corpse_hit_count -= 1;
            }
        }
    }

    pub fn check_falling_in_water_damage(&mut self) -> bool {
        let dead_zone_height = get_scene_manager().get_dead_zone_height();
        if self.get_position().y <= dead_zone_height {
            self.set_damage(self._character_stats.get_hp());

            let effect_create_info = EffectCreateInfo {
                _effect_position: Vector3::new(self.get_position().x, dead_zone_height, self.get_position().z),
                _effect_data_name: String::from(EFFECT_FALLING_WATER),
                ..Default::default()
            };
            get_scene_manager_mut().add_effect(EFFECT_FALLING_WATER, &effect_create_info);
            get_audio_manager_mut().play_audio_bank(AUDIO_FALLING_WATER, AudioLoop::ONCE, None);
            return true;
        }
        false
    }

    pub fn check_falling_on_ground_damage(&mut self, falling_height: f32) {
        let falling_height = falling_height - self.get_position().y;
        if FALLING_HEIGHT < falling_height {
            let falling_damage: i32 = (falling_height - FALLING_HEIGHT).ceil() as i32 * FALLING_DAMAGE_RATIO;
            self.set_hit_damage(falling_damage, None);
        }
    }

    pub fn set_hit_damage(&mut self, damage: i32, attack_dir: Option<&Vector3<f32>>) {
        if 0 < damage {
            self.set_damage(damage);

            if self.is_alive()
                && let Some(attack_dir) = attack_dir
            {
                self._controller.set_hit_direction(attack_dir);
            }

            self._character_stats._hit_blink_time = HIT_BLINK_TIME;

            let effect_create_info = EffectCreateInfo {
                _effect_position: *self.get_bounding_box().get_center(),
                _effect_data_name: String::from(EFFECT_HIT),
                ..Default::default()
            };

            get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
            get_audio_manager_mut().play_audio_bank(AUDIO_HIT, AudioLoop::ONCE, None);
        }
    }

    pub fn get_hunger(&self) -> f32 {
        self._character_stats.get_hunger()
    }

    pub fn add_hunger(&mut self, hunger: f32) {
        self._character_stats.add_hunger(hunger)
    }

    pub fn set_hunger(&mut self, hunger: f32) {
        self._character_stats.set_hunger(hunger)
    }

    pub fn set_invincibility(&mut self, invincibility: bool) {
        self._character_stats._invincibility = invincibility;
    }

    pub fn set_is_dead_loop(&mut self, is_corpse: bool) {
        self._character_stats._is_dead_loop = is_corpse;
    }

    pub fn get_is_stat_displayed(&self) -> bool {
        self._character_stats._is_stat_displayed
    }

    pub fn set_is_stat_displayed(&mut self, is_stat_displayed: bool) {
        self._character_stats._is_stat_displayed = is_stat_displayed;
    }

    pub fn set_behavior_none(&mut self) {
        self.set_next_behavior(BehaviorState::None, true);
    }

    pub fn set_next_behavior(&mut self, behavior_state: BehaviorState, is_force: bool) {
        self._behavior.set_next_behavior(behavior_state, is_force);
    }

    pub fn set_dead(&mut self) {
        self._character_stats._is_alive = false;
        self._character_stats._corpse_hit_count = MAX_CORPSE_HIT_COUNT;
        self._dead_time = 0.0;
        self._character_stats.set_is_stat_displayed(false);
        get_game_ui_manager_mut().remove_text_box_item((self as *const Self) as *const c_void);
        self.set_action_dead();
        self.set_next_behavior(BehaviorState::Dead, true);
    }

    pub fn set_action_none(&mut self) {
        self.set_next_action_animation(ActionAnimationState::None, 1.0);
    }

    pub fn set_action_dance(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Dance, 1.0);
    }

    pub fn set_action_wake_up(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::WakeUp, 1.0);
    }

    pub fn set_action_laying_down(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::LayingDown, 2.0);
    }

    pub fn set_action_sleep(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Sleep, 1.0);
    }

    pub fn set_action_sleep_no_snoring(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::SleepNoSnoring, 1.0);
    }

    pub fn set_action_hungry(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Hungry, 1.0);
    }

    pub fn set_action_eating(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Eating, 1.0);
    }

    pub fn set_action_interaction(&mut self) {
        if self._controller.is_on_ground() && self.is_available_move() && self.is_idle_action() {
            let item_manager = get_game_scene_manager().get_item_manager_mut();
            let target_interaction = self
                ._controller
                ._interaction_objects
                .values()
                .find(|obj| matches!(obj, InteractionObject::Taming(_)))
                .cloned()
                .unwrap_or_else(|| self._controller._nearest_interaction_object.clone());
            match target_interaction {
                InteractionObject::PropBed(_) => {
                    self.set_move_idle();
                    get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
                }
                InteractionObject::PropPickup(_) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, 2.0);
                }
                InteractionObject::PropMonolith(_) => {
                    get_game_client_mut().set_next_game_phase(GamePhase::OpenToolbox);
                    self.set_move_idle();
                }
                InteractionObject::PropTable(prop) => {
                    self.look_at(prop.borrow().get_position());
                    if self.is_move_state(MoveAnimationState::SitDownLoop) {
                        self.set_move_idle();
                    } else {
                        self.set_sit_down();
                    }
                }
                InteractionObject::Npc(character) => {
                    // interaction
                    self.look_at(character.borrow().get_position());
                    character.borrow_mut().set_next_behavior(BehaviorState::Interaction, false);

                    // give item
                    let mut give_item = false;
                    if character.borrow().get_attached_item().is_none()
                        && let Some(attached_item) = self.get_attached_item()
                        && attached_item.borrow().get_item_data_type().is_eatable()
                    {
                        give_item = true;
                        let item_data_name = attached_item.borrow()._item_data_name.clone();
                        item_manager.remove_inventory_item(item_data_name.as_str(), 1);
                        item_manager.attach_item(&mut character.borrow_mut(), item_data_name.as_str());
                    }

                    // increase intimacy (2x multiplier if fed food)
                    let intimacy_add = if give_item {
                        INTIMACY_INTERACTION_ADD * INTIMACY_FEEDING_MULTIPLIER
                    } else {
                        INTIMACY_INTERACTION_ADD
                    };
                    character.borrow_mut().add_intimacy(intimacy_add);

                    if !give_item {
                        character.borrow_mut().set_is_stat_displayed(true);
                    }
                }
                InteractionObject::Taming(character) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, 2.0);
                    let target_position = *character.borrow().get_position();

                    let item_create_info = ItemCreateInfo {
                        _item_data_name: String::from(ITEM_ENERGY_BALL),
                        _position: target_position,
                        _velocity: Vector3::new(0.0, 3.0, 0.0),
                        _pickup_delay: 0.5,
                        ..Default::default()
                    };
                    item_manager.create_item(item_create_info._item_data_name.as_str(), &item_create_info, None);

                    character.borrow_mut().tame();

                    self._controller.remove_interaction_object(InteractionObject::Taming(character.clone()));
                    self._controller.remove_interaction_object(InteractionObject::Farming(character.clone()));
                }
                InteractionObject::Farming(character) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, 2.0);
                    let face_dir = self.get_face_direction();
                    let is_destroyed = {
                        let mut corpse = character.borrow_mut();
                        corpse.set_hit_damage(1, Some(face_dir));
                        corpse.get_corpse_hit_count() <= 0
                    };
                    if is_destroyed {
                        get_character_manager_mut().farm_character(&character);
                    }
                }
                _ => {}
            }
        }
    }
    pub fn callback_changed_interaction_object(&mut self) {
        if let InteractionObject::PropGate(_) = self._controller._nearest_interaction_object.clone() {
            get_game_client_mut().set_next_game_phase(GamePhase::WorldMapOpen);
            self.set_move_idle();
        }
    }
    pub fn set_action_attack(&mut self) {
        let mut animation_speed: f32 = 1.0;
        if self._is_player {
            if self._character_stats._stamina < STAMINA_ATTACK {
                get_game_ui_manager_mut().trigger_stamina_warning();
                animation_speed = ANIMATION_SPEED_BY_STAMINA;
            }

            self._character_stats._stamina -= STAMINA_ATTACK;
            if self._character_stats._stamina < 0.0 {
                self._character_stats._stamina = 0.0;
            }
        }
        self.set_next_action_animation(ActionAnimationState::Attack, animation_speed);
    }

    pub fn set_action_power_attack(&mut self) {
        let mut animation_speed: f32 = 1.0;
        if self._is_player {
            if self._character_stats._stamina < STAMINA_POWER_ATTACK {
                get_game_ui_manager_mut().trigger_stamina_warning();
                animation_speed = ANIMATION_SPEED_BY_STAMINA;
            }

            self._character_stats._stamina -= STAMINA_POWER_ATTACK;
            if self._character_stats._stamina < 0.0 {
                self._character_stats._stamina = 0.0;
            }
        }
        self.set_next_action_animation(ActionAnimationState::PowerAttack, animation_speed);
    }

    pub fn set_action_kick(&mut self) {
        if self.is_available_attack() {
            let mut animation_speed: f32 = 1.0;
            if self._is_player {
                if self._character_stats._stamina < STAMINA_ATTACK {
                    get_game_ui_manager_mut().trigger_stamina_warning();
                    animation_speed = ANIMATION_SPEED_BY_STAMINA;
                }

                self._character_stats._stamina -= STAMINA_ATTACK;
                if self._character_stats._stamina < 0.0 {
                    self._character_stats._stamina = 0.0;
                }
            }
            self.set_move_idle();
            self.set_next_action_animation(ActionAnimationState::Kick, animation_speed);
        }
    }

    pub fn set_action_hit(&mut self) {
        self.set_next_action_animation(ActionAnimationState::Hit, 1.0);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_idle();
        self.set_next_action_animation(ActionAnimationState::Dead, 1.0);
    }

    pub fn set_next_move_animation(&mut self, move_animation_state: MoveAnimationState, animation_speed: f32) {
        self._animation_state._next_move_animation_state = move_animation_state;
        self._animation_state._next_move_animation_speed = animation_speed;
    }

    pub fn set_next_action_animation(&mut self, action_animation_state: ActionAnimationState, animation_speed: f32) {
        self._animation_state._next_action_animation_state = Some(action_animation_state);
        self._animation_state._next_action_animation_speed = animation_speed;
    }

    pub fn set_run(&mut self, run: bool) {
        self._controller.set_run(run);
    }

    pub fn toggle_run(&mut self) {
        if self._is_player && self._character_stats._stamina < STAMINA_RUN {
            get_game_ui_manager_mut().trigger_stamina_warning();
        }
        if self.is_move_state(MoveAnimationState::Run) || self.is_move_state(MoveAnimationState::Walk) {
            self._controller.toggle_run();
        }
    }

    pub fn set_move_idle(&mut self) {
        self.set_run(false);
        self.set_move_speed(0.0);
        if !self.is_move_state(MoveAnimationState::Idle) {
            self.set_next_move_animation(MoveAnimationState::Idle, 1.0);
        }
    }

    pub fn set_move_control_stop(&mut self) {
        if !self.is_move_state(MoveAnimationState::Roll) {
            self.set_run(false);
            self.set_move_speed(0.0);
            if !self.is_move_stop() && self.is_on_ground() {
                self.set_next_move_animation(MoveAnimationState::Idle, 1.0);
            }
        }
    }

    pub fn set_move_control_sit_down(&mut self) {
        if self.is_idle_action() && self.is_move_state(MoveAnimationState::Idle) && self.is_on_ground() {
            self.set_sit_down();
        }
    }

    pub fn set_sit_down(&mut self) {
        self.set_run(false);
        self.set_move_speed(0.0);
        if !self.is_move_state(MoveAnimationState::SitDownLoop) {
            self.set_next_move_animation(MoveAnimationState::SitDownLoop, 1.0);
        }
    }

    pub fn stop_animations(&mut self, apply_immediately: bool) {
        self.set_next_action_animation(ActionAnimationState::None, 1.0);
        self.set_next_move_animation(MoveAnimationState::None, 1.0);
        self.set_run(false);
        self.set_move_speed(0.0);
        if apply_immediately {
            self.update_action_keyframe_event(0.0);
            self.update_move_keyframe_event();
        }
    }

    pub fn set_position_xy(&mut self, position: &Vector3<f32>) {
        self._controller.set_position_xy(position);
    }

    pub fn set_position(&mut self, position: &Vector3<f32>) {
        self._controller.set_position(position);
    }

    pub fn set_on_ground(&mut self, ground_height: f32, ground_normal: &Vector3<f32>) {
        self._controller.set_on_ground(ground_height, ground_normal);
    }
    pub fn get_move_speed(&self) -> f32 {
        self._controller.get_move_speed()
    }
    pub fn set_move_speed(&mut self, speed: f32) {
        self._controller.set_move_speed(speed);
    }
    pub fn get_move_direction(&self) -> &Vector3<f32> {
        self._controller.get_move_direction()
    }

    pub fn set_move_direction(&mut self, move_direction: &Vector3<f32>, force_update: bool) {
        if self.is_available_move() || force_update {
            self._controller.set_move_direction(move_direction);
        }
    }

    pub fn set_move(&mut self, move_direction: &Vector3<f32>) {
        if self.is_available_move() {
            if self._controller._face_direction.dot(move_direction) < 0.0 {
                self.set_run(false);
            }

            let character_data = self.get_character_data();
            let (move_animation, move_speed) = if self._controller._is_running {
                (MoveAnimationState::Run, character_data._stat_data._run_speed)
            } else {
                (MoveAnimationState::Walk, character_data._stat_data._walk_speed)
            };

            self.set_move_direction(move_direction, false);

            if GAME_VIEW_MODE != GameViewMode::GameViewMode2D || move_direction.x.abs() >= move_direction.z.abs() {
                self.set_move_speed(move_speed);
                if !self.is_move_state(move_animation) && self._controller._is_ground {
                    self.set_next_move_animation(move_animation, 1.0);
                }
            } else {
                self.set_move_control_stop();
            }
        }
    }

    pub fn move_to_target(&mut self, target_position: &Vector3<f32>, radius: f32) -> bool {
        if self.check_arrival_with_radius(target_position, radius, true) {
            self.set_position(&Vector3::new(
                target_position.x,
                self.get_position().y,
                target_position.z,
            ));
            return true;
        }
        self.set_move(&(target_position - self.get_position()));
        false
    }

    pub fn set_jump(&mut self) {
        if self._is_player && self._character_stats._stamina < STAMINA_JUMP {
            get_game_ui_manager_mut().trigger_stamina_warning();
        }
        if self.is_available_jump() {
            let mut not_enough_stamina = false;
            if self._is_player {
                self._character_stats._stamina -= STAMINA_JUMP;
                not_enough_stamina = self._character_stats._stamina < 0.0;
            }

            let move_anim = if self._controller._is_running && !not_enough_stamina {
                MoveAnimationState::RunningJump
            } else {
                MoveAnimationState::Jump
            };
            self._controller.set_jump_start();
            self.set_next_move_animation(move_anim, 1.0);
        }
    }

    pub fn set_roll(&mut self) {
        if self._is_player && self._character_stats._stamina < STAMINA_ROLL {
            get_game_ui_manager_mut().trigger_stamina_warning();
        }
        if self.is_available_roll() {
            if self._is_player {
                self._character_stats._stamina -= STAMINA_ROLL;
            }

            let character_data = self.get_character_data();
            if self.is_move_state(MoveAnimationState::Run) {
                self.set_move_speed(character_data._stat_data._run_speed);
            } else {
                self.set_move_speed(character_data._stat_data._roll_speed);
            }
            self.set_move_direction(&self._controller._face_direction.clone(), false);
            self.set_action_none();
            self.set_next_move_animation(MoveAnimationState::Roll, 1.0);
        }
    }

    pub fn update_move_keyframe_event(&mut self) {
        let current_move_animation_state = self._animation_state._move_animation_state;
        let next_move_animation_state = self._animation_state._next_move_animation_state;
        let next_move_animation_speed = self._animation_state._next_move_animation_speed;
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());

        for state in State::iter() {
            if current_move_animation_state == next_move_animation_state
                && (state == State::End || state == State::Begin)
            {
                continue;
            }

            let update_move_animation_state: MoveAnimationState = match state {
                State::End => current_move_animation_state,
                State::Begin => {
                    self._animation_state._move_animation_state = next_move_animation_state;
                    next_move_animation_state
                }
                State::Update => next_move_animation_state,
            };

            match update_move_animation_state {
                MoveAnimationState::None => {
                    if state == State::Begin {
                        render_object.set_animation_none(AnimationLayer::BaseLayer);
                    }
                }
                MoveAnimationState::Idle => {
                    if state == State::Begin {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_speed =
                            animation_data._idle_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._idle_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                    }
                }
                MoveAnimationState::Walk => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_speed =
                            animation_data._walk_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._walk_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player
                            && (animation_play_info.check_animation_event_time(0.2)
                                || animation_play_info.check_animation_event_time(0.9))
                        {
                            get_audio_manager_mut().play_audio_bank(AUDIO_FOOTSTEP, AudioLoop::ONCE, Some(0.5));
                        }
                    }
                    _ => {}
                },
                MoveAnimationState::Run => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_speed =
                            animation_data._run_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._run_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player
                            && (animation_play_info.check_animation_event_time(0.1)
                                || animation_play_info.check_animation_event_time(0.5))
                        {
                            get_audio_manager_mut().play_audio_bank(AUDIO_FOOTSTEP, AudioLoop::ONCE, Some(0.5));
                        }
                    }
                    _ => {}
                },
                MoveAnimationState::Jump => {
                    if state == State::Begin {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_loop = false;
                        animation_info._animation_speed =
                            animation_data._jump_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._jump_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                        get_audio_manager_mut().play_audio_bank(AUDIO_JUMP, AudioLoop::ONCE, None);
                    }
                }
                MoveAnimationState::Roll => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_loop = false;
                        animation_info._animation_speed =
                            animation_data._roll_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._roll_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                        self.set_invincibility(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
                        if self._is_player && animation_play_info.check_animation_event_time(0.2) {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ROLL, AudioLoop::ONCE, None);
                        } else if animation_play_info._is_animation_end {
                            self.set_move_idle();
                        }
                    }
                    State::End => {
                        self._controller.set_roll_delay();
                        self.set_invincibility(false);
                    }
                },
                MoveAnimationState::RunningJump => {
                    if state == State::Begin {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_loop = false;
                        animation_info._animation_speed =
                            animation_data._running_jump_animation_speed * next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._running_jump_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                        get_audio_manager_mut().play_audio_bank(AUDIO_JUMP, AudioLoop::ONCE, None);
                    }
                }
                MoveAnimationState::SitDownLoop => {
                    if state == State::Begin {
                        let mut animation_info = AnimationPlayArgs::default();
                        animation_info._animation_speed = next_move_animation_speed;
                        render_object.set_animation(
                            &animation_data._sit_down_loop_animation,
                            &animation_info,
                            AnimationLayer::BaseLayer,
                        );
                    }
                }
            }
        }
        self.update_animation_layers();
    }

    pub fn update_action_keyframe_event(&mut self, delta_time: f32) {
        self._animation_state.set_action_event(ActionEvent::None);

        let current_action_animation_state = self._animation_state._action_animation_state;
        let next_action_animation_state = self._animation_state._next_action_animation_state;
        self._animation_state._next_action_animation_state = None;
        let next_action_animation_speed = self._animation_state._next_action_animation_speed;
        let character_data = ptr_as_ref(self._character_data.as_ptr());
        let animation_data = &character_data._animation_data;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let item_manager = get_game_scene_manager().get_item_manager_mut();

        for state in State::iter() {
            if next_action_animation_state.is_none() && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_action_animation_state: ActionAnimationState = match state {
                State::End => current_action_animation_state,
                State::Begin => {
                    if let Some(next_action) = next_action_animation_state {
                        self._animation_state._action_animation_state = next_action;
                        next_action
                    } else {
                        current_action_animation_state
                    }
                }
                State::Update => self._animation_state._action_animation_state,
            };

            match update_action_animation_state {
                ActionAnimationState::None => {
                    if state == State::Begin {
                        render_object.set_animation_none(AnimationLayer::ActionLayer);
                    }
                }
                ActionAnimationState::Attack => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed =
                            animation_data._attack_animation_speed * next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._attack_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(character_data._stat_data._attack_event_time)
                        {
                            self._animation_state.set_action_event(ActionEvent::Attack);
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        }

                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::Dance => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: true,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._dance_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(false);
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                    _ => {}
                },
                ActionAnimationState::Dead => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.0,
                            ..Default::default()
                        };
                        animation_info._animation_speed =
                            animation_data._dead_animation_speed * next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._dead_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(false);
                        self.set_invincibility(true);
                        self.set_is_dead_loop(false);
                    }
                    State::Update => {
                        // respawn
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_invincibility(false);
                            self.set_is_dead_loop(true);

                            if self._is_player {
                                let game_scene_manager = get_game_scene_manager_mut();
                                if !game_scene_manager.is_teleport_mode() {
                                    game_scene_manager
                                        .set_teleport_spawn_point(Stages::Home.get_stage_data_name(), BED_FOR_ARU);
                                    get_game_client_mut().set_next_game_phase(GamePhase::Respawn);
                                }
                            }
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                        self.set_invincibility(false);
                        self.set_is_dead_loop(false);
                    }
                },
                ActionAnimationState::Hit => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed =
                            animation_data._hit_animation_speed * next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._hit_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::Kick => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed =
                            animation_data._kick_animation_speed * next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._kick_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(character_data._stat_data._kick_event_time) {
                            self._animation_state.set_action_event(ActionEvent::Kick);
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        } else if self.is_on_ground() {
                            self.set_move_idle();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::LayingDown => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.0,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._laying_down_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_sleep();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::Pickup => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._pickup_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info.check_animation_event_time(PICKUP_EVENT_TIME) {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                            self._animation_state.set_action_event(ActionEvent::Pickup);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::PowerAttack => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed =
                            animation_data._power_attack_animation_speed * next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._power_attack_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info
                            .check_animation_event_time(character_data._stat_data._power_attack_event_time)
                        {
                            get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                            self._animation_state.set_action_event(ActionEvent::PowerAttack);
                        }
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::Sleep | ActionAnimationState::SleepNoSnoring => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: true,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.0,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._sleep_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        if self._is_player && update_action_animation_state == ActionAnimationState::Sleep {
                            if let Some(audio_instance) = self._audio_snoring.as_ref() {
                                get_audio_manager().stop_audio_instance(audio_instance)
                            }
                            self._audio_snoring =
                                get_audio_manager_mut().play_audio_bank(AUDIO_SNORING, AudioLoop::LOOP, Some(1.0));
                        }
                        self.set_weapon_visible(false);
                    }
                    State::End => {
                        if self._is_player
                            && let Some(audio_instance) = self._audio_snoring.as_ref()
                        {
                            get_audio_manager().stop_audio_instance(audio_instance)
                        }
                        self.set_weapon_visible(true);
                    }
                    _ => {}
                },
                ActionAnimationState::Eating => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._eating_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        get_audio_manager_mut().play_audio_bank(AUDIO_EATING, AudioLoop::ONCE, None);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        if let Some(attached_item) = self.get_attached_item().clone() {
                            self.get_stats_mut().add_hunger(-1.0);
                            self.get_stats_mut().add_hp(10);
                            self.get_stats_mut().add_stamina(10.0);

                            if self._is_player {
                                item_manager.remove_inventory_item(attached_item.borrow()._item_data_name.as_str(), 1);
                            } else {
                                item_manager.detach_item(self);
                            }
                        }
                    }
                },
                ActionAnimationState::Hungry => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: true,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._hungry_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::WakeUp => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._wake_up_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(false);
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    State::End => {
                        self.set_weapon_visible(true);
                    }
                },
                ActionAnimationState::FishingBegin => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._fishing_begin_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        self.set_weapon_visible(true);
                    }
                    State::Update => {
                        let (anim_play_time, animation_length, is_anim_end) = {
                            let anim_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                            (
                                anim_info._animation_play_time,
                                anim_info.get_animation_length(),
                                anim_info._is_animation_end,
                            )
                        };

                        if self._fishing_state._is_fishing_button_held {
                            self.update_fishing(delta_time);
                            render_object.get_animation_play_info_mut(AnimationLayer::ActionLayer)._animation_speed =
                                if 0.0 < animation_length {
                                    (1.0 - (anim_play_time / (animation_length * 0.5)).min(1.0)).powf(2.0)
                                } else {
                                    1.0
                                };
                        } else {
                            render_object.get_animation_play_info_mut(AnimationLayer::ActionLayer)._animation_speed =
                                1.0;
                        }

                        if is_anim_end {
                            if self.is_fishing_spot() {
                                self.set_next_action_animation(ActionAnimationState::FishingLoop, 1.0);
                            } else {
                                self.set_action_none();
                            }
                        }
                    }
                    _ => {}
                },
                ActionAnimationState::FishingLoop => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: true,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._fishing_loop_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        self.set_weapon_visible(true);
                        self.start_fishing_minigame();
                    }
                    State::Update => {
                        self.update_fishing_minigame(delta_time);
                    }
                    _ => {}
                },
                ActionAnimationState::FishingEnd => match state {
                    State::Begin => {
                        let mut animation_info = AnimationPlayArgs {
                            _animation_loop: false,
                            _force_animation_setting: true,
                            _animation_fade_out_time: 0.1,
                            ..Default::default()
                        };
                        animation_info._animation_speed = next_action_animation_speed;
                        render_object.set_animation(
                            &animation_data._fishing_end_animation,
                            &animation_info,
                            AnimationLayer::ActionLayer,
                        );
                        get_audio_manager_mut().play_audio_bank(AUDIO_ATTACK, AudioLoop::ONCE, None);
                        self.set_weapon_visible(true);

                        if self._is_player && self._fishing_state._minigame_success == Some(true) {
                            item_manager.pick_item(ITEM_COCONUT, 1);
                        }
                    }
                    State::Update => {
                        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                        if animation_play_info._is_animation_end {
                            self.set_action_none();
                        }
                    }
                    _ => {}
                },
            }
        }
        self.update_animation_layers();
    }

    pub fn update_transform(&mut self) {
        let render_object = self._render_object.borrow();
        let transform = render_object.get_transform_object_data_mut();
        transform.set_position(&self._controller._position);
        transform.set_rotation(&self._controller._rotation);
        transform.set_scale(&self._controller._scale);
    }

    pub fn update_render_object(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object.update_render_object_data(0.0);
    }

    pub fn update_animation_layers(&self) {
        let render_object = ptr_as_mut(self._render_object.as_ptr());

        // clear
        render_object.clear_animation_layers(AnimationLayer::ActionLayer);

        // set an additive animation layer
        if self.is_additive_animation_for_action() {
            render_object.set_animation_layers(
                self._character_data.borrow()._animation_data._upper_animation_layer.as_ptr(),
                AnimationLayer::ActionLayer,
            );
        }
    }

    fn update_hit_blink_color(&mut self, delta_time: f32) {
        self._character_stats._hit_blink_time -= delta_time;
        if self._character_stats._hit_blink_time <= 0.0 {
            self._character_stats._hit_blink_time = 0.0;
        }

        let blink_phase = 0.5 - (self._character_stats._hit_blink_time * HIT_BLINK_SPEED).cos() * 0.5;
        let hit_color = Vector4::new(
            1.0 + blink_phase * HIT_BLINK_INTENSITY,
            1.0 + blink_phase * HIT_BLINK_INTENSITY,
            1.0 + blink_phase * HIT_BLINK_INTENSITY,
            1.0
        );

        self._render_object.borrow_mut().set_push_constant_parameter(
            "_color",
            &PushConstantParameter::Float4(hit_color),
        );

    }

    pub fn update_character(
        &mut self,
        scene_manager: &SceneManager<'a>,
        target: Option<&Character<'a>>,
        delta_time: f32,
    ) {
        let was_on_ground = self.is_on_ground();
        let falling_height = self._controller.get_falling_height();

        // update animation key frames
        self.update_move_keyframe_event();
        self.update_action_keyframe_event(delta_time);

        // update hit blink color
        if 0.0 < self._character_stats._hit_blink_time {
            self.update_hit_blink_color(delta_time);
        }

        // behavior
        if !self._is_player {
            self._behavior.update_behavior(ptr_as_mut(self), target, delta_time);
        }

        // update stats - stamina
        let owner = ptr_as_ref(self);
        self._character_stats.update_stat(owner, delta_time);
        if owner.is_move_state(MoveAnimationState::Run) && self._character_stats._stamina == 0.0 {
            self.set_run(false);
        }

        // controller
        self._controller.update_character_controller(
            owner,
            scene_manager,
            &self._character_data.borrow(),
            self._animation_state._move_animation_state,
            &self._render_object.borrow()._collision,
            delta_time,
        );

        if self._controller.update_interaction_objects() {
            self.callback_changed_interaction_object();
        }

        // falling water or falling on ground
        if self.is_alive() {
            if self.check_falling_in_water_damage() {
                // falling in water
            } else if !was_on_ground && self.is_on_ground() {
                self.check_falling_on_ground_damage(falling_height);
            }
        }

        // transform
        self.update_transform();
    }
}
