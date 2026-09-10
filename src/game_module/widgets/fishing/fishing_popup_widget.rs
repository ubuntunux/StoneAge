use crate::game_module::game_service_locator::get_game_resources;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

pub const FISHING_POPUP_WIDTH: f32 = 340.0;
pub const FISHING_POPUP_HEIGHT: f32 = 200.0;
pub const FISHING_POPUP_DISPLAY_DURATION: f32 = 3.5;
pub const FISHING_POPUP_FADE_DURATION: f32 = 0.6;

pub struct FishingPopupWidget<'a> {
    pub _container_widget: *const WidgetDefault<'a>,
    pub _title_widget: *const WidgetDefault<'a>,
    pub _icon_widget: *const WidgetDefault<'a>,
    pub _name_widget: *const WidgetDefault<'a>,
    pub _perfect_badge_widget: *const WidgetDefault<'a>,
    pub _remaining_time: f32,
    pub _is_perfect: bool,
}

impl<'a> FishingPopupWidget<'a> {
    pub fn create_fishing_popup_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<FishingPopupWidget<'a>> {
        let container = UIManager::create_widget("fishing_popup_container", UIWidgetTypes::Default);
        let container_ptr = ptr_as_mut(container.as_ref());
        {
            let ui = container_ptr.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::VERTICAL);
            ui.set_pivot_preset(PIVOT_CENTER);
            ui.set_pos_hint(Some(0.5), Some(0.45));
            ui.set_size(FISHING_POPUP_WIDTH, FISHING_POPUP_HEIGHT);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::TOP);
            ui.set_padding(12.0);
            ui.set_round(16.0);
            ui.set_color(get_color32(15, 25, 40, 235));
            ui.set_border_color(get_color32(255, 215, 0, 255));
            ui.set_border(3.0);
            ui.set_visible(false);
        }
        parent_widget.add_widget(&container);

        // Title Header (e.g. PERFECT CATCH! or FISH CAUGHT!)
        let title_widget = UIManager::create_widget("fishing_popup_title", UIWidgetTypes::Default);
        let title_ptr = ptr_as_mut(title_widget.as_ref());
        {
            let ui = title_ptr.get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(32.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(22.0);
            ui.set_font_color(get_color32(255, 215, 0, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("★ PERFECT CATCH! ★");
        }
        container_ptr.add_widget(&title_widget);

        // Item Icon Widget
        let icon_widget = UIManager::create_widget("fishing_popup_icon", UIWidgetTypes::Default);
        let icon_ptr = ptr_as_mut(icon_widget.as_ref());
        {
            let ui = icon_ptr.get_ui_component_mut();
            ui.set_size(54.0, 54.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_top(8.0);
            ui.set_margin_bottom(8.0);
            ui.set_color(get_color32(255, 255, 255, 255));
        }
        container_ptr.add_widget(&icon_widget);

        // Item Name Widget
        let name_widget = UIManager::create_widget("fishing_popup_name", UIWidgetTypes::Default);
        let name_ptr = ptr_as_mut(name_widget.as_ref());
        {
            let ui = name_ptr.get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(26.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(20.0);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("Fish Item");
        }
        container_ptr.add_widget(&name_widget);

        // Perfect Badge / Subtitle Widget
        let perfect_badge_widget = UIManager::create_widget("fishing_popup_perfect_badge", UIWidgetTypes::Default);
        let perfect_badge_ptr = ptr_as_mut(perfect_badge_widget.as_ref());
        {
            let ui = perfect_badge_ptr.get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(22.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(15.0);
            ui.set_font_color(get_color32(255, 220, 100, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ui.set_text("Perfect Alignment Bonus!");
        }
        container_ptr.add_widget(&perfect_badge_widget);

        Box::new(FishingPopupWidget {
            _container_widget: container_ptr,
            _title_widget: title_ptr,
            _icon_widget: icon_ptr,
            _name_widget: name_ptr,
            _perfect_badge_widget: perfect_badge_ptr,
            _remaining_time: 0.0,
            _is_perfect: false,
        })
    }

    pub fn show_popup(&mut self, item_data_name: &str, is_perfect: bool) {
        let (item_name, material_instance) = {
            let game_resources = get_game_resources();
            if game_resources.has_item_data(item_data_name) {
                let item_data = game_resources.get_item_data(item_data_name).borrow();
                let name = item_data._name.clone();
                let material = get_engine_resources()
                    .get_material_instance_data(item_data._ui_material_instance.as_str())
                    .clone();
                (name, material)
            } else {
                (
                    item_data_name.to_string(),
                    get_engine_resources()
                        .get_material_instance_data("materials/ui/material_ui_none")
                        .clone(),
                )
            }
        };

        self._remaining_time = FISHING_POPUP_DISPLAY_DURATION;
        self._is_perfect = is_perfect;

        let container_ui = ptr_as_mut(self._container_widget).get_ui_component_mut();
        container_ui.set_visible(true);

        if is_perfect {
            container_ui.set_border_color(get_color32(255, 215, 0, 255));
            let title_ui = ptr_as_mut(self._title_widget).get_ui_component_mut();
            title_ui.set_text("★ PERFECT CATCH! ★");
            title_ui.set_font_color(get_color32(255, 215, 0, 255));

            let badge_ui = ptr_as_mut(self._perfect_badge_widget).get_ui_component_mut();
            badge_ui.set_text("Perfect Alignment Bonus!");
            badge_ui.set_font_color(get_color32(255, 220, 100, 255));
            badge_ui.set_visible(true);
        } else {
            container_ui.set_border_color(get_color32(0, 200, 255, 255));
            let title_ui = ptr_as_mut(self._title_widget).get_ui_component_mut();
            title_ui.set_text("FISH CAUGHT!");
            title_ui.set_font_color(get_color32(100, 220, 255, 255));

            let badge_ui = ptr_as_mut(self._perfect_badge_widget).get_ui_component_mut();
            badge_ui.set_text("");
            badge_ui.set_visible(false);
        }

        let icon_ui = ptr_as_mut(self._icon_widget).get_ui_component_mut();
        icon_ui.set_material_instance(Some(material_instance));

        let name_ui = ptr_as_mut(self._name_widget).get_ui_component_mut();
        name_ui.set_text(&item_name);
    }

    pub fn update(&mut self, delta_time: f32) {
        if self._remaining_time <= 0.0 {
            return;
        }

        self._remaining_time -= delta_time;

        if self._remaining_time <= 0.0 {
            self._remaining_time = 0.0;
            let container_ui = ptr_as_mut(self._container_widget).get_ui_component_mut();
            container_ui.set_visible(false);
            return;
        }

        let alpha = if self._remaining_time < FISHING_POPUP_FADE_DURATION {
            (self._remaining_time / FISHING_POPUP_FADE_DURATION).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let bg_alpha = (alpha * 235.0) as u8;
        let border_alpha = (alpha * 255.0) as u8;
        let text_alpha = (alpha * 255.0) as u8;

        let container_ui = ptr_as_mut(self._container_widget).get_ui_component_mut();
        container_ui.set_color(get_color32(15, 25, 40, bg_alpha.into()));

        if self._is_perfect {
            container_ui.set_border_color(get_color32(255, 215, 0, border_alpha.into()));
            let title_ui = ptr_as_mut(self._title_widget).get_ui_component_mut();
            title_ui.set_font_color(get_color32(255, 215, 0, text_alpha.into()));
            let badge_ui = ptr_as_mut(self._perfect_badge_widget).get_ui_component_mut();
            badge_ui.set_font_color(get_color32(255, 220, 100, text_alpha.into()));
        } else {
            container_ui.set_border_color(get_color32(0, 200, 255, border_alpha.into()));
            let title_ui = ptr_as_mut(self._title_widget).get_ui_component_mut();
            title_ui.set_font_color(get_color32(100, 220, 255, text_alpha.into()));
        }

        let name_ui = ptr_as_mut(self._name_widget).get_ui_component_mut();
        name_ui.set_font_color(get_color32(255, 255, 255, text_alpha.into()));

        let icon_ui = ptr_as_mut(self._icon_widget).get_ui_component_mut();
        icon_ui.set_color(get_color32(255, 255, 255, text_alpha.into()));
    }
}
