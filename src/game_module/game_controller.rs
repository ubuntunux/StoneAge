use crate::game_module::actors::character::{ActionAnimationState, Character};
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::*;
use crate::game_module::game_service_locator::{get_character_manager, get_game_client_mut, get_game_ui_manager_mut};
use crate::game_module::widgets::game_menu_widget::GameMenuTab;
use nalgebra::{Matrix4, Vector2, Vector3};
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_scene_manager;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::scene::collision::{CollisionCreateInfo, CollisionData, CollisionType};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use winit::keyboard::KeyCode;

// Hold Repeat Navigation Utility
pub const NAV_INITIAL_DELAY: f32 = 0.3;
pub const NAV_REPEAT_INTERVAL: f32 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuickSlotNavDirection {
    Previous,
    Next,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoldRepeatController<T: PartialEq + Copy> {
    pub _initial_delay: f32,
    pub _repeat_interval: f32,
    pub _timer: f32,
    pub _is_repeating: bool,
    pub _last_state: Option<T>,
}

impl<T: PartialEq + Copy> HoldRepeatController<T> {
    pub fn new(initial_delay: f32, repeat_interval: f32) -> Self {
        Self {
            _initial_delay: initial_delay,
            _repeat_interval: repeat_interval,
            _timer: 0.0,
            _is_repeating: false,
            _last_state: None,
        }
    }

    pub fn reset(&mut self) {
        self._timer = 0.0;
        self._is_repeating = false;
        self._last_state = None;
    }

    pub fn update(&mut self, current_state: Option<T>, delta_time: f32) -> (bool, Option<T>) {
        if let Some(state) = current_state {
            if Some(state) != self._last_state {
                self._last_state = Some(state);
                self._timer = self._initial_delay;
                self._is_repeating = false;
                (true, Some(state))
            } else {
                self._timer -= delta_time;
                if self._timer <= 0.0 {
                    if !self._is_repeating {
                        self._is_repeating = true;
                        self._timer = self._repeat_interval;
                    } else {
                        self._timer += self._repeat_interval;
                    }
                    (true, Some(state))
                } else {
                    (false, None)
                }
            }
        } else {
            self.reset();
            (false, None)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WidgetNavRepeatController {
    pub _controller: HoldRepeatController<(i32, i32)>,
}

impl Default for WidgetNavRepeatController {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetNavRepeatController {
    pub fn new() -> Self {
        Self {
            _controller: HoldRepeatController::new(NAV_INITIAL_DELAY, NAV_REPEAT_INTERVAL),
        }
    }

    pub fn with_delay_and_interval(initial_delay: f32, repeat_interval: f32) -> Self {
        Self {
            _controller: HoldRepeatController::new(initial_delay, repeat_interval),
        }
    }

    pub fn reset(&mut self) {
        self._controller.reset();
    }

    pub fn get_direction(
        keyboard_input_data: &KeyboardInputData,
        joystick_input_data: &JoystickInputData,
    ) -> Option<(i32, i32)> {
        let is_left = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || keyboard_input_data.get_key_hold(KeyCode::KeyA)
            || joystick_input_data._btn_left == ButtonState::Pressed
            || joystick_input_data._btn_left == ButtonState::Hold
            || joystick_input_data._stick_left_direction.x < -10000;

        let is_right = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || keyboard_input_data.get_key_hold(KeyCode::KeyD)
            || joystick_input_data._btn_right == ButtonState::Pressed
            || joystick_input_data._btn_right == ButtonState::Hold
            || joystick_input_data._stick_left_direction.x > 10000;

        let is_up = keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_hold(KeyCode::KeyW)
            || joystick_input_data._btn_up == ButtonState::Pressed
            || joystick_input_data._btn_up == ButtonState::Hold
            || joystick_input_data._stick_left_direction.y > 10000;

        let is_down = keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_hold(KeyCode::KeyS)
            || joystick_input_data._btn_down == ButtonState::Pressed
            || joystick_input_data._btn_down == ButtonState::Hold
            || joystick_input_data._stick_left_direction.y < -10000;

        if is_left {
            Some((-1, 0))
        } else if is_right {
            Some((1, 0))
        } else if is_up {
            Some((0, -1))
        } else if is_down {
            Some((0, 1))
        } else {
            None
        }
    }

    pub fn update(
        &mut self,
        keyboard_input_data: &KeyboardInputData,
        joystick_input_data: &JoystickInputData,
        delta_time: f32,
    ) -> (bool, Option<(i32, i32)>) {
        let current_dir = Self::get_direction(keyboard_input_data, joystick_input_data);
        self._controller.update(current_dir, delta_time)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display, EnumIter, EnumString, EnumCount)]
pub enum KeyBindingType {
    None,
    Help,
    Attack,
    PowerAttack,
    Interaction,
    Request,
    EnterGate,
    Gathering,
    Taming,
    Farming,
    CameraRotation,
    Zoom,
    Move,
    Sprint,
    Jump,
    Roll,
    SelectPrevItem,
    SelectNextItem,
    SelectItem01,
    SelectItem02,
    SelectItem03,
    SelectItem04,
    SelectItem05,
    SelectItem06,
    SelectItem07,
    SelectItem08,
    SelectItem09,
    SelectItem10,
}

pub struct GameController<'a> {
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _camera_goal_pitch: f32,
    pub _camera_goal_yaw: f32,
    pub _camera_pitch: f32,
    pub _camera_yaw: f32,
    pub _camera_position: Vector3<f32>,
    pub _camera_blend_ratio: f32,
    pub _is_game_camera_auto_blend_mode: bool,
    pub _quick_slot_repeat_controller: HoldRepeatController<QuickSlotNavDirection>,
    pub _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GameController<'a> {
    pub fn create_game_controller() -> Box<GameController<'a>> {
        Box::new(GameController {
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: CAMERA_DISTANCE_MAX,
            _camera_goal_pitch: 0.0,
            _camera_goal_yaw: 0.0,
            _camera_pitch: 0.0,
            _camera_yaw: 0.0,
            _camera_position: Vector3::zeros(),
            _camera_blend_ratio: 0.0,
            _is_game_camera_auto_blend_mode: false,
            _quick_slot_repeat_controller: HoldRepeatController::new(NAV_INITIAL_DELAY, NAV_REPEAT_INTERVAL),
            _marker: std::marker::PhantomData,
        })
    }

    pub fn initialize_game_controller(&mut self) {
        log::info!("initialize_game_controller");
    }

    pub fn is_game_camera_auto_blend_mode(&self) -> bool {
        self._is_game_camera_auto_blend_mode
    }

    pub fn set_game_camera_auto_blend_mode(&mut self, is_game_camera_auto_blend_mode: bool) {
        if is_game_camera_auto_blend_mode {
            self.update_current_game_camera_transform();
        }

        self._is_game_camera_auto_blend_mode = is_game_camera_auto_blend_mode;
    }

    pub fn update_game_camera_auto_blend(
        &mut self,
        main_camera: &mut CameraObjectData,
        player: &RcRefCell<Character>,
        delta_time: f32,
    ) {
        let mut done_auto_blend_mode = true;

        // blend distance
        let pivot = player.borrow().get_bounding_box()._center + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
        let rotation_matrix: Matrix4<f32> = math::make_rotation_matrix(self._camera_pitch, self._camera_yaw, 0f32);
        let goal_camera_position = pivot - rotation_matrix.column(2).xyz() * self._camera_goal_distance;
        let mut to_goal_camera = goal_camera_position - main_camera._transform_object.get_position();
        let mut to_goal_dist = to_goal_camera.magnitude();
        if 0.0 < to_goal_dist {
            to_goal_camera /= to_goal_dist;
        }
        to_goal_dist -= CAMERA_POSITION_BLEND_SPEED_MIN.max(to_goal_dist * 2.0) * delta_time;
        if to_goal_dist < 0.0 {
            self._camera_position = goal_camera_position;
        } else {
            self._camera_position = goal_camera_position - to_goal_camera * to_goal_dist;
            done_auto_blend_mode = false;
        }
        self._camera_distance = (pivot - self._camera_position).magnitude();

        // blend pitch
        let pitch_diff =
            math::get_normalized_diff_radian(main_camera._transform_object.get_pitch(), self._camera_goal_pitch);
        let t = 1.0 - (-pitch_diff.abs()).exp();
        let blend_speed = math::lerp(CAMERA_ROTATION_SPEED_MIN, CAMERA_ROTATION_SPEED_MAX, t) * delta_time;
        if pitch_diff.abs() < blend_speed {
            self._camera_pitch = self._camera_goal_pitch;
        } else {
            self._camera_pitch = self._camera_goal_pitch - (pitch_diff - blend_speed * pitch_diff.signum());
            done_auto_blend_mode = false;
        }

        // blend yaw
        let yaw_diff = math::get_normalized_diff_radian(main_camera._transform_object.get_yaw(), self._camera_goal_yaw);
        let t = 1.0 - (-yaw_diff.abs()).exp();
        let blend_speed = math::lerp(CAMERA_ROTATION_SPEED_MIN, CAMERA_ROTATION_SPEED_MAX, t) * delta_time;
        if yaw_diff.abs() < blend_speed {
            self._camera_yaw = self._camera_goal_yaw;
        } else {
            self._camera_yaw = self._camera_goal_yaw - (yaw_diff - blend_speed * yaw_diff.signum());
            done_auto_blend_mode = false;
        }

        main_camera._transform_object.set_position(&self._camera_position);
        main_camera._transform_object.set_pitch(self._camera_pitch);
        main_camera._transform_object.set_yaw(self._camera_yaw);

        if done_auto_blend_mode {
            self.set_game_camera_auto_blend_mode(false);
        }
    }

    pub fn set_game_camera_goal_transform(&mut self, goal_distance_ratio: f32, goal_pitch: f32, goal_yaw: f32) {
        self._camera_goal_distance = math::lerp(CAMERA_DISTANCE_MIN, CAMERA_DISTANCE_MAX, goal_distance_ratio);
        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D || GAME_VIEW_MODE == GameViewMode::GameViewMode25D {
            self._camera_goal_pitch = self.get_camera_pitch_by_distance(self._camera_goal_distance);
        } else {
            self._camera_goal_pitch = CAMERA_PITCH_MIN.max(CAMERA_PITCH_MAX.min(goal_pitch % math::TWO_PI));
        }
        self._camera_goal_yaw = goal_yaw % math::TWO_PI;
    }

    pub fn update_current_game_camera_transform(&mut self) {
        let (pitch, yaw, position) = {
            let main_camera = get_scene_manager().get_main_camera();
            (
                main_camera._transform_object.get_pitch(),
                main_camera._transform_object.get_yaw(),
                *main_camera._transform_object.get_position(),
            )
        };
        self._camera_pitch = pitch;
        self._camera_yaw = yaw;
        self._camera_position = position;

        let calculated_distance = if let Some(player) = get_character_manager().get_maybe_player() {
            let pivot = player.borrow().get_bounding_box()._center + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
            Some((pivot - self._camera_position).magnitude())
        } else {
            None
        };

        if let Some(distance) = calculated_distance {
            self._camera_distance = distance;
        }

        self.set_game_camera_goal_transform(1.0, self._camera_pitch, self._camera_yaw);
    }

    pub fn update_camera_smooth_rotation(
        &self,
        mut goal_value: f32,
        mut target_value: f32,
        value: f32,
        delta_time: f32,
    ) -> (f32, f32) {
        goal_value += value;
        let diff_value = (goal_value - target_value).abs();
        let t: f32 = 0f32.max(1f32.min(diff_value / std::f32::consts::PI));
        let value_speed: f32 = math::lerp(CAMERA_ROTATION_SPEED_MIN, CAMERA_ROTATION_SPEED_MAX, t);
        let value_delta = value_speed * delta_time;
        target_value = if diff_value < value_delta {
            goal_value
        } else if goal_value < target_value {
            target_value - value_delta
        } else {
            target_value + value_delta
        };
        (goal_value, target_value)
    }

    pub fn update_camera_rotation(&mut self, pitch_control: f32, yaw_control: f32, delta_time: f32) {
        (self._camera_goal_pitch, self._camera_pitch) =
            self.update_camera_smooth_rotation(self._camera_goal_pitch, self._camera_pitch, pitch_control, delta_time);
        self._camera_goal_pitch = CAMERA_PITCH_MIN.max(CAMERA_PITCH_MAX.min(self._camera_goal_pitch));
        self._camera_pitch = CAMERA_PITCH_MIN.max(CAMERA_PITCH_MAX.min(self._camera_pitch));

        (self._camera_goal_yaw, self._camera_yaw) =
            self.update_camera_smooth_rotation(self._camera_goal_yaw, self._camera_yaw, yaw_control, delta_time);
    }

    pub fn get_camera_pitch_by_distance(&self, camera_distance: f32) -> f32 {
        let dist_ratio = (camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN);
        math::degree_to_radian(math::lerp(
            CAMERA_PITCH_MIN_BY_DISTANCE,
            CAMERA_PITCH_MAX_BY_DISTANCE,
            dist_ratio,
        ))
    }

    pub fn update_camera_distance(&mut self, zoom_control: f32, delta_time: f32) {
        self._camera_goal_distance += zoom_control;
        self._camera_goal_distance = CAMERA_DISTANCE_MIN.max(CAMERA_DISTANCE_MAX.min(self._camera_goal_distance));
        if self._camera_goal_distance != self._camera_distance {
            let diff = (self._camera_goal_distance - self._camera_distance) * CAMERA_ZOOM_SPEED;
            let sign = diff.signum();
            let delta = diff * delta_time;
            self._camera_distance += delta;
            if sign != (self._camera_goal_distance - self._camera_distance).signum() {
                self._camera_distance = self._camera_goal_distance;
            }
        }
    }

    pub fn update_game_camera(&mut self, pitch_control: f32, yaw_control: f32, zoom_control: f32, delta_time: f32) {
        self.update_camera_distance(zoom_control, delta_time);
        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D || GAME_VIEW_MODE == GameViewMode::GameViewMode25D {
            self._camera_pitch = self.get_camera_pitch_by_distance(self._camera_distance);
        } else {
            self.update_camera_rotation(pitch_control, yaw_control, delta_time);
        }
    }

    pub fn update_camera_shake(&self, main_camera: &CameraObjectData, hit_blink_time: f32) -> Vector3<f32> {
        let shake_ratio = hit_blink_time / HIT_BLINK_TIME;
        let shake_amount = shake_ratio * CAMERA_SHAKE_INTENSITY;
        let shake_x = (hit_blink_time * CAMERA_SHAKE_SPEED_X).sin() * shake_amount;
        let shake_y = (hit_blink_time * CAMERA_SHAKE_SPEED_Y).cos() * shake_amount;
        let camera_right = *main_camera._transform_object.get_right();
        let camera_up = *main_camera._transform_object.get_up();
        camera_right * shake_x + camera_up * shake_y
    }

    pub fn apply_game_camera_transform(&mut self, main_camera: &mut CameraObjectData, player: &mut Character) {
        main_camera._transform_object.set_pitch(self._camera_pitch);
        main_camera._transform_object.set_yaw(self._camera_yaw);
        main_camera._transform_object.set_roll(0.0);
        main_camera._transform_object.update_transform_object();

        let pivot = player.get_bounding_box()._center + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
        let camera_dir = -main_camera._transform_object.get_front();
        let mut prev_camera_position = self._camera_position;
        let mut camera_position = pivot + camera_dir * self._camera_distance;
        let camera_move_delta = self._camera_position - prev_camera_position;
        let scene_manager = get_scene_manager();

        // check collide with block
        {
            let bound_size = 0.5;
            let collision_info = CollisionCreateInfo {
                _collision_type: CollisionType::CYLINDER,
                _location: camera_position,
                _extents: Vector3::new(bound_size, bound_size, bound_size),
            };
            let camera_collision = CollisionData::create_collision(&collision_info);
            let mut collision_pos_min = math::get_min(
                &camera_collision._bounding_box._min,
                &(camera_collision._bounding_box._min - camera_move_delta),
            );
            collision_pos_min = math::get_min(&collision_pos_min, &pivot);

            let mut collision_pos_max = math::get_max(
                &camera_collision._bounding_box._max,
                &(camera_collision._bounding_box._max - camera_move_delta),
            );
            collision_pos_max = math::get_max(&collision_pos_max, &pivot);

            let collision_objects = scene_manager.collect_collision_objects(&collision_pos_min, &collision_pos_max);

            // check ground and side
            for collision_object in collision_objects.values() {
                let block_render_object = ptr_as_ref(collision_object.as_ptr());
                let block_bound_box = &block_render_object._collision._bounding_box;
                if camera_collision.collide_collision(&block_render_object._collision) {
                    let prev_height = prev_camera_position.y;
                    prev_camera_position = camera_position;

                    if camera_move_delta.y <= 0.0 && block_bound_box._max.y <= prev_height {
                        camera_position.y = block_bound_box._max.y + bound_size;
                    } else if 0.0 < camera_move_delta.y
                        && camera_collision._bounding_box._max.y < block_bound_box._min.y
                    {
                        camera_position.y = prev_height;
                    } else {
                        let push_vec = camera_collision.push_by_collide(&block_render_object._collision);
                        if push_vec.x != 0.0 || push_vec.z != 0.0 {
                            camera_position.x += push_vec.x;
                            camera_position.z += push_vec.z;
                        }
                    }

                    // check line of sight
                    if let Some(hit_dist) = block_render_object._collision.collide_ray(&pivot, &camera_dir)
                        && (hit_dist - bound_size) < self._camera_distance
                    {
                        self._camera_distance = hit_dist - bound_size;
                        camera_position = pivot + camera_dir * (hit_dist - bound_size);
                    }
                }
            }
        }
        self._camera_position = camera_position;

        // check height map collision
        if scene_manager.get_height_map_data().get_collision_point(
            &(pivot + camera_dir),
            &camera_dir,
            self._camera_distance,
            CAMERA_COLLIDE_PADDING,
            &mut camera_position,
        ) {
            self._camera_position = camera_position;
        }

        // apply hit camera shake
        if 0.0 < player._character_stats._hit_blink_time {
            self._camera_position += self.update_camera_shake(main_camera, player._character_stats._hit_blink_time);
        }
        main_camera._transform_object.set_position(&self._camera_position);
    }

    pub fn update_game_controller(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
        player: &RcRefCell<Character<'a>>,
    ) {
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let is_attack_or_use_item: bool =
            mouse_input_data._btn_l_pressed || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let is_power_attack: bool =
            mouse_input_data._btn_r_pressed || joystick_input_data._btn_right_trigger == ButtonState::Pressed;
        let _is_cancel = keyboard_input_data.get_key_pressed(KeyCode::KeyB)
            || keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_b == ButtonState::Pressed;
        let is_left =
            keyboard_input_data.get_key_hold(KeyCode::KeyA) || joystick_input_data._stick_left_direction.x < 0;
        let is_right =
            keyboard_input_data.get_key_hold(KeyCode::KeyD) || 0 < joystick_input_data._stick_left_direction.x;
        let is_down =
            keyboard_input_data.get_key_hold(KeyCode::KeyS) || joystick_input_data._stick_left_direction.y < 0;
        let is_up = keyboard_input_data.get_key_hold(KeyCode::KeyW) || 0 < joystick_input_data._stick_left_direction.y;
        let is_jump =
            keyboard_input_data.get_key_pressed(KeyCode::Space) || joystick_input_data._btn_a == ButtonState::Pressed;
        let is_run = keyboard_input_data.get_key_pressed(KeyCode::ShiftLeft)
            || joystick_input_data._btn_left_shoulder == ButtonState::Pressed;
        let is_roll =
            keyboard_input_data.get_key_pressed(KeyCode::AltLeft) || joystick_input_data._btn_b == ButtonState::Pressed;
        let is_interaction =
            keyboard_input_data.get_key_pressed(KeyCode::KeyF) || joystick_input_data._btn_x == ButtonState::Pressed;
        let is_request =
            keyboard_input_data.get_key_pressed(KeyCode::KeyC) || joystick_input_data._btn_y == ButtonState::Pressed;
        let is_previous_item = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyQ)
            || keyboard_input_data.get_key_hold(KeyCode::KeyQ)
            || joystick_input_data._btn_left == ButtonState::Pressed
            || joystick_input_data._btn_left == ButtonState::Hold;
        let is_next_item = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyE)
            || keyboard_input_data.get_key_hold(KeyCode::KeyE)
            || joystick_input_data._btn_right == ButtonState::Pressed
            || joystick_input_data._btn_right == ButtonState::Hold;
        let is_help =
            keyboard_input_data.get_key_pressed(KeyCode::F1) || joystick_input_data._btn_back == ButtonState::Pressed;
        let open_inventory = keyboard_input_data.get_key_pressed(KeyCode::KeyI)
            || joystick_input_data._btn_start == ButtonState::Pressed;
        let open_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_start == ButtonState::Pressed;
        let prev_quick_slot_row = joystick_input_data._btn_up == ButtonState::Pressed;
        let next_quick_slot_row =
            keyboard_input_data.get_key_pressed(KeyCode::Tab) || joystick_input_data._btn_down == ButtonState::Pressed;

        let stick_left_direction = Vector2::<f32>::new(
            joystick_input_data._stick_left_direction.x as f32,
            joystick_input_data._stick_left_direction.y as f32,
        ) * JOYSTICK_SENSITIVITY;

        if is_help {
            get_game_ui_manager_mut().toggle_controls_visibility();
        }

        // game menu, inventory, quick slot
        if open_menu {
            get_game_ui_manager_mut().open_game_menu(None);
            get_game_client_mut().set_next_game_phase(GamePhase::GameMenu);
        } else if open_inventory {
            get_game_ui_manager_mut().open_game_menu(Some(GameMenuTab::Inventory));
            get_game_client_mut().set_next_game_phase(GamePhase::GameMenu);
        } else if prev_quick_slot_row || next_quick_slot_row {
            get_game_ui_manager_mut().switch_quick_slot_row();
        }

        // item control
        let quick_nav_state = if is_previous_item {
            Some(QuickSlotNavDirection::Previous)
        } else if is_next_item {
            Some(QuickSlotNavDirection::Next)
        } else {
            None
        };

        let (should_move_quick_slot, quick_nav_dir) =
            self._quick_slot_repeat_controller.update(quick_nav_state, delta_time);

        let selectable_item = player.borrow().is_available_move() && player.borrow().is_idle_action();
        if selectable_item {
            let game_ui_manager = get_game_ui_manager_mut();
            if should_move_quick_slot {
                match quick_nav_dir {
                    Some(QuickSlotNavDirection::Previous) => {
                        game_ui_manager.select_previous_item();
                    }
                    Some(QuickSlotNavDirection::Next) => {
                        game_ui_manager.select_next_item();
                    }
                    None => {}
                }
            } else if keyboard_input_data.is_any_key_pressed() {
                const NUMPAD_KEY_MAP: [KeyCode; 10] = [
                    KeyCode::Digit1,
                    KeyCode::Digit2,
                    KeyCode::Digit3,
                    KeyCode::Digit4,
                    KeyCode::Digit5,
                    KeyCode::Digit6,
                    KeyCode::Digit7,
                    KeyCode::Digit8,
                    KeyCode::Digit9,
                    KeyCode::Digit0,
                ];

                for (item_index, numpad_key) in NUMPAD_KEY_MAP.iter().enumerate() {
                    if keyboard_input_data.get_key_pressed(*numpad_key) {
                        game_ui_manager.select_quick_slot(item_index);
                        break;
                    }
                }
            }
        }

        // set action & move
        let player_mut = ptr_as_mut(player.as_ptr());
        {
            let mut move_direction: Vector3<f32> = Vector3::zeros();

            if is_left || is_right {
                move_direction.x = if stick_left_direction.x != 0.0 {
                    stick_left_direction.x
                } else {
                    if is_left { -1.0 } else { 1.0 }
                };
            }

            if is_up || is_down {
                move_direction.z = if stick_left_direction.y != 0.0 {
                    -stick_left_direction.y
                } else {
                    if is_down { -1.0 } else { 1.0 }
                };
            }

            if move_direction.x != 0.0 || move_direction.z != 0.0 {
                let mut camera_front = *main_camera._transform_object.get_front();
                let mut camera_right = *main_camera._transform_object.get_right();
                camera_front.y = 0.0;
                camera_right.y = 0.0;
                camera_front.normalize_mut();
                camera_right.normalize_mut();

                move_direction = camera_right * move_direction.x + camera_front * move_direction.z;
                move_direction.normalize_mut();

                player_mut.set_move(&move_direction);
            } else {
                // look_at_target
                // if player_mut.is_available_move() && mouse_move_data._mouse_pos_delta.x != 0 || mouse_move_data._mouse_pos_delta.y != 0 {
                //     let player_pos = player_mut.get_position();
                //     let camera_pos = main_camera.get_camera_position();
                //     let relative_pos = main_camera.convert_screen_to_relative_world(&mouse_move_data._mouse_pos);
                //     let world_pos = relative_pos / relative_pos.y * (player_pos.y - camera_pos.y) + camera_pos;
                //     let mut move_direction: Vector3<f32> = world_pos - player_pos;
                //     move_direction.y = 0.0;
                //     move_direction.normalize_mut();
                //     player_mut.set_move_direction(&move_direction);
                // }

                // stop
                player_mut.set_move_control_stop();
            }
        }

        if is_run {
            player_mut.toggle_run();
        }

        if is_jump {
            player_mut.set_jump();
        }

        if is_roll {
            player_mut.set_roll();
        }

        let is_available_attack = player_mut.is_available_attack();
        let item_type = player_mut.get_attached_item_data_type();
        if is_request && player_mut.is_in_interaction_range() {
            player_mut.set_action_request();
        } else if is_interaction && player_mut.is_in_interaction_range() {
            player_mut.set_action_interaction();
        } else if is_attack_or_use_item && is_available_attack {
            if item_type.is_fishing_item_type() {
                player_mut.set_action_fishing_begin();
            } else if item_type.is_eatable() {
                player.borrow_mut().set_action_eating();
            } else {
                player_mut.set_action_attack();
            }
        } else if is_power_attack && is_available_attack {
            if item_type.is_weapon_item_type() {
                player_mut.set_action_power_attack();
            } else {
                player_mut.set_action_kick();
            }
        }

        self.process_camera_inputs(
            joystick_input_data,
            keyboard_input_data,
            mouse_move_data,
            delta_time,
            main_camera,
            player_mut,
        );
    }

    pub fn process_camera_inputs(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        delta_time: f32,
        main_camera: &mut CameraObjectData,
        player_mut: &mut Character<'a>,
    ) {
        let mouse_sensitivity: f32 = 0.001;
        let mouse_pos_delta = Vector2::<f32>::new(
            mouse_move_data._mouse_pos_delta.x as f32,
            mouse_move_data._mouse_pos_delta.y as f32,
        ) * mouse_sensitivity;
        let mouse_scroll_delta = Vector2::<f32>::new(
            mouse_move_data._scroll_delta.x as f32,
            mouse_move_data._scroll_delta.y as f32,
        );

        let stick_right_direction = Vector2::<f32>::new(
            joystick_input_data._stick_right_direction.x as f32,
            joystick_input_data._stick_right_direction.y as f32,
        ) * JOYSTICK_SENSITIVITY;

        let is_zoom_in = keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || 0 < mouse_move_data._scroll_delta.y
            || joystick_input_data._btn_up == ButtonState::Hold;
        let is_zoom_out = keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || mouse_move_data._scroll_delta.y < 0
            || joystick_input_data._btn_down == ButtonState::Hold;

        let pitch_control: f32 = if mouse_pos_delta.y != 0.0 {
            mouse_pos_delta.y
        } else {
            stick_right_direction.y
        };

        let yaw_control: f32 = if mouse_pos_delta.x != 0.0 {
            mouse_pos_delta.x
        } else {
            stick_right_direction.x
        };

        let zoom_control: f32 = if is_zoom_in || is_zoom_out {
            if mouse_scroll_delta.y != 0.0 {
                -mouse_scroll_delta.y
            } else {
                if is_zoom_in { -0.5 } else { 0.5 }
            }
        } else {
            0.0
        };

        self.update_game_camera(pitch_control, yaw_control, zoom_control, delta_time);
        self.apply_game_camera_transform(main_camera, player_mut);
    }

    pub fn update_game_controller_fishing(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
        player: &RcRefCell<Character<'a>>,
    ) {
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let is_attack: bool =
            mouse_input_data._btn_l_pressed || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let is_attack_hold: bool =
            mouse_input_data._btn_l_hold || joystick_input_data._btn_right_shoulder == ButtonState::Hold;
        let is_cancel = mouse_input_data._btn_r_pressed
            || keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_right_trigger == ButtonState::Pressed
            || joystick_input_data._btn_b == ButtonState::Pressed;
        let is_left =
            keyboard_input_data.get_key_hold(KeyCode::KeyA) || joystick_input_data._stick_left_direction.x < 0;
        let is_right =
            keyboard_input_data.get_key_hold(KeyCode::KeyD) || 0 < joystick_input_data._stick_left_direction.x;

        let mut player_mut = player.borrow_mut();
        if player_mut.is_action(ActionAnimationState::FishingLoop) {
            if is_cancel {
                player_mut.set_action_fishing_end();
            } else {
                if is_left {
                    player_mut.rotate_player_angle(-1.0, delta_time);
                } else if is_right {
                    player_mut.rotate_player_angle(1.0, delta_time);
                } else {
                    player_mut.rotate_player_angle(0.0, delta_time);
                }

                if is_attack {
                    player_mut.on_pull_press();
                }

                player_mut.set_pulling(is_attack_hold);
            }
        } else if player_mut.is_action(ActionAnimationState::FishingBegin) && !is_attack_hold {
            player_mut.release_fishing_cast();
        }

        self.process_camera_inputs(
            joystick_input_data,
            keyboard_input_data,
            mouse_move_data,
            delta_time,
            main_camera,
            &mut player_mut,
        );
    }
}
