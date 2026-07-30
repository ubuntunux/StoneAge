use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, DEFAULT_GATE_NAME, MATERIAL_WORLDMAP};
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::get_game_scene_manager_mut;
use crate::game_module::widgets::world_map::api::{WorldMapDirection, WorldMapPlayer, WorldMapStage, WorldMapWidget};
use crate::game_module::widgets::world_map::control_trait::WorldMapControl;
use crate::game_module::widgets::world_map::layout_trait::WorldMapLayout;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::collections::HashMap;
use std::rc::Rc;
use winit::keyboard::KeyCode;

impl<'a> WorldMapWidget<'a> {
    pub fn new(root_widget: &mut WidgetDefault<'a>, window_size: &Vector2<i32>) -> Box<WorldMapWidget<'a>> {
        Self::create_world_map_widget(root_widget, window_size)
    }

    pub fn create_world_map_widget(
        root_widget: &mut WidgetDefault<'a>,
        _window_size: &Vector2<i32>,
    ) -> Box<WorldMapWidget<'a>> {
        let background_layout = UIManager::create_widget("background image layout", UIWidgetTypes::Default);
        let background_layout_mut = ptr_as_mut(background_layout.as_ref());
        let ui_component = background_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_color(get_color32(80, 80, 180, 255));
        ui_component.set_enable(false);
        root_widget.add_widget(&background_layout);

        let world_map_material_instance = get_engine_resources().get_material_instance_data(MATERIAL_WORLDMAP);
        let texture_parameter =
            world_map_material_instance.borrow()._material_parameters.get("texture_color").unwrap().clone();
        let texture_name = texture_parameter.as_str().unwrap();
        let texture = get_engine_resources().get_texture_data(texture_name);
        let image_width = texture.borrow()._image_width as f32;
        let image_height = texture.borrow()._image_height as f32;
        let image_aspect = image_width / image_height;

        let max_bounds = Vector2::new(1400.0, 800.0);
        let scale = (max_bounds.x / image_width).min(max_bounds.y / image_height);
        let map_size = Vector2::new(image_width * scale, image_height * scale);

        let world_map_widget = UIManager::create_widget("world_map_widget", UIWidgetTypes::Default);
        let world_map_widget_mut = ptr_as_mut(world_map_widget.as_ref());
        let ui_component = world_map_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_material_instance(Some(world_map_material_instance.clone()));
        background_layout_mut.add_widget(&world_map_widget);

        let bridge_layer_widget = UIManager::create_widget("bridge_layer_widget", UIWidgetTypes::Default);
        let bridge_layer_widget_mut = ptr_as_mut(bridge_layer_widget.as_ref());
        let ui_component = bridge_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&bridge_layer_widget);

        let stage_layer_widget = UIManager::create_widget("stage_layer_widget", UIWidgetTypes::Default);
        let stage_layer_widget_mut = ptr_as_mut(stage_layer_widget.as_ref());
        let ui_component = stage_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&stage_layer_widget);

        let player_layer_widget = UIManager::create_widget("player_layer_widget", UIWidgetTypes::Default);
        let player_layer_widget_mut = ptr_as_mut(player_layer_widget.as_ref());
        let ui_component = player_layer_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_size(map_size.x, map_size.y);
        ui_component.set_pos(0.0, 0.0);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        world_map_widget_mut.add_widget(&player_layer_widget);

        let mut world_map_widget = Box::new(WorldMapWidget {
            _root_widget: root_widget,
            _background_layout: background_layout.clone(),
            _world_map_widget: world_map_widget.clone(),
            _bridge_layer_widget: bridge_layer_widget.clone(),
            _stage_layer_widget: stage_layer_widget.clone(),
            _player_layer_widget: player_layer_widget.clone(),
            _image_aspect: image_aspect,
            _selected_stage_name: String::new(),
            _world_map_player: None,
            _world_map_stages: HashMap::new(),
            _is_opened_world_map: false,
            _request_close_world_map: false,
        });

        world_map_widget.as_mut()._world_map_player = Some(WorldMapPlayer::create_world_map_player(
            world_map_widget.as_ref(),
            player_layer_widget_mut,
        ));
        world_map_widget.as_mut()._world_map_stages = <Self as WorldMapLayout>::create_world_map_stages(
            world_map_widget.as_ref(),
            stage_layer_widget_mut,
            bridge_layer_widget_mut,
            &map_size,
        );

        world_map_widget
    }

    // Direct methods forwarding to trait implementations for convenience
    pub fn is_opened_world_map(&self) -> bool {
        <Self as WorldMapControl>::is_opened_world_map(self)
    }

    pub fn open_world_map(&mut self) {
        <Self as WorldMapControl>::open_world_map(self)
    }

    pub fn close_world_map(&mut self) {
        <Self as WorldMapControl>::close_world_map(self)
    }

    pub fn is_requested_close_world_map(&self) -> bool {
        <Self as WorldMapControl>::is_requested_close_world_map(self)
    }

    pub fn request_close_world_map(&mut self) {
        <Self as WorldMapControl>::request_close_world_map(self)
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        <Self as WorldMapControl>::changed_window_size(self, window_size)
    }

    pub fn teleport_selected_world_map_stage(&mut self) {
        <Self as WorldMapControl>::teleport_selected_world_map_stage(self)
    }

    pub fn get_selected_world_map_stage_data_name(&self) -> &String {
        <Self as WorldMapControl>::get_selected_world_map_stage_data_name(self)
    }

    pub fn set_selected_world_map_stage(&mut self, selected_stage_name: &String) {
        <Self as WorldMapControl>::set_selected_world_map_stage(self, selected_stage_name)
    }

    pub fn change_selected_world_map_stage(&mut self, direction: WorldMapDirection) {
        <Self as WorldMapControl>::change_selected_world_map_stage(self, direction)
    }

    pub fn update_world_map(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        <Self as WorldMapControl>::update_world_map(self, joystick_input_data, keyboard_input_data)
    }
}

