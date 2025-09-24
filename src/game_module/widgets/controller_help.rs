use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation,
    UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_scene_manager::GameSceneManager;

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (250.0, 150.0);
const ITEM_HEIGHT: f32 = 24.0;

pub struct ControllerHelpWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _attack_key_binding_widget: *const WidgetDefault<'a>,
    pub _power_attack_key_binding_widget: *const WidgetDefault<'a>,
}

// ControllerHelpWidget
impl<'a> ControllerHelpWidget<'a> {
    pub fn create_key_binding_widget(parent_widget: &mut WidgetDefault<'a>, widget_name: &str, key_binding_text: &str) -> *const WidgetDefault<'a> {
        let key_binding_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(key_binding_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_HEIGHT);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(ITEM_HEIGHT);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text(key_binding_text);
        parent_widget.add_widget(&key_binding_widget);
        key_binding_widget.as_ref()
    }

    pub fn create_controller_help_widget(root_widget: &mut WidgetDefault<'a>) -> ControllerHelpWidget<'a> {
        let controller_help_widget = UIManager::create_widget("controller_help_widget", UIWidgetTypes::Default);
        let controller_help_widget_mut = ptr_as_mut(controller_help_widget.as_ref());
        let ui_component = ptr_as_mut(controller_help_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_padding(10.0);
        ui_component.set_round(15.0);
        ui_component.set_size(MAIN_LAYOUT_SIZE.0, MAIN_LAYOUT_SIZE.1);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&controller_help_widget);

        let attack_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(controller_help_widget_mut, "attack_key_binding", "Attack: Left Click");
        let power_attack_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(controller_help_widget_mut, "power_attack_key_binding", "Power Attack: Right Click");

        ControllerHelpWidget {
            _widget: controller_help_widget_mut,
            _attack_key_binding_widget: attack_key_binding_widget,
            _power_attack_key_binding_widget: power_attack_key_binding_widget,
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_size_x() - MAIN_LAYOUT_MARGIN);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - MAIN_LAYOUT_MARGIN);
    }

    pub fn update_controller_help_widget(&mut self, _game_scene_manager: &GameSceneManager) {
    }
}
