use crate::game_module::game_constants::ITEM_NONE;
use crate::game_module::game_service_locator::{get_character_manager, get_game_scene_manager, get_game_ui_manager};
use crate::game_module::widgets::item_bar::{EQUIPMENT_SLOT_START_INDEX, NUM_EQUIPMENT_SLOTS};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

const STAT_ROW_LABEL_WIDTH: f32 = 100.0;
const STAT_ROW_VALUE_WIDTH: f32 = 110.0;
const STAT_ROW_HEIGHT: f32 = 28.0;

pub struct PlayerStatRowWidget<'a> {
    pub _widget: Rc<WidgetDefault<'a>>,
    pub _label_widget: Rc<WidgetDefault<'a>>,
    pub _value_widget: Rc<WidgetDefault<'a>>,
}

impl<'a> PlayerStatRowWidget<'a> {
    pub fn create(parent: &mut WidgetDefault<'a>, label_text: &str, row_id: &str) -> PlayerStatRowWidget<'a> {
        let row_layout = UIManager::create_widget(&format!("stat_row_{}", row_id), UIWidgetTypes::Default);
        let row_layout_mut = ptr_as_mut(row_layout.as_ref());
        let ui_comp = row_layout_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_y(STAT_ROW_HEIGHT);
        ui_comp.set_margin_bottom(2.0);
        ui_comp.set_color(get_color32(0, 0, 0, 80));
        ui_comp.set_round(3.0);
        parent.add_widget(&row_layout);

        let label_w = UIManager::create_widget(&format!("stat_label_{}", row_id), UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(label_w.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::LEFT);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(STAT_ROW_LABEL_WIDTH, STAT_ROW_HEIGHT);
        ui_comp.set_margin_left(6.0);
        ui_comp.set_text(label_text);
        ui_comp.set_font_size(18.0);
        ui_comp.set_font_color(get_color32(200, 210, 225, 255));
        ui_comp.set_color(get_color32(0, 0, 0, 0));
        row_layout_mut.add_widget(&label_w);

        let value_w = UIManager::create_widget(&format!("stat_val_{}", row_id), UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(value_w.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::RIGHT);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(STAT_ROW_VALUE_WIDTH, STAT_ROW_HEIGHT);
        ui_comp.set_margin_right(6.0);
        ui_comp.set_text("-");
        ui_comp.set_font_size(18.0);
        ui_comp.set_font_color(get_color32(255, 255, 255, 255));
        ui_comp.set_color(get_color32(0, 0, 0, 0));
        row_layout_mut.add_widget(&value_w);

        PlayerStatRowWidget {
            _widget: row_layout,
            _label_widget: label_w,
            _value_widget: value_w,
        }
    }

    pub fn set_value(&mut self, text: &str) {
        let ui_comp = ptr_as_mut(self._value_widget.as_ref()).get_ui_component_mut();
        ui_comp.set_text(text);
    }
}

pub struct PlayerStatWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _title_widget: Rc<WidgetDefault<'a>>,
    pub _hp_row: PlayerStatRowWidget<'a>,
    pub _stamina_row: PlayerStatRowWidget<'a>,
    pub _hunger_row: PlayerStatRowWidget<'a>,
    pub _tired_row: PlayerStatRowWidget<'a>,
    pub _attack_row: PlayerStatRowWidget<'a>,
    pub _defence_row: PlayerStatRowWidget<'a>,
    pub _temp_row: PlayerStatRowWidget<'a>,
}

impl<'a> PlayerStatWidget<'a> {
    pub fn create_player_stat_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<PlayerStatWidget<'a>> {
        let layer = UIManager::create_widget("player_stat_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_size(240.0, 0.0);
        ui_component.set_expandable(true);
        ui_component.set_padding(8.0);
        ui_component.set_margin_left(10.0);
        ui_component.set_color(get_color32(40, 44, 52, 220));
        ui_component.set_border_color(get_color32(80, 90, 110, 255));
        ui_component.set_round(5.0);

        let title_widget = UIManager::create_widget("stat_title", UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_y(30.0);
        ui_comp.set_margin_bottom(6.0);
        ui_comp.set_text("Player Status");
        ui_comp.set_round(5.0);
        ui_comp.set_color(get_color32(0, 0, 0, 128));
        ui_comp.set_font_size(22.0);
        ui_comp.set_font_color(get_color32(240, 210, 130, 255));
        layer_mut.add_widget(&title_widget);

        let hp_row = PlayerStatRowWidget::create(layer_mut, "HP", "hp");
        let stamina_row = PlayerStatRowWidget::create(layer_mut, "Stamina", "stamina");
        let hunger_row = PlayerStatRowWidget::create(layer_mut, "Hunger", "hunger");
        let tired_row = PlayerStatRowWidget::create(layer_mut, "Tired", "tired");
        let attack_row = PlayerStatRowWidget::create(layer_mut, "Attack", "attack");
        let defence_row = PlayerStatRowWidget::create(layer_mut, "Defence", "defence");
        let temp_row = PlayerStatRowWidget::create(layer_mut, "Temperature", "temp");

        parent_widget.add_widget(&layer);

        let mut stat_widget = Box::new(PlayerStatWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _title_widget: title_widget,
            _hp_row: hp_row,
            _stamina_row: stamina_row,
            _hunger_row: hunger_row,
            _tired_row: tired_row,
            _attack_row: attack_row,
            _defence_row: defence_row,
            _temp_row: temp_row,
        });

        stat_widget.refresh_player_stat_widget();
        stat_widget
    }

    pub fn refresh_player_stat_widget(&mut self) {
        let character_manager = get_character_manager();
        if character_manager.is_valid_player() {
            let player = character_manager.get_player().borrow();
            let stats = &player._character_stats;
            let char_data = player._character_data.borrow();
            let stat_data = &char_data._stat_data;

            self._hp_row.set_value(&format!("{}/{}", stats._hp, stats._max_hp));
            self._stamina_row.set_value(&format!("{:.0}/{:.0}", stats._stamina, stats._max_stamina));
            self._hunger_row.set_value(&format!("{:.0}%", stats._hunger));
            self._tired_row.set_value(&format!("{:.0}", stats._tired));
            self._attack_row.set_value(&format!("{}", stat_data._attack_damage));

            let game_ui_mgr = get_game_ui_manager();
            let item_bar = game_ui_mgr.get_item_bar_widget();
            let mut defence_val: i32 = 0;
            for eq_idx in 0..NUM_EQUIPMENT_SLOTS {
                let slot_idx = EQUIPMENT_SLOT_START_INDEX + eq_idx;
                let slot_data = item_bar.get_inventory_slot_data(slot_idx);
                if slot_data._item_count > 0 && slot_data._item_data_name != ITEM_NONE {
                    defence_val += 15;
                }
            }
            self._defence_row.set_value(&format!("{}", defence_val));

            let temp = get_game_scene_manager().temperature();
            self._temp_row.set_value(&format!("{:.1}°C", temp));
        }
    }
}
