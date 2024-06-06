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
    None,
    Idle,
    Jump,
    Roll,
    Run,
    RunningJump,
    Walk,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAnimationState {
    None,
    Attack,
    Dead,
    Hit,
    PowerAttack,
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
    pub _attack_animation_mesh: String,
    pub _dead_animation_mesh: String,
    pub _idle_animation_mesh: String,
    pub _hit_animation_mesh: String,
    pub _jump_animation_mesh: String,
    pub _power_attack_animation_mesh: String,
    pub _roll_animation_mesh: String,
    pub _run_animation_mesh: String,
    pub _running_jump_animation_mesh: String,
    pub _walk_animation_mesh: String,
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
    pub _is_jump_start: bool,
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

pub struct Character<'a> {
    pub _character_manager: *const CharacterManager<'a>,
    pub _character_name: String,
    pub _character_id: u64,
    pub _is_player: bool,
    pub _is_alive: bool,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _character_property: Box<CharacterProperty>,
    pub _controller: Box<CharacterController>,
    pub _behavior: Box<CharacterBehavior>,
    pub _move_animation_state: MoveAnimationState,
    pub _action_animation_state: ActionAnimationState,
    pub _attack_animation: RcRefCell<MeshData>,
    pub _dead_animation: RcRefCell<MeshData>,
    pub _hit_animation: RcRefCell<MeshData>,
    pub _idle_animation: RcRefCell<MeshData>,
    pub _jump_animation: RcRefCell<MeshData>,
    pub _power_attack_animation: RcRefCell<MeshData>,
    pub _roll_animation: RcRefCell<MeshData>,
    pub _run_animation: RcRefCell<MeshData>,
    pub _running_jump_animation: RcRefCell<MeshData>,
    pub _walk_animation: RcRefCell<MeshData>,
    pub _animation_blend_masks: *const AnimationBlendMasks
}

