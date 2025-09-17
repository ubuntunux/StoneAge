use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use winit::keyboard::KeyCode;

use crate::application::application::Application;
use crate::game_module::actors::character::Character;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::*;
use crate::game_module::game_ui_manager::GameUIManager;

pub struct GameController<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _camera_goal_pitch: f32,
    pub _camera_goal_yaw: f32,
    pub _camera_pitch: f32,
    pub _camera_yaw: f32,
}

impl<'a> GameController<'a> {
    pub fn create_game_controller() -> Box<GameController<'a>> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: 0.0,
            _camera_goal_pitch: 0.0,
            _camera_goal_yaw: 0.0,
            _camera_pitch: 0.0,
            _camera_yaw: 0.0
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

    pub fn update_camera_smooth_rotation(&self, mut goal_value: f32, mut target_value: f32,value: f32, delta_time: f32) -> (f32, f32) {
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
        (self._camera_goal_pitch, self._camera_pitch) = self.update_camera_smooth_rotation(
            self._camera_goal_pitch, self._camera_pitch, pitch_control, delta_time
        );

        (self._camera_goal_yaw, self._camera_yaw) = self.update_camera_smooth_rotation(
            self._camera_goal_yaw, self._camera_yaw, yaw_control, delta_time
        );
    }

    pub fn update_camera_pitch_by_distance(&mut self) {
        let dist_ratio = (self._camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN);
        self._camera_pitch = math::degree_to_radian(math::lerp(CAMERA_PITCH_MIN, CAMERA_PITCH_MAX, dist_ratio));
    }

    pub fn update_camera_distance(&mut self, zoom_control: f32, delta_time: f32) {
        self._camera_goal_distance += zoom_control;
        self._camera_goal_distance = CAMERA_DISTANCE_MIN.max(CAMERA_DISTANCE_MAX.min(self._camera_goal_distance));
        if self._camera_goal_distance != self._camera_distance {
            let diff = (self._camera_goal_distance - self._camera_distance) * CAMERA_ZOOM_SPEED;
            let sign = diff.signum();
            let delta =  diff * delta_time;
            self._camera_distance += delta;
            if sign != (self._camera_goal_distance - self._camera_distance).signum() {
                self._camera_distance = self._camera_goal_distance;
            }
        }
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
        player: &RcRefCell<Character>
    ) {
        let lock_camera_rotation: bool = false;
        let delta_time: f32 = time_data._delta_time as f32;
        let is_attack: bool =
            mouse_input_data._btn_l_pressed ||
            joystick_input_data._btn_x == ButtonState::Pressed;
        let is_power_attack: bool =
            mouse_input_data._btn_r_pressed ||
            joystick_input_data._btn_y == ButtonState::Pressed;
        let is_left =
            keyboard_input_data.get_key_hold(KeyCode::ArrowLeft) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyA) ||
            joystick_input_data._btn_left == ButtonState::Hold;
        let is_right =
            keyboard_input_data.get_key_hold(KeyCode::ArrowRight) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyD) ||
            joystick_input_data._btn_right == ButtonState::Hold;
        let is_down =
            keyboard_input_data.get_key_hold(KeyCode::ArrowDown) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyS) ||
            joystick_input_data._btn_down  == ButtonState::Hold;
        let is_up =
            keyboard_input_data.get_key_hold(KeyCode::ArrowUp) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyW) ||
            joystick_input_data._btn_up  == ButtonState::Hold;
        let is_jump =
            keyboard_input_data.get_key_pressed(KeyCode::Space) ||
            joystick_input_data._btn_a == ButtonState::Pressed;
        let is_run =
            keyboard_input_data.get_key_pressed(KeyCode::ShiftLeft) ||
            joystick_input_data._btn_left_shoulder == ButtonState::Pressed;
        let is_roll =
            keyboard_input_data.get_key_pressed(KeyCode::AltLeft) ||
            joystick_input_data._btn_b == ButtonState::Pressed ||
            joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let is_zoom_in = joystick_input_data._btn_left_trigger == ButtonState::Hold;
        let is_zoom_out = joystick_input_data._btn_right_trigger == ButtonState::Hold;

        let _pitch_control: f32 = if mouse_move_data._mouse_pos_delta.x != 0 {
            mouse_move_data._mouse_pos_delta.y as f32 * 0.001
        } else {
            joystick_input_data._stick_right_direction.y as f32 / 327670.0
        };

        let _yaw_control: f32 = if mouse_move_data._mouse_pos_delta.x != 0 {
            mouse_move_data._mouse_pos_delta.x as f32 * 0.001
        } else {
            joystick_input_data._stick_right_direction.x as f32 / 327670.0
        };

        let zoom_control: f32 = if 0 != mouse_move_data._scroll_delta.y {
            -mouse_move_data._scroll_delta.y as f32
        } else if lock_camera_rotation && 0 != joystick_input_data._stick_right_direction.y {
            joystick_input_data._stick_right_direction.y as f32 / 65535.0
        } else if lock_camera_rotation == false && (is_zoom_in || is_zoom_out) {
            if is_zoom_in { -0.5 } else { 0.5 }
        } else {
            0.0
        };

        // set action & move
        let mut player_mut = player.borrow_mut();
        {
            let mut move_direction: Vector3<f32> = Vector3::new(
                joystick_input_data._stick_left_direction.x as f32,
                0.0,
                -(joystick_input_data._stick_left_direction.y as f32)
            );
            if is_left {
                move_direction.x = -1.0;
            }
            else if is_right {
                move_direction.x = 1.0;
            }

            if is_up {
                move_direction.z = 1.0;
            }
            else if is_down {
                move_direction.z = -1.0;
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
            if player_mut.is_in_pickup_prop_range() {
                player_mut.set_action_pickup();
            } else {
                player_mut.set_action_attack();
            }
        }

        if is_power_attack {
            player_mut.set_action_power_attack();
        }

        // update camera
        self.update_camera_distance(zoom_control, delta_time);

        if lock_camera_rotation {
            self.update_camera_pitch_by_distance();
        } else {
            self.update_camera_rotation(_pitch_control, _yaw_control, delta_time);
        }

        // update camera transform
        main_camera._transform_object.set_pitch(self._camera_pitch);
        main_camera._transform_object.set_yaw(self._camera_yaw);
        main_camera._transform_object.set_roll(0.0);
        main_camera._transform_object.update_transform_object();

        let mut camera_position = player_mut.get_position() - main_camera._transform_object.get_front() * self._camera_distance;
        camera_position.y += CAMERA_OFFSET_Y;
        let camera_min_height = self.get_game_client().get_game_scene_manager().get_scene_manager().get_sea_height() + CAMERA_SEA_HEIGHT_OFFSET;
        if camera_position.y < camera_min_height {
            camera_position.y = camera_min_height;
        }
        main_camera._transform_object.set_position(&camera_position);
    }
}
