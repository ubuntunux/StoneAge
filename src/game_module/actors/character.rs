use nalgebra::Vector3;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::utilities::bounding_box::BoundingBox;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};

use crate::game_module::actors::animation_blend_mask::AnimationBlendMasks;
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::GameSceneManager;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveAnimationState {
    NONE,
    IDLE,
    WALK,
    JUMP,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAnimationState {
    NONE,
    ATTACK,
    HIT,
    DEAD
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveDirections {
    NONE,
    LEFT,
    RIGHT,
    UP,
    DOWN
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SpawnPointData {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>
}

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    UrsusArctos,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _dead_animation_mesh: String,
    pub _idle_animation_mesh: String,
    pub _hit_animation_mesh: String,
    pub _walk_animation_mesh: String,
    pub _jump_animation_mesh: String,
    pub _attack_animation_mesh: String,
    pub _max_hp: i32,
}

pub struct CharacterProperty {
    pub _hp: i32,
}

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_ground: bool,
    pub _is_jump: bool,
    pub _is_blocked: bool,
    pub _move_direction: MoveDirections
}

pub struct CharacterBehavior {
    pub _move_time: f32,
    pub _move_direction: MoveDirections
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Character {
    pub _character_manager: *const CharacterManager,
    pub _character_name: String,
    pub _character_id: u64,
    pub _is_player: bool,
    pub _is_alive: bool,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _character_property: Box<CharacterProperty>,
    pub _controller: Box<CharacterController>,
    pub _behavior: Box<CharacterBehavior>,
    pub _move_animation_state: MoveAnimationState,
    pub _action_animation_state: ActionAnimationState,
    pub _dead_animation: RcRefCell<MeshData>,
    pub _idle_animation: RcRefCell<MeshData>,
    pub _hit_animation: RcRefCell<MeshData>,
    pub _walk_animation: RcRefCell<MeshData>,
    pub _jump_animation: RcRefCell<MeshData>,
    pub _attack_animation: RcRefCell<MeshData>,
    pub _animation_blend_masks: *const AnimationBlendMasks
}

// Implementations
impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::UrsusArctos,
            _model_data_name: String::default(),
            _dead_animation_mesh: String::default(),
            _idle_animation_mesh: String::default(),
            _hit_animation_mesh: String::default(),
            _walk_animation_mesh: String::default(),
            _jump_animation_mesh: String::default(),
            _attack_animation_mesh: String::default(),
            _max_hp: 100,
        }
    }
}

impl CharacterProperty {
    pub fn create_character_property() -> CharacterProperty {
        CharacterProperty {
            _hp: 100,
        }
    }
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _is_jump: false,
            _is_ground: false,
            _is_blocked: false,
            _move_direction: MoveDirections::NONE
        }
    }

    pub fn initialize(&mut self) {
        *self = CharacterController::create_character_controller();
    }

    pub fn is_stop(&self) -> bool {
        self._velocity.x == 0.0 && self._velocity.y == 0.0
    }

    pub fn set_move_walk(&mut self, move_direction: MoveDirections) {
        self._move_direction = move_direction;
    }

    pub fn set_move_jump(&mut self) {
        if self._is_ground {
            self._is_jump = true;
        }
    }

    pub fn get_direction(&self) -> f32 {
        if self._rotation.y.is_sign_positive() { -1.0 } else { 1.0 }
    }

    pub fn set_direction(&mut self, direction: f32) {
        self._rotation.y = direction * std::f32::consts::PI * -0.5;
    }

    pub fn set_on_ground(&mut self, ground_height: f32) {
        self._position.y = ground_height;
        self._is_ground = true;
        self._velocity.y = 0.0;
    }

    pub fn update_character_controller(&mut self, game_scene_manager: &GameSceneManager, actor_bound_box: &BoundingBox, delta_time: f32) {
        let prev_position = self._position.clone_owned();

        // move on ground
        match self._move_direction {
            MoveDirections::LEFT => {
                self._velocity.x = -PLAYER_MOVE_SPEED;
                self._rotation.y = std::f32::consts::PI * 0.5
            },
            MoveDirections::RIGHT => {
                self._velocity.x = PLAYER_MOVE_SPEED;
                self._rotation.y = -std::f32::consts::PI * 0.5
            },
            MoveDirections::UP => {
                self._velocity.x = 0.0;
                self._rotation.y = std::f32::consts::PI;
            },
            MoveDirections::DOWN => {
                self._velocity.x = 0.0;
                self._rotation.y = 0.0;
            },
            _ => {
                self._velocity.x = 0.0;
            }
        }
        self._position.x += self._velocity.x * delta_time;

        if self._is_jump {
            self._velocity.y = PLAYER_JUMP_SPEED;
            self._is_ground = false;
        }

        // fall
        if false == self._is_ground {
            self._velocity.y -= GRAVITY * delta_time;
        }
        self._position.y += self._velocity.y * delta_time;

        // check delta limited - prevent pass block
        {
            let delta = self._position - prev_position;
            if BLOCK_WIDTH < delta.x.abs() {
                self._position.x = prev_position.x + delta.x.signum() * BLOCK_WIDTH;
            }

            if BLOCK_HEIGHT < delta.y.abs() {
                self._position.y = prev_position.y + delta.y.signum() * BLOCK_HEIGHT;
            }
        }

        // check collide with block
        let move_delta = self._position - prev_position;
        let bound_box_min = actor_bound_box._min.clone() + move_delta;
        let bound_box_max = actor_bound_box._max.clone() + move_delta;
        self._is_blocked = false;
        self._is_ground = false;
        if let Some(collide_pos) = game_scene_manager.check_is_on_block(&bound_box_min, &bound_box_max) {
            if collide_pos.y <= prev_position.y {
                self.set_on_ground(collide_pos.y);
            } else {
                self._position.x = prev_position.x;
                self._is_blocked = true;
            }
        }

        // check ground
        if self._position.y <= GROUND_HEIGHT {
            self.set_on_ground(GROUND_HEIGHT);
        }

        // reset
        self._is_jump = false;
        self._move_direction = MoveDirections::NONE;
    }
}