// Implementations
impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::UrsusArctos,
            _model_data_name: String::default(),
            _attack_animation_mesh: String::default(),
            _dead_animation_mesh: String::default(),
            _hit_animation_mesh: String::default(),
            _idle_animation_mesh: String::default(),
            _jump_animation_mesh: String::default(),
            _power_attack_animation_mesh: String::default(),
            _roll_animation_mesh: String::default(),
            _run_animation_mesh: String::default(),
            _running_jump_animation_mesh: String::default(),
            _walk_animation_mesh: String::default(),
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
            _is_jump_start: false,
            _is_ground: false,
            _is_blocked: false,
            _move_direction: MoveDirections::NONE
        }
    }

    pub fn initialize(&mut self) {
        *self = CharacterController::create_character_controller();
    }

    pub fn is_stopped(&self) -> bool {
        self._velocity.x == 0.0 && self._velocity.y == 0.0
    }

    pub fn set_move_direction(&mut self, move_direction: MoveDirections) {
        self._move_direction = move_direction;
    }

    pub fn set_jump_start(&mut self) {
        if self._is_ground {
            self._is_jump_start = true;
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

    pub fn update_character_controller(
        &mut self,
        game_scene_manager: &GameSceneManager,
        is_speed_running: bool,
        actor_bound_box: &BoundingBox,
        delta_time: f32
    ) {
        let prev_position = self._position.clone_owned();

        // move on ground
        match self._move_direction {
            MoveDirections::LEFT => {
                self._velocity.x = -(if is_speed_running { PLAYER_RUN_SPEED } else { PLAYER_MOVE_SPEED });
                self._rotation.y = std::f32::consts::PI * 0.5
            },
            MoveDirections::RIGHT => {
                self._velocity.x = if is_speed_running { PLAYER_RUN_SPEED } else { PLAYER_MOVE_SPEED };
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

        if self._is_jump_start {
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
            if MOVE_LIMIT < delta.x.abs() {
                self._position.x = prev_position.x + delta.x.signum() * MOVE_LIMIT;
            }

            if MOVE_LIMIT < delta.y.abs() {
                self._position.y = prev_position.y + delta.y.signum() * MOVE_LIMIT;
            }
        }

        // check collide with block
        let move_delta = self._position - prev_position;
        let bound_box_min = actor_bound_box._min.clone() + move_delta;
        let bound_box_max = actor_bound_box._max.clone() + move_delta;
        self._is_blocked = false;
        self._is_ground = false;
        for (_key, block) in game_scene_manager.get_blocks().iter() {
            let block_ref = block.borrow();
            let render_object_ref = block_ref._render_object.borrow();
            if render_object_ref._bound_box.collide_bound_box_xy(&bound_box_min, &bound_box_max) {
                let collide_pos_y = render_object_ref._bound_box._max.y;
                if self._velocity.y <= 0.0 && (collide_pos_y <= prev_position.y || (collide_pos_y - BLOCK_TOLERANCE) <= self._position.y) {
                    self.set_on_ground(collide_pos_y);
                } else {
                    // side
                    self._position.x = prev_position.x;
                    self._is_blocked = true;
                }
            }
        }

        // check ground
        if self._position.y <= GROUND_HEIGHT {
            self.set_on_ground(GROUND_HEIGHT);
        }

        // reset
        self._is_jump_start = false;
        self.set_move_direction(MoveDirections::NONE);
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
        let movable: bool = !character.is_action(ActionAnimationState::Dead) && !character.is_action(ActionAnimationState::Hit);

        if movable {
            self._move_time += delta_time;
            if 2.0 <= self._move_time || toggle_move_direction {
                self._move_direction = if MoveDirections::LEFT == self._move_direction { MoveDirections::RIGHT } else { MoveDirections::LEFT };
                self._move_time = 0.0;
            }

            let is_running = false;
            character.set_move(self._move_direction, is_running);
        }
    }
}


impl<'a> Character<'a> {
    pub fn create_character_instance(
        character_manager: &CharacterManager<'a>,
        character_id: u64,
        is_player: bool,
        character_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        attack_animation: &RcRefCell<MeshData>,
        dead_animation: &RcRefCell<MeshData>,
        hit_animation: &RcRefCell<MeshData>,
        idle_animation: &RcRefCell<MeshData>,
        jump_animation: &RcRefCell<MeshData>,
        power_attack_animation: &RcRefCell<MeshData>,
        roll_animation: &RcRefCell<MeshData>,
        run_animation: &RcRefCell<MeshData>,
        running_jump_animation: &RcRefCell<MeshData>,
        walk_animation: &RcRefCell<MeshData>,
        animation_blend_masks: *const AnimationBlendMasks,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
    ) -> Character<'a> {
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
            _move_animation_state: MoveAnimationState::None,
            _action_animation_state: ActionAnimationState::None,
            _attack_animation: attack_animation.clone(),
            _dead_animation: dead_animation.clone(),
            _hit_animation: hit_animation.clone(),
            _idle_animation: idle_animation.clone(),
            _jump_animation: jump_animation.clone(),
            _power_attack_animation: power_attack_animation.clone(),
            _roll_animation: roll_animation.clone(),
            _run_animation: run_animation.clone(),
            _running_jump_animation: running_jump_animation.clone(),
            _walk_animation: walk_animation.clone(),
            _animation_blend_masks: animation_blend_masks,
        };
        character._controller._position.clone_from(position);
        character._controller._rotation.clone_from(rotation);
        character._controller._scale.clone_from(scale);
        character
    }

    pub fn get_character_manager(&self) -> &CharacterManager<'a> {
        ptr_as_ref(self._character_manager)
    }

    pub fn get_character_manager_mut(&self) -> &mut CharacterManager<'a> {
        ptr_as_mut(self._character_manager)
    }

    pub fn get_character_id(&self) -> u64 { self._character_id }

    pub fn set_move_animation(&mut self, move_animation_state: MoveAnimationState) {
        let mut animation_info = AnimationPlayArgs {
            ..Default::default()
        };

        let mut render_object = self._render_object.borrow_mut();
        match move_animation_state {
            MoveAnimationState::Idle => {
                render_object.set_animation(&self._idle_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::Walk => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._walk_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::Run => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._run_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::Jump => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._jump_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::RunningJump => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._running_jump_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            _ => log::info!("Unimplemented move animation: {:?}", move_animation_state)
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
            ActionAnimationState::Attack => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._attack_animation, &animation_info, AnimationLayer::AdditiveLayer);
            },
            ActionAnimationState::Dead => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._dead_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            ActionAnimationState::Hit => {
                animation_info._animation_speed = 2.0;
                render_object.set_animation(&self._hit_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            ActionAnimationState::PowerAttack => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._power_attack_animation, &animation_info, AnimationLayer::AdditiveLayer);
            },
            _ => log::info!("Unimplemented action animation: {:?}", action_animation_state)
        }
        self._action_animation_state = action_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._move_animation_state
    }

    pub fn set_move_none(&mut self) {
        self.set_move_animation(MoveAnimationState::None);
    }

    pub fn set_move_idle(&mut self) {
        self.set_move_animation(MoveAnimationState::Idle);
    }

    pub fn set_move(&mut self, move_direction: MoveDirections, is_running: bool) {
        if self.is_action_available_move() {
            self._controller.set_move_direction(move_direction);
            let move_animation = if is_running { MoveAnimationState::Run } else { MoveAnimationState::Walk };
            if false == self.is_move_state(move_animation) && self._controller._is_ground {
                self.set_move_animation(move_animation);
            }
        }
    }

    pub fn set_jump(&mut self) {
        if self.is_available_jump() {
            self._controller.set_jump_start();
            self.set_move_animation(
                if self.is_move_state(MoveAnimationState::Run) {
                    MoveAnimationState::RunningJump
                } else {
                    MoveAnimationState::Jump
                }
            );
        }
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._action_animation_state
    }

    pub fn set_action_none(&mut self) {
        self.set_action_animation(ActionAnimationState::None);
    }

    pub fn set_action_attack(&mut self) {
        if self.is_available_attack() {
            self.set_action_animation(ActionAnimationState::Attack);
        }
    }

    pub fn set_action_power_attack(&mut self) {
        if self.is_available_attack() {
            self.set_action_animation(ActionAnimationState::PowerAttack);
        }
    }

    pub fn set_action_hit(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::Hit);
    }

    pub fn set_action_dead(&mut self) {
        self.set_move_idle();
        self.set_action_animation(ActionAnimationState::Dead);
    }

    pub fn get_animation_play_info(&self, layer: AnimationLayer) -> &AnimationPlayInfo {
        &ptr_as_ref(self._render_object.as_ptr())._animation_play_infos[layer as usize]
    }

    pub fn update_keyframe_event(&self) -> bool {
        let mut is_attack_event = false;
        let base_animation_play_info = self.get_animation_play_info(AnimationLayer::BaseLayer);
        let additive_animation_play_info = self.get_animation_play_info(AnimationLayer::AdditiveLayer);
        if self.is_action(ActionAnimationState::Attack) {
            let attack_time: f32 = 0.15;
            if additive_animation_play_info._prev_animation_play_time == 0.0 {
                self.get_character_manager().play_audio(AUDIO_ATTACK);
            } else if additive_animation_play_info._prev_animation_play_time < attack_time && attack_time <= additive_animation_play_info._animation_play_time {
                is_attack_event = true;
            }
        } else if self.is_action(ActionAnimationState::PowerAttack) {
            let attack_time: f32 = 1.0;
            if additive_animation_play_info._prev_animation_play_time < attack_time && attack_time <= additive_animation_play_info._animation_play_time {
                self.get_character_manager().play_audio(AUDIO_ATTACK);
                is_attack_event = true;
            }
        } else if self.is_action(ActionAnimationState::Dead) {
            if base_animation_play_info._prev_animation_play_time == 0.0 {
                self.get_character_manager().play_audio(AUDIO_DEAD);
            }
        }

        is_attack_event
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
        match self._action_animation_state {
            ActionAnimationState::Attack => 50,
            ActionAnimationState::PowerAttack => 100,
            _ => 0,
        }
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
        if self.is_action(ActionAnimationState::Attack) || self.is_action(ActionAnimationState::PowerAttack) {
            let additive_animation_play_info = render_object.get_animation_play_info(AnimationLayer::AdditiveLayer);
            if false == additive_animation_play_info._is_animation_end {
                if self.is_move_state(MoveAnimationState::Idle) {
                    render_object.clear_animation_blend_masks(AnimationLayer::AdditiveLayer);
                } else {
                    render_object.set_animation_blend_masks(
                        &ptr_as_ref(self._animation_blend_masks)._upper_animation_mask,
                        AnimationLayer::AdditiveLayer,
                    );
                }
            }
        } else {
            render_object.clear_animation_blend_masks(AnimationLayer::AdditiveLayer);
        }
    }

    pub fn is_available_attack(&self) -> bool {
        let additive_animation_play_info = self.get_animation_play_info(AnimationLayer::AdditiveLayer);
        if self.is_action(ActionAnimationState::None) ||
            self.is_action(ActionAnimationState::Attack) && CONTINUOUS_ATTACK_TIME < additive_animation_play_info._animation_play_time {
            return true;
        }
        false
    }

    pub fn is_action_available_move(&self) -> bool {
        !self.is_action(ActionAnimationState::Dead) && !self.is_action(ActionAnimationState::Hit)
    }

    pub fn is_available_jump(&self) -> bool {
        self._controller._is_ground && self.is_action_available_move()
    }

    pub fn is_speed_running(&self) -> bool {
        self._move_animation_state == MoveAnimationState::Run || self._move_animation_state == MoveAnimationState::RunningJump
    }

    pub fn update_character(&mut self, game_scene_manager: &GameSceneManager, delta_time: f32) {
        if false == self._is_player {
            self._behavior.update_behavior(ptr_as_mut(self), delta_time, self._controller._is_blocked);
        }

        let is_speed_running: bool = self.is_speed_running();
        self._controller.update_character_controller(
            game_scene_manager,
            is_speed_running,
            &self._render_object.borrow()._bound_box,
            delta_time
        );
        self.update_transform();

        // update action animations
        if self.is_action(ActionAnimationState::Attack) {
            if self.get_animation_play_info(AnimationLayer::AdditiveLayer)._is_animation_end {
                self.set_action_none();
            }
        } else if self.is_action(ActionAnimationState::PowerAttack) {
            if self.get_animation_play_info(AnimationLayer::AdditiveLayer)._is_animation_end {
                self.set_action_none();
            }
        } else if self.is_action(ActionAnimationState::Hit) {
            if self.get_animation_play_info(AnimationLayer::BaseLayer)._is_animation_end {
                self.set_action_none();
            }
        }

        // update move animation
        if self.is_action_available_move() && !self.is_move_state(MoveAnimationState::Idle) && self._controller.is_stopped() {
            self.set_move_idle();
        }
    }
}