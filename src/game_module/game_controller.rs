use crate::application::application::Application;
use crate::game_module::actors::character::Character;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::*;
use crate::game_module::game_ui_manager::GameUIManager;
use nalgebra::{Matrix4, Vector2, Vector3};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{
    ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData,
};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use winit::keyboard::KeyCode;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display, EnumIter, EnumString, EnumCount)]
pub enum InputControlType {
    Attack,
    PowerAttack,
    Interaction,
    EnterGate,
    Gathering,
    CameraRotation,
    Zoom,
    Move,
    Sprint,
    Jump,
    Roll,
    SelectItem,
    UseCurrentItem,
    UseItem01,
    UseItem02,
    UseItem03,
    UseItem04,
    UseItem05,
    UseItem06,
    UseItem07,
    UseItem08,
    UseItem09,
    UseItem10,
}

pub struct GameController<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _camera_goal_pitch: f32,
    pub _camera_goal_yaw: f32,
    pub _camera_pitch: f32,
    pub _camera_yaw: f32,
    pub _camera_position: Vector3<f32>,
    pub _camera_blend_ratio: f32,
    pub _is_game_camera_auto_blend_mode: bool,
    pub _is_keyboard_input_mode: bool,
}

impl<'a> GameController<'a> {
    pub fn create_game_controller() -> Box<GameController<'a>> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: CAMERA_DISTANCE_MAX,
            _camera_goal_pitch: 0.0,
            _camera_goal_yaw: 0.0,
            _camera_pitch: 0.0,
            _camera_yaw: 0.0,
            _camera_position: Vector3::zeros(),
            _camera_blend_ratio: 0.0,
            _is_game_camera_auto_blend_mode: false,
            _is_keyboard_input_mode: true,
        })
    }

    pub fn initialize_game_controller(&mut self, application: &Application<'a>) {
        log::info!("initialize_game_controller");
        self._game_client = application.get_game_client();
        self._game_ui_manager = application.get_game_ui_manager();
    }
    pub fn get_game_client(&self) -> &GameClient<'a> {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient<'a> {
        ptr_as_mut(self._game_client)
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager<'a> {
        ptr_as_ref(self._game_ui_manager)
    }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager<'a> {
        ptr_as_mut(self._game_ui_manager)
    }
    pub fn get_main_camera(&self) -> &CameraObjectData {
        self.get_game_client()
            .get_game_scene_manager()
            .get_scene_manager()
            .get_main_camera()
    }
    pub fn get_main_camera_mut(&self) -> &mut CameraObjectData {
        self.get_game_client()
            .get_game_scene_manager()
            .get_scene_manager()
            .get_main_camera_mut()
    }

    pub fn is_keyboard_input_mode(&self) -> bool {
        self._is_keyboard_input_mode
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

    pub fn update_game_camera_auto_blend(&mut self, main_camera: &mut CameraObjectData, player: &RcRefCell<Character>, delta_time: f32) {
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
        let t = 1.0 - (-to_goal_dist).exp();
        to_goal_dist -= CAMERA_POSITION_BLEND_SPEED_MIN.max(to_goal_dist * 2.0) * delta_time;
        if to_goal_dist < 0.0 {
            self._camera_position = goal_camera_position;
        } else {
            self._camera_position = goal_camera_position - to_goal_camera * to_goal_dist;
            done_auto_blend_mode = false;
        }
        self._camera_distance = (pivot - self._camera_position).magnitude();

        // blend pitch
        let pitch_diff = math::get_normalized_diff_radian(main_camera._transform_object.get_pitch(), self._camera_goal_pitch);
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
            let main_camera = self.get_main_camera();
            (
                main_camera._transform_object.get_pitch(),
                main_camera._transform_object.get_yaw(),
                main_camera._transform_object.get_position().clone(),
            )
        };
        self._camera_pitch = pitch;
        self._camera_yaw = yaw;
        self._camera_position = position;

        let calculated_distance = if let Some(player) = self.get_game_client().get_game_scene_manager().get_character_manager().get_maybe_player() {
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

    pub fn update_camera_rotation(
        &mut self,
        pitch_control: f32,
        yaw_control: f32,
        delta_time: f32,
    ) {
        (self._camera_goal_pitch, self._camera_pitch) = self.update_camera_smooth_rotation(
            self._camera_goal_pitch,
            self._camera_pitch,
            pitch_control,
            delta_time,
        );
        self._camera_goal_pitch = CAMERA_PITCH_MIN.max(CAMERA_PITCH_MAX.min(self._camera_goal_pitch));
        self._camera_pitch = CAMERA_PITCH_MIN.max(CAMERA_PITCH_MAX.min(self._camera_pitch));

        (self._camera_goal_yaw, self._camera_yaw) = self.update_camera_smooth_rotation(
            self._camera_goal_yaw,
            self._camera_yaw,
            yaw_control,
            delta_time,
        );
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

    pub fn apply_game_camera_transform(&mut self, main_camera: &mut CameraObjectData, player: &mut Character) {
        main_camera._transform_object.set_pitch(self._camera_pitch);
        main_camera._transform_object.set_yaw(self._camera_yaw);
        main_camera._transform_object.set_roll(0.0);
        main_camera._transform_object.update_transform_object();

        let pivot = player.get_bounding_box()._center + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
        let camera_dir = -main_camera._transform_object.get_front();
        let scene_manager = self.get_game_client().get_game_scene_manager().get_scene_manager();

        let mut min_collision_distance = self._camera_distance;

        // Step 1: Check collision with height map
        let mut height_map_collision_point: Vector3<f32> = Vector3::zeros();
        if scene_manager.get_height_map_data().get_collision_point(
            &pivot,
            &camera_dir,
            self._camera_distance,
            0.0, // No padding for initial check
            &mut height_map_collision_point
        ) {
            min_collision_distance = (height_map_collision_point - pivot).magnitude();
        }

        // Step 2: Check collision with scene objects
        // if GAME_VIEW_MODE == GameViewMode::GameViewMode3D {
        //     let ideal_camera_pos = pivot + camera_dir * self._camera_distance;
        //     let search_min = pivot.inf(&ideal_camera_pos);
        //     let search_max = pivot.sup(&ideal_camera_pos);
        //     let collision_objects = scene_manager.collect_collision_objects(&search_min, &search_max);
        //
        //     for collision_object in collision_objects.values() {
        //         let block_render_object = ptr_as_ref(collision_object.as_ptr());
        //         if let Some(distance) = block_render_object._collision.ray_vs_aabb(&pivot, &camera_dir) {
        //             if distance < min_collision_distance {
        //                 min_collision_distance = distance;
        //             }
        //         }
        //     }
        // }

        // Step 3: Apply padding and set final position
        let final_distance = (min_collision_distance - CAMERA_COLLIDE_PADDING).max(CAMERA_DISTANCE_MIN);
        self._camera_position = pivot + camera_dir * final_distance;

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
        player: &RcRefCell<Character>,
    ) {
        let delta_time: f32 = time_data._delta_time as f32;
        let is_attack: bool = mouse_input_data._btn_l_pressed
            || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let is_power_attack: bool = mouse_input_data._btn_r_pressed
            || joystick_input_data._btn_right_trigger == ButtonState::Pressed;
        let is_left = keyboard_input_data.get_key_hold(KeyCode::KeyA)
            || joystick_input_data._stick_left_direction.x < 0;
        let is_right = keyboard_input_data.get_key_hold(KeyCode::KeyD)
            || 0 < joystick_input_data._stick_left_direction.x;
        let is_down = keyboard_input_data.get_key_hold(KeyCode::KeyS)
            || joystick_input_data._stick_left_direction.y < 0;
        let is_up = keyboard_input_data.get_key_hold(KeyCode::KeyW)
            || 0 < joystick_input_data._stick_left_direction.y;
        let is_jump = keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed;
        let is_run = keyboard_input_data.get_key_pressed(KeyCode::ShiftLeft)
            || joystick_input_data._btn_left_shoulder == ButtonState::Pressed;
        let is_roll = keyboard_input_data.get_key_pressed(KeyCode::AltLeft)
            || joystick_input_data._btn_b == ButtonState::Pressed;
        let is_interaction = keyboard_input_data.get_key_pressed(KeyCode::KeyF)
            || joystick_input_data._btn_x == ButtonState::Pressed;
        let is_zoom_in = keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || 0 < mouse_move_data._scroll_delta.y
            || joystick_input_data._btn_up == ButtonState::Hold;
        let is_zoom_out = keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || mouse_move_data._scroll_delta.y < 0
            || joystick_input_data._btn_down == ButtonState::Hold;
        let use_item = keyboard_input_data.get_key_pressed(KeyCode::KeyC)
            || joystick_input_data._btn_y == ButtonState::Pressed;
        let is_previous_item = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyQ)
            || joystick_input_data._btn_left == ButtonState::Pressed;
        let is_next_item = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyE)
            || joystick_input_data._btn_right == ButtonState::Pressed;

        let mouse_sensitivity: f32 = 0.001;
        let mouse_pos_delta = Vector2::<f32>::new(
            mouse_move_data._mouse_pos_delta.x as f32,
            mouse_move_data._mouse_pos_delta.y as f32,
        ) * mouse_sensitivity;
        let mouse_scroll_delta = Vector2::<f32>::new(
            mouse_move_data._scroll_delta.x as f32,
            mouse_move_data._scroll_delta.y as f32,
        );

        let joystick_sensitivity: f32 = 0.1 / 32767.0;
        let stick_left_direction = Vector2::<f32>::new(
            joystick_input_data._stick_left_direction.x as f32,
            joystick_input_data._stick_left_direction.y as f32,
        ) * joystick_sensitivity;
        let stick_right_direction = Vector2::<f32>::new(
            joystick_input_data._stick_right_direction.x as f32,
            joystick_input_data._stick_right_direction.y as f32,
        ) * joystick_sensitivity;

        //
        if keyboard_input_data.is_any_key_pressed() {
            self._is_keyboard_input_mode = true;
        } else if joystick_input_data.is_any_button_pressed() {
            self._is_keyboard_input_mode = false;
        }

        // item control
        let item_manager = self.get_game_client().get_game_scene_manager().get_item_manager();
        if is_previous_item {
            item_manager.select_previous_item();
        } else if is_next_item {
            item_manager.select_next_item();
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
                    item_manager.use_inventory_item_by_index(item_index);
                    break;
                }
            }
        }

        if use_item {
            let item_data_type = item_manager.get_selected_inventory_item_data_type();
            item_manager.use_inventory_item(&item_data_type, 1);
        }

        // character control
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
                if is_zoom_in {
                    -0.5
                } else {
                    0.5
                }
            }
        } else {
            0.0
        };

        // set action & move
        let mut player_mut = player.borrow_mut();
        {
            let mut move_direction: Vector3<f32> = Vector3::zeros();

            if is_left || is_right {
                move_direction.x = if stick_left_direction.x != 0.0 {
                    stick_left_direction.x
                } else {
                    if is_left {
                        -1.0
                    } else {
                        1.0
                    }
                };
            }

            if is_up || is_down {
                move_direction.z = if stick_left_direction.y != 0.0 {
                    -stick_left_direction.y
                } else {
                    if is_down {
                        -1.0
                    } else {
                        1.0
                    }
                };
            }

            if move_direction.x != 0.0 || move_direction.z != 0.0 {
                let mut camera_front = main_camera._transform_object.get_front().clone();
                let mut camera_right = main_camera._transform_object.get_right().clone();
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
                player_mut.set_move_stop();
            }

            if is_up && player_mut.is_in_interaction_range() {
                player_mut.set_action_enter_gate();
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

        if is_attack {
            player_mut.set_action_attack();
        }

        if is_interaction && player_mut.is_in_interaction_range() {
            player_mut.set_action_interaction();
        }

        if is_power_attack {
            if player_mut.get_weapon().is_none() {
                player_mut.set_action_kick();
            } else {
                player_mut.set_action_power_attack();
            }
        }

        self.update_game_camera(pitch_control, yaw_control, zoom_control, delta_time);
        self.apply_game_camera_transform(main_camera, &mut player_mut);
    }
}
