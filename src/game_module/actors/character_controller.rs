use nalgebra::Vector3;
use rust_engine_3d::utilities::bounding_box::BoundingBox;

use crate::game_module::actors::character_data::MoveDirections;
use crate::game_module::game_constants::{BLOCK_TOLERANCE, GRAVITY, GROUND_HEIGHT, MOVE_LIMIT, PLAYER_JUMP_SPEED, PLAYER_WALK_SPEED};
use crate::game_module::game_scene_manager::GameSceneManager;

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_ground: bool,
    pub _is_jump_start: bool,
    pub _move_speed: f32,
    pub _is_blocked: bool,
    pub _face_direction: MoveDirections,
    pub _move_direction: MoveDirections,
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _is_jump_start: false,
            _move_speed: PLAYER_WALK_SPEED,
            _is_ground: false,
            _is_blocked: false,
            _face_direction: MoveDirections::NONE,
            _move_direction: MoveDirections::NONE,
        }
    }

    pub fn initialize(&mut self) {
        *self = CharacterController::create_character_controller();
    }

    pub fn is_move_stopped(&self) -> bool {
        self._velocity.x == 0.0 && self._is_ground && !self._is_jump_start
    }

    pub fn set_move_direction(&mut self, move_direction: MoveDirections) {
        self._move_direction = move_direction;
        if move_direction != MoveDirections::NONE {
            self._face_direction = move_direction;
        }
    }

    pub fn set_jump_start(&mut self) {
        self._is_jump_start = true;
    }

    pub fn set_move_speed(&mut self, rolling_speed: f32) {
        self._move_speed = rolling_speed;
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
        is_rolling: bool,
        actor_bound_box: &BoundingBox,
        delta_time: f32,
    ) {
        let prev_position = self._position.clone_owned();

        // move on ground
        let move_direction = if is_rolling { self._face_direction } else { self._move_direction };
        match move_direction {
            MoveDirections::LEFT => {
                self._velocity.x = -self._move_speed;
                self._rotation.y = std::f32::consts::PI * 0.5
            }
            MoveDirections::RIGHT => {
                self._velocity.x = self._move_speed;
                self._rotation.y = -std::f32::consts::PI * 0.5
            }
            MoveDirections::UP => {
                self._velocity.x = 0.0;
                self._rotation.y = std::f32::consts::PI;
            }
            MoveDirections::DOWN => {
                self._velocity.x = 0.0;
                self._rotation.y = 0.0;
            }
            _ => {
                self._velocity.x = 0.0;
            }
        }
        self._position.x += self._velocity.x * delta_time;

        // jump
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