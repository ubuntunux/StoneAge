use crate::game_module::actors::character::fishing::FISHING_ALIGNMENT_MATCH_DOT;
use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::MATERIAL_TIME_OF_DAY;
use crate::game_module::widgets::status_bar_widget::StatusBarWidget;
use ash::vk;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::ui::{
    PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

// UI Layout & Style Tuning Constants
pub const FISHING_UI_MAIN_PANEL_WIDTH: f32 = 360.0;
pub const FISHING_UI_MAIN_PANEL_HEIGHT: f32 = 220.0;
pub const FISHING_UI_DIRECTION_PANEL_SIZE: f32 = 170.0;
pub const FISHING_UI_NEEDLE_SIZE: f32 = 150.0;
pub const FISHING_UI_FISH_ICON_SIZE: f32 = 26.0;
pub const FISHING_UI_FISH_ORBIT_RADIUS: f32 = 65.0;
pub const FISHING_UI_VERTICAL_GAUGE_WIDTH: f32 = 35.0;
pub const FISHING_UI_VERTICAL_GAUGE_HEIGHT: f32 = 170.0;

// UI Color & Animation Tuning Constants
pub const FISHING_UI_BLINK_SPEED_BASE: f32 = 6.0;
pub const FISHING_UI_BLINK_SPEED_SCALE: f32 = 14.0;

pub struct FishingGaugeWidget<'a> {
    pub _main_layer: *const WidgetDefault<'a>,
    pub _direction_panel: *const WidgetDefault<'a>,
    pub _player_needle: *const WidgetDefault<'a>,
    pub _fish_indicator: *const WidgetDefault<'a>,
    pub _fish_icon_widget: *const WidgetDefault<'a>,
    pub _status_text: *const WidgetDefault<'a>,
    pub _vertical_gauge: StatusBarWidget<'a>,
    pub _cast_gauge: StatusBarWidget<'a>,
}

impl<'a> FishingGaugeWidget<'a> {
    pub fn create_fishing_gauge_widget(parent_widget: &mut WidgetDefault<'a>) -> FishingGaugeWidget<'a> {
        let main_layer = UIManager::create_widget("fishing_main_layer", UIWidgetTypes::Default);
        let main_layer_ptr = ptr_as_mut(main_layer.as_ref());
        let ui = main_layer_ptr.get_ui_component_mut();
        ui.set_size(FISHING_UI_MAIN_PANEL_WIDTH, FISHING_UI_MAIN_PANEL_HEIGHT);
        ui.set_layout_type(UILayoutType::FloatLayout);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.72));
        ui.set_color(get_color32(15, 20, 30, 200));
        ui.set_border_color(get_color32(60, 100, 150, 255));
        ui.set_border(3.0);
        ui.set_round(20.0);
        ui.set_visible(false);
        parent_widget.add_widget(&main_layer);

        // Direction dial panel (referencing TimeOfDayWidget circular layout)
        let direction_panel = UIManager::create_widget("direction_panel", UIWidgetTypes::Default);
        let direction_panel_ptr = ptr_as_mut(direction_panel.as_ref());
        let ui = direction_panel_ptr.get_ui_component_mut();
        ui.set_size(FISHING_UI_DIRECTION_PANEL_SIZE, FISHING_UI_DIRECTION_PANEL_SIZE);
        ui.set_pos(20.0, 25.0);
        ui.set_round(FISHING_UI_DIRECTION_PANEL_SIZE * 0.5);
        ui.set_color(get_color32(30, 45, 65, 230));
        ui.set_border_color(get_color32(80, 130, 190, 255));
        ui.set_border(3.0);
        main_layer_ptr.add_widget(&direction_panel);