impl<'a> WorldMapLayout<'a> for WorldMapWidget<'a> {
    fn create_world_map_stages(
        world_map_widget: &WorldMapWidget<'a>,
        stage_layer: &mut WidgetDefault<'a>,
        bridge_layer: &mut WidgetDefault<'a>,
        map_size: &Vector2<f32>,
    ) -> HashMap<String, Rc<WorldMapStage<'a>>> {
        let mut world_map_stages = HashMap::new();
        let world_map_stage_home =
            WorldMapStage::create_world_map_stage(&mut world_map_stages, world_map_widget, stage_layer, Stages::Home);
        let world_map_stage_forest =
            WorldMapStage::create_world_map_stage(&mut world_map_stages, world_map_widget, stage_layer, Stages::Forest);
        let world_map_stage_cave =
            WorldMapStage::create_world_map_stage(&mut world_map_stages, world_map_widget, stage_layer, Stages::Cave);
        let world_map_stage_ufo =
            WorldMapStage::create_world_map_stage(&mut world_map_stages, world_map_widget, stage_layer, Stages::Ufo);

        let map_center = map_size * 0.5;
        ptr_as_mut(world_map_stage_home.as_ref()).set_center_pos(map_center.x, map_center.y);
        Self::set_linked_stage(
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_forest,
            WorldMapDirection::RIGHT,
            map_size,
        );
        Self::set_linked_stage(
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_cave,
            WorldMapDirection::DOWN,
            map_size,
        );
        Self::set_linked_stage(
            bridge_layer,
            &world_map_stage_home,
            &world_map_stage_ufo,
            WorldMapDirection::LEFT,
            map_size,
        );

        world_map_stages
    }

    fn set_linked_stage(
        bridge_layer: &mut WidgetDefault<'a>,
        stage: &Rc<WorldMapStage<'a>>,
        linked_stage: &Rc<WorldMapStage<'a>>,
        direction: WorldMapDirection,
        map_size: &Vector2<f32>,
    ) {
        ptr_as_mut(stage.as_ref()).set_linked_stage(bridge_layer, direction, linked_stage, map_size, true);
        ptr_as_mut(linked_stage.as_ref()).set_linked_stage(
            bridge_layer,
            direction.get_opposite_direction(),
            stage,
            map_size,
            false,
        );
    }
}

