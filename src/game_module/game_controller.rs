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
    pub _camera_goal_yaw: f32,
    pub _camera_yaw: f32
}

impl<'a> GameController<'a> {
    pub fn create_game_controller() -> Box<GameController<'a>> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: 0.0,
            _camera_goal_yaw: 0.0,
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
            player_mut.set_action_attack();
        }

        if is_power_attack {
            player_mut.set_action_power_attack();
        }

        // update camera zoom
        let mut zoom = -mouse_move_data._scroll_delta.y as f32;
        if is_zoom_in {
            zoom = -0.5;
        } else if is_zoom_out {
            zoom = 0.5;
        }

        self._camera_goal_distance += zoom;
        self._camera_goal_distance = CAMERA_DISTANCE_MIN.max(CAMERA_DISTANCE_MAX.min(self._camera_goal_distance));
        if self._camera_goal_distance != self._camera_distance {
            let diff = (self._camera_goal_distance - self._camera_distance) * CAMERA_ZOOM_SPEED;
            let sign = diff.signum();
            let delta =  diff * time_data._delta_time as f32;
            self._camera_distance += delta;
            if sign != (self._camera_goal_distance - self._camera_distance).signum() {
                self._camera_distance = self._camera_goal_distance;
            }
        }

        // yaw
        static SMOOTH_YAW: bool = false;
        let yaw_control: f32 = if mouse_move_data._mouse_pos_delta.x != 0 {
            mouse_move_data._mouse_pos_delta.x as f32 * 0.001
        } else {
            joystick_input_data._stick_right_direction.x as f32 / 327670.0
        };

        if SMOOTH_YAW {
            self._camera_goal_yaw += yaw_control;
            let diff_yaw = (self._camera_goal_yaw - self._camera_yaw).abs();
            let t: f32 = 0f32.max(1f32.min(diff_yaw / std::f32::consts::PI));
            let yaw_speed: f32 = math::lerp(CAMERA_YAW_SPEED_MIN, CAMERA_YAW_SPEED_MAX, t);
            let yaw_delta = yaw_speed * time_data._delta_time as f32;
            if diff_yaw < yaw_delta {
                self._camera_yaw = self._camera_goal_yaw;
            } else if self._camera_goal_yaw < self._camera_yaw {
                self._camera_yaw -= yaw_delta;
            } else {
                self._camera_yaw += yaw_delta;
            }
        } else {
            self._camera_yaw = main_camera._transform_object.get_yaw() + yaw_control;
        }

        // update camera transform
        let dist_ratio = (self._camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN);
        let pitch: f32 = math::degree_to_radian(math::lerp(CAMERA_PITCH_MIN, CAMERA_PITCH_MAX, dist_ratio));
        main_camera._transform_object.set_pitch(pitch);
        main_camera._transform_object.set_yaw(self._camera_yaw);
        main_camera._transform_object.set_roll(0.0);
        main_camera._transform_object.update_transform_object();

        let player_pos = player_mut.get_position();
        let mut camera_position = player_pos - main_camera._transform_object.get_front() * self._camera_distance;
        camera_position.y += CAMERA_OFFSET_Y;
        main_camera._transform_object.set_position(&camera_position);
    }
}
