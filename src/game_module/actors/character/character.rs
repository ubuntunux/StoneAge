use crate::game_module::actors::character::controller::CharacterController;
use crate::game_module::actors::character::data::*;
use crate::game_module::actors::character::fishing::CharacterFishingState;
use crate::game_module::actors::character::lantern::Lantern;
use crate::game_module::actors::character::manager::{CharacterCreateInfo, CharacterID, CharacterSaveData};
use crate::game_module::actors::character::stats::*;
use crate::game_module::actors::common::{
    calculate_hit_shake_position, play_hit_effect_and_audio, update_actor_hit_blink_color,
};
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::{Item, ItemDataType, ItemID};
use crate::game_module::behavior::behavior_base::{BehaviorBase, BehaviorState, create_character_behavior};
use crate::game_module::game_constants::*;
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_resources, get_game_scene_manager, get_game_ui_manager_mut, get_item_manager,
};
use crate::game_module::widgets::game_menu_widget::character_list_helper::{AffinityTier, get_affinity_tier};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager, get_scene_manager_mut};
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::make_rotation_matrix;
use rust_engine_3d::utilities::system::{RcRefCell, format_name_with_uuid, ptr_as_mut, ptr_as_ref};
use std::ffi::c_void;

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
    pub _lantern: Option<Box<Lantern>>,
    pub _is_enable_update: bool,
    pub _cooking_recipe_index: Option<usize>,
    pub _cooking_progress: f32,
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
        let is_ufo = character_data.borrow()._character_type == CharacterDataType::Ufo;
        let lantern = if is_ufo {
            Some(Box::new(Lantern::create_ufo_lantern(
                format!("{}_lantern", character_name).as_str(),
                position,
            )))
        } else if is_player {
            Some(Box::new(Lantern::create_lantern(
                format!("{}_lantern", character_name).as_str(),
                position,
            )))
        } else {
            None
        };

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
            _lantern: lantern,
            _is_enable_update: true,
            _cooking_recipe_index: None,
            _cooking_progress: 0.0,
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
        if let Some(lantern) = &mut self._lantern {
            lantern.destroy_lantern();
        }
        self._lantern = None;
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

    pub fn start_cooking(&mut self, recipe_index: usize) {
        self._cooking_recipe_index = Some(recipe_index);
        self._cooking_progress = 0.0;
        self.set_next_behavior(
            crate::game_module::behavior::behavior_base::BehaviorState::Cooking,
            true,
        );
    }

    pub fn attach_lantern(&mut self) {
        if self._lantern.is_none() {
            let is_ufo = self.get_character_type() == CharacterDataType::Ufo;
            let lantern = if is_ufo {
                Lantern::create_ufo_lantern(
                    format!("{}_lantern", self._character_name).as_str(),
                    self.get_position(),
                )
            } else {
                Lantern::create_lantern(
                    format!("{}_lantern", self._character_name).as_str(),
                    self.get_position(),
                )
            };
            self._lantern = Some(Box::new(lantern));
        }
    }

    pub fn detach_lantern(&mut self) {
        if let Some(mut lantern) = self._lantern.take() {
            lantern.destroy_lantern();
        }
    }

    pub fn has_lantern(&self) -> bool {
        self._lantern.is_some()
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

    pub fn is_alive(&self) -> bool {
        self._character_stats._is_alive
    }

    pub fn is_tamed(&self) -> bool {
        self._character_stats._is_tamed
    }

    pub fn is_civilian(&self) -> bool {
        matches!(
            self._character_data.borrow()._character_type,
            CharacterDataType::Civilian | CharacterDataType::Wife | CharacterDataType::Daughter
        )
    }

    pub fn is_wild_hostile(&self) -> bool {
        self.is_alive() && !self.is_player() && !self.is_tamed() && !self.is_civilian()
    }

    pub fn is_active_player_ally(&self) -> bool {
        self.is_alive()
            && (self.is_civilian()
                || (self.is_tamed() && get_affinity_tier(self.get_intimacy()) >= AffinityTier::CloseFriend))
    }

    pub fn get_character_type(&self) -> CharacterDataType {
        self._character_data.borrow()._character_type
    }

    /// Family members other than the player itself (e.g. Wife, Daughter). They always stay close to the player.
    pub fn is_family_npc(&self) -> bool {
        !self._is_player && self.get_character_type().is_family()
    }

    pub fn get_request_type(&self) -> RequestType {
        self._character_data.borrow()._character_type.get_request_type()
    }

    pub fn is_corpse(&self) -> bool {
        !self.is_alive() && !self.is_tamed() && !self.is_civilian() && self._character_stats._is_down_loop
    }

    pub fn get_intimacy(&self) -> f32 {
        self._character_stats.get_intimacy()
    }

    pub fn set_intimacy(&mut self, intimacy: f32) {
        self._character_stats.set_intimacy(intimacy);
    }

    pub fn add_intimacy(&mut self, intimacy: f32) {
        let old_tier = get_affinity_tier(self.get_intimacy());
        self._character_stats.add_intimacy(intimacy);
        let new_tier = get_affinity_tier(self.get_intimacy());
        if old_tier < AffinityTier::Friend && new_tier >= AffinityTier::Friend {
            get_game_ui_manager_mut().notify_friend_made();
        }
    }

    pub fn is_following_intimacy(&self) -> bool {
        self.is_alive() && get_affinity_tier(self._character_stats.get_intimacy()) >= AffinityTier::CloseFriend
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

    pub fn is_available_attack(&self) -> bool {
        if self._is_player && self._character_stats._stamina < STAMINA_ATTACK && self.is_attack_animation() {
            return false;
        }
        let action_animation_play_info = self.get_animation_play_info(AnimationLayer::ActionLayer);
        if self.is_available_move() {
            if self.is_idle_action() || self.is_action(ActionAnimationState::Hit) {
                return true;
            } else if self.is_action(ActionAnimationState::Attack) {
                let attackable_time = (self.get_character_data()._stat_data._attack_event_time + ATTACK_DELAY)
                    / action_animation_play_info._animation_speed;
                return attackable_time < action_animation_play_info._animation_play_time;
            }
        } else if self.is_action(ActionAnimationState::Kick) {
            let attackable_time = (self.get_character_data()._stat_data._kick_event_time + KICK_DELAY)
                / action_animation_play_info._animation_speed;
            return attackable_time < action_animation_play_info._animation_play_time;
        }
        false
    }

    pub fn is_available_move(&self) -> bool {
        self.is_alive()
            && !self.is_move_state(MoveAnimationState::Roll)
            && (!self.is_on_ground() || !self.is_action(ActionAnimationState::Kick))
            && !matches!(
                self._animation_state._action_animation_state,
                ActionAnimationState::LayingDown
                    | ActionAnimationState::Sleep
                    | ActionAnimationState::SleepNoSnoring
                    | ActionAnimationState::WakeUp
                    | ActionAnimationState::FishingBegin
                    | ActionAnimationState::FishingLoop
                    | ActionAnimationState::FishingEnd
                    | ActionAnimationState::PassedOut
            )
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

    pub fn get_attack_range(&self, attack_event: ActionAnimationState) -> f32 {
        match attack_event {
            ActionAnimationState::Attack => self.get_character_data()._stat_data._attack_range,
            ActionAnimationState::PowerAttack => self.get_character_data()._stat_data._power_attack_range,
            ActionAnimationState::Kick => self.get_character_data()._stat_data._kick_range,
            _ => {
                log::warn!("get_attack_range not implemented for {:?}", attack_event);
                0.0
            }
        }
    }

    fn check_in_range_internal(
        &self,
        target_collision: &CollisionData,
        check_range: f32,
        check_direction: bool,
        ignore_z: bool,
    ) -> bool {
        let collision = self.get_collision();
        let height_diff = (target_collision._bounding_box._min.y - collision._bounding_box._min.y).abs();
        if collision._bounding_box._extents.y < height_diff {
            return false;
        }

        let mut to_target = target_collision._bounding_box._center - collision._bounding_box._center;
        if ignore_z {
            to_target.y = 0.0;
            to_target.z = 0.0;
        }
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

    pub fn check_in_range(&self, target_collision: &CollisionData, check_range: f32, check_direction: bool) -> bool {
        self.check_in_range_internal(target_collision, check_range, check_direction, false)
    }

    pub fn check_in_range_xy(&self, target_collision: &CollisionData, check_range: f32, check_direction: bool) -> bool {
        self.check_in_range_internal(target_collision, check_range, check_direction, true)
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
            _ => {
                log::warn!("get_power not implemented for {:?}", attack_event);
                0
            }
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
            self.set_hit_damage(falling_damage, None, false);
        }
    }

    pub fn set_hit_damage(&mut self, damage: i32, attacker_pos: Option<&Vector3<f32>>, is_enable_knockback: bool) {
        if 0 < damage {
            self.set_damage(damage);

            if self.is_alive()
                && is_enable_knockback
                && let Some(attacker_pos) = attacker_pos
            {
                let attack_dir = math::make_normalize_xz(&(self.get_position() - attacker_pos));
                self._controller.set_hit_direction(&attack_dir);
            }

            self._character_stats._hit_blink_time = HIT_BLINK_TIME;
            let center = *self.get_bounding_box().get_center();
            play_hit_effect_and_audio(&center);
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

    pub fn apply_food_item_stats(&mut self, item_data_name: &str) {
        let item_data = get_game_resources().get_item_data(item_data_name);
        let item_data = item_data.borrow();
        if item_data.get_hunger() != 0.0 {
            self._character_stats.add_hunger(item_data.get_hunger());
        }
        if item_data.get_hp() != 0 {
            self._character_stats.add_hp(item_data.get_hp());
        }
        if item_data.get_stamina() != 0.0 {
            self._character_stats.add_stamina(item_data.get_stamina());
        }
    }

    pub fn get_body_temperature(&self) -> f32 {
        self._character_stats.get_body_temperature()
    }

    pub fn set_body_temperature(&mut self, temp: f32) {
        self._character_stats.set_body_temperature(temp);
    }

    pub fn get_wetness(&self) -> f32 {
        self._character_stats.get_wetness()
    }

    pub fn set_wetness(&mut self, wetness: f32) {
        self._character_stats.set_wetness(wetness);
    }

    pub fn is_hypothermia(&self) -> bool {
        self._character_stats.is_hypothermia()
    }

    pub fn set_invincibility(&mut self, invincibility: bool) {
        self._character_stats._invincibility = invincibility;
    }

    pub fn set_is_down_loop(&mut self, is_down: bool) {
        self._character_stats._is_down_loop = is_down;
    }

    pub fn get_is_stat_displayed(&self) -> bool {
        self._character_stats._is_stat_displayed
    }

    pub fn set_is_stat_displayed(&mut self, is_stat_displayed: bool) {
        self._character_stats._is_stat_displayed = is_stat_displayed;
    }

    pub fn is_interacting(&self) -> bool {
        self._character_stats._is_interacting
    }

    pub fn set_is_interacting(&mut self, is_interacting: bool) {
        self._character_stats._is_interacting = is_interacting;
        if is_interacting && !self.is_move_state(MoveAnimationState::SitDownLoop) {
            self.set_move_idle();
        }
    }

    pub fn set_behavior_none(&mut self) {
        self.set_next_behavior(BehaviorState::None, true);
    }

    pub fn set_next_behavior(&mut self, behavior_state: BehaviorState, is_force: bool) {
        self._behavior.set_next_behavior(behavior_state, is_force);
    }

    pub fn set_dead(&mut self) {
        if self._is_player {
            get_game_ui_manager_mut().notify_player_died();
        }
        self._character_stats._is_alive = false;
        self._character_stats._corpse_hit_count = MAX_CORPSE_HIT_COUNT;
        self._dead_time = 0.0;
        self._character_stats.set_is_stat_displayed(false);
        get_game_ui_manager_mut().remove_text_box_item((self as *const Self) as *const c_void);
        self.set_action_dead();
        self.set_next_behavior(BehaviorState::Dead, true);
    }

    pub fn set_passed_out(&mut self) {
        if self._is_player {
            get_game_ui_manager_mut().notify_player_passed_out();
        }
        self._dead_time = 0.0;
        self.set_action_passed_out();
        self.set_next_behavior(BehaviorState::PassedOut, true);
    }

    fn consume_attack_stamina(&mut self, stamina_cost: f32) -> f32 {
        let mut animation_speed: f32 = 1.0;
        if self._is_player {
            if self._character_stats._stamina < stamina_cost {
                get_game_ui_manager_mut().trigger_stamina_warning();
                animation_speed = ANIMATION_SPEED_BY_STAMINA;
            }
            self._character_stats._stamina = (self._character_stats._stamina - stamina_cost).max(0.0);
        }
        animation_speed
    }

    pub fn set_action_attack(&mut self) {
        let animation_speed = self.consume_attack_stamina(STAMINA_ATTACK);
        self.set_next_action_animation(ActionAnimationState::Attack, animation_speed);
    }

    pub fn set_action_power_attack(&mut self) {
        let animation_speed = self.consume_attack_stamina(STAMINA_POWER_ATTACK);
        self.set_next_action_animation(ActionAnimationState::PowerAttack, animation_speed);
    }

    pub fn set_action_kick(&mut self) {
        if self.is_available_attack() {
            let animation_speed = self.consume_attack_stamina(STAMINA_ATTACK);
            self.set_move_idle();
            self.set_next_action_animation(ActionAnimationState::Kick, animation_speed);
        }
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

    pub fn set_position_xy(&mut self, position: &Vector3<f32>) {
        self._controller.set_position_xy(position);
    }

    pub fn set_position_xz(&mut self, position: &Vector3<f32>) {
        self._controller.set_position_xz(position);
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

    pub fn get_walk_speed(&self) -> f32 {
        self.get_character_data()._stat_data._walk_speed
    }

    pub fn set_move(&mut self, move_direction: &Vector3<f32>) {
        if self.is_available_move() {
            if self._controller._face_direction.dot(move_direction) < 0.0 {
                self.set_run(false);
            }

            let move_animation = if self._controller._is_running {
                MoveAnimationState::Run
            } else {
                MoveAnimationState::Walk
            };

            self.set_move_direction(move_direction, false);

            if GAME_VIEW_MODE != GameViewMode::GameViewMode2D || move_direction.x.abs() >= move_direction.z.abs() {
                self._controller._is_high_speed_moving = move_animation == MoveAnimationState::Run;
                self.set_move_speed(self.get_character_data()._stat_data._walk_speed);
                if !self.is_move_state(move_animation) && self._controller._is_ground {
                    self.set_next_move_animation(move_animation, 1.0);
                }
            } else {
                self.set_move_control_stop();
            }
        }
    }

    pub fn move_to_target(&mut self, target_position: &Vector3<f32>, radius: f32, delta_time: f32) -> bool {
        let diff = target_position - self.get_position();
        let (target_dir, target_dist) = math::make_normalize_xz_with_norm(&diff);
        let move_dist = self.get_walk_speed() * delta_time;
        if (target_dist - 0.1f32.max(radius)) <= move_dist {
            self.set_position_xz(target_position);
            return true;
        }
        self.set_move(&target_dir);
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

            self._controller._is_high_speed_moving = true;
            self.set_move_speed(self.get_character_data()._stat_data._roll_speed);
            self.set_move_direction(&self._controller._face_direction.clone(), false);
            self.set_action_none();
            self.set_next_move_animation(MoveAnimationState::Roll, 1.0);
        }
    }

    pub fn update_transform(&mut self) {
        let render_object = self._render_object.borrow();
        let transform = render_object.get_transform_object_data_mut();
        let position = calculate_hit_shake_position(&self._controller._position, self._character_stats._hit_blink_time);

        transform.set_position(&position);
        transform.set_rotation(&self._controller._rotation);
        transform.set_scale(&self._controller._scale);
    }

    pub fn is_enable_update(&self) -> bool {
        self._is_enable_update
    }

    pub fn set_enable_update(&mut self, is_enable_update: bool) {
        self._is_enable_update = is_enable_update;
    }

    pub fn set_visible(&self, visible: bool) {
        self._render_object.borrow_mut().set_visible(visible);
        if let Some(attached_item) = &self._attached_item {
            attached_item.borrow()._render_object.borrow_mut().set_visible(visible);
        }
    }

    pub fn update_render_object(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object.update_render_object_data(0.0);
    }

    pub fn update_character(
        &mut self,
        scene_manager: &SceneManager<'a>,
        target: Option<&Character<'a>>,
        delta_time: f32,
    ) {
        if !self._is_enable_update {
            return;
        }

        let was_on_ground = self.is_on_ground();
        let falling_height = self._controller.get_falling_height();

        // update animation key frames
        self.update_move_keyframe_event();
        self.update_action_keyframe_event(delta_time);

        // update hit blink color
        if 0.0 < self._character_stats._hit_blink_time {
            update_actor_hit_blink_color(
                &self._render_object,
                &mut self._character_stats._hit_blink_time,
                delta_time,
            );
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
        if self.is_alive() && !self.check_falling_in_water_damage() && !was_on_ground && self.is_on_ground() {
            self.check_falling_on_ground_damage(falling_height);
        }

        // transform
        self.update_transform();

        // update lantern
        let position = *self.get_position();
        if let Some(lantern) = &mut self._lantern {
            let is_night = get_game_scene_manager().is_night();
            lantern.update_lantern(&position, is_night, delta_time);
        }
    }
}