        // Fish escaping direction indicator (inside dial panel)
        let tod_material = get_engine_resources().get_material_instance_data(MATERIAL_TIME_OF_DAY);
        let fish_indicator = UIManager::create_widget("fish_indicator", UIWidgetTypes::Default);
        let fish_indicator_ptr = ptr_as_mut(fish_indicator.as_ref());
        let ui = fish_indicator_ptr.get_ui_component_mut();
        ui.set_size(FISHING_UI_NEEDLE_SIZE, FISHING_UI_NEEDLE_SIZE);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_texture_wrap_mode(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        ui.set_material_instance(Some(tod_material.clone()));
        ui.set_enable_renderable_area(true);
        ui.set_color(get_color32(255, 200, 50, 160));
        direction_panel_ptr.add_widget(&fish_indicator);

        // Fish icon widget moving along circular perimeter of direction_panel
        let fish_icon_widget = UIManager::create_widget("fish_icon_widget", UIWidgetTypes::Default);
        let fish_icon_ptr = ptr_as_mut(fish_icon_widget.as_ref());
        let ui = fish_icon_ptr.get_ui_component_mut();
        ui.set_size(FISHING_UI_FISH_ICON_SIZE, FISHING_UI_FISH_ICON_SIZE);
        ui.set_round(FISHING_UI_FISH_ICON_SIZE * 0.5);
        ui.set_color(get_color32(255, 190, 40, 255));
        ui.set_border_color(get_color32(255, 255, 255, 255));
        ui.set_border(2.0);
        ui.set_font_size(14.0);
        ui.set_font_color(get_color32(20, 20, 20, 255));
        ui.set_text("Fish");
        //ui.set_renderable(false);
        direction_panel_ptr.add_widget(&fish_icon_widget);

        // Player needle widget (rotates with player angle input)
        let player_needle = UIManager::create_widget("player_needle", UIWidgetTypes::Default);
        let player_needle_ptr = ptr_as_mut(player_needle.as_ref());
        let ui = player_needle_ptr.get_ui_component_mut();
        ui.set_size(FISHING_UI_NEEDLE_SIZE, FISHING_UI_NEEDLE_SIZE);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_texture_wrap_mode(vk::SamplerAddressMode::CLAMP_TO_EDGE);
        ui.set_material_instance(Some(tod_material.clone()));
        ui.set_enable_renderable_area(true);
        ui.set_color(get_color32(0, 255, 200, 255));
        direction_panel_ptr.add_widget(&player_needle);

        // Vertical status bar widget (referencing StatusBarWidget)
        let vertical_gauge = StatusBarWidget::create_vertical_status_widget(
            main_layer_ptr,
            get_color32(0, 200, 255, 230),
            FISHING_UI_VERTICAL_GAUGE_WIDTH,
            FISHING_UI_VERTICAL_GAUGE_HEIGHT,
        );
        let ui = ptr_as_mut(vertical_gauge._status_layer).get_ui_component_mut();
        ui.set_pos(210.0, 25.0);

        // Status / Instruction text
        let status_text = UIManager::create_widget("fishing_status_text", UIWidgetTypes::Default);
        let status_text_ptr = ptr_as_mut(status_text.as_ref());
        let ui = status_text_ptr.get_ui_component_mut();
        ui.set_pos(260.0, 25.0);
        ui.set_size(85.0, 170.0);
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_text("A/D: Turn\nSpace: Pull");
        main_layer_ptr.add_widget(&status_text);

        // Cast gauge (horizontal bar for rod casting power)
        let cast_gauge = StatusBarWidget::create_status_widget(parent_widget, get_color32(0, 200, 255, 230));
        let ui = ptr_as_mut(cast_gauge._status_layer).get_ui_component_mut();
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.8));
        ui.set_size(300.0, 30.0);
        ui.set_visible(false);

        FishingGaugeWidget {
            _main_layer: main_layer_ptr,
            _direction_panel: direction_panel_ptr,
            _player_needle: player_needle_ptr,
            _fish_indicator: fish_indicator_ptr,
            _fish_icon_widget: fish_icon_ptr,
            _status_text: status_text_ptr,
            _vertical_gauge: vertical_gauge,
            _cast_gauge: cast_gauge,
        }
    }

    pub fn set_visible_fishing_gauge(&self, visible: bool) {
        ptr_as_mut(self._main_layer).get_ui_component_mut().set_visible(visible);
        if !visible {
            ptr_as_mut(self._cast_gauge._status_layer).get_ui_component_mut().set_visible(false);
        }
    }

    pub fn update_fishing_gauge_widget(&self, player: &Character<'a>, delta_time: f64) {
        if player.is_action(crate::game_module::actors::character::ActionAnimationState::FishingBegin) {
            ptr_as_mut(self._main_layer).get_ui_component_mut().set_visible(false);
            ptr_as_mut(self._cast_gauge._status_layer).get_ui_component_mut().set_visible(true);
            self._cast_gauge.update_status_widget(
                player.get_fishing_gauge(),
                1.0,
                1.0,
                delta_time,
                false,
            );
            return;
        }

        ptr_as_mut(self._cast_gauge._status_layer).get_ui_component_mut().set_visible(false);

        if !player.is_action(crate::game_module::actors::character::ActionAnimationState::FishingLoop) {
            ptr_as_mut(self._main_layer).get_ui_component_mut().set_visible(false);
            return;
        }

        ptr_as_mut(self._main_layer).get_ui_component_mut().set_visible(true);

        let fishing_state = &player._fishing_state;
        if !fishing_state._is_minigame_active {
            ptr_as_mut(self._status_text).get_ui_component_mut().set_text("Waiting...\nFish Bite");
            self._vertical_gauge.set_bar_color(get_color32(0, 200, 255, 230));
            self._vertical_gauge.update_vertical_status_widget(0.5, 1.0, delta_time, false);
            return;
        }

        // Update needle rotation (player angle: 9 o'clock -90° to 3 o'clock +90°)
        let player_needle_ui = ptr_as_mut(self._player_needle).get_ui_component_mut();
        player_needle_ui.set_rotation(fishing_state._player_angle);

        // Update fish indicator rotation
        let fish_indicator_ui = ptr_as_mut(self._fish_indicator).get_ui_component_mut();
        fish_indicator_ui.set_rotation(fishing_state._fish_angle);

        // Update fish icon position along circular perimeter of direction_panel
        let center = FISHING_UI_DIRECTION_PANEL_SIZE * 0.5;
        let fish_rad = fishing_state._fish_angle.to_radians();
        let half_icon = FISHING_UI_FISH_ICON_SIZE * 0.5;
        let fish_x = center + FISHING_UI_FISH_ORBIT_RADIUS * fish_rad.sin() - half_icon;
        let fish_y = center - FISHING_UI_FISH_ORBIT_RADIUS * fish_rad.cos() - half_icon;

        let fish_icon_ui = ptr_as_mut(self._fish_icon_widget).get_ui_component_mut();
        fish_icon_ui.set_pos(fish_x, fish_y);

        // Linear color & text feedback based on dot product alignment
        let dot = fishing_state._direction_dot;

        if dot >= FISHING_ALIGNMENT_MATCH_DOT {
            // High alignment -> Solid Blue
            let blue_intensity = (180.0 + 75.0 * ((dot - FISHING_ALIGNMENT_MATCH_DOT) / (1.0 - FISHING_ALIGNMENT_MATCH_DOT))) as u32;
            self._vertical_gauge.set_bar_color(get_color32(0, blue_intensity, 255, 230));
            if fishing_state._is_pulling {
                ptr_as_mut(self._status_text).get_ui_component_mut().set_text("MATCH!\nPULLING!");
            } else {
                ptr_as_mut(self._status_text).get_ui_component_mut().set_text("MATCH!\nPRESS SPACE");
            }
        } else {
            // Misaligned -> Linear flashing RED based on dot alignment
            let mismatch_factor = ((FISHING_ALIGNMENT_MATCH_DOT - dot) / (1.0 + FISHING_ALIGNMENT_MATCH_DOT)).clamp(0.0, 1.0);
            let time_sec = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f32();
            let blink = (time_sec * (FISHING_UI_BLINK_SPEED_BASE + mismatch_factor * FISHING_UI_BLINK_SPEED_SCALE)).sin() * 0.5 + 0.5;

            let red = (120.0 + (135.0 * mismatch_factor) * blink) as u32;
            let green_blue = ((1.0 - mismatch_factor) * 120.0 * (1.0 - blink)) as u32;

            self._vertical_gauge.set_bar_color(get_color32(red, green_blue, green_blue, 240));

            if fishing_state._is_pulling {
                ptr_as_mut(self._status_text).get_ui_component_mut().set_text("MISMATCH!\nDANGER!");
            } else {
                ptr_as_mut(self._status_text).get_ui_component_mut().set_text("ALIGN DIR!\nA / D");
            }
        }

        self._vertical_gauge.update_vertical_status_widget(
            fishing_state._fish_gauge,
            1.0,
            delta_time,
            false,
        );
    }
}
