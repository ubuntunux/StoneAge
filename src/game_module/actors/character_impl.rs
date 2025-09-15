use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::scene::transform_object::TransformObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character, CharacterAnimationState, CharacterStats};
use crate::game_module::actors::character_controller::CharacterController;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterData, MoveAnimationState};
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::actors::weapons::Weapon;
use crate::game_module::behavior::behavior_base::create_character_behavior;
use crate::game_module::game_constants::{ATTACK_DELAY, AUDIO_ATTACK, AUDIO_FALLING_WATER, AUDIO_FOOTSTEP, AUDIO_HIT, AUDIO_JUMP, AUDIO_ROLL, EFFECT_FALLING_WATER, EFFECT_HIT, FALLING_DAMAGE_RATIO, FALLING_HEIGHT, MAX_STAMINA, STAMINA_ATTACK, STAMINA_JUMP, STAMINA_POWER_ATTACK, STAMINA_RECOVERY, STAMINA_ROLL, STAMINA_RUN};
use crate::game_module::game_scene_manager::BlockArray;

impl CharacterStats {
    pub fn create_character_stats() -> CharacterStats {
        CharacterStats {
            _is_alive: true,
            _hp: 100,
            _stamina: MAX_STAMINA,
            _invincibility: false,
        }
    }

    pub fn initialize_character_stats(&mut self, character_data: &CharacterData) {
        self._is_alive = true;
        self._hp = character_data._stat_data._max_hp;
        self._stamina = MAX_STAMINA;
        self._invincibility = false;
    }
}

impl CharacterStats {
    pub fn update_stamina<'a>(&mut self, owner: &Character<'a>, delta_time: f32) {
        if owner._is_player && self._is_alive {
            if owner.is_move_state(MoveAnimationState::Run) {
                self._stamina -= STAMINA_RUN * delta_time;
                if self._stamina < 0.0 {
                    self._stamina = 0.0;
                }
            } else if owner.is_action(ActionAnimationState::None) && (owner.is_move_state(MoveAnimationState::None) || owner.is_move_state(MoveAnimationState::Idle) || owner.is_move_state(MoveAnimationState::Walk)) {
                if self._stamina < 0.0 {
                    self._stamina = 0.0;
                }

                let stamina_scale = if owner.is_move_state(MoveAnimationState::Walk) { 0.5 } else { 1.0 };
                self._stamina += STAMINA_RECOVERY * stamina_scale * delta_time;
                if MAX_STAMINA < self._stamina {
                    self._stamina = MAX_STAMINA;
                }
            }
        }
    }
}

impl<'a> Character<'a> {
    pub fn create_character_instance(
        character_manager: &CharacterManager<'a>,
        character_id: u64,
        is_player: bool,
        character_data_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>
    ) -> Character<'a> {
        let character_data_borrow = character_data.borrow();
        let mut character = Character {
            _character_manager: character_manager,
            _character_id: character_id,
            _is_player: is_player,
            _character_data_name: String::from(character_data_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_stats: Box::new(CharacterStats::default()),
            _animation_state: Box::new(CharacterAnimationState::default()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _behavior: create_character_behavior(character_data_borrow._character_type),
            _weapon: None
        };

        character.initialize_character(position, rotation, scale);
        character
    }

    pub fn initialize_character(
        &mut self,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>
    ) {
        self._character_stats.initialize_character_stats(&self._character_data.borrow());
        self._controller.initialize_controller(position, rotation, scale);
        self._behavior.initialize_behavior(ptr_as_mut(self), position);

        self.set_move_idle();
        self.set_action_none();
        self.update_transform();
        self.update_render_object();
    }

    pub fn add_weapon(&mut self, weapon: Box<Weapon<'a>>) {
        if self._weapon.is_some() {
            panic!("already has weapon!")
        } else {
            self._weapon = Some(weapon);
        }
    }

    pub fn get_weapon(&self) -> &Option<Box<Weapon<'a>>> {
        &self._weapon
    }

