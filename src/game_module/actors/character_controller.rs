use nalgebra::{Vector3};
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::begin_block;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::math::HALF_PI;
use rust_engine_3d::utilities::system::ptr_as_ref;
use crate::game_module::actors::character::{Character, InteractionObject};
use crate::game_module::actors::character_data::{CharacterData, MoveAnimationState};
use crate::game_module::game_constants::*;

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _falling_height: f32,
    pub _last_ground_normal: Vector3<f32>,
    pub _face_direction: Vector3<f32>,
    pub _move_direction: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _slop_velocity: Vector3<f32>,
    pub _hit_velocity: Vector3<f32>,
    pub _move_speed: f32,
    pub _fall_time: f32,
    pub _is_falling: bool,
    pub _is_ground: bool,
    pub _is_running: bool,
    pub _is_jump_start: bool,
    pub _is_jump: bool,
    pub _is_cliff: bool,
    pub _is_blocked: bool,
    pub _interaction_objects: Vec<InteractionObject>
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _falling_height: 0.0,
            _last_ground_normal: Vector3::new(0.0, 1.0, 0.0),
            _face_direction: Vector3::zeros(),
            _move_direction: Vector3::zeros(),
            _velocity: Vector3::zeros(),
            _slop_velocity: Vector3::zeros(),
            _hit_velocity: Vector3::zeros(),
            _move_speed: 0.0,
            _fall_time: 0.0,
            _is_falling: false,
            _is_ground: false,
            _is_running: false,
            _is_jump_start: false,
            _is_jump: false,
            _is_cliff: false,
            _is_blocked: false,
            _interaction_objects: Vec::new()
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
        self._falling_height = position.y;
        self._last_ground_normal = Vector3::new(0.0, 1.0, 0.0);
        self._velocity = Vector3::zeros();
        self._slop_velocity = Vector3::zeros();
        self._hit_velocity = Vector3::zeros();
        self._move_speed = 0.0;
        self._fall_time = 0.0;
        self._is_falling = false;
        self._is_jump_start = false;
        self._is_jump = false;
        self._is_running = false;
        self._is_ground = true;
        self._is_blocked = false;
        self._is_cliff = false;
        self._interaction_objects.clear();
    }
    pub fn is_falling(&self) -> bool {
        self._is_falling
    }
    pub fn is_on_ground(&self) -> bool {
        self._is_ground && !self._is_jump_start
    }
    pub fn get_falling_height(&self) -> f32 {
        self._falling_height
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
    pub fn is_cliff(&self) -> bool {
        self._is_cliff
    }
    pub fn is_blocked(&self) -> bool {
        self._is_blocked
    }
    pub fn get_interaction_object(&self) -> InteractionObject {
        if self._interaction_objects.is_empty() {
            return InteractionObject::None;
        }
        self._interaction_objects[0].clone()
    }
    pub fn is_in_interaction_range(&self) -> bool {
        self._interaction_objects.is_empty() == false
    }
    pub fn add_interaction_object(&mut self, object: InteractionObject) {
        if self._interaction_objects.contains(&object) == false {
            self._interaction_objects.push(object);
        }
    }
    pub fn get_interaction_objects(&self) -> &Vec<InteractionObject> {
        &self._interaction_objects
    }
    pub fn remove_interaction_object(&mut self, object: InteractionObject) {
        self._interaction_objects.retain(|&x| x != object);
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

    pub fn set_hit_direction(&mut self, direction: &Vector3<f32>) {
        self._hit_velocity = direction.clone() * HIT_VELOCITY_SPEED;
    }

    pub fn set_on_ground(&mut self, ground_height: f32, ground_normal: &Vector3<f32>) {
        if self._is_ground == false || self._position.y < ground_height {
            self._position.y = ground_height;
            self._is_ground = true;
            self._is_falling = false;
            self._is_jump = false;
            self._velocity = Vector3::zeros();
            self._last_ground_normal.clone_from(ground_normal);
        }
    }

    pub fn update_character_controller<'a>(
        &mut self,
        owner: &Character,
        scene_manager: &SceneManager<'a>,
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
        } else {
            self._velocity.x = 0.0;
            self._velocity.z = 0.0;
        }

        begin_block!("apply hit velocity"); {
            if self._hit_velocity.x != 0.0 || self._hit_velocity.z != 0.0 {
                self._position += self._hit_velocity * delta_time;
                let (hit_move_dir, hit_move_distance) = math::make_normalize_with_norm(&self._hit_velocity);
                self._hit_velocity = hit_move_dir * (hit_move_distance - HIT_VELOCITY_DECAY * delta_time).max(0.0);
            }
        }

        begin_block!("apply slop velocity"); {
            if (self._slop_velocity.x * self._velocity.x + self._slop_velocity.z * self._velocity.z) <= 0.0 {
                self._position += self._slop_velocity * delta_time;
            }

            if self.is_on_ground() {
                let ground_normal_y = self._last_ground_normal.y.abs();
                let (slope_move_dir, mut slope_move_distance) = math::make_normalize_with_norm(&self._slop_velocity);
                if SLOPE_ANGLE <= ground_normal_y {
                    let slope_decay = SLOPE_VELOCITY_DECAY * (ground_normal_y - SLOPE_VELOCITY_DECAY) / (1.0 - SLOPE_VELOCITY_DECAY);
                    slope_move_distance = (slope_move_distance - slope_decay * delta_time).max(0.0);
                    self._slop_velocity = slope_move_dir * slope_move_distance;
                }

                if 0.0 < slope_move_distance {
                    let (move_dir, move_distance) = math::make_normalize_with_norm(&self._velocity);
                    let move_decay = move_dir.dot(&slope_move_dir) * (1.0 - ground_normal_y);
                    if move_decay <= 0.0 {
                        self._velocity = move_dir * move_distance * (1.0 + move_decay);
                    }
                }
            }
        }

        // move
        self._position.x += self._velocity.x * delta_time;
        self._position.z += self._velocity.z * delta_time;

        // update rotation
        self.rotate_to_direction(&move_direction, delta_time);

        // jump
        if self._is_jump_start {
            let not_enough_stamina = owner._character_stats._stamina < 0.0;
            let jump_speed = character_data._stat_data._jump_speed * if not_enough_stamina { 0.5 } else { 1.0 };
            self._velocity.y = jump_speed;

            // let ground_normal = math::safe_normalize(&Vector3::new(self._last_ground_normal.x, self._last_ground_normal.y.abs(), self._last_ground_normal.z));
            // let slop_speed = SLOPE_SPEED.max(self._move_speed);
            // self._slop_velocity += Vector3::new(ground_normal.x, 0.0, ground_normal.z) * jump_speed * (1.0 - ground_normal.y);
            // let (slope_move_dir, slope_move_distance) = math::make_normalize_with_norm(&self._slop_velocity);
            // self._slop_velocity = slope_move_dir * slope_move_distance.min(slop_speed);

            self._is_ground = false;
            self._is_jump = true;
        }

        // fall
        if false == self._is_ground {
            let velocity_prev_y = self._velocity.y;
            self._velocity.y -= GRAVITY * delta_time;
            if 0.0 <= velocity_prev_y && self._velocity.y < 0.0 {
                self._falling_height = self._position.y;
            }
        } else {
            self._falling_height = self._position.y;
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
        let _was_on_ground = self._is_ground;
        self._is_cliff = true;
        self._is_blocked = false;
        self._is_ground = false;

        let height_map_data = scene_manager.get_height_map_data();
        begin_block!("Check Ground & Slope"); {
            let ground_height = height_map_data.get_height_bilinear(&self._position, 0);
            if self._position.y <= ground_height && self._velocity.y <= 0.0 {
                let ground_normal = height_map_data.get_normal_bilinear(&prev_position);

                let move_delta = self._position - prev_position;
                let (move_dir, move_distance) = math::make_normalize_with_norm(&move_delta);
                let new_move_dir = math::safe_normalize(&(Vector3::new(self._position.x, ground_height, self._position.z) - prev_position));
                self._position = prev_position + new_move_dir * move_distance;

                if 0.0 < move_distance && SLOPE_ANGLE <= new_move_dir.y || ground_normal.y < SLOPE_ANGLE && ground_normal.dot(&move_dir) < 0.0 {
                    let slop_speed = SLOPE_SPEED.max(self._move_speed);
                    self._slop_velocity += math::make_normalize_xz(&ground_normal) * slop_speed;

                    let (slope_move_dir, slope_move_distance) = math::make_normalize_with_norm(&self._slop_velocity);
                    self._slop_velocity = slope_move_dir * slope_move_distance.min(slop_speed);
                    self._position += self._slop_velocity * delta_time;

                    self._is_blocked = true;
                }
                self.set_on_ground(self._position.y, &ground_normal);
            }
        }

        // check collide with block
        let mut move_delta = self._position - prev_position;
        let mut current_actor_collision = actor_collision.clone();
        current_actor_collision._bounding_box._center += move_delta;
        current_actor_collision._bounding_box._min += move_delta;
        current_actor_collision._bounding_box._max += move_delta;

        let collision_pos_min = math::get_min(&actor_collision._bounding_box._min, &current_actor_collision._bounding_box._min);
        let collision_pos_max = math::get_max(&actor_collision._bounding_box._max, &current_actor_collision._bounding_box._max);
        let collision_objects = scene_manager.collect_collision_objects(&collision_pos_min, &collision_pos_max);

        // check ground and side
        for collision_object in collision_objects.values() {
            let block_render_object = ptr_as_ref(collision_object.as_ptr());
            let block_bound_box = &block_render_object._collision._bounding_box;
            let block_location = &block_bound_box._center;
            let mut collided_block: *const RenderObjectData<'a> = std::ptr::null();

            // check collide with block
            if current_actor_collision.collide_collision(&block_render_object._collision) {
                if self._velocity.y <= 0.0 && block_bound_box._max.y <= prev_position.y {
                    self.set_on_ground(block_bound_box._max.y, &Vector3::new(0.0, 1.0, 0.0));
                } else if 0.0 < self._velocity.y && actor_collision._bounding_box._max.y < block_bound_box._min.y {
                    self._velocity.y = 0.0;
                    self._position.y = prev_position.y;
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
                for recheck_collision_object in collision_objects.values() {
                    let recheck_block = ptr_as_ref(recheck_collision_object.as_ptr());
                    if collided_block != recheck_block {
                        let recheck_block_bound_box = &recheck_block._collision._bounding_box;
                        if current_actor_collision.collide_collision(&recheck_block._collision) {
                            if self._velocity.y <= 0.0 && recheck_block_bound_box._max.y <= prev_position.y {
                                self.set_on_ground(recheck_block_bound_box._max.y, &Vector3::new(0.0, 1.0, 0.0));
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

            for collision_object in collision_objects.values() {
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

        // reset
        self._is_jump_start = false;
    }
}