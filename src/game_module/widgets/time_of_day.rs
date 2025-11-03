use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub struct TimeOfDayWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _date_widget: *const WidgetDefault<'a>,
    pub _time_widget: *const WidgetDefault<'a>,
    pub _temperature: *const WidgetDefault<'a>,
}

// TimeOfDayWidget
impl<'a> TimeOfDayWidget<'a> {
    pub fn create_time_of_day_widget(root_widget: &mut WidgetDefault<'a>) -> TimeOfDayWidget<'a> {
        let time_of_day_widget =
            UIManager::create_widget("time_of_day_widget", UIWidgetTypes::Default);
        let time_of_day_widget_ptr = ptr_as_mut(time_of_day_widget.as_ref());
        let ui_component = ptr_as_mut(time_of_day_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(250.0, 150.0);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_margin(10.0);
        ui_component.set_padding(10.0);
        ui_component.set_round(15.0);
        ui_component.set_border(2.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_color(get_color32(200, 180, 0, 200));
        root_widget.add_widget(&time_of_day_widget);

        let date_widget = UIManager::create_widget("date_widget", UIWidgetTypes::Default);
        let date_widget_ptr = ptr_as_mut(date_widget.as_ref());
        let ui_component = ptr_as_mut(date_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.3));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        time_of_day_widget_ptr.add_widget(&date_widget);

        let time_widget = UIManager::create_widget("time_widget", UIWidgetTypes::Default);
        let time_widget_ptr = ptr_as_mut(time_widget.as_ref());
        let ui_component = ptr_as_mut(time_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.3));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        time_of_day_widget_ptr.add_widget(&time_widget);

        let temperature_widget = UIManager::create_widget("temperature", UIWidgetTypes::Default);
        let temperature_widget_ptr = ptr_as_mut(temperature_widget.as_ref());
        let ui_component = ptr_as_mut(temperature_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.3));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        time_of_day_widget_ptr.add_widget(&temperature_widget);

        TimeOfDayWidget {
            _widget: time_of_day_widget_ptr,
            _date_widget: date_widget_ptr,
            _time_widget: time_widget_ptr,
            _temperature: temperature_widget_ptr,
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_pos_x(window_size.x as f32 - ui_component.get_size_x());
        ui_component.set_pos_y(50.0);
    }

    pub fn update_time_of_day_widget(&mut self, game_scene_manager: &GameSceneManager) {
        let date_ui_component = ptr_as_mut(self._date_widget).get_ui_component_mut();
        date_ui_component.set_text(format!("DAY {}", game_scene_manager.get_date()).as_str());

        let time_ui_component = ptr_as_mut(self._time_widget).get_ui_component_mut();
        let time_of_day = game_scene_manager.get_time_of_day();
        let time = time_of_day as u32;
        let minute = (time_of_day.fract() * 60.0) as u32;
        time_ui_component.set_text(format!("{:02}:{:02}", time, minute).as_str());

        let temperature_ui_component = ptr_as_mut(self._temperature).get_ui_component_mut();
        temperature_ui_component
            .set_text(format!("Temperature {:.01}", game_scene_manager.get_temperature()).as_str());
    }
}