impl CharacterBehavior {
    pub fn create_character_behavior() -> CharacterBehavior {
        CharacterBehavior {
            _move_time: 0.0,
            _move_direction: MoveDirections::NONE,
        }
    }

    pub fn update_behavior(&mut self, character: &mut Character, delta_time: f32, toggle_move_direction: bool) {
        let movable: bool = !character.is_action(ActionAnimationState::DEAD) && !character.is_action(ActionAnimationState::HIT);

        if movable {
            self._move_time += delta_time;
            if 2.0 <= self._move_time || toggle_move_direction {
                self._move_direction = if MoveDirections::LEFT == self._move_direction { MoveDirections::RIGHT } else { MoveDirections::LEFT };
                self._move_time = 0.0;
            }

            character.set_move_walk(self._move_direction);
        }
    }
}


impl Character {
    pub fn create_character_instance(
        character_manager: &CharacterManager,
        character_id: u64,
        is_player: bool,
        character_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData>,
        dead_animation: &RcRefCell<MeshData>,
        idle_animation: &RcRefCell<MeshData>,
        hit_animation: &RcRefCell<MeshData>,
        walk_animation: &RcRefCell<MeshData>,
        jump_animation: &RcRefCell<MeshData>,
        attack_animation: &RcRefCell<MeshData>,
        animation_blend_masks: *const AnimationBlendMasks,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
    ) -> Character {
        let mut character = Character {
            _character_manager: character_manager,
            _character_id: character_id,
            _is_player: is_player,
            _is_alive: true,
            _character_name: String::from(character_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _behavior: Box::new(CharacterBehavior::create_character_behavior()),
            _move_animation_state: MoveAnimationState::NONE,
            _action_animation_state: ActionAnimationState::NONE,
            _dead_animation: dead_animation.clone(),
            _idle_animation: idle_animation.clone(),
            _hit_animation: hit_animation.clone(),
            _walk_animation: walk_animation.clone(),
            _jump_animation: jump_animation.clone(),
            _attack_animation: attack_animation.clone(),
            _animation_blend_masks: animation_blend_masks,
        };
        character._controller._position.clone_from(position);
        character._controller._rotation.clone_from(rotation);
        character._controller._scale.clone_from(scale);
        character
    }

    pub fn get_character_manager(&self) -> &CharacterManager {
        ptr_as_ref(self._character_manager)
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager {
        ptr_as_mut(self._character_manager)
    }

    pub fn get_character_id(&self) -> u64 { self._character_id }

    pub fn set_move_animation(&mut self, move_animation_state: MoveAnimationState) {
        let mut animation_info = AnimationPlayArgs {
            ..Default::default()
        };
        let mut render_object = self._render_object.borrow_mut();
        match move_animation_state {
            MoveAnimationState::IDLE => {
                render_object.set_animation(&self._idle_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::WALK => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._walk_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::JUMP => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._jump_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            _ => ()
        }
        self._move_animation_state = move_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn set_action_animation(&mut self, action_animation_state: ActionAnimationState) {
        let mut animation_info = AnimationPlayArgs {
            _animation_loop: false,
            _force_animation_setting: true,
            _animation_fade_out_time: 0.1,
            ..Default::default()
        };
        let mut render_object = self._render_object.borrow_mut();
        match action_animation_state {
            ActionAnimationState::ATTACK => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._attack_animation, &animation_info, AnimationLayer::AdditiveLayer);
            },
            ActionAnimationState::DEAD => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._dead_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            ActionAnimationState::HIT => {
                animation_info._animation_speed = 2.0;
                render_object.set_animation(&self._hit_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            _ => ()
        }
        self._action_animation_state = action_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._move_animation_state
    }

    pub fn set_move_none(&mut self) {
        self.set_move_animation(MoveAnimationState::NONE);
    }

    pub fn set_move_idle(&mut self) {
        self.set_move_animation(MoveAnimationState::IDLE);
    }

    pub fn set_move_walk(&mut self, move_direction: MoveDirections) {
        self._controller.set_move_walk(move_direction);
        if false == self.is_move_state(MoveAnimationState::WALK) && self._controller._is_ground {
            self.set_move_animation(MoveAnimationState::WALK);
        }
    }

    pub fn set_move_jump(&mut self) {
        if self._controller._is_ground {
            self._controller.set_move_jump();
            self.set_move_animation(MoveAnimationState::JUMP);
        }
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._action_animation_state
    }

    pub fn set_action_none(&mut self) {
        self.set_action_animation(ActionAnimationState::NONE);
    }

    pub fn set_action_attack(&mut self) {
        let additive_animation_play_info = self.get_animation_play_info(AnimationLayer::AdditiveLayer);
        if self.is_action(ActionAnimationState::NONE) || self.is_action(ActionAnimationState::ATTACK) && CONTINUOUS_ATTACK_TIME < additive_animation_play_info._animation_play_time {
            self.set_action_animation(ActionAnimationState::ATTACK);
            self.get_character_manager().play_audio(AUDIO_ATTACK);
        }
    }

    pub fn set_action_hit(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::HIT);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::DEAD);
        self.get_character_manager().play_audio(AUDIO_DEAD);
    }

    pub fn get_animation_play_info(&self, layer: AnimationLayer) -> &AnimationPlayInfo {
        &ptr_as_ref(self._render_object.as_ptr())._animation_play_infos[layer as usize]
    }

    pub fn is_attacking(&self) -> bool {
        if self.is_action(ActionAnimationState::ATTACK) {
            let animation_play_info = self.get_animation_play_info(AnimationLayer::AdditiveLayer);
            let attack_time: f32 = 0.15;
            return animation_play_info._prev_animation_play_time < attack_time && attack_time <= animation_play_info._animation_play_time;
        }
        false
    }

    pub fn get_attack_point(&self) -> Vector3<f32> {
        self._controller._position + Vector3::new(self._controller.get_direction(), 1.0, 0.0)
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self._controller._position
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._bound_box.collide_in_radius(pos)
    }

    pub fn get_power(&self) -> i32 {
        50
    }

    pub fn set_damage(&mut self, attack_point: Vector3<f32>, damage: i32) {
        self._character_property._hp -= damage;
        if self._character_property._hp <= 0 {
            self.set_dead();
        } else {
            self.set_action_hit();
        }

        let effect_create_info = EffectCreateInfo {
            _effect_position: attack_point.clone_owned(),
            _effect_data_name: String::from("effect_test"),
            ..Default::default()
        };
        self.get_character_manager().play_effect(EFFECT_HIT, &effect_create_info);
        self.get_character_manager().play_audio(AUDIO_HIT);
    }

    pub fn set_dead(&mut self) {
        self._is_alive = false;
        self.set_action_dead();
    }

    pub fn update_transform(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&self._controller._position);
        render_object._transform_object.set_rotation(&self._controller._rotation);
        render_object._transform_object.set_scale(&self._controller._scale);
    }

    pub fn update_animation_blend_masks(&self) {
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        if self.is_action(ActionAnimationState::ATTACK) {
            let additive_animation_play_info = render_object.get_animation_play_info(AnimationLayer::AdditiveLayer);
            if false == additive_animation_play_info._is_animation_end {
                if self.is_move_state(MoveAnimationState::IDLE) {
                    render_object.clear_animation_blend_masks(AnimationLayer::AdditiveLayer);
                } else {
                    render_object.set_animation_blend_masks(
                        &ptr_as_ref(self._animation_blend_masks)._upper_animation_mask,
                        AnimationLayer::AdditiveLayer,
                    );
                }
            }
        }
    }

    pub fn update_character(&mut self, game_scene_manager: &GameSceneManager, delta_time: f32) {
        if false == self._is_player {
            self._behavior.update_behavior(ptr_as_mut(self), delta_time, self._controller._is_blocked);
        }

        self._controller.update_character_controller(game_scene_manager, &self._render_object.borrow()._bound_box, delta_time);
        self.update_transform();

        // update action animations
        if self.is_action(ActionAnimationState::ATTACK) {
            if self.get_animation_play_info(AnimationLayer::AdditiveLayer)._is_animation_end {
                self.set_action_none();
            }
        } else if self.is_action(ActionAnimationState::HIT) {
            if self.get_animation_play_info(AnimationLayer::BaseLayer)._is_animation_end {
                self.set_action_none();
            }
        }

        // update move animation
        let is_movable_action: bool = self.is_action(ActionAnimationState::NONE) || self.is_action(ActionAnimationState::ATTACK);
        if is_movable_action && !self.is_move_state(MoveAnimationState::IDLE) && self._controller.is_stop() {
            self.set_move_idle();
        }
    }
}