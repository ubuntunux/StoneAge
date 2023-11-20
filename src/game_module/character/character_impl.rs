use nalgebra::{Vector3};
use rust_engine_3d::scene::animation::AnimationPlayArgs;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::character::character::*;
use crate::game_module::game_constants::{GRAVITY, GROUND_HEIGHT, PLAYER_JUMP_SPEED, PLAYER_MOVE_SPEED};


impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::UrsusArctos,
            _model_data_name: String::default(),
            _idle_animation_mesh: String::default(),
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
            _hp: 0.0,
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
            _is_ground: true,
            _move_direction: 0.0
        }
    }

    pub fn initialize(&mut self) {
        self._position = Vector3::zeros();
        self._rotation = Vector3::zeros();
        self._scale = Vector3::new(1.0, 1.0, 1.0);
        self._velocity = Vector3::zeros();
        self._is_ground = true;
        self._is_jump = false;
        self._move_direction = 0.0;
    }

    pub fn is_stop(&self) -> bool {
        self._velocity.x == 0.0 && self._velocity.y == 0.0
    }

    pub fn set_move_walk(&mut self, is_left: bool) {
        self._move_direction = if is_left { -1.0 } else { 1.0 };
    }

    pub fn set_move_jump(&mut self) {
        if self._is_ground {
            self._is_jump = true;
        }
    }

    pub fn update_character_controller(&mut self, delta_time: f32) {
        if 0.0 != self._move_direction {
            self._velocity.x = self._move_direction * PLAYER_MOVE_SPEED;
            self._position.x += self._velocity.x * delta_time;
            self._rotation.y = self._move_direction * std::f32::consts::PI * -0.5;
        } else {
            self._velocity.x = 0.0;
        }

        if self._is_jump {
            self._velocity.y = PLAYER_JUMP_SPEED;
            self._is_ground = false;
        }

        if false == self._is_ground {
            self._velocity.y -= GRAVITY * delta_time;
            self._position.y += self._velocity.y * delta_time;
            if self._position.y <= GROUND_HEIGHT {
                self._position.y = GROUND_HEIGHT;
                self._is_ground = true;
                self._velocity.y = 0.0;
            }
        }

        // reset
        self._is_jump = false;
        self._move_direction = 0.0;
    }
}


impl Character {
    pub fn create_character_instance(
        character_id: u64,
        character_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData>,
        idle_animation: &RcRefCell<MeshData>,
        walk_animation: &RcRefCell<MeshData>,
        jump_animation: &RcRefCell<MeshData>,
        attack_animation: &RcRefCell<MeshData>
    ) -> Character {
        Character {
            _character_id: character_id,
            _character_name: String::from(character_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _move_animation_state: MoveAnimationState::NONE,
            _action_animation_state: ActionAnimationState::NONE,
            _idle_animation: idle_animation.clone(),
            _walk_animation: walk_animation.clone(),
            _jump_animation: jump_animation.clone(),
            _attack_animation: attack_animation.clone(),
        }
    }
    pub fn get_character_id(&self) -> u64 { self._character_id }
    pub fn get_character_controller(&self) -> &CharacterController {
        &self._controller
    }
    pub fn get_character_controller_mut(&mut self) -> &mut CharacterController { &mut self._controller }

    pub fn set_move_animation(&mut self, move_animation_state: MoveAnimationState) {
        let mut animation_info = AnimationPlayArgs::default();
        let mut render_object = self._render_object.borrow_mut();
        match move_animation_state {
            MoveAnimationState::IDLE => {
                render_object.set_animation(&self._idle_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::WALK => {
                render_object.set_animation(&self._walk_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::JUMP => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._jump_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            _ => ()
        }
        self._move_animation_state = move_animation_state;
    }

    pub fn set_action_animation(&mut self, action_animation_state: ActionAnimationState) {
        let mut animation_info = AnimationPlayArgs::default();
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Hips"), 0.5);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Spine"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Spine1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Spine2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Neck"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Head"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:HeadTop_End"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftEye"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightEye"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Hair1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Hair2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Hair3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Hair4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftShoulder"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftArm"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftForeArm"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHand"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandThumb1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandThumb2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandThumb3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandThumb4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandIndex1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandIndex2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandIndex3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandIndex4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandMiddle1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandMiddle2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandMiddle3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandMiddle4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandRing1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandRing2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandRing3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandRing4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandPinky1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandPinky2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandPinky3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:LeftHandPinky4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightShoulder"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightArm"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightForeArm"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHand"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandThumb1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandThumb2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandThumb3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandThumb4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandIndex1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandIndex2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandIndex3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandIndex4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandMiddle1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandMiddle2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandMiddle3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandMiddle4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandRing1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandRing2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandRing3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandRing4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandPinky1"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandPinky2"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandPinky3"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:RightHandPinky4"), 1.0);
        animation_info._animation_blend_masks.insert(String::from("mixamorig:Weapon"), 1.0);

        let mut render_object = self._render_object.borrow_mut();
        match action_animation_state {
            ActionAnimationState::ATTACK => {
                animation_info._animation_loop = false;
                animation_info._force_animation_setting = true;
                render_object.set_animation(&self._attack_animation, &animation_info, AnimationLayer::AdditiveLayer);
            },
            _ => ()
        }
        self._action_animation_state = action_animation_state;
    }

    pub fn set_move_idle(&mut self) {
        self.set_move_animation(MoveAnimationState::IDLE);
    }

    pub fn set_move_walk(&mut self, is_left: bool) {
        self.get_character_controller_mut().set_move_walk(is_left);
        if MoveAnimationState::WALK != self._move_animation_state &&
            self.get_character_controller()._is_ground {
            self.set_move_animation(MoveAnimationState::WALK);
        }
    }

    pub fn set_move_jump(&mut self) {
        if self.get_character_controller()._is_ground {
            self.get_character_controller_mut().set_move_jump();
            self.set_move_animation(MoveAnimationState::JUMP);
        }
    }

    pub fn set_action_idle(&mut self) {
        self.set_action_animation(ActionAnimationState::NONE);
    }

    pub fn set_action_attack(&mut self) {
        self.set_action_animation(ActionAnimationState::ATTACK);
    }

    pub fn update_transform(&mut self) {
        let controller = self.get_character_controller();
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&controller._position);
        render_object._transform_object.set_rotation(&controller._rotation);
        render_object._transform_object.set_scale(&controller._scale);
    }

    pub fn update_character(&mut self, delta_time: f32) {
        self.get_character_controller_mut().update_character_controller(delta_time);
        self.update_transform();

        if MoveAnimationState::IDLE != self._move_animation_state &&
            self.get_character_controller().is_stop() {
            self.set_move_idle();
        }

        if ActionAnimationState::ATTACK == self._action_animation_state {
            if self._render_object.borrow()._animation_play_infos[AnimationLayer::AdditiveLayer as usize]._is_animation_end {
                self.set_action_idle();
            }
        }
    }
}