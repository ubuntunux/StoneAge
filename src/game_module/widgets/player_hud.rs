use crate::game_module::actors::character::Character;
use crate::game_module::widgets::status_bar_widget::StatusBarWidget;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub struct PlayerHud<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _hp_widget: StatusBarWidget<'a>,
    pub _stamina_widget: StatusBarWidget<'a>,
}

// PlayerHud
impl<'a> PlayerHud<'a> {
    pub fn create_player_hud(root_widget: &mut WidgetDefault<'a>) -> PlayerHud<'a> {
        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;

        let player_widget = UIManager::create_widget("player_widget", UIWidgetTypes::Default);
        let player_widget_ptr = ptr_as_mut(player_widget.as_ref());
        let ui_component = ptr_as_mut(player_widget.as_ref()).get_ui_component_mut();
        ui_component.set_pos(100.0, 100.0);
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_round(10.0);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        root_widget.add_widget(&player_widget);

        PlayerHud {
            _widget: player_widget_ptr,
            _hp_widget: StatusBarWidget::create_status_widget(
                player_widget_ptr,
                get_color32(255, 64, 0, 128),
            ),
            _stamina_widget: StatusBarWidget::create_status_widget(
                player_widget_ptr,
                get_color32(128, 128, 255, 128),
            ),
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_pos_x(10.0);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_ui_size().y - 10.0);
    }

    pub fn update_status_widget(&mut self, player: &Character<'a>, delta_time: f64) {
        self._hp_widget.update_status_widget(
            player.get_stats().get_hp() as f32,
            player.get_stats().get_max_hp() as f32,
            player.get_stats().get_max_hp_data() as f32,
            delta_time
        );
        self._stamina_widget.update_status_widget(
            player.get_stats().get_stamina(),
            player.get_stats().get_max_stamina(),
            player.get_stats().get_max_stamina_data(),
            delta_time
        );
    }
}