    pub fn remove_weapon(&mut self) {
        self._weapon = None;
    }

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        ptr_as_ref(self._character_manager)
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager)
    }

    pub fn get_character_id(&self) -> u64 {
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

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._animation_state._move_animation_state
    }

    pub fn is_on_ground(&self) -> bool {
        self._controller.is_on_ground()
    }

    pub fn is_falling(&self) -> bool {
        self._controller.is_falling()
    }

    pub fn is_jump(&self) -> bool {
        self._controller.is_jump()
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._animation_state._action_animation_state
    }

    pub fn is_attack_animation(&self) -> bool {
        self.is_action(ActionAnimationState::Attack) || self.is_action(ActionAnimationState::PowerAttack)
    }

    pub fn is_available_attack(&self) -> bool {
        if self.is_available_move() {
            let action_animation_play_info = self.get_animation_play_info(AnimationLayer::ActionLayer);
            if self.is_action(ActionAnimationState::None) || self.is_action(ActionAnimationState::Hit) {
                return true;
            }
            else if self.is_action(ActionAnimationState::Attack) {
                let attackable_time = self.get_character_data()._stat_data._attack_event_time + ATTACK_DELAY;
                return attackable_time < action_animation_play_info._animation_play_time;
            }
        }
        false
    }

    pub fn is_available_move(&self) -> bool {
        self._character_stats._is_alive && !self.is_move_state(MoveAnimationState::Roll)
    }

    pub fn is_available_jump(&self) -> bool {
        // if self._is_player && self._character_stats._stamina < STAMINA_JUMP {
        //     return false;
        // }
        !self.is_jump() && !self.is_falling() && self.is_available_move()
    }

    pub fn is_available_roll(&self) -> bool {
        if self._is_player && self._character_stats._stamina < STAMINA_ROLL {
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
            _ => panic!("check_attack_range not implemented: {:?}", attack_event)
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
        let r0 = math::lerp(collision._bounding_box._extents.z, collision._bounding_box._extents.x, d0);
        let d1 = target_collision._bounding_box._orientation.column(0).dot(&to_target_dir).abs();
        let r1 = math::lerp(target_collision._bounding_box._extents.z, target_collision._bounding_box._extents.x, d1);
        distance <= (r0 + check_range + r1) && (check_direction == false || self.get_transform().get_front().dot(&to_target_dir) < 0.0)
    }

    pub fn get_front(&self) -> &Vector3<f32> {
        &self._controller._face_direction
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self._controller._position
    }

    pub fn get_power(&self, attack_event: ActionAnimationState) -> i32 {
        match attack_event {
            ActionAnimationState::Attack => self.get_character_data()._stat_data._attack_damage,
            ActionAnimationState::PowerAttack => self.get_character_data()._stat_data._power_attack_damage,
            _ => panic!("get_power not implemented: {:?}", attack_event)
        }
    }

    pub fn set_damage(&mut self, damage: i32) {
        if 0 < damage && self._character_stats._is_alive {
            let character_manager = ptr_as_ref(self._character_manager);
            self._character_stats._hp -= damage;
            if self._character_stats._hp <= 0 {
                character_manager.get_scene_manager().play_audio(&self._character_data.borrow()._audio_data._audio_dead);
                self.set_dead();
            } else {
                character_manager.get_scene_manager().play_audio(&self._character_data.borrow()._audio_data._audio_pain);
                if self._is_player {
                    // TODO: additive animation
                    self.set_action_hit();
                }
            }
        }
    }

    pub fn check_falling_in_water_damage(&mut self) -> bool {
        let dead_zone_height = self.get_character_manager().get_scene_manager().get_dead_zone_height();
        if self.get_position().y <= dead_zone_height {
            self.set_damage(self._character_stats._hp);

            let effect_create_info = EffectCreateInfo {
                _effect_position: Vector3::new(self.get_position().x, dead_zone_height, self.get_position().z),
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

    pub fn check_falling_on_ground_damage(&mut self, last_ground_height: f32) {
        let falling_height = last_ground_height - self.get_position().y;
        if 0.0 < falling_height {
            let falling_damage: i32 = (falling_height - FALLING_HEIGHT).ceil() as i32 * FALLING_DAMAGE_RATIO;
            self.set_hit_damage( falling_damage, None );
        }
    }

    pub fn set_hit_damage(&mut self, damage: i32, attack_dir: Option<&Vector3<f32>>) {
        if 0 < damage {
            self.set_damage(damage);

            if let Some(attack_dir) = attack_dir {
                self._controller.set_hit_direction(&attack_dir);
            }

            let effect_create_info = EffectCreateInfo {
                _effect_position: self.get_position().clone(),
                _effect_data_name: String::from(EFFECT_HIT),
                ..Default::default()
            };

            let character_manager = ptr_as_ref(self._character_manager);
            character_manager.get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
            character_manager.get_scene_manager().play_audio_bank(AUDIO_HIT);
        }
    }

    pub fn set_invincibility(&mut self, invincibility: bool) {
        self._character_stats._invincibility = invincibility;
    }

    pub fn set_dead(&mut self) {
        self._character_stats._is_alive = false;
        self.set_action_dead();
    }

    pub fn set_action_none(&mut self) {
        self.set_action_animation(ActionAnimationState::None, 1.0);
    }

    pub fn set_action_attack(&mut self) {
        if self.is_available_attack() {
            let mut animation_speed: f32 = 1.0;
            if self._is_player {
                let render_object = self._render_object.borrow();
                let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
                if self._character_stats._stamina < STAMINA_ATTACK && animation_play_info._is_animation_end == false {
                    return;
                }

                self._character_stats._stamina -= STAMINA_ATTACK;
                if self._character_stats._stamina < 0.0 {
                    animation_speed = 0.5;
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
                if self._character_stats._stamina < STAMINA_ATTACK && animation_play_info._is_animation_end == false {
                    return;
                }

                self._character_stats._stamina -= STAMINA_POWER_ATTACK;
                if self._character_stats._stamina < 0.0 {
                    animation_speed = 0.5;
                }
            }
            self.set_action_animation(ActionAnimationState::PowerAttack, animation_speed);
        }
    }

    pub fn set_action_hit(&mut self) {
        self.set_action_animation(ActionAnimationState::Hit, 1.0);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_stop();
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
                render_object.set_animation(&animation_data._idle_animation, &animation_info, AnimationLayer::BaseLayer);
            }
            MoveAnimationState::Walk => {
                animation_info._animation_speed = animation_data._walk_animation_speed;
                render_object.set_animation(&animation_data._walk_animation, &animation_info, AnimationLayer::BaseLayer);
            }
            MoveAnimationState::Run => {
                animation_info._animation_speed = animation_data._run_animation_speed;
                render_object.set_animation(&animation_data._run_animation, &animation_info, AnimationLayer::BaseLayer);
            }
            MoveAnimationState::Jump => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._jump_animation_speed;
                render_object.set_animation(&animation_data._jump_animation, &animation_info, AnimationLayer::BaseLayer);
            }
            MoveAnimationState::Roll => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._roll_animation_speed;
                render_object.set_animation(&animation_data._roll_animation, &animation_info, AnimationLayer::BaseLayer);
            }
            MoveAnimationState::RunningJump => {
                animation_info._animation_loop = false;
                animation_info._animation_speed = animation_data._running_jump_animation_speed;
                render_object.set_animation(&animation_data._running_jump_animation, &animation_info, AnimationLayer::BaseLayer);
            }
        }

        self._animation_state._move_animation_state = move_animation_state;
        self.update_animation_layers();
    }

    pub fn set_action_animation(&mut self, action_animation_state: ActionAnimationState, animation_speed: f32) {
        let mut animation_info = AnimationPlayArgs {
            _animation_loop: false,
            _force_animation_setting: true,
            _animation_fade_out_time: 0.1,
            ..Default::default()
        };

        let character_data = self.get_character_data();
        let animation_data = &character_data._animation_data;
        let mut render_object = self._render_object.borrow_mut();
        match action_animation_state {
            ActionAnimationState::None => {
                render_object.set_animation_none(AnimationLayer::ActionLayer);
            },
            ActionAnimationState::Attack => {
                animation_info._animation_speed = animation_data._attack_animation_speed * animation_speed;
                render_object.set_animation(&animation_data._attack_animation, &animation_info, AnimationLayer::ActionLayer);
            }
            ActionAnimationState::Dead => {
                animation_info._animation_speed = animation_data._dead_animation_speed * animation_speed;
                animation_info._animation_fade_out_time = 0.0; // keep end of animation
                render_object.set_animation(&animation_data._dead_animation, &animation_info, AnimationLayer::ActionLayer);
            }
            ActionAnimationState::Hit => {
                animation_info._animation_speed = animation_data._hit_animation_speed * animation_speed;
                render_object.set_animation(&animation_data._hit_animation, &animation_info, AnimationLayer::ActionLayer);
            }
            ActionAnimationState::Pickup => {
                animation_info._animation_speed = animation_speed;
                render_object.set_animation(&animation_data._pickup_animation, &animation_info, AnimationLayer::ActionLayer);
            }
            ActionAnimationState::PowerAttack => {
                animation_info._animation_speed = animation_data._power_attack_animation_speed * animation_speed;
                render_object.set_animation(&animation_data._power_attack_animation, &animation_info, AnimationLayer::ActionLayer);
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
        //self.set_move_direction(&Vector3::zeros());
        self.set_move_animation(MoveAnimationState::Idle);
    }

    pub fn set_move_stop(&mut self) {
        if !self.is_move_state(MoveAnimationState::Roll) {
            self.set_run(false);
            self.set_move_speed(0.0);
            if !self.is_move_state(MoveAnimationState::Idle) && self.is_on_ground() {
                self.set_move_animation(MoveAnimationState::Idle);
            }
        }
    }

    pub fn set_position(&mut self, position: &Vector3<f32>) {
        self._controller.set_position(position);
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self._controller.set_move_speed(speed);
    }

    pub fn set_move_direction(&mut self, move_direction: &Vector3<f32>) {
        if self.is_available_move() {
            self._controller.set_move_direction(move_direction);
        }
    }

    pub fn set_move(&mut self, move_direction: &Vector3<f32>) {
        if self.is_available_move() {
            if self._controller._face_direction.dot(move_direction) < 0.0 {
                self.set_run(false);
            }

            let character_data = self.get_character_data();
            let (move_animation, move_speed) =
                if self._controller._is_running {
                    (MoveAnimationState::Run, character_data._stat_data._run_speed)
                } else {
                    (MoveAnimationState::Walk, character_data._stat_data._walk_speed)
                };

            self.set_move_speed(move_speed);
            self.set_move_direction(move_direction);

            if false == self.is_move_state(move_animation) && self._controller._is_ground {
                self.set_move_animation(move_animation);
            }
        }
    }

    pub fn set_jump(&mut self) {
        if self.is_available_jump() {
            let mut not_enought_stamina = false;
            if self._is_player {
                self._character_stats._stamina -= STAMINA_JUMP;
                not_enought_stamina = self._character_stats._stamina < 0.0;
            }

            let move_anim = if self._controller._is_running && not_enought_stamina == false {
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
            self.set_move_direction(&self._controller._face_direction.clone());
            self.set_move_animation(MoveAnimationState::Roll);
        }
    }

    pub fn update_move_animation_begin_event(&mut self) {
        let character_manager = self.get_character_manager();
        match self._animation_state._move_animation_state {
            MoveAnimationState::None => {
                // nothing
            },
            MoveAnimationState::Idle => {
                // nothing
            },
            MoveAnimationState::Jump => {
                character_manager.get_scene_manager().play_audio_bank(AUDIO_JUMP);
            },
            MoveAnimationState::Roll => {
                self.set_invincibility(true);
            },
            MoveAnimationState::Run => {
                // nothing
            },
            MoveAnimationState::RunningJump => {
                character_manager.get_scene_manager().play_audio_bank(AUDIO_JUMP);
            },
            MoveAnimationState::Walk => {
                // nothing
            }
        }
    }

    pub fn update_move_animation_end_event(&mut self) {
        match self._animation_state._move_animation_state_prev {
            MoveAnimationState::None => {
                // nothing
            },
            MoveAnimationState::Idle => {
                // nothing
            },
            MoveAnimationState::Jump => {
                // nothing
            },
            MoveAnimationState::Roll => {
                self.set_invincibility(false);
            },
            MoveAnimationState::Run => {
                // nothing
            },
            MoveAnimationState::RunningJump => {
                // nothing
            },
            MoveAnimationState::Walk => {
                // nothing
            }
        }
    }

    pub fn update_move_animation_loop_event(&mut self) {
        let character_manager = self.get_character_manager();
        let move_animation = self._animation_state._move_animation_state;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::BaseLayer);
        match move_animation {
            MoveAnimationState::None => {
                // nothing
            },
            MoveAnimationState::Idle => {
                // nothing
            },
            MoveAnimationState::Jump => {
                // nothing
            },
            MoveAnimationState::Roll => {
                if self._is_player && animation_play_info.check_animation_event_time(0.2) {
                    character_manager.get_scene_manager().play_audio_bank(AUDIO_ROLL);
                }
                else if animation_play_info._is_animation_end {
                    self.set_move_idle();
                }
            },
            MoveAnimationState::Run => {
                if self._is_player && (animation_play_info.check_animation_event_time(0.1) || animation_play_info.check_animation_event_time(0.5)) {
                    character_manager.get_scene_manager().play_audio_options(AUDIO_FOOTSTEP, AudioLoop::ONCE, Some(0.5));
                }
            },
            MoveAnimationState::RunningJump => {
                // nothing
            },
            MoveAnimationState::Walk => {
                if self._is_player && (animation_play_info.check_animation_event_time(0.2) || animation_play_info.check_animation_event_time(0.9)) {
                    character_manager.get_scene_manager().play_audio_options(AUDIO_FOOTSTEP, AudioLoop::ONCE, Some(0.5));
                }
            }
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
            ActionAnimationState::None => {
                // nothing
            },
            ActionAnimationState::Attack => {
                // nothing
            },
            ActionAnimationState::Dead => {
                // nothing
            },
            ActionAnimationState::Hit => {
                // nothing
            },
            ActionAnimationState::Pickup => {
                // nothing
            },
            ActionAnimationState::PowerAttack => {
                // nothing
            }
        }
    }

    pub fn update_action_animation_loop_event(&mut self) {
        let character_data = self.get_character_data();
        let action_animation = self._animation_state._action_animation_state;
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(AnimationLayer::ActionLayer);
        match action_animation {
            ActionAnimationState::None => {
                // nothing
            },
            ActionAnimationState::Attack => {
                if animation_play_info.check_animation_event_time(character_data._stat_data._attack_event_time) {
                    self._animation_state._attack_event = ActionAnimationState::Attack;
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            },
            ActionAnimationState::Dead => {
                if self._is_player && animation_play_info._is_animation_end {
                    // resurrection
                    self.initialize_character(
                        &self.get_character_manager().get_game_scene_manager().get_spawn_point().clone(),
                        &self._controller._rotation.clone(),
                        &self._controller._scale.clone(),
                    );
                }
            },
            ActionAnimationState::Hit => {
                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            },
            ActionAnimationState::Pickup => {
                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            },
            ActionAnimationState::PowerAttack => {
                if animation_play_info.check_animation_event_time(character_data._stat_data._power_attack_event_time) {
                    self.get_character_manager().get_scene_manager().play_audio_bank(AUDIO_ATTACK);
                    self._animation_state._attack_event = ActionAnimationState::PowerAttack;
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
        }
    }

    pub fn update_action_animation_end_event(&mut self) {
        match self._animation_state._action_animation_state_prev {
            ActionAnimationState::None => {
                // nothing
            },
            ActionAnimationState::Attack => {
                // nothing
            },
            ActionAnimationState::Dead => {
                // nothing
            },
            ActionAnimationState::Hit => {
                // nothing
            },
            ActionAnimationState::Pickup => {
                // nothing
            },
            ActionAnimationState::PowerAttack => {
                // nothing
            }
        }
    }

    pub fn update_action_keyframe_event(&mut self) {
        self._animation_state._attack_event = ActionAnimationState::None;

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

        // set additive animation layer
        if self.is_action(ActionAnimationState::Attack) || self.is_action(ActionAnimationState::PowerAttack) || self.is_action(ActionAnimationState::Hit) {
            if !self.is_move_state(MoveAnimationState::Idle) && !self.is_move_state(MoveAnimationState::None) {
                render_object.set_animation_layers(
                    self._character_data.borrow()._animation_data._upper_animation_layer.as_ptr(),
                    AnimationLayer::ActionLayer
                );
            }
        }
    }

    pub fn update_character(
        &mut self,
        scene_manager: &SceneManager,
        collision_objects: &BlockArray<'a>,
        player: &Character<'a>,
        delta_time: f32
    ) {
        let was_on_ground = self.is_on_ground();
        let last_ground_height = self._controller.get_last_ground_position().y;

        // update animation key frames
        self.update_move_keyframe_event();
        self.update_action_keyframe_event();

        // behavior
        if false == self._is_player && self._character_stats._is_alive {
            self._behavior.update_behavior(ptr_as_mut(self), player, delta_time);
        }

        // update stats - stamina
        let owner = ptr_as_ref(self);
        self._character_stats.update_stamina(owner, delta_time);
        if owner.is_move_state(MoveAnimationState::Run) {
            if self._character_stats._stamina == 0.0 {
                self.set_run(false);
            }
        }

        // controller
        self._controller.update_character_controller(
            owner,
            scene_manager.get_height_map_data(),
            collision_objects,
            &self._character_data.borrow(),
            self._animation_state._move_animation_state,
            &self._render_object.borrow()._collision,
            delta_time
        );

        // falling water or falling on ground
        if self._character_stats._is_alive {
            if self.check_falling_in_water_damage() {
                // falling in water
            } else if !was_on_ground && self.is_on_ground() {
                self.check_falling_on_ground_damage(last_ground_height);
            }
        }
        // transform
        self.update_transform();

        // update weapon
        if self._weapon.is_some() {
            let weapon = self._weapon.as_mut().unwrap();
            let weapon_socket_transform = weapon._weapon_socket.borrow()._transform.clone();
            weapon.update_weapon(&weapon_socket_transform, delta_time);
        }
    }
}