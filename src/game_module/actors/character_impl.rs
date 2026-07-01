use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioLoop};
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::make_rotation_matrix;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character, CharacterAnimationState, CharacterCreateInfo, CharacterStats};
use crate::game_module::actors::character_controller::CharacterController;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterData, MoveAnimationState};
use crate::game_module::actors::character_manager::{CharacterID, CharacterManager};
use crate::game_module::behavior::behavior_base::{create_character_behavior, BehaviorState};
use crate::game_module::game_constants::*;
use crate::game_module::actors::items::{ItemDataType, ItemManager};
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::{Item};
use crate::game_module::game_client::GamePhase;
use crate::game_module::scenario::scenario::ScenarioType;

impl CharacterAnimationState {
    pub fn is_attack_event(&self) -> bool {
        self._action_event == ActionAnimationState::Attack || self._action_event == ActionAnimationState::PowerAttack || self._action_event == ActionAnimationState::Kick
    }
    pub fn is_action_event(&self, action_event: ActionAnimationState) -> bool {
        self._action_event == action_event
    }
    pub fn get_action_event(&self) -> ActionAnimationState {
        self._action_event
    }
    pub fn set_action_event(&mut self, action_event: ActionAnimationState) {
        self._action_event = action_event;
    }
}

impl CharacterStats {
    pub fn create_character_stats() -> CharacterStats {
        CharacterStats {
            _is_alive: true,
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
            _invincibility: false,
            _is_stat_displayed: false
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
        } else if owner.is_idle_action() &&
            (owner.is_move_stop() ||
            owner.is_move_state(MoveAnimationState::SitDownLoop) ||
            owner.is_move_state(MoveAnimationState::Walk)) {
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
}

impl<'a> Character<'a> {
    pub fn create_character_instance(
        character_manager: *const CharacterManager<'a>,
        item_manager: *const ItemManager<'a>,
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
            _character_manager: character_manager,
            _item_manager: item_manager,
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
            _audio_snoring: None
        };

        character.initialize_character(position, rotation, scale);
        character
    }

    pub fn initialize_character(
        &mut self,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
    ) {
        self._character_stats.initialize_character_stats(&self._character_data.borrow());
        self._controller.initialize_controller(&self._character_data.borrow(), position, rotation, scale);
        self._behavior.initialize_behavior(ptr_as_mut(self), position);

        self.set_move_idle();
        self.set_action_none();
        self.update_transform();
        self.update_render_object();
    }

    pub fn initialize_transform(&mut self, position: &Vector3<f32>, rotation: &Vector3<f32>, scale: &Vector3<f32>) {
        self._controller._position = position.clone();
        self._controller._position.y = self._controller._position.y.max(self.get_character_manager().get_scene_manager().get_height_map_data().get_height_bilinear(position, 0));
        self._controller._rotation = rotation.clone();
        self._controller._scale = scale.clone();
        let direction: Vector3<f32> = make_rotation_matrix(self._controller._rotation.x, self._controller._rotation.y, self._controller._rotation.z).column(2).xyz();
        self._controller.set_move_direction(&direction);
        self.update_transform();
        self.update_render_object();
    }

    pub fn destroy_character(&mut self) {
        self.stop_animations(true);
    }

    pub fn load_character_save_data(&mut self, character_create_info: &CharacterCreateInfo) {
        self.initialize_transform(
            &character_create_info._position,
            &character_create_info._rotation,
            &character_create_info._scale
        )
    }

    pub fn get_character_save_data(&self) -> CharacterCreateInfo {
        CharacterCreateInfo {
            _character_data_name: self._character_data_name.clone(),
            _position: self.get_position().clone(),
            _rotation: self.get_rotation().clone(),
            _scale: self.get_scale().clone(),
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

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        ptr_as_ref(self._character_manager)
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager)
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

    pub fn get_transform(&self) -> &TransformObjectData {
        &ptr_as_ref(self._render_object.as_ptr())._transform_object
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
        self.is_move_state(MoveAnimationState::None) || self.is_move_state(MoveAnimationState::Idle) || self.is_move_state(MoveAnimationState::SitDownLoop)
    }

    pub fn is_alive(&self) -> bool {
        self._character_stats._is_alive
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
        if self.is_action(ActionAnimationState::Attack) ||
            self.is_action(ActionAnimationState::PowerAttack) ||
            self.is_action(ActionAnimationState::Hit) ||
            self.is_action(ActionAnimationState::Eating) ||
            self.is_action(ActionAnimationState::Hungry) ||
            self.is_action(ActionAnimationState::Pickup) {
            if self.is_move_state(MoveAnimationState::Jump) ||
                self.is_move_state(MoveAnimationState::Run) ||
                self.is_move_state(MoveAnimationState::RunningJump) ||
                self.is_move_state(MoveAnimationState::SitDownLoop) ||
                self.is_move_state(MoveAnimationState::Walk) {
                return true;
            }
        }
        false
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._animation_state._action_animation_state
    }

    pub fn is_attack_animation(&self) -> bool {
        self.is_action(ActionAnimationState::Attack) || self.is_action(ActionAnimationState::PowerAttack) || self.is_action(ActionAnimationState::Kick)
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
        self.is_alive() &&
        self.is_move_state(MoveAnimationState::Roll) == false &&
        (self.is_on_ground() == false || self.is_action(ActionAnimationState::Kick) == false) &&
        self.is_action(ActionAnimationState::LayingDown) == false &&
        self.is_action(ActionAnimationState::Sleep) == false &&
        self.is_action(ActionAnimationState::SleepNoSnoring) == false &&
        self.is_action(ActionAnimationState::WakeUp) == false
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

    pub fn check_in_range(
        &self,
        target_collision: &CollisionData,
        check_range: f32,
        check_direction: bool,
    ) -> bool {
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
        distance <= (r0 + check_range + r1) && (check_direction == false || self.get_transform().get_front().dot(&to_target_dir) < 0.0)
    }

    pub fn check_in_range_xy(
        &self,
        target_collision: &CollisionData,
        check_range: f32,
        check_direction: bool,
    ) -> bool {
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
        distance <= (r0 + check_range + r1) && (check_direction == false || self.get_transform().get_front().dot(&to_target_dir) < 0.0)
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self._controller.get_rotation()
    }
    pub fn get_face_direction(&self) -> &Vector3<f32> {
        &self._controller.get_face_direction()
    }
    pub fn get_scale(&self) -> &Vector3<f32> {
        &self._controller.get_scale()
    }
    pub fn look_at(&mut self, target_position: &Vector3<f32>) {
        let direction = math::make_normalize_xz(&(target_position - self.get_position()));
        self._controller.set_move_direction(&direction);
        if self.is_move_stop() == false {
            self.set_move_idle();
        }
    }

    pub fn get_prev_position(&self) -> &Vector3<f32> { &self._controller._prev_position }
    pub fn get_position(&self) -> &Vector3<f32> { &self._controller._position }
    pub fn get_velocity(&self) -> &Vector3<f32> { &self._controller._velocity }
    pub fn get_final_velocity(&self) -> &Vector3<f32> { &self._controller._final_velocity }
    pub fn get_center(&self) -> &Vector3<f32> {
        &self.get_bounding_box().get_center()
    }

    pub fn check_arrival_with_radius(&self, target_position: &Vector3<f32>, radius: f32, ignore_y_axis: bool) -> bool {
        self._controller.check_arrival_with_radius(target_position, radius, ignore_y_axis)
    }

    pub fn get_power(&self, attack_event: ActionAnimationState) -> i32 {
        match attack_event {
            ActionAnimationState::Attack => self.get_character_data()._stat_data._attack_damage,
            ActionAnimationState::PowerAttack => self.get_character_data()._stat_data._power_attack_damage,
            ActionAnimationState::Kick => self.get_character_data()._stat_data._kick_damage,
            _ => panic!("get_power not implemented: {:?}", attack_event),
        }
    }

    pub fn set_damage(&mut self, damage: i32) {
        if 0 < damage && self.is_alive() {
            let character_manager = ptr_as_ref(self._character_manager);
            let hp = self._character_stats.get_hp() - damage;
            self._character_stats.set_hp(hp);
            if hp <= 0 {
                character_manager.get_scene_manager().play_audio(&self._character_data.borrow()._audio_data._audio_dead);
                self.set_dead();
            } else {
                character_manager.get_scene_manager().play_audio(&self._character_data.borrow()._audio_data._audio_pain);
                if self._is_player && self.is_move_state(MoveAnimationState::Roll) == false {
                    self.set_action_hit();
                }
            }
        }
    }

    pub fn check_falling_in_water_damage(&mut self) -> bool {
        let dead_zone_height = self.get_character_manager().get_scene_manager().get_dead_zone_height();
        if self.get_position().y <= dead_zone_height {
            self.set_damage(self._character_stats.get_hp());

            let effect_create_info = EffectCreateInfo {
                _effect_position: Vector3::new(
                    self.get_position().x,
                    dead_zone_height,
                    self.get_position().z,
                ),
                _effect_data_name: String::from(EFFECT_FALLING_WATER),
                ..Default::default()
            };
            let character_manager = ptr_as_ref(self._character_manager);
            character_manager.get_scene_manager_mut().add_effect(EFFECT_FALLING_WATER, &effect_create_info);
            character_manager.get_scene_manager().play_audio_bank(AUDIO_FALLING_WATER);
            return true;
        }
        false
    }

    pub fn check_falling_on_ground_damage(&mut self, falling_height: f32) {
        let falling_height = falling_height - self.get_position().y;
        if FALLING_HEIGHT < falling_height {
            let falling_damage: i32 =
                (falling_height - FALLING_HEIGHT).ceil() as i32 * FALLING_DAMAGE_RATIO;
            self.set_hit_damage(falling_damage, None);
        }
    }

    pub fn set_hit_damage(&mut self, damage: i32, attack_dir: Option<&Vector3<f32>>) {
        if 0 < damage {
            self.set_damage(damage);

            if let Some(attack_dir) = attack_dir {
                self._controller.set_hit_direction(&attack_dir);
            }

            let effect_create_info = EffectCreateInfo {
                _effect_position: self.get_bounding_box().get_center().clone(),
                _effect_data_name: String::from(EFFECT_HIT),
                ..Default::default()
            };

            let character_manager = ptr_as_ref(self._character_manager);
            character_manager.get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
            character_manager.get_scene_manager().play_audio_bank(AUDIO_HIT);
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

    pub fn get_is_stat_displayed(&self) -> bool {
        self._character_stats._is_stat_displayed
    }

    pub fn set_is_stat_displayed(&mut self, is_stat_displayed: bool) {
        self._character_stats._is_stat_displayed = is_stat_displayed;
    }

    pub fn set_behavior_none(&mut self) {
        self.set_behavior(BehaviorState::None, None, true);
    }

    pub fn set_behavior(
        &mut self,
        behavior_state: BehaviorState,
        target: Option<&Character>,
        is_force: bool
    ) {
        let owner = ptr_as_mut(self);
        self._behavior.set_behavior(behavior_state, owner, target, is_force);
    }

    pub fn set_dead(&mut self) {
        self._character_stats._is_alive = false;
        self.set_action_dead();
    }

    pub fn set_action_none(&mut self) {
        self.set_action_animation(ActionAnimationState::None, 1.0);
    }

    pub fn set_action_dance(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::Dance, 1.0);
    }

    pub fn set_action_wake_up(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::WakeUp, 1.0);
    }

    pub fn set_action_laying_down(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::LayingDown, 2.0);
    }

    pub fn set_action_sleep(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::Sleep, 1.0);
    }

    pub fn set_action_sleep_no_snoring(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::SleepNoSnoring, 1.0);
    }