impl<'a> WorldMapControl<'a> for WorldMapWidget<'a> {
    fn is_opened_world_map(&self) -> bool {
        self._is_opened_world_map
    }

    fn open_world_map(&mut self) {
        if !self._is_opened_world_map {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            ptr_as_mut(self._background_layout.as_ref()).get_ui_component_mut().set_enable(true);
            self._request_close_world_map = false;
            self._is_opened_world_map = true;
        }
    }

    fn close_world_map(&mut self) {
        if self._is_opened_world_map {
            ptr_as_mut(self._background_layout.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened_world_map = false;
        }
    }

    fn is_requested_close_world_map(&self) -> bool {
        self._request_close_world_map
    }

    fn request_close_world_map(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self._request_close_world_map = true;
    }

    fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    fn teleport_selected_world_map_stage(&mut self) {
        self.set_selected_world_map_stage(&self._selected_stage_name.clone());
    }

    fn get_selected_world_map_stage_data_name(&self) -> &String {
        &self._selected_stage_name
    }

    fn set_selected_world_map_stage(&mut self, selected_stage_name: &String) {
        if !self._selected_stage_name.is_empty() && !selected_stage_name.is_empty() {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }

        if self._selected_stage_name == *selected_stage_name {
            if let Some(selected_stage) = self._world_map_stages.get_mut(selected_stage_name) {
                let teleport_stage: &String = ptr_as_ref(selected_stage.as_ref()).get_stage_data_name();
                get_game_scene_manager_mut().set_teleport_stage(teleport_stage, DEFAULT_GATE_NAME);
            }
        } else {
            if let Some(prev_selected_stage) = self._world_map_stages.get_mut(&self._selected_stage_name) {
                ptr_as_mut(prev_selected_stage.as_ref()).set_selected(false);
            }

            if let Some(selected_stage) = self._world_map_stages.get_mut(selected_stage_name) {
                ptr_as_mut(selected_stage.as_ref()).set_selected(true);
                let pos: Vector2<f32> = ptr_as_ref(selected_stage.as_ref()).get_center_pos();
                self._world_map_player.as_mut().unwrap().set_center_pos(pos.x, pos.y);
            }

            self._selected_stage_name = selected_stage_name.clone();
        }
    }

    fn change_selected_world_map_stage(&mut self, direction: WorldMapDirection) {
        if let Some(selected_stage) = self._world_map_stages.get_mut(&self._selected_stage_name)
            && let Some(linked_stage) = ptr_as_ref(selected_stage.as_ref()).get_linked_stage(direction).as_ref()
        {
            self.set_selected_world_map_stage(linked_stage.get_stage_data_name());
        }
    }

    fn update_world_map(&mut self, joystick_input_data: &JoystickInputData, keyboard_input_data: &KeyboardInputData) {
        let is_left = keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || joystick_input_data._btn_left == ButtonState::Pressed;
        let is_right = keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || joystick_input_data._btn_right == ButtonState::Pressed;
        let is_down = keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || joystick_input_data._btn_down == ButtonState::Pressed;
        let is_up = keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || joystick_input_data._btn_up == ButtonState::Pressed;
        let is_interaction = keyboard_input_data.get_key_pressed(KeyCode::KeyF)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || joystick_input_data._btn_x == ButtonState::Pressed
            || joystick_input_data._btn_a == ButtonState::Pressed;

        let joystick_sensitivity: f32 = 0.1 / 32767.0;
        let _stick_left_direction = Vector2::<f32>::new(
            joystick_input_data._stick_left_direction.x as f32,
            joystick_input_data._stick_left_direction.y as f32,
        ) * joystick_sensitivity;
        let _stick_right_direction = Vector2::<f32>::new(
            joystick_input_data._stick_right_direction.x as f32,
            joystick_input_data._stick_right_direction.y as f32,
        ) * joystick_sensitivity;

        if keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_start == ButtonState::Pressed
            || joystick_input_data._btn_b == ButtonState::Pressed
        {
            self.request_close_world_map();
        }

        let world_map_direction = if is_left {
            WorldMapDirection::LEFT
        } else if is_right {
            WorldMapDirection::RIGHT
        } else if is_up {
            WorldMapDirection::UP
        } else if is_down {
            WorldMapDirection::DOWN
        } else {
            WorldMapDirection::COUNT
        };
        self.change_selected_world_map_stage(world_map_direction);

        if is_interaction {
            self.teleport_selected_world_map_stage();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_map_direction_opposite() {
        assert_eq!(
            WorldMapDirection::LEFT.get_opposite_direction(),
            WorldMapDirection::RIGHT
        );
        assert_eq!(
            WorldMapDirection::RIGHT.get_opposite_direction(),
            WorldMapDirection::LEFT
        );
        assert_eq!(WorldMapDirection::UP.get_opposite_direction(), WorldMapDirection::DOWN);
        assert_eq!(WorldMapDirection::DOWN.get_opposite_direction(), WorldMapDirection::UP);
        assert_eq!(
            WorldMapDirection::COUNT.get_opposite_direction(),
            WorldMapDirection::COUNT
        );
    }
}
