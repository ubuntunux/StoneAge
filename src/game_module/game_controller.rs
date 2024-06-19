use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use winit::keyboard::KeyCode;

use crate::application::application::Application;
use crate::game_module::actors::character::Character;
use crate::game_module::actors::character_data::MoveDirections;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::*;
use crate::game_module::game_ui_manager::GameUIManager;

pub struct GameController<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32
}

impl<'a> GameController<'a> {
    pub fn create_game_controller() -> Box<GameController<'a>> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: 0.0,
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
            joystick_input_data._btn_x == ButtonState::Pressed ||
            joystick_input_data._btn_right_trigger == ButtonState::Pressed;
        let is_power_attack: bool =
            mouse_input_data._btn_r_pressed ||
            joystick_input_data._btn_y == ButtonState::Pressed ||
            joystick_input_data._btn_left_trigger == ButtonState::Pressed;
        let is_left =
            keyboard_input_data.get_key_hold(KeyCode::ArrowLeft) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyA) ||
            joystick_input_data._btn_left == ButtonState::Hold ||
            joystick_input_data._stick_left_direction.x < 0;
        let is_right =
            keyboard_input_data.get_key_hold(KeyCode::ArrowRight) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyD) ||
            joystick_input_data._btn_right == ButtonState::Hold||
            0 < joystick_input_data._stick_left_direction.x;
        let _is_down =
            keyboard_input_data.get_key_hold(KeyCode::ArrowDown) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyS) ||
            joystick_input_data._btn_down  == ButtonState::Hold ||
            0 < joystick_input_data._stick_left_direction.y;
        let _is_up =
            keyboard_input_data.get_key_hold(KeyCode::ArrowUp) ||
            keyboard_input_data.get_key_hold(KeyCode::KeyW) ||
            joystick_input_data._btn_up  == ButtonState::Hold ||
            joystick_input_data._stick_left_direction.y < 0;
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

        // set action & move
        let mut player_mut = player.borrow_mut();
        if is_left {
            player_mut.set_move(MoveDirections::LEFT);
        }
        else if is_right {
            player_mut.set_move(MoveDirections::RIGHT);
        }
        // else if is_up {
        //     player_mut.set_move(MoveDirections::UP);
        // }
        // else if is_down {
        //     player_mut.set_move(MoveDirections::DOWN);
        // }
        else {
            player_mut.set_move_stop();
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

        // update camera
        self._camera_goal_distance -= mouse_move_data._scroll_delta.y as f32;
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

        let player_pos = player_mut.get_position();
        let mut camera_pos_y = main_camera._transform_object.get_position().y;
        let mut camera_position = player_pos - main_camera._transform_object.get_front() * self._camera_distance;
        let upper_camera_pos_y = camera_pos_y + UPPER_CAMERA_OFFSET_Y;
        let bottom_camera_pos_y = camera_pos_y - BOTTOM_CAMERA_OFFSET_Y;
        if upper_camera_pos_y < player_pos.y {
            camera_pos_y = player_pos.y - UPPER_CAMERA_OFFSET_Y;
        } else if player_pos.y < bottom_camera_pos_y {
            camera_pos_y = player_pos.y + BOTTOM_CAMERA_OFFSET_Y;
        }

        if camera_pos_y < CAMERA_POSITION_Y_MIN {
            camera_pos_y = CAMERA_POSITION_Y_MIN;
        }
        camera_position.y = camera_pos_y;

        main_camera._transform_object.set_position(&camera_position);
        main_camera._transform_object.set_pitch(CAMERA_PITCH);
        main_camera._transform_object.set_yaw(0.0);
        main_camera._transform_object.set_roll(0.0);
    }
}
