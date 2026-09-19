use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::{
    HP_WARNING_RATIO, HUNGER_WARNING_THRESHOLD, HYPOTHERMIA_THRESHOLD, MAX_HUNGER, MIN_BODY_TEMPERATURE,
    MAX_BODY_TEMPERATURE,
};
use crate::game_module::widgets::fishing::FishingGaugeWidget;
use crate::game_module::widgets::status_bar_widget::StatusBarWidget;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub const HUD_HP_BAR_COLOR: u32 = get_color32(255, 64, 0, 128);
pub const HUD_STAMINA_BAR_COLOR: u32 = get_color32(128, 128, 255, 128);
pub const HUD_HUNGER_BAR_COLOR: u32 = get_color32(240, 170, 100, 128);
pub const HUD_TEMPERATURE_BAR_COLOR: u32 = get_color32(80, 200, 240, 128);

pub struct PlayerHud<'a> {
    pub _widget: *const WidgetDefault<'a>,
    pub _hp_widget: StatusBarWidget<'a>,
    pub _stamina_widget: StatusBarWidget<'a>,
    pub _hunger_widget: StatusBarWidget<'a>,
    pub _temperature_widget: StatusBarWidget<'a>,
    pub _fishing_gauge_widget: FishingGaugeWidget<'a>,
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
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_round(10.0);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_pivot_preset(rust_engine_3d::scene::ui::PIVOT_BOTTOM_LEFT);
        ui_component.set_pos_hint(Some(0.0), Some(1.0));
        ui_component.set_margin_left(10.0);
        ui_component.set_margin_bottom(10.0);
        root_widget.add_widget(&player_widget);

        let hp_widget = StatusBarWidget::create_status_widget(player_widget_ptr, HUD_HP_BAR_COLOR, true);
        let stamina_widget = StatusBarWidget::create_status_widget(player_widget_ptr, HUD_STAMINA_BAR_COLOR, true);
        let hunger_widget = StatusBarWidget::create_status_widget(player_widget_ptr, HUD_HUNGER_BAR_COLOR, false);
        let temperature_widget = StatusBarWidget::create_status_widget(player_widget_ptr, HUD_TEMPERATURE_BAR_COLOR, false);

        PlayerHud {
            _widget: player_widget_ptr,
            _hp_widget: hp_widget,
            _stamina_widget: stamina_widget,
            _hunger_widget: hunger_widget,
            _temperature_widget: temperature_widget,
            _fishing_gauge_widget: FishingGaugeWidget::create_fishing_gauge_widget(root_widget),
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn trigger_stamina_warning(&self) {
        self._stamina_widget.trigger_warning();
    }

    pub fn trigger_hunger_warning(&self) {
        self._hunger_widget.trigger_warning();
    }

    pub fn trigger_temperature_warning(&self) {
        self._temperature_widget.trigger_warning();
    }

    pub fn update_status_widget(&mut self, player: &Character<'a>, delta_time: f64) {
        self._hp_widget.update_status_widget(
            player.get_stats().get_hp() as f32,
            player.get_stats().get_max_hp() as f32,
            player.get_stats().get_max_hp_data() as f32,
            delta_time,
            true,
            Some(HP_WARNING_RATIO),
        );

        self._stamina_widget.update_status_widget(
            player.get_stats().get_stamina(),
            player.get_stats().get_max_stamina(),
            player.get_stats().get_max_stamina_data(),
            delta_time,
            true,
            None,
        );

        let hunger = player.get_stats().get_hunger();
        let satiety = (MAX_HUNGER - hunger).max(0.0);
        self._hunger_widget.update_status_widget(
            satiety,
            MAX_HUNGER,
            MAX_HUNGER,
            delta_time,
            true,
            Some(HUNGER_WARNING_THRESHOLD),
        );

        let body_temp = player.get_stats().get_body_temperature();
        let temp_range = MAX_BODY_TEMPERATURE - MIN_BODY_TEMPERATURE;
        let normalized_temp = (body_temp - MIN_BODY_TEMPERATURE).clamp(0.0, temp_range);
        let warning_threshold = (HYPOTHERMIA_THRESHOLD - MIN_BODY_TEMPERATURE) / temp_range;

        self._temperature_widget.update_status_widget(
            normalized_temp,
            temp_range,
            temp_range,
            delta_time,
            false,
            Some(warning_threshold),
        );

        if player.get_stats().is_hypothermia() {
            self._temperature_widget.trigger_warning();
        }

        if player.is_fishing_gauge_active() {
            self._fishing_gauge_widget.set_visible_fishing_gauge(true);
            self._fishing_gauge_widget.update_fishing_gauge_widget(player, delta_time);
        } else {
            self._fishing_gauge_widget.set_visible_fishing_gauge(false);
        }
    }
}
