use crate::game_module::widgets::world_map::api::WorldMapDirection;
use nalgebra::Vector2;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};

pub trait WorldMapControl<'a> {
    fn is_opened_world_map(&self) -> bool;
    fn open_world_map(&mut self);
    fn close_world_map(&mut self);
    fn is_requested_close_world_map(&self) -> bool;
    fn request_close_world_map(&mut self);
    fn changed_window_size(&mut self, window_size: &Vector2<i32>);
    fn teleport_selected_world_map_stage(&mut self);
    fn get_selected_world_map_stage_data_name(&self) -> &String;
    fn set_selected_world_map_stage(&mut self, selected_stage_name: &String);
    fn change_selected_world_map_stage(&mut self, direction: WorldMapDirection);
    fn update_world_map(&mut self, joystick_input_data: &JoystickInputData, keyboard_input_data: &KeyboardInputData);
}
