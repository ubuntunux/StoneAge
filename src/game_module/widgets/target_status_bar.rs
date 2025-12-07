use crate::game_module::actors::character::Character;
use crate::game_module::widgets::status_bar_widget::StatusBarWidget;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub struct TargetStatusWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _target_name_widget: *const WidgetDefault<'a>,
    pub _hp_widget: StatusBarWidget<'a>,
    pub _target: *const Character<'a>,
    pub _fade_time: f64,
}

// TargetStatusWidget
impl<'a> TargetStatusWidget<'a> {
    pub fn create_target_status_widget(
        root_widget: &mut WidgetDefault<'a>,
    ) -> TargetStatusWidget<'a> {
        let target_status_widget = UIManager::create_widget("target_status_widget", UIWidgetTypes::Default);
        let target_status_widget_ptr = ptr_as_mut(target_status_widget.as_ref());
        let ui_component = ptr_as_mut(target_status_widget.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_round(10.0);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&target_status_widget);

        let target_name_widget = UIManager::create_widget("target_name_widget", UIWidgetTypes::Default);
        let target_name_widget_ptr = ptr_as_mut(target_name_widget.as_ref());
        let ui_component = ptr_as_mut(target_name_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text("Name");
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(20.0);
        ui_component.set_font_size(35.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        target_status_widget_ptr.add_widget(&target_name_widget);

        let hp_widget = StatusBarWidget::create_status_widget(
            target_status_widget_ptr,
            get_color32(255, 64, 0, 128),
        );

        TargetStatusWidget {
            _widget: target_status_widget_ptr,
            _target_name_widget: target_name_widget_ptr,
            _hp_widget: hp_widget,
            _target: std::ptr::null(),
            _fade_time: 0.0,
        }
    }
    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_center_x(window_size.x as f32 * 0.5);
        ui_component.set_pos_y(50.0);
    }
    pub fn update_status_widget(&mut self, target: &Character<'a>, delta_time: f64) {
        let mut smooth_update: bool = true;
        if self._target != target {
            smooth_update = false;
            ptr_as_mut(self._widget).get_ui_component_mut().set_visible(true);
            let name = target._character_data.borrow()._name.clone();
            ptr_as_mut(self._target_name_widget).get_ui_component_mut().set_text(name.as_str());
            self._target = target;
        }
        let hp = target.get_stats().get_hp() as f32;
        let max_hp = target.get_stats().get_max_hp() as f32;
        let max_hp_data = target.get_character_data()._stat_data._max_hp as f32;
        self._hp_widget.update_status_widget(hp, max_hp, max_hp_data, delta_time, smooth_update);
    }
    pub fn fade_out_status_widget(&mut self) {
        ptr_as_mut(self._widget).get_ui_component_mut().set_visible(false);
        self._target = std::ptr::null();
    }
}
