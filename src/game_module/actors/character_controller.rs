use nalgebra::Vector3;
use rust_engine_3d::scene::collision::{CollisionCreateInfo, CollisionData, CollisionType};
use rust_engine_3d::utilities::system::ptr_as_ref;

use crate::game_module::actors::character_data::{CharacterData, MoveAnimationState};
use crate::game_module::game_constants::{GRAVITY, GROUND_HEIGHT, MOVE_LIMIT};
use crate::game_module::game_scene_manager::GameSceneManager;

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_ground: bool,
    pub _is_running: bool,
    pub _is_jump_start: bool,
    pub _is_cliff: bool,
    pub _move_speed: f32,
    pub _is_blocked: bool,
    pub _face_direction: Vector3<f32>,
    pub _move_direction: Vector3<f32>,
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _is_jump_start: false,
            _move_speed: 0.0,
            _is_running: false,
            _is_ground: false,
            _is_blocked: false,
            _is_cliff: false,
            _face_direction: Vector3::zeros(),
            _move_direction: Vector3::zeros(),
        }
    }

    pub fn initialize_controller(
        &mut self,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>
    ) {
        self._position = position.clone();
        self._rotation = rotation.clone();
        self._scale = scale.clone();
        self._velocity = Vector3::zeros();
        self._is_jump_start = false;
        self._move_speed = 0.0;
        self._is_running = false;
        self._is_ground = false;
        self._is_blocked = false;
        self._is_cliff = false;
    }

    pub fn is_on_ground(&self) -> bool {
        self._is_ground && !self._is_jump_start
    }

    pub fn set_run(&mut self, is_running: bool) {
        self._is_running = is_running;
    }

    pub fn toggle_run(&mut self) {
        self._is_running = !self._is_running;
    }

    pub fn set_move_direction(&mut self, move_direction: &Vector3<f32>) {
        self._move_direction.clone_from(move_direction);
        if move_direction.x != 0.0 || move_direction.y != 0.0 || move_direction.z != 0.0 {
            self._face_direction.clone_from(move_direction);
        }
    }

    pub fn set_jump_start(&mut self) {
        self._is_jump_start = true;
    }

    pub fn set_move_speed(&mut self, move_speed: f32) {
        self._move_speed = move_speed;
    }

    pub fn set_direction(&mut self, direction: &Vector3<f32>) {
        self._rotation.y = direction.z.atan2(-direction.x) + std::f32::consts::PI * 0.5;
    }

    pub fn set_on_ground(&mut self, ground_height: f32) {
        self._position.y = ground_height;
        self._is_ground = true;
        self._velocity.y = 0.0;
    }

    pub fn update_character_controller(
        &mut self,
        _is_player: bool,
        game_scene_manager: &GameSceneManager,
        character_data: &CharacterData,
        move_animation: MoveAnimationState,
        actor_collision: &CollisionData,
        delta_time: f32,
    ) {
        let prev_position = self._position.clone_owned();

        // move on ground
        let mut move_direction = if MoveAnimationState::Roll == move_animation {
            self._face_direction.clone()
        } else {
            self._move_direction.clone()
        };

        if move_direction.x != 0.0 || move_direction.z != 0.0 {
            move_direction.normalize_mut();
            self._velocity.x = move_direction.x * self._move_speed;
            self._velocity.z = move_direction.z * self._move_speed;
            self._position.x += self._velocity.x * delta_time;
            self._position.z += self._velocity.z * delta_time;
        } else {
            self._velocity.x = 0.0;
            self._velocity.z = 0.0;
        }

        self.set_direction(&move_direction);

        // jump
        if self._is_jump_start {
            self._velocity.y = character_data._jump_speed;
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

            if MOVE_LIMIT < delta.z.abs() {
                self._position.z = prev_position.z + delta.z.signum() * MOVE_LIMIT;
            }
        }

        // check collide with block
        let move_delta = self._position - prev_position;
        let prev_bound_box_min = actor_collision._bounding_box._min.clone_owned();
        let prev_bound_box_max = actor_collision._bounding_box._max.clone_owned();
        let current_actor_collision_info = CollisionCreateInfo {
            _collision_type: actor_collision._collision_type,
            _location: (prev_bound_box_min + prev_bound_box_max) * 0.5 + move_delta,
            _radius: (prev_bound_box_max.x - prev_bound_box_min.x) * 0.5,
            _height: prev_bound_box_max.y - prev_bound_box_min.y
        };
        let current_actor_collision = CollisionData::create_collision(&current_actor_collision_info);
        let bound_box_min = &current_actor_collision._bounding_box._min;
        let bound_box_max = &current_actor_collision._bounding_box._max;

        // reset flags
        self._is_cliff = true;
        self._is_blocked = false;
        self._is_ground = false;

        // check ground and side
        for (_key, block) in game_scene_manager.get_blocks().iter() {
            let block = ptr_as_ref(block.as_ptr());
            let block_render_object = ptr_as_ref(block._render_object.as_ptr());
            let block_collision_type = block_render_object._collision._collision_type;
            let block_bound_box = &block_render_object._collision._bounding_box;
            let block_location = &block_bound_box._center;

            // check collide with block
            if current_actor_collision.collide_collision(&block_render_object._collision) {
                if self._velocity.y <= 0.0 && block_bound_box._max.y <= prev_position.y {
                    self.set_on_ground(block_bound_box._max.y);
                } else {
                    const SMOOTH_COLLIDE: bool = false;
                    if block_collision_type == CollisionType::BOX {
                        if SMOOTH_COLLIDE {
                            if block_bound_box._min.z <= prev_bound_box_max.z && prev_bound_box_min.z <= block_bound_box._max.z {
                                self._position.x = prev_position.x;
                            } else {
                                self._position.z = prev_position.z;
                            }
                        } else {
                            self._position.x = prev_position.x;
                            self._position.z = prev_position.z;
                        }

                        self._is_blocked = true;
                    } else if block_collision_type == CollisionType::CYLINDER {
                        let block_to_player = Vector3::new(self._position.x - block_location.x, 0.0, self._position.z - block_location.z).normalize();
                        if block_to_player.dot(&move_delta.normalize()) < 0.0 {
                            if SMOOTH_COLLIDE {
                                let dist = Vector3::new(prev_position.x - block_location.x, 0.0, prev_position.z - block_location.z).norm();
                                let new_pos = block_to_player * dist + block_location;
                                self._position.x = new_pos.x;
                                self._position.z = new_pos.z;
                            } else {
                                self._position.x = prev_position.x;
                                self._position.z = prev_position.z;
                            }
                            self._is_blocked = true;
                        }
                    } else {
                        panic!("not implemented");
                    }
                }
            }
        }

        // check cliff
        // TODO: check cylinder, box
        let front_bottom_point: Vector3<f32> = Vector3::new(
            if 0.0 < move_delta.x { bound_box_max.x + 0.1 } else { bound_box_min.x - 0.1 },
            bound_box_min.y - 0.1,
            if 0.0 < move_delta.z { bound_box_max.z + 0.1 } else { bound_box_min.z - 0.1 }
        );
        for (_key, block) in game_scene_manager.get_blocks().iter() {
            let block = ptr_as_ref(block.as_ptr());
            let block_render_object = ptr_as_ref(block._render_object.as_ptr());
            let block_bound_box = &block_render_object._bounding_box;
            // check front bottom block
            if false == self._is_ground ||
                0.0 == move_delta.x ||
                0.0 == move_delta.z ||
                0.0 < move_delta.y ||
                block_bound_box.collide_point(&front_bottom_point) {
                self._is_cliff = false;
            }
        }

        // check ground
        if self._position.y <= GROUND_HEIGHT {
            self.set_on_ground(GROUND_HEIGHT);
        }

        // reset
        self._is_jump_start = false;
    }
}