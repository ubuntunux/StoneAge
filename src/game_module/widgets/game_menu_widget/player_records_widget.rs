use crate::game_module::game_service_locator::get_game_ui_manager;
use nalgebra::Vector2;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

const STATS_CARD_HEIGHT: f32 = 220.0;
const HEADER_FONT_SIZE: f32 = 20.0;
const BODY_FONT_SIZE: f32 = 16.0;
const LIST_FONT_SIZE: f32 = 15.0;

/// Dynamically formats any entity/monster/item key string into Title Case display text.
pub fn format_entity_name(key: &str) -> String {
    let clean = key
        .trim_start_matches("characters/")
        .trim_start_matches("monsters/")
        .trim_start_matches("items/")
        .replace('_', " ")
        .replace('-', " ");

    let mut result = String::new();
    for word in clean.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase());
            result.push_str(chars.as_str());
        }
    }

    if result.eq_ignore_ascii_case("Trex") {
        "T-Rex".to_string()
    } else {
        result
    }
}

pub struct PlayerRecordsWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _content_container: Rc<WidgetDefault<'a>>,
    pub _is_opened_player_records_widget: bool,
}

impl<'a> PlayerRecordsWidget<'a> {
    pub fn create_player_records_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<PlayerRecordsWidget<'a>> {
        let layer = UIManager::create_widget("player_records_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(35, 40, 45, 230));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);

        // Title Header
        let title_widget = UIManager::create_widget("records_title", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(400.0, 32.0);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_text("Player Records");
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 220, 100, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&title_widget);

        // Content Container
        let content_container = UIManager::create_widget("records_content_container", UIWidgetTypes::Default);
        let content_container_mut = ptr_as_mut(content_container.as_ref());
        let ui_component = content_container_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(false);
        ui_component.set_scroll_y(true);
        ui_component.set_enable_renderable_area(true);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_padding(5.0);
        ui_component.set_margin(5.0);
        ui_component.set_color(get_color32(0, 0, 0, 100));
        layer_mut.add_widget(&content_container);

        Box::new(PlayerRecordsWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _content_container: content_container,
            _is_opened_player_records_widget: false,
        })
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_player_records_widget(&self) -> bool {
        self._is_opened_player_records_widget
    }

