use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::Vector2;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_constants::{MATERIAL_UI_CONTROLLER_JUMP, MATERIAL_UI_CONTROLLER_MOUSE_L, MATERIAL_UI_CONTROLLER_MOUSE_R, MATERIAL_UI_CONTROLLER_MOVE, MATERIAL_UI_CONTROLLER_PICKUP, MATERIAL_UI_CONTROLLER_ROLL, MATERIAL_UI_CONTROLLER_SPRINT, MATERIAL_UI_CONTROLLER_ZOOM};
use crate::game_module::game_resource::GameResources;

const MAIN_LAYOUT_MARGIN: f32 = 10.0;
const MAIN_LAYOUT_SIZE: (f32, f32) = (250.0, 420.0);
const ITEM_SIZE: f32 = 50.0;
const FONT_SIZE: f32 = 24.0;

pub struct ControllerHelpWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _attack_key_binding_widget: *const WidgetDefault<'a>,
    pub _power_attack_key_binding_widget: *const WidgetDefault<'a>,
}

// ControllerHelpWidget
impl<'a> ControllerHelpWidget<'a> {
    pub fn create_key_binding_widget(
        parent_widget: &mut WidgetDefault<'a>,
        material_instance: &RcRefCell<MaterialInstanceData<'a>>,
        widget_name: &str,
        key_binding_text: &str,
    ) -> *const WidgetDefault<'a> {
        let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
        let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        parent_widget.add_widget(&layout_widget);

        let key_binding_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(key_binding_widget.as_ref()).get_ui_component_mut();
        ui_component.set_material_instance(Some(material_instance.clone()));
        ui_component.set_size_x(ITEM_SIZE);
        ui_component.set_size_y(ITEM_SIZE);
        layout_widget_mut.add_widget(&key_binding_widget);

        let key_binding_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(key_binding_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(ITEM_SIZE);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_text(key_binding_text);
        layout_widget_mut.add_widget(&key_binding_widget);

        layout_widget.as_ref()
    }

    pub fn create_controller_help_widget(
        root_widget: &mut WidgetDefault<'a>,
        game_resources: &GameResources<'a>,
    ) -> ControllerHelpWidget<'a> {
        let engine_resources = game_resources.get_engine_resources();
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

        let attack_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_MOUSE_L),
            "attack_key_binding",
            "Attack",
        );

        let power_attack_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_MOUSE_R),
            "power_attack_key_binding",
            "Power Attack",
        );

        let zoom_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_ZOOM),
            "zoom_key_binding",
            "Zoom",
        );

        let move_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_MOVE),
            "move_key_binding",
            "Move",
        );

        let sprint_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_SPRINT),
            "sprint_key_binding",
            "Sprint",
        );

        let jump_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_JUMP),
            "jump_key_binding",
            "Jump",
        );

        let roll_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_ROLL),
            "roll_key_binding",
            "Roll",
        );

        let pickup_key_binding_widget = ControllerHelpWidget::create_key_binding_widget(
            controller_help_widget_mut,
            engine_resources.get_material_instance_data(MATERIAL_UI_CONTROLLER_PICKUP),
            "pickup_key_binding",
            "Pickup",
        );

        ControllerHelpWidget {
            _widget: controller_help_widget_mut,
            _attack_key_binding_widget: attack_key_binding_widget,
            _power_attack_key_binding_widget: power_attack_key_binding_widget,
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component
            .set_pos_x(window_size.x as f32 - ui_component.get_size_x() - MAIN_LAYOUT_MARGIN);
        ui_component
            .set_pos_y(window_size.y as f32 - ui_component.get_size_y() - MAIN_LAYOUT_MARGIN);
    }

    pub fn update_controller_help_widget(&mut self, _game_scene_manager: &GameSceneManager) {}
}
