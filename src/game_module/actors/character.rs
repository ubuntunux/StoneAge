use nalgebra::Vector3;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::animation::{AnimationPlayArgs, AnimationPlayInfo};
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};

use crate::game_module::actors::animation_blend_mask::AnimationBlendMasks;
use crate::game_module::actors::character_behavior::{self, BehaviorBase};
use crate::game_module::actors::character_controller::CharacterController;
use crate::game_module::actors::character_data::{ActionAnimationState, CharacterData, MoveAnimationState, MoveDirections};
use crate::game_module::actors::character_manager::CharacterManager;
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::GameSceneManager;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct CharacterProperty {
    pub _hp: i32,
}

pub struct Character<'a> {
    pub _character_manager: *const CharacterManager<'a>,
    pub _character_name: String,
    pub _character_id: u64,
    pub _is_player: bool,
    pub _is_alive: bool,
    pub _is_attack_event: bool,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _character_property: Box<CharacterProperty>,
    pub _controller: Box<CharacterController>,
    pub _behavior: Box<dyn BehaviorBase>,
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
    pub _animation_blend_masks: *const AnimationBlendMasks,
}

impl CharacterProperty {
    pub fn create_character_property() -> CharacterProperty {
        CharacterProperty {
            _hp: 100,
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
            _is_attack_event: false,
            _character_name: String::from(character_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _behavior: character_behavior::create_character_behavior(character_data.borrow()._character_type),
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
        let animation_layer = Character::get_move_animation_layer(move_animation_state);
        match move_animation_state {
            MoveAnimationState::Idle => {
                render_object.set_animation(&self._idle_animation, &animation_info, animation_layer);
            }
            MoveAnimationState::Walk => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._walk_animation, &animation_info, animation_layer);
            }
            MoveAnimationState::Run => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._run_animation, &animation_info, animation_layer);
            }
            MoveAnimationState::Jump => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._jump_animation, &animation_info, animation_layer);
            }
            MoveAnimationState::Roll => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._roll_animation, &animation_info, animation_layer);
            }
            MoveAnimationState::RunningJump => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._running_jump_animation, &animation_info, animation_layer);
            }
            _ => log::info!("Unimplemented move animation: {:?}", move_animation_state)
        }
        self._move_animation_state = move_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn set_action_animation(&mut self, action_animation_state: ActionAnimationState) {
        let mut render_object = self._render_object.borrow_mut();

        // clear animation layer mask
        if !self.is_action(action_animation_state) {
            let animation_layer = self.get_current_action_animation_layer();
            render_object.clear_animation_blend_masks(animation_layer);
        }

        // set action animation
        let mut animation_info = AnimationPlayArgs {
            _animation_loop: false,
            _force_animation_setting: true,
            _animation_fade_out_time: 0.1,
            ..Default::default()
        };
        let animation_layer = Character::get_action_animation_layer(action_animation_state);
        match action_animation_state {
            ActionAnimationState::None => {
                // nothing
            },
            ActionAnimationState::Attack => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._attack_animation, &animation_info, animation_layer);
            }
            ActionAnimationState::Dead => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._dead_animation, &animation_info, animation_layer);
            }
            ActionAnimationState::Hit => {
                animation_info._animation_speed = 2.0;
                render_object.set_animation(&self._hit_animation, &animation_info, animation_layer);
            }
            ActionAnimationState::PowerAttack => {
                animation_info._animation_speed = 1.5;
                render_object.set_animation(&self._power_attack_animation, &animation_info, animation_layer);
            }
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

    pub fn set_run(&mut self, run: bool) {
        self._controller.set_run(run);
    }

    pub fn toggle_run(&mut self) {
        if self.is_move_state(MoveAnimationState::Run) || self.is_move_state(MoveAnimationState::Walk) {
            self._controller.toggle_run();
        }
    }

    pub fn set_move(&mut self, move_direction: MoveDirections) {
        if self.is_available_move() {
            let (move_animation, move_speed) =
                if self._controller._is_running {
                    (MoveAnimationState::Run, PLAYER_RUN_SPEED)
                } else {
                    (MoveAnimationState::Walk, PLAYER_WALK_SPEED)
                };
            self._controller.set_move_direction(move_direction);
            if false == self.is_move_state(move_animation) && self._controller._is_ground {
                self._controller.set_move_speed(move_speed);
                self.set_move_animation(move_animation);
            }
        }
    }

    pub fn set_jump(&mut self) {
        if self.is_available_jump() {
            let (move_anim, move_speed) = if self.is_move_state(MoveAnimationState::Run) {
                (MoveAnimationState::RunningJump, PLAYER_RUN_SPEED)
            } else {
                (MoveAnimationState::Jump, PLAYER_WALK_SPEED)
            };
            self._controller.set_jump_start();
            self._controller.set_move_speed(move_speed);
            self.set_move_animation(move_anim);
        }
    }

    pub fn set_roll(&mut self) {
        if self.is_available_roll() {
            if self.is_move_state(MoveAnimationState::Run) {
                self._controller.set_move_speed(PLAYER_RUN_SPEED);
            } else {
                self._controller.set_move_speed(PLAYER_ROLL_SPEED);
            }
            self.set_move_animation(MoveAnimationState::Roll);
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

    pub fn update_move_keyframe_event(&mut self) {
        let move_animation = self._move_animation_state;
        let animation_layer = Character::get_move_animation_layer(move_animation);
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(animation_layer);
        match move_animation {
            MoveAnimationState::None => {
                // nothing
            },
            MoveAnimationState::Idle => {
                // nothing
            },
            MoveAnimationState::Jump => {
                if animation_play_info._is_animation_start {
                    self.get_character_manager().play_audio(AUDIO_ATTACK);
                }
            },
            MoveAnimationState::Roll => {
                if animation_play_info._is_animation_start {
                    self.get_character_manager().play_audio(AUDIO_ATTACK);
                } else if animation_play_info._is_animation_end {
                    self.set_move_idle();
                }
            },
            MoveAnimationState::Run => {
                // nothing
            },
            MoveAnimationState::RunningJump => {
                if animation_play_info._is_animation_start {
                    self.get_character_manager().play_audio(AUDIO_ATTACK);
                }
            },
            MoveAnimationState::Walk => {
                // nothing
            }
        }

        // set idle animation
        if self.is_available_move_idle() {
            self.set_move_idle();
        }
    }

    pub fn update_action_keyframe_event(&mut self) {
        self._is_attack_event = false;

        let action_animation = self._action_animation_state;
        let animation_layer = Character::get_action_animation_layer(action_animation);
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        let animation_play_info = render_object.get_animation_play_info(animation_layer);
        match action_animation {
            ActionAnimationState::None => {
                // nothing
            },
            ActionAnimationState::Attack => {
                let attack_time: f32 = 0.15;
                if animation_play_info._is_animation_start {
                    self.get_character_manager().play_audio(AUDIO_ATTACK);
                } else if animation_play_info._prev_animation_play_time < attack_time && attack_time <= animation_play_info._animation_play_time {
                    self._is_attack_event = true;
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            },
            ActionAnimationState::Dead => {
                if animation_play_info._is_animation_start {
                    self.get_character_manager().play_audio(AUDIO_DEAD);
                }
            },
            ActionAnimationState::Hit => {
                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            },
            ActionAnimationState::PowerAttack => {
                let attack_time: f32 = 1.0;
                if animation_play_info._prev_animation_play_time < attack_time && attack_time <= animation_play_info._animation_play_time {
                    self.get_character_manager().play_audio(AUDIO_ATTACK);
                    self._is_attack_event = true;
                }

                if animation_play_info._is_animation_end {
                    self.set_action_none();
                }
            }
        };
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
        let animation_layer = self.get_current_action_animation_layer();
        if animation_layer == AnimationLayer::AdditiveLayer {
            if self.is_action(ActionAnimationState::Attack) || self.is_action(ActionAnimationState::PowerAttack) {
                if !self.is_move_state(MoveAnimationState::Idle) && !self.is_move_state(MoveAnimationState::None) {
                    render_object.set_animation_blend_masks(
                        &ptr_as_ref(self._animation_blend_masks)._upper_animation_mask,
                        animation_layer,
                    );
                }
            }
        }
    }

    pub fn get_current_action_animation_layer(&self) -> AnimationLayer {
        Character::get_action_animation_layer(self._action_animation_state)
    }

    pub fn get_action_animation_layer(action_anim: ActionAnimationState) -> AnimationLayer {
        match action_anim {
            ActionAnimationState::None |
            ActionAnimationState::Attack |
            ActionAnimationState::PowerAttack => {
                AnimationLayer::AdditiveLayer
            },
            ActionAnimationState::Dead |
            ActionAnimationState::Hit => {
                AnimationLayer::BaseLayer
            }
        }
    }

    pub fn get_current_move_animation_layer(&self) -> AnimationLayer {
        Character::get_move_animation_layer(self._move_animation_state)
    }

    pub fn get_move_animation_layer(_move_anim: MoveAnimationState) -> AnimationLayer {
        AnimationLayer::BaseLayer
    }

    pub fn is_available_attack(&self) -> bool {
        if self.is_available_move() {
            let action_animation_play_info = self.get_animation_play_info(self.get_current_action_animation_layer());
            if self.is_action(ActionAnimationState::None) ||
                self.is_action(ActionAnimationState::Attack) && CONTINUOUS_ATTACK_TIME < action_animation_play_info._animation_play_time {
                return true;
            }
        }
        false
    }

    pub fn is_available_move(&self) -> bool {
        !self.is_move_state(MoveAnimationState::Roll) &&
        !self.is_action(ActionAnimationState::Dead) &&
        !self.is_action(ActionAnimationState::Hit)
    }

    pub fn is_available_move_idle(&self) -> bool {
        !self.is_move_state(MoveAnimationState::Idle) &&
        self._controller.is_move_stopped() &&
        self.is_available_move()
    }

    pub fn is_available_jump(&self) -> bool {
        self._controller._is_ground && self.is_available_move()
    }

    pub fn is_available_roll(&self) -> bool {
        self._controller._is_ground && self.is_available_attack() && !self.is_move_state(MoveAnimationState::Roll)
    }

    pub fn is_speed_running(&self) -> bool {
        self.is_move_state(MoveAnimationState::Run) ||
        self.is_move_state(MoveAnimationState::RunningJump)
    }

    pub fn update_character(&mut self, game_scene_manager: &GameSceneManager, delta_time: f32, player: &Character<'a>) {
        if false == self._is_player {
            self._behavior.update_behavior(ptr_as_mut(self), delta_time, self._controller._is_blocked, player);
        }

        self._controller.update_character_controller(
            game_scene_manager,
            self.is_move_state(MoveAnimationState::Roll),
            &self._render_object.borrow()._bound_box,
            delta_time,
        );
        self.update_transform();
    }
}