    pub fn open_player_records_widget(&mut self) {
        if !self._is_opened_player_records_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._is_opened_player_records_widget = true;
            self.refresh_player_records();
        }
    }

    pub fn close_player_records_widget(&mut self) {
        if self._is_opened_player_records_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened_player_records_widget = false;
        }
    }

    pub fn update_player_records_widget(
        &mut self,
        _joystick_input_data: &JoystickInputData,
        _keyboard_input_data: &KeyboardInputData,
    ) {
    }

    pub fn refresh_player_records(&mut self) {
        let container_mut = ptr_as_mut(self._content_container.as_ref());
        container_mut.clear_widgets();

        let game_ui_mgr = get_game_ui_manager();
        let records = game_ui_mgr.get_player_records();

        // 1. General Records Section
        let stats_card = UIManager::create_widget("stats_card", UIWidgetTypes::Default);
        let card_mut = ptr_as_mut(stats_card.as_ref());
        let ui_comp = card_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_size_hint_x(Some(0.95));
        ui_comp.set_size_y(STATS_CARD_HEIGHT);
        ui_comp.set_margin(4.0);
        ui_comp.set_padding(6.0);
        ui_comp.set_color(get_color32(25, 45, 65, 230));
        ui_comp.set_border_color(get_color32(70, 130, 200, 255));
        ui_comp.set_round(5.0);

        let stats_header = UIManager::create_widget("stats_header", UIWidgetTypes::Default);
        let ui = ptr_as_mut(stats_header.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(24.0);
        ui.set_text("Activity Statistics");
        ui.set_font_size(HEADER_FONT_SIZE);
        ui.set_font_color(get_color32(100, 200, 255, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        card_mut.add_widget(&stats_header);

        let stat_items = [
            format!("Total Items: {}", records._item_count),
            format!("Craft Count: {}", records._craft_count),
            format!("Death Count: {}", records._death_count),
            format!("Tamed: {}", records._taming_count),
            format!("Friends: {}", records._friend_count),
            format!("Visited Maps: {}", records.get_visited_map_count()),
            format!("Energy Balls: {}", records._energy_balls),
            format!("Spirit Balls: {}", records._spirit_balls),
        ];

        for (idx, stat_text) in stat_items.iter().enumerate() {
            let row = UIManager::create_widget(&format!("stats_row_{}", idx), UIWidgetTypes::Default);
            let ui = ptr_as_mut(row.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(22.0);
            ui.set_text(stat_text);
            ui.set_font_size(BODY_FONT_SIZE);
            ui.set_font_color(get_color32(230, 230, 230, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            card_mut.add_widget(&row);
        }

        container_mut.add_widget(&stats_card);

        // 2. Item Acquisitions by ItemType Section (Dynamic auto-recording for any ItemDataType)
        let item_type_card = UIManager::create_widget("item_type_card", UIWidgetTypes::Default);
        let card_mut = ptr_as_mut(item_type_card.as_ref());
        let ui_comp = card_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_size_hint_x(Some(0.95));

        let mut item_type_entries: Vec<(&String, &u32)> =
            records._item_type_counts.iter().filter(|(k, v)| **v > 0 && k.as_str() != "None").collect();
        item_type_entries.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

        let line_count = if item_type_entries.is_empty() { 1 } else { item_type_entries.len() };
        let list_height = (line_count as f32) * 22.0;
        let card_height = 36.0 + list_height;
        ui_comp.set_size_y(card_height);
        ui_comp.set_margin(4.0);
        ui_comp.set_padding(6.0);
        ui_comp.set_color(get_color32(30, 55, 45, 230));
        ui_comp.set_border_color(get_color32(60, 180, 120, 255));
        ui_comp.set_round(5.0);

        let item_type_header = UIManager::create_widget("item_type_header", UIWidgetTypes::Default);
        let ui = ptr_as_mut(item_type_header.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(24.0);
        ui.set_text("Item Acquisitions by ItemType");
        ui.set_font_size(HEADER_FONT_SIZE);
        ui.set_font_color(get_color32(120, 240, 160, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        card_mut.add_widget(&item_type_header);

        let mut item_type_text = String::new();
        if item_type_entries.is_empty() {
            item_type_text.push_str("No Item Acquisitions Recorded Yet");
        } else {
            let mut count_displayed = 0;
            for (type_name, count) in item_type_entries {
                if count_displayed > 0 {
                    item_type_text.push('\n');
                }
                item_type_text.push_str(&format!("{}: {}", type_name, count));
                count_displayed += 1;
            }
        }

        let item_type_list_widget = UIManager::create_widget("item_type_list_widget", UIWidgetTypes::Default);
        let ui = ptr_as_mut(item_type_list_widget.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(list_height);
        ui.set_text(&item_type_text);
        ui.set_font_size(LIST_FONT_SIZE);
        ui.set_font_color(get_color32(220, 240, 220, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        card_mut.add_widget(&item_type_list_widget);

        container_mut.add_widget(&item_type_card);

        // 3. Monster Kills Section (Dynamic name resolution for any present or future monster)
        let kill_card = UIManager::create_widget("kill_card", UIWidgetTypes::Default);
        let card_mut = ptr_as_mut(kill_card.as_ref());
        let ui_comp = card_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_size_hint_x(Some(0.95));

        let mut kill_entries: Vec<(String, u32)> = Vec::new();
        for (monster_key, &kills) in records._monster_kill_counts.iter() {
            if kills > 0 {
                let display_name = format_entity_name(monster_key);
                kill_entries.push((display_name, kills));
            }
        }
        kill_entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let line_count = if kill_entries.is_empty() { 1 } else { kill_entries.len() };
        let list_height = (line_count as f32) * 22.0;
        let card_height = 36.0 + list_height;
        ui_comp.set_size_y(card_height);
        ui_comp.set_margin(4.0);
        ui_comp.set_padding(6.0);
        ui_comp.set_color(get_color32(50, 25, 30, 230));
        ui_comp.set_border_color(get_color32(200, 70, 80, 255));
        ui_comp.set_round(5.0);

        let kill_header = UIManager::create_widget("kill_header", UIWidgetTypes::Default);
        let ui = ptr_as_mut(kill_header.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(24.0);
        ui.set_text(&format!(
            "Monster Kills by Species (Total: {})",
            records.get_total_monster_kills()
        ));
        ui.set_font_size(HEADER_FONT_SIZE);
        ui.set_font_color(get_color32(255, 120, 120, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        card_mut.add_widget(&kill_header);

        let mut kill_items_text = String::new();
        if kill_entries.is_empty() {
            kill_items_text.push_str("No Monster Kills Recorded Yet");
        } else {
            let mut count_displayed = 0;
            for (display_name, kills) in kill_entries {
                if count_displayed > 0 {
                    kill_items_text.push('\n');
                }
                kill_items_text.push_str(&format!("{}: {} Kills", display_name, kills));
                count_displayed += 1;
            }
        }

        let kill_list_widget = UIManager::create_widget("kill_list_widget", UIWidgetTypes::Default);
        let ui = ptr_as_mut(kill_list_widget.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(list_height);
        ui.set_text(&kill_items_text);
        ui.set_font_size(LIST_FONT_SIZE);
        ui.set_font_color(get_color32(240, 220, 220, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        card_mut.add_widget(&kill_list_widget);

        container_mut.add_widget(&kill_card);
    }
}