    pub fn set_action_hungry(&mut self) {
        if self.is_action(ActionAnimationState::None) {
            self.set_action_animation(ActionAnimationState::Hungry, 1.0);
        }
    }

    pub fn set_action_eating(&mut self) {
        if self.is_idle_action() {
            self.set_action_animation(ActionAnimationState::Eating, 1.0);
        }
    }

    pub fn set_action_interaction(&mut self) {
        if self._controller.is_on_ground() && self.is_available_move() && self.is_idle_action() {
            match self._controller._nearest_interaction_object.clone() {
                InteractionObject::PropBed(_) => {
                    self.get_character_manager().get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWrapUpTheDay, false);
                }
                InteractionObject::PropPickup(_) => {
                    self.set_action_animation(ActionAnimationState::Pickup, 2.0);
                }
                InteractionObject::PropMonolith(_) => {
                    self.get_character_manager().get_game_client_mut().set_next_game_phase(GamePhase::OpenToolbox);
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
                    character.borrow_mut().set_behavior(BehaviorState::Interaction, None, false);

                    // give item
                    let mut give_item = false;
                    if character.borrow().get_attached_item().is_none() {
                        if let Some(attached_item) = self.get_attached_item() {
                            if attached_item.borrow().get_item_data_type().is_eatable() {
                                give_item = true;
                                let item_data_name = attached_item.borrow()._item_data_name.clone();
                                ptr_as_mut(self._item_manager).remove_inventory_item(item_data_name.as_str(), 1);
                                ptr_as_mut(self._item_manager).attach_item(&mut *character.borrow_mut(), item_data_name.as_str());
                            }
                        }
                    }

                    if give_item == false {
                        character.borrow_mut().set_is_stat_displayed(true);
                    }
                }
                _ => {},
            }
        }
    }
    pub fn callback_changed_interaction_object(&mut self) {
        match self._controller._nearest_interaction_object.clone() {
            InteractionObject::PropGate(_) => {
                self.get_character_manager().get_game_client_mut().set_next_game_phase(GamePhase::WorldMapOpen);
                self.set_move_idle();
            }
            _ => {},
        }
    }
    pub fn set_action_attack(&mut self) {
        if self.is_available_attack() {
            let mut animation_speed: f32 = 1.0;
            if self._is_player {
                let render_object = self._render_object.borrow();
                let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                if self._character_stats._stamina < STAMINA_ATTACK  && animation_play_info._is_animation_end == false {
                    return;
                }

                self._character_stats._stamina -= STAMINA_ATTACK;
                if self._character_stats._stamina < 0.0 {
                    animation_speed = ANIMATION_SPEED_BY_STAMINA;
                }
            }
            self.set_action_animation(ActionAnimationState::Attack, animation_speed);
        }
    }

    pub fn set_action_power_attack(&mut self) {
        if self.is_available_attack() {
            let mut animation_speed: f32 = 1.0;
            if self._is_player {
                let render_object = self._render_object.borrow();
                let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                if self._character_stats._stamina < STAMINA_POWER_ATTACK && animation_play_info._is_animation_end == false {
                    return;
                }

                self._character_stats._stamina -= STAMINA_POWER_ATTACK;
                if self._character_stats._stamina < 0.0 {
                    animation_speed = ANIMATION_SPEED_BY_STAMINA;
                }
            }
            self.set_action_animation(ActionAnimationState::PowerAttack, animation_speed);
        }
    }

    pub fn set_action_kick(&mut self) {
        if self.is_available_attack() {
            let animation_speed: f32 = 1.0;
            if self._is_player {
                let render_object = self._render_object.borrow();
                let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                if self._character_stats._stamina < STAMINA_ATTACK  && animation_play_info._is_animation_end == false {
                    return;
                }

                self._character_stats._stamina -= STAMINA_ATTACK;
                // if self._character_stats._stamina < 0.0 {
                //     animation_speed = 0.5;
                // }
            }
            self.set_move_idle();
            self.set_action_animation(ActionAnimationState::Kick, animation_speed);
        }
    }

    pub fn set_action_hit(&mut self) {
        self.set_action_animation(ActionAnimationState::Hit, 1.0);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::Dead, 1.0);
    }

    pub fn set_move_animation(&mut self, move_animation_state: MoveAnimationState) {
        let mut animation_info = AnimationPlayArgs {
            ..Default::default()
        };

        let character_data = self.get_character_data();
        let animation_data = &character_data._animation_data;
        let mut render_object = self._render_object.borrow_mut();
        match move_animation_state {
            MoveAnimationState::None | MoveAnimationState::Idle => {
                animation_info._animation_speed = animation_data._idle_animation_speed;
                render_object.set_animation(
                    &animation_data._idle_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::Walk => {
                animation_info._animation_speed = animation_data._walk_animation_speed;
                render_object.set_animation(
                    &animation_data._walk_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::Run => {
                animation_info._animation_speed = animation_data._run_animation_speed;
                render_object.set_animation(
                    &animation_data._run_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::Jump => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._jump_animation_speed;
                render_object.set_animation(
                    &animation_data._jump_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::Roll => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._roll_animation_speed;
                render_object.set_animation(
                    &animation_data._roll_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::RunningJump => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._running_jump_animation_speed;
                render_object.set_animation(
                    &animation_data._running_jump_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
            MoveAnimationState::SitDownLoop => {
                render_object.set_animation(
                    &animation_data._sit_down_loop_animation,
                    &animation_info,
                    AnimationLayer::BaseLayer,
                );
            }
        }

        self._animation_state._move_animation_state = move_animation_state;
        self.update_animation_layers();
    }

    pub fn set_action_animation(
        &mut self,
        action_animation_state: ActionAnimationState,
        animation_speed: f32,
    ) {
        let mut animation_info = AnimationPlayArgs {
            _animation_loop: false,
            _force_animation_setting: true,
            _animation_fade_out_time: 0.1,
            _animation_speed: animation_speed,
            ..Default::default()
        };

        let character_data = self.get_character_data();
        let animation_data = &character_data._animation_data;
        let mut render_object = self._render_object.borrow_mut();
        match action_animation_state {
            ActionAnimationState::None => {
                render_object.set_animation_none(AnimationLayer::ActionLayer);
            }
            ActionAnimationState::Attack => {
                animation_info._animation_speed *= animation_data._attack_animation_speed;
                render_object.set_animation(
                    &animation_data._attack_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Dance => {
                animation_info._animation_loop = true;
                render_object.set_animation(
                    &animation_data._dance_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Dead => {
                animation_info._animation_speed *= animation_data._dead_animation_speed;
                animation_info._animation_fade_out_time = 0.0; // keep end of animation
                render_object.set_animation(
                    &animation_data._dead_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Hit => {
                animation_info._animation_speed *= animation_data._hit_animation_speed;
                render_object.set_animation(
                    &animation_data._hit_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Kick => {
                animation_info._animation_speed *= animation_data._kick_animation_speed;
                render_object.set_animation(
                    &animation_data._kick_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::LayingDown => {
                animation_info._animation_fade_out_time = 0.0; // keep end of animation
                render_object.set_animation(
                    &animation_data._laying_down_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Pickup => {
                render_object.set_animation(
                    &animation_data._pickup_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::PowerAttack => {
                animation_info._animation_speed *= animation_data._power_attack_animation_speed;
                render_object.set_animation(
                    &animation_data._power_attack_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Sleep | ActionAnimationState::SleepNoSnoring => {
                animation_info._animation_loop = true;
                animation_info._animation_fade_out_time = 0.0; // keep end of animation
                render_object.set_animation(
                    &animation_data._sleep_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Eating => {
                render_object.set_animation(
                    &animation_data._eating_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::Hungry => {
                animation_info._animation_loop = true;
                render_object.set_animation(
                    &animation_data._hungry_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
            ActionAnimationState::WakeUp => {
                render_object.set_animation(
                    &animation_data._wake_up_animation,
                    &animation_info,
                    AnimationLayer::ActionLayer,
                );
            }
        }

        self._animation_state._action_animation_state = action_animation_state;
        self.update_animation_layers();
    }

    pub fn set_run(&mut self, run: bool) {
        self._controller.set_run(run);
    }

    pub fn toggle_run(&mut self) {
        if self.is_move_state(MoveAnimationState::Run) || self.is_move_state(MoveAnimationState::Walk) {
            self._controller.toggle_run();
        }
    }

    pub fn set_move_idle(&mut self) {
        self.set_run(false);
        self.set_move_speed(0.0);
        if self.is_move_state(MoveAnimationState::Idle) == false {
            self.set_move_animation(MoveAnimationState::Idle);
        }
    }

    pub fn set_move_control_stop(&mut self) {
        if self.is_move_state(MoveAnimationState::Roll) == false {
            self.set_run(false);
            self.set_move_speed(0.0);
            if self.is_move_stop() == false && self.is_on_ground() {
                self.set_move_animation(MoveAnimationState::Idle);
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
        if self.is_move_state(MoveAnimationState::SitDownLoop) == false {
            self.set_move_animation(MoveAnimationState::SitDownLoop);
        }
    }

    pub fn stop_animations(&mut self, apply_immediately: bool) {
        self.set_action_animation(ActionAnimationState::None, 1.0);
        self.set_move_animation(MoveAnimationState::None);
        self.set_run(false);
        self.set_move_speed(0.0);
        if apply_immediately {
            self.update_action_keyframe_event();
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
    pub fn get_move_speed(&self) -> f32 { self._controller.get_move_speed() }
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
                if false == self.is_move_state(move_animation) && self._controller._is_ground {
                    self.set_move_animation(move_animation);
                }
            } else {
                self.set_move_control_stop();
            }
        }
    }

    pub fn move_to_target(&mut self, target_position: &Vector3<f32>, radius: f32) -> bool {
        if self.check_arrival_with_radius(target_position, radius, true) {
            self.set_position(&Vector3::new(target_position.x, self.get_position().y, target_position.z));
            return true;
        }
        self.set_move(&(target_position - self.get_position()));
        false
    }

    pub fn set_jump(&mut self) {
        if self.is_available_jump() {
            let mut not_enough_stamina = false;
            if self._is_player {
                self._character_stats._stamina -= STAMINA_JUMP;
                not_enough_stamina = self._character_stats._stamina < 0.0;
            }

            let move_anim = if self._controller._is_running && not_enough_stamina == false {
                MoveAnimationState::RunningJump
            } else {
                MoveAnimationState::Jump
            };
            self._controller.set_jump_start();
            self.set_move_animation(move_anim);
        }
    }

    pub fn set_roll(&mut self) {
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
            self.set_move_animation(MoveAnimationState::Roll);
        }
    }

    pub fn update_move_animation_begin_event(&mut self) {
        let character_manager = self.get_character_manager();
        match self._animation_state._move_animation_state {
            MoveAnimationState::Jump => {
                character_manager.get_scene_manager().play_audio_bank(AUDIO_JUMP);
            }
            MoveAnimationState::Roll => {
                self.set_invincibility(true);
            }
            MoveAnimationState::RunningJump => {
                character_manager.get_scene_manager().play_audio_bank(AUDIO_JUMP);
            }
            _ => (),
        }
    }

    pub fn update_move_animation_end_event(&mut self) {
        match self._animation_state._move_animation_state_prev {
            MoveAnimationState::Roll => {
                self._controller.set_roll_delay();
                self.set_invincibility(false);
            }
            _ => (),
        }
    }

    pub fn update_move_animation_loop_event(&mut self) {
        let character_manager = self.get_character_manager();
        let move_animation = self._animation_state._move_animation_state;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
        match move_animation {
            MoveAnimationState::Roll => {
                if self._is_player && animation_play_info.check_animation_event_time(0.2) {
                    character_manager.get_scene_manager().play_audio_bank(AUDIO_ROLL);
                } else if animation_play_info._is_animation_end {
                    self.set_move_idle();
                }
            }
            MoveAnimationState::Run => {
                if self._is_player && (animation_play_info.check_animation_event_time(0.1) || animation_play_info.check_animation_event_time(0.5)) {
                    character_manager.get_scene_manager().play_audio_options(
                        AUDIO_FOOTSTEP,
                        AudioLoop::ONCE,
                        Some(0.5),
                    );
                }
            }
            MoveAnimationState::Walk => {
                if self._is_player && (animation_play_info.check_animation_event_time(0.2) || animation_play_info.check_animation_event_time(0.9)) {
                    character_manager.get_scene_manager().play_audio_options(
                        AUDIO_FOOTSTEP,
                        AudioLoop::ONCE,
                        Some(0.5),
                    );
                }
            }
            _ => (),
        }
    }

    pub fn update_move_keyframe_event(&mut self) {
        if self._animation_state._move_animation_state_prev != self._animation_state._move_animation_state {
            self.update_move_animation_end_event();
            self.update_move_animation_begin_event();
            self._animation_state._move_animation_state_prev = self._animation_state._move_animation_state;
        }

        self.update_move_animation_loop_event();
    }

    pub fn update_action_animation_begin_event(&mut self) {
        match self._animation_state._action_animation_state {
            ActionAnimationState::Eating => {
                self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_EATING);
            },
            ActionAnimationState::Sleep => {
                if self._is_player {
                    if let Some(audio_instance) = self._audio_snoring.as_ref() {
                        self.get_character_manager().get_scene_manager().stop_audio_instance(&audio_instance)
                    }
                    self._audio_snoring = self.get_character_manager().get_scene_manager().play_audio_options(
                        AUDIO_SNORING,
                        AudioLoop::LOOP,
                        Some(1.0),
                    );
                }
            }
            _ => ()
        }
    }

    pub fn update_action_animation_loop_event(&mut self) {
        let character_data = self.get_character_data();
        let action_animation = self._animation_state._action_animation_state;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
        match action_animation {
            ActionAnimationState::Attack => {
                if animation_play_info.check_animation_event_time(character_data._stat_data._attack_event_time) {
                    self._animation_state.set_action_event(ActionAnimationState::Attack);
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
            ActionAnimationState::Dead => {
                if self._is_player && animation_play_info._is_animation_end {
                    // resurrection
                    self.initialize_character(
                        &self.get_character_manager().get_game_scene_manager().get_spawn_point().clone(),
                        &self._controller._rotation.clone(),
                        &self._controller._scale.clone(),
                    );
                }
            }
            ActionAnimationState::Kick => {
                if animation_play_info.check_animation_event_time(character_data._stat_data._kick_event_time) {
                    self._animation_state.set_action_event(ActionAnimationState::Kick);
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                } else if self.is_on_ground() {
                    // prevent slip after jump kick
                    self.set_move_idle();
                }
            }
            ActionAnimationState::LayingDown => {
                if animation_play_info._is_animation_end {
                    self.set_action_sleep();
                }
            }
            ActionAnimationState::Pickup => {
                if animation_play_info.check_animation_event_time(PICKUP_EVENT_TIME) {
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                    self._animation_state.set_action_event(ActionAnimationState::Pickup);
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
            ActionAnimationState::PowerAttack => {
                if animation_play_info.check_animation_event_time(character_data._stat_data._power_attack_event_time) {
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                    self._animation_state.set_action_event(ActionAnimationState::PowerAttack);
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
            _ => {
                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
        }
    }

    pub fn update_action_animation_end_event(&mut self) {
        match self._animation_state._action_animation_state_prev {
            ActionAnimationState::Eating => {
                if let Some(attached_item) = self.get_attached_item().clone() {
                    self.get_stats_mut().add_hunger(-1.0);
                    self.get_stats_mut().add_hp(10);
                    self.get_stats_mut().add_stamina(10.0);

                    if self._is_player {
                        ptr_as_mut(self._item_manager).remove_inventory_item(attached_item.borrow()._item_data_name.as_str(), 1);
                    } else {
                        ptr_as_mut(self._item_manager).detach_item(self);
                    }
                }
            }
            ActionAnimationState::Sleep => {
                if self._is_player {
                    if let Some(audio_instance) = self._audio_snoring.as_ref() {
                        self.get_character_manager().get_scene_manager().stop_audio_instance(&audio_instance)
                    }
                }
            }
            _ => ()
        }
    }

    pub fn update_action_keyframe_event(&mut self) {
        self._animation_state.set_action_event(ActionAnimationState::None);

        if self._animation_state._action_animation_state_prev != self._animation_state._action_animation_state {
            self.update_action_animation_end_event();
            self.update_action_animation_begin_event();
            self._animation_state._action_animation_state_prev = self._animation_state._action_animation_state;
        }

        self.update_action_animation_loop_event();
    }

    pub fn update_transform(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&self._controller._position);
        render_object._transform_object.set_rotation(&self._controller._rotation);
        render_object._transform_object.set_scale(&self._controller._scale);
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

    pub fn update_character(
        &mut self,
        scene_manager: &SceneManager<'a>,
        player: &Character<'a>,
        delta_time: f32
    ) {
        let was_on_ground = self.is_on_ground();
        let falling_height = self._controller.get_falling_height();

        // update animation key frames
        self.update_move_keyframe_event();
        self.update_action_keyframe_event();

        // behavior
        if false == self._is_player && self.is_alive() {
            self._behavior.update_behavior(ptr_as_mut(self), Some(player), delta_time);
        }

        // update stats - stamina
        let owner = ptr_as_ref(self);
        self._character_stats.update_stat(owner, delta_time);
        if owner.is_move_state(MoveAnimationState::Run) {
            if self._character_stats._stamina == 0.0 {
                self.set_run(false);
            }
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
