use crate::game_module::game_constants::MATERIAL_TIME_OF_DAY;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager, Stages};
use crate::game_module::game_ui_manager::GameUIManager;
use ash::vk;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub struct TimeOfDayWidget<'a> {
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _time_of_day_widget: *const WidgetDefault<'a>,
    pub _date_widget: *const WidgetDefault<'a>,
    pub _time_widget: *const WidgetDefault<'a>,
    pub _temperature: *const WidgetDefault<'a>,
    pub _stage_widget: *const WidgetDefault<'a>,
}

// TimeOfDayWidget
impl<'a> TimeOfDayWidget<'a> {
    pub fn create_time_of_day_widget(
        root_widget: &mut WidgetDefault<'a>,
        game_resources: &GameResources<'a>,
        game_ui_manager: &GameUIManager<'a>,
    ) -> TimeOfDayWidget<'a> {
        let background_color = get_color32(0, 0, 0, 200);

        let parent_layer = UIManager::create_widget("time_of_day_widget", UIWidgetTypes::Default);
        let parent_layer_ptr = ptr_as_mut(parent_layer.as_ref());
        let ui_component = parent_layer_ptr.get_ui_component_mut();
        ui_component.set_size(250.0, 300.0);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Right(1.0));
        ui_component.set_pos_hint_y(PosHintY::Top(0.0));
        ui_component.set_margin_top(30.0);
        ui_component.set_round(15.0);
        ui_component.set_expandable(true);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        root_widget.add_widget(&parent_layer);

        // top layer
        let top_widget = UIManager::create_widget("tod_layer", UIWidgetTypes::Default);
        let top_widget_mut = ptr_as_mut(top_widget.as_ref());
        let ui_component = top_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.5));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_round(15.0);
        ui_component.set_color(background_color);
        parent_layer_ptr.add_widget(&top_widget);

        let tod_material_instance =
            game_resources.get_engine_resources().get_material_instance_data(MATERIAL_TIME_OF_DAY);
        let time_of_day_widget = UIManager::create_widget("tod_widget", UIWidgetTypes::Default);
        let time_of_day_widget_ptr = ptr_as_mut(time_of_day_widget.as_ref());
        let ui_component = time_of_day_widget_ptr.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(2.0));
        ui_component.set_texture_wrap_mode(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        ui_component.set_material_instance(Some(tod_material_instance.clone()));
        ui_component.set_enable_renderable_area(true);
        top_widget_mut.add_widget(&time_of_day_widget);

        // bottom layer
        let bottom_widget = UIManager::create_widget("top_widget", UIWidgetTypes::Default);
        let bottom_widget_mut = ptr_as_mut(bottom_widget.as_ref());
        let ui_component = bottom_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.5));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_round(15.0);
        ui_component.set_margin(10.0);
        ui_component.set_color(background_color);
        parent_layer_ptr.add_widget(&bottom_widget);

        let date_widget = UIManager::create_widget("date_widget", UIWidgetTypes::Default);
        let date_widget_ptr = ptr_as_mut(date_widget.as_ref());
        let ui_component = ptr_as_mut(date_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.25));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        bottom_widget_mut.add_widget(&date_widget);

        let time_widget = UIManager::create_widget("time_widget", UIWidgetTypes::Default);
        let time_widget_ptr = ptr_as_mut(time_widget.as_ref());
        let ui_component = ptr_as_mut(time_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.25));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        bottom_widget_mut.add_widget(&time_widget);

        let temperature_widget = UIManager::create_widget("temperature", UIWidgetTypes::Default);
        let temperature_widget_ptr = ptr_as_mut(temperature_widget.as_ref());
        let ui_component = ptr_as_mut(temperature_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(0.25));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_font_size(35.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(get_color32(255, 255, 255, 0));
        bottom_widget_mut.add_widget(&temperature_widget);

        let stage_widget = UIManager::create_widget("stage", UIWidgetTypes::Default);
        let stage_widget_ptr = ptr_as_mut(stage_widget.as_ref());
        let ui_component = ptr_as_mut(stage_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Top(0.0));
        ui_component.set_size_hint_x(Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_size_y(50.0);
        ui_component.set_padding(10.0);
        ui_component.set_margin_top(30.0);
        ui_component.set_round(15.0);
        ui_component.set_font_size(70.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_color(background_color);
        root_widget.add_widget(&stage_widget);

        TimeOfDayWidget {
            _game_ui_manager: game_ui_manager,
            _time_of_day_widget: time_of_day_widget_ptr,
            _date_widget: date_widget_ptr,
            _time_widget: time_widget_ptr,
            _temperature: temperature_widget_ptr,
            _stage_widget: stage_widget_ptr,
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn update_time_of_day_widget(&mut self, game_scene_manager: &GameSceneManager) {
        let time_of_day = game_scene_manager.get_time_of_day();
        let game_ui_manager = ptr_as_ref(self._game_ui_manager);

        let tod_ui_component = ptr_as_mut(self._time_of_day_widget).get_ui_component_mut();
        let tod_to_angle = (time_of_day - 12.0) / 24.0 * 360.0;
        tod_ui_component.set_rotation(tod_to_angle);

        let date_ui_component = ptr_as_mut(self._date_widget).get_ui_component_mut();
        date_ui_component.set_text(format!("DAY {}", game_scene_manager.get_date()).as_str());

        let time_ui_component = ptr_as_mut(self._time_widget).get_ui_component_mut();
        let time = time_of_day as u32;
        let minute = (time_of_day.fract() * 60.0) as u32;
        time_ui_component.set_text(format!("{:02}:{:02}", time, minute).as_str());

        let temperature_ui_component = ptr_as_mut(self._temperature).get_ui_component_mut();
        temperature_ui_component
            .set_text(format!("Temperature {:.01}", game_scene_manager.temperature()).as_str());

        let stage_widget_component = ptr_as_mut(self._stage_widget).get_ui_component_mut();
        if game_ui_manager.is_opened_world_map() {
            stage_widget_component.set_text(
                Stages::find_stage_value(
                    game_ui_manager.get_selected_world_map_stage_data_name().as_str(),
                )
                .get_stage_display_name(),
            );
        } else {
            let stage_data_name = game_scene_manager.get_current_game_scene_data_name();
            stage_widget_component
                .set_text(Stages::find_stage_value(stage_data_name).get_stage_display_name());
        }
    }
}
