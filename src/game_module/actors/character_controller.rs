use nalgebra::Vector3;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::height_map::HeightMapData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::begin_block;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::HALF_PI;
use rust_engine_3d::utilities::system::ptr_as_ref;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::{CharacterData, MoveAnimationState};
use crate::game_module::game_constants::{CHARACTER_ROTATION_SPEED, CLIFF_HEIGHT, FALLING_TIME, GRAVITY, MOVE_LIMIT, SLOPE_ANGLE, SLOPE_SPEED};
use crate::game_module::game_scene_manager::BlockArray;

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _last_ground_position: Vector3<f32>,
    pub _last_ground_normal: Vector3<f32>,
    pub _face_direction: Vector3<f32>,
    pub _move_direction: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _move_speed: f32,
    pub _fall_time: f32,
    pub _is_falling: bool,
    pub _is_ground: bool,
    pub _is_running: bool,
    pub _is_jump_start: bool,
    pub _is_jump: bool,
    pub _is_cliff: bool,
    pub _is_blocked: bool
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _last_ground_position: Vector3::zeros(),
            _last_ground_normal: Vector3::new(0.0, 1.0, 0.0),
            _face_direction: Vector3::zeros(),
            _move_direction: Vector3::zeros(),
            _velocity: Vector3::zeros(),
            _move_speed: 0.0,
            _fall_time: 0.0,
            _is_falling: false,
            _is_ground: false,
            _is_running: false,
            _is_jump_start: false,
            _is_jump: false,
            _is_cliff: false,
            _is_blocked: false
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
        self._face_direction = Vector3::zeros();
        self._move_direction = Vector3::zeros();
        self._last_ground_position = position.clone();
        self._last_ground_normal = Vector3::new(0.0, 1.0, 0.0);
        self._velocity = Vector3::zeros();
        self._move_speed = 0.0;
        self._fall_time = 0.0;
        self._is_falling = false;
        self._is_jump_start = false;
        self._is_jump = false;
        self._is_running = false;
        self._is_ground = false;
        self._is_blocked = false;
        self._is_cliff = false;
    }

    pub fn is_falling(&self) -> bool {
        self._is_falling
    }
    pub fn is_on_ground(&self) -> bool {
        self._is_ground && !self._is_jump_start
    }
    pub fn get_last_ground_position(&self) -> &Vector3<f32> {
        &self._last_ground_position
    }
    pub fn get_last_ground_normal(&self) -> &Vector3<f32> {
        &self._last_ground_normal
    }
    pub fn is_slope_ground_normal(&self) -> bool {
        self._last_ground_normal.y < SLOPE_ANGLE
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
    pub fn is_jump(&self) -> bool {
        self._is_jump
    }
    pub fn set_position(&mut self, position: &Vector3<f32>) {
        self._position = position.clone();
    }
    pub fn set_move_speed(&mut self, move_speed: f32) {
        self._move_speed = move_speed;
    }
    pub fn set_direction(&mut self, direction: &Vector3<f32>) {
        self._rotation.y = direction.z.atan2(-direction.x) + HALF_PI;
    }
    pub fn rotate_to_direction(&mut self, direction: &Vector3<f32>, delta_time: f32) {
        let yaw: f32 = direction.z.atan2(-direction.x) + HALF_PI;
        let mut diff: f32 = yaw - self._rotation.y;
        if diff < -std::f32::consts::PI {
            diff += std::f32::consts::PI * 2.0;
        } else if std::f32::consts::PI < diff {
            diff -= std::f32::consts::PI * 2.0;
        }

        if diff.abs() < 0.01 {
            self._rotation.y = yaw;
        } else {
            self._rotation.y += diff.abs().min(CHARACTER_ROTATION_SPEED * delta_time) * diff.signum();
        }
    }

    pub fn set_on_ground(&mut self, ground_height: f32) {
        if self._is_ground == false || self._position.y < ground_height {
            self._position.y = ground_height;
            self._is_ground = true;
            self._is_falling = false;
            self._is_jump = false;
            self._velocity.y = 0.0;
        }
    }

    pub fn update_character_controller<'a>(
        &mut self,
        _owner: &Character,
        height_map_data: &HeightMapData,
        collision_objects: &BlockArray<'a>,
        character_data: &CharacterData,
        move_animation: MoveAnimationState,
        actor_collision: &CollisionData,
        delta_time: f32,
    ) {
        let prev_position = self._position.clone();

        // update fall time
        if self._is_ground {
            self._fall_time = 0.0;
            self._is_falling = false;
        } else {
            self._fall_time += delta_time;
            if FALLING_TIME < self._fall_time {
                self._is_falling = true;
            }
        }

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

        // update rotation
        self.rotate_to_direction(&move_direction, delta_time);

        // jump
        if self._is_jump_start {
            self._velocity.y = character_data._stat_data._jump_speed;
            self._is_ground = false;
            self._is_jump = true;
        }

        // fall
        if false == self._is_ground {
            self._velocity.y -= GRAVITY * delta_time;
        }
        self._position.y += self._velocity.y * delta_time;

        begin_block!("check delta limited - prevent pass block"); {
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

        // reset flags
        let was_on_ground = self._is_ground;
        self._is_cliff = true;
        self._is_blocked = false;
        self._is_ground = false;

        let ground_normal = height_map_data.get_normal_bilinear(&self._position);

        begin_block!("Check Ground & Slope"); {
            // check slope
            let mut ground_height = height_map_data.get_height_bilinear(&self._position, 0);
            if ground_normal.y < SLOPE_ANGLE && (self._position.y <= ground_height || was_on_ground) {
                let new_move_dir = Vector3::new(ground_normal.x, 0.0, ground_normal.z);
                self._position = prev_position + new_move_dir * SLOPE_SPEED * delta_time;
                ground_height = height_map_data.get_height_bilinear(&self._position, 0);
            }

            // check ground
            if self._position.y <= ground_height {
                let move_delta = self._position - prev_position;
                let (_move_dir, move_distance) = math::make_normalize_with_norm(&move_delta);
                let new_move_dir = math::safe_normalize(&(Vector3::new(self._position.x, ground_height, self._position.z) - prev_position));
                self._position = new_move_dir * move_distance + prev_position;
                self.set_on_ground(self._position.y);
            }
        }

        // check collide with block
        let mut move_delta = self._position - prev_position;
        let mut current_actor_collision = actor_collision.clone();
        current_actor_collision._bounding_box._center += move_delta;
        current_actor_collision._bounding_box._min += move_delta;
        current_actor_collision._bounding_box._max += move_delta;

        // check ground and side
        for collision_object in collision_objects.iter() {
            let block_render_object = ptr_as_ref(collision_object.as_ptr());
            let block_bound_box = &block_render_object._collision._bounding_box;
            let block_location = &block_bound_box._center;
            let mut collided_block: *const RenderObjectData<'a> = std::ptr::null();

            // check collide with block
            if current_actor_collision.collide_collision(&block_render_object._collision) {
                if self._velocity.y <= 0.0 && block_bound_box._max.y <= prev_position.y {
                    self.set_on_ground(block_bound_box._max.y);
                } else {
                    let block_to_player = Vector3::new(self._position.x - block_location.x, 0.0, self._position.z - block_location.z).normalize();
                    if block_to_player.dot(&move_delta.normalize()) < 0.0 {
                        let dist = Vector3::new(prev_position.x - block_location.x, 0.0, prev_position.z - block_location.z).norm();
                        let new_pos = block_to_player * dist + block_location;
                        self._position.x = new_pos.x;
                        self._position.z = new_pos.z;
                        self._is_blocked = true;
                        collided_block = collision_object.as_ptr();
                    }
                }
            }

            // Recheck whether the adjusted position due to a collision with a block collides with another blocks.
            if collided_block != std::ptr::null() {
                // update delta & bound_box
                move_delta = self._position - prev_position;
                current_actor_collision._bounding_box._center = actor_collision._bounding_box._center + move_delta;
                current_actor_collision._bounding_box._min = actor_collision._bounding_box._min + move_delta;
                current_actor_collision._bounding_box._max = actor_collision._bounding_box._max + move_delta;

                // Recheck collide with another blocks
                for collision_object in collision_objects.iter() {
                    let recheck_block = ptr_as_ref(collision_object.as_ptr());
                    if collided_block != recheck_block {
                        let recheck_block_bound_box = &recheck_block._collision._bounding_box;
                        if current_actor_collision.collide_collision(&recheck_block._collision) {
                            if self._velocity.y <= 0.0 && recheck_block_bound_box._max.y <= prev_position.y {
                                self.set_on_ground(recheck_block_bound_box._max.y);
                            } else {
                                // move back
                                self._position.x = prev_position.x;
                                self._position.z = prev_position.z;
                            }
                        }
                    }
                }
            }
        }

        // check cliff
        if !self._is_falling && (move_delta.x != 0.0 || move_delta.z != 0.0) {
            let point: Vector3<f32> = Vector3::new(
                if 0.0 < move_delta.x { current_actor_collision._bounding_box._max.x + 0.1 } else { current_actor_collision._bounding_box._min.x - 0.1 },
                current_actor_collision._bounding_box._min.y - 0.1,
                if 0.0 < move_delta.z { current_actor_collision._bounding_box._max.z + 0.1 } else { current_actor_collision._bounding_box._min.z - 0.1 }
            );

            for collision_object in collision_objects.iter() {
                let block_render_object = ptr_as_ref(collision_object.as_ptr());
                if block_render_object._collision.collide_point(&point) {
                    self._is_cliff = false;
                    break;
                }
            }

            if self._is_cliff && (point.y - CLIFF_HEIGHT) <= height_map_data.get_height_bilinear(&point, 0) {
                self._is_cliff = false;
            }
        }

        // update last ground position
        if self.is_on_ground() {
            self._last_ground_position = self._position;
            self._last_ground_normal = ground_normal;
        }

        // reset
        self._is_jump_start = false;
    }
}