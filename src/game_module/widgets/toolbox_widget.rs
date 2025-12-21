use std::rc::Rc;
use nalgebra::Vector2;
use winit::keyboard::KeyCode;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::character::Character;

const ITEM_HEIGHT: f32 = 100.0;

pub struct ToolboxItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
}

impl<'a> ToolboxItem<'a> {
    pub fn create_toolbox_item(
        parent_widget: &mut WidgetDefault<'a>
    ) -> ToolboxItem<'a> {
        let layout = UIManager::create_widget("toolbox_item", UIWidgetTypes::Default);
        let layout_mut = ptr_as_mut(layout.as_ref());
        let ui_component = layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_HEIGHT);
        ui_component.set_renderable(false);
        parent_widget.add_widget(&layout);

        let item_icon = UIManager::create_widget("icon", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(item_icon.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(ITEM_HEIGHT);
        ui_component.set_size_y(ITEM_HEIGHT);
        ui_component.set_color(get_color32(50, 50, 50, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_margin(10.0);
        ui_component.set_round(5.0);
        ui_component.set_text("ICON");
        ui_component.set_font_size(40.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        layout_mut.add_widget(&item_icon);

        let ingredient_layout = UIManager::create_widget("ingredient", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(ingredient_layout.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_x(ITEM_HEIGHT * 2.0);
        ui_component.set_size_y(ITEM_HEIGHT);
        ui_component.set_color(get_color32(50, 50, 50, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_margin(10.0);
        ui_component.set_round(5.0);
        ui_component.set_text("INGREDIENT");
        ui_component.set_font_size(40.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        layout_mut.add_widget(&ingredient_layout);

        ToolboxItem {
            _layout: layout,
        }
    }
}

pub struct ToolboxWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _is_opened_toolbox: bool,
}

impl<'a> ToolboxWidget<'a> {
    pub fn create_toolbox_widget(
        _engine_resources: &EngineResources<'a>,
        parent_widget: &mut WidgetDefault<'a>,
    ) -> ToolboxWidget<'a> {
        let layer = UIManager::create_widget("toolbox_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);

        ToolboxItem::create_toolbox_item(layer_mut);

        ToolboxWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _is_opened_toolbox: false,
        }
    }
    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(0.5));
        ui_component.set_size_hint_y(Some(0.5));
    }
    pub fn is_opened_toolbox(&self) -> bool {
        self._is_opened_toolbox
    }
    pub fn open_toolbox(&mut self) {
        if self._is_opened_toolbox == false {
            ptr_as_mut(self._parent_widget).add_widget(&self._layer);
            self._is_opened_toolbox = true;
        }
    }
    pub fn close_toolbox(&mut self) {
        if self._is_opened_toolbox {
            ptr_as_mut(self._parent_widget).remove_widget(self._layer.as_ref());
            self._is_opened_toolbox = false;
        }
    }
    pub fn update_toolbox_widget(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character>
    ) {
        let _move_menu_up = keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || 0 < mouse_move_data._scroll_delta.y
            || joystick_input_data._btn_up == ButtonState::Hold;
        let _move_menu_down = keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || mouse_move_data._scroll_delta.y < 0
            || joystick_input_data._btn_down == ButtonState::Hold;
        let close_toolbox = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_b == ButtonState::Pressed;

        if close_toolbox {
            self.close_toolbox();
        }
    }
}
