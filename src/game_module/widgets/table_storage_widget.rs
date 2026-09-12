use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, ITEM_HAND, ITEM_NONE};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{get_game_resources, get_game_ui_manager, get_game_ui_manager_mut};
use crate::game_module::widgets::game_menu_widget::item_info_widget::ItemInfoWidget;
use crate::game_module::widgets::item_bar::{
    INVALID_ITEM_INDEX, ITEM_UI_SIZE, ITEM_WIDGET_UI_MARGIN, InventoryItemCreateInfo, InventorySlotData, SLOTS_PER_ROW,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_core};
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub const TABLE_STORAGE_ROWS: usize = 2;
pub const TABLE_STORAGE_SLOTS_PER_ROW: usize = SLOTS_PER_ROW;
pub const TOTAL_TABLE_STORAGE_SLOTS: usize = TABLE_STORAGE_ROWS * TABLE_STORAGE_SLOTS_PER_ROW;

pub struct TableStorageSlotWidget<'a> {
    pub _table_storage_widget: *const TableStorageWidget<'a>,
    pub _is_table_slot: bool,
    pub _slot_index: usize,
    pub _widget: Rc<WidgetDefault<'a>>,
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_count: usize,
}

impl<'a> TableStorageSlotWidget<'a> {
    pub fn create(
        table_storage_widget: &TableStorageWidget<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        is_table_slot: bool,
        slot_index: usize,
    ) -> Box<TableStorageSlotWidget<'a>> {
        let prefix = if is_table_slot { "table" } else { "player" };
        let slot_widget = UIManager::create_widget(&format!("{}_tbl_slot_{}", prefix, slot_index), UIWidgetTypes::Default);
        let slot_widget_mut = ptr_as_mut(slot_widget.as_ref());
        let ui_component = slot_widget_mut.get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_margin(ITEM_WIDGET_UI_MARGIN);
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_border_color(get_color32(100, 100, 120, 255));
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_touchable(true);

        parent_widget.add_widget(&slot_widget);

        let slot = Box::new(TableStorageSlotWidget {
            _table_storage_widget: table_storage_widget,
            _is_table_slot: is_table_slot,
            _slot_index: slot_index,
            _widget: slot_widget,
            _item_data_name: String::new(),
            _item_name: String::new(),
            _item_data_type: ItemDataType::None,
            _item_count: 0,
        });

        let ui_component = ptr_as_mut(slot._widget.as_ref()).get_ui_component_mut();
        ui_component.set_callback_touch_down(Some(Box::new(TableStorageWidget::callback_slot_click)));
        ui_component.set_callback_touch_over(Some(Box::new(TableStorageWidget::callback_slot_touch_over)));
        ui_component.set_user_data(slot.as_ref() as *const TableStorageSlotWidget<'a> as *const c_void);

        slot
    }

    pub fn set_data(
        &mut self,
        item_name: &str,
        item_data_name: &str,
        item_data_type: ItemDataType,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        item_count: usize,
        is_selected_item: bool,
    ) {
        self._item_name = item_name.to_string();
        self._item_data_name = item_data_name.to_string();
        self._item_data_type = item_data_type;
        self._item_count = item_count;

        let ui_component = ptr_as_mut(self._widget.as_ref()).get_ui_component_mut();
        if material_instance.is_some() {
            ui_component.set_color(get_color32(255, 255, 255, 255));
        } else if self._is_table_slot {
            ui_component.set_color(get_color32(50, 40, 30, 200));
        } else {
            ui_component.set_color(get_color32(35, 45, 55, 200));
        }

        if is_selected_item {
            ui_component.set_border_color(get_color32(255, 255, 0, 255));
        } else if self._is_table_slot {
            if item_count > 0 && item_data_name != ITEM_NONE {
                ui_component.set_border_color(get_color32(230, 180, 80, 255));
            } else {
                ui_component.set_border_color(get_color32(120, 100, 80, 255));
            }
        } else {
            if item_count > 0 && item_data_name != ITEM_NONE {
                ui_component.set_border_color(get_color32(80, 200, 240, 255));
            } else {
                ui_component.set_border_color(get_color32(80, 90, 110, 255));
            }
        }

        if item_count > 1 && item_data_name != ITEM_NONE {
            ui_component.set_font_size(24.0);
            ui_component.set_font_color(get_color32(255, 255, 255, 255));
            ui_component.set_text(&format!("{}", item_count));
        } else {
            ui_component.set_text("");
        }

        ui_component.set_material_instance(material_instance);
    }
}

pub struct TableStorageWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _main_bg: Rc<WidgetDefault<'a>>,
    pub _drag_widget: Rc<WidgetDefault<'a>>,
    pub _item_info_widget: Box<ItemInfoWidget<'a>>,
    pub _table_slot_widgets: Vec<Box<TableStorageSlotWidget<'a>>>,
    pub _player_slot_widgets: Vec<Box<TableStorageSlotWidget<'a>>>,
    pub _table_inventory_slots: Vec<InventorySlotData<'a>>,
    pub _focused_is_table_slot: bool,
    pub _focused_slot_index: usize,
    pub _drag_source_is_table: bool,
    pub _drag_source_slot_index: usize,
    pub _is_opened: bool,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> TableStorageWidget<'a> {
    pub fn create_table_storage_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<TableStorageWidget<'a>> {
        let layer = UIManager::create_widget("table_storage_widget_root", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_expandable(false);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);

        let main_bg = UIManager::create_widget("table_storage_bg", UIWidgetTypes::Default);
        let main_bg_mut = ptr_as_mut(main_bg.as_ref());
        let ui_component = main_bg_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(15.0);
        ui_component.set_color(get_color32(35, 30, 25, 230));
        ui_component.set_border_color(get_color32(180, 150, 90, 255));
        ui_component.set_border(3.0);
        ui_component.set_round(8.0);
        layer_mut.add_widget(&main_bg);

        let drag_widget = UIManager::create_widget("table_drag_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(drag_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_draggable(false);
        ui_component.set_touchable(false);
        ui_component.set_visible(false);
        layer_mut.add_widget(&drag_widget);

        let item_info_widget = ItemInfoWidget::create_item_info_widget(layer_mut);

        let mut table_slots = Vec::with_capacity(TOTAL_TABLE_STORAGE_SLOTS);
        for _ in 0..TOTAL_TABLE_STORAGE_SLOTS {
            table_slots.push(InventorySlotData::default());
        }

        let mut widget = Box::new(TableStorageWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _main_bg: main_bg,
            _drag_widget: drag_widget,
            _item_info_widget: item_info_widget,
            _table_slot_widgets: Vec::new(),
            _player_slot_widgets: Vec::new(),
            _table_inventory_slots: table_slots,
            _focused_is_table_slot: true,
            _focused_slot_index: 0,
            _drag_source_is_table: true,
            _drag_source_slot_index: INVALID_ITEM_INDEX,
            _is_opened: false,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        });

        widget.rebuild_grid();
        widget
    }

    pub fn rebuild_grid(&mut self) {
        let item_bar = get_game_ui_manager().get_item_bar_widget();
        let player_inv_rows = item_bar.get_inventory_rows();
        let total_player_widgets = player_inv_rows * SLOTS_PER_ROW;

        if self._table_slot_widgets.len() == TOTAL_TABLE_STORAGE_SLOTS
            && self._player_slot_widgets.len() == total_player_widgets
        {
            return;
        }

        let bg_mut = ptr_as_mut(self._main_bg.as_ref());
        bg_mut.clear_widgets();
        self._table_slot_widgets.clear();
        self._player_slot_widgets.clear();

        // Title Bar
        let title_widget = UIManager::create_widget("table_storage_title", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(32.0);
        ui_component.set_margin_bottom(10.0);
        ui_component.set_text("Table Storage");
        ui_component.set_font_size(28.0);
        ui_component.set_font_color(get_color32(255, 215, 120, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        bg_mut.add_widget(&title_widget);

        // --- TOP SECTION: Table Storage Inventory ---
        let table_section = UIManager::create_widget("table_section", UIWidgetTypes::Default);
        let table_section_mut = ptr_as_mut(table_section.as_ref());
        let ui_component = table_section_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_color(get_color32(45, 38, 30, 200));
        ui_component.set_border_color(get_color32(140, 110, 70, 255));
        ui_component.set_border(1.0);
        ui_component.set_round(6.0);
        ui_component.set_padding(8.0);
        ui_component.set_margin_bottom(10.0);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(0.0);
        ui_component.set_expandable(true);
        bg_mut.add_widget(&table_section);

        let table_label = UIManager::create_widget("table_sec_label", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(table_label.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(24.0);
        ui_component.set_margin_bottom(4.0);
        ui_component.set_text("[ Table Inventory ]");
        ui_component.set_font_size(22.0);
        ui_component.set_font_color(get_color32(230, 190, 100, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        table_section_mut.add_widget(&table_label);

        for row in 0..TABLE_STORAGE_ROWS {
            let row_layout = TableStorageWidget::create_row_layout(table_section_mut, &format!("tbl_row_{}", row));
            for col in 0..TABLE_STORAGE_SLOTS_PER_ROW {
                let slot_idx = row * TABLE_STORAGE_SLOTS_PER_ROW + col;
                let slot_widget = TableStorageSlotWidget::create(self, ptr_as_mut(row_layout.as_ref()), true, slot_idx);
                self._table_slot_widgets.push(slot_widget);
            }
        }

        // --- BOTTOM SECTION: Player Inventory ---
        let player_section = UIManager::create_widget("player_section", UIWidgetTypes::Default);
        let player_section_mut = ptr_as_mut(player_section.as_ref());
        let ui_component = player_section_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_color(get_color32(30, 40, 50, 200));
        ui_component.set_border_color(get_color32(70, 100, 130, 255));
        ui_component.set_border(1.0);
        ui_component.set_round(6.0);
        ui_component.set_padding(8.0);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(0.0);
        ui_component.set_expandable(true);
        bg_mut.add_widget(&player_section);

        let player_label = UIManager::create_widget("player_sec_label", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(player_label.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(24.0);
        ui_component.set_margin_bottom(4.0);
        ui_component.set_text("[ Player Inventory ]");
        ui_component.set_font_size(22.0);
        ui_component.set_font_color(get_color32(120, 210, 240, 255));
        ui_component.set_color(get_color32(0, 0, 0, 0));
        player_section_mut.add_widget(&player_label);

        let item_bar = get_game_ui_manager().get_item_bar_widget();
        let player_inv_rows = item_bar.get_inventory_rows();
        for row in 0..player_inv_rows {
            let row_layout = TableStorageWidget::create_row_layout(player_section_mut, &format!("plr_row_{}", row));
            for col in 0..SLOTS_PER_ROW {
                let slot_idx = row * SLOTS_PER_ROW + col;
                let slot_widget = TableStorageSlotWidget::create(self, ptr_as_mut(row_layout.as_ref()), false, slot_idx);
                self._player_slot_widgets.push(slot_widget);
            }
        }
    }

    fn create_row_layout(parent_widget: &mut WidgetDefault<'a>, name: &str) -> Rc<WidgetDefault<'a>> {
        let row_layout = UIManager::create_widget(name, UIWidgetTypes::Default);
        let row_layout_mut = ptr_as_mut(row_layout.as_ref());
        let ui_component = row_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(0, 0, 0, 0));
        ui_component.set_margin(2.0);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(0.0);
        ui_component.set_expandable(true);
        parent_widget.add_widget(&row_layout);
        row_layout
    }

    pub fn callback_slot_click(
        ui_component: &UIComponentInstance<'a>,
        touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ui_component.get_user_data();
        if user_data.is_null() {
            return false;
        }

        let slot_ptr = user_data as *const TableStorageSlotWidget<'a>;
        if slot_ptr.is_null() {
            return false;
        }

        let slot_item = unsafe { &*slot_ptr };
        if slot_item._table_storage_widget.is_null() {
            return false;
        }

        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        let table_widget = ptr_as_mut(slot_item._table_storage_widget);
        let clicked_is_table = slot_item._is_table_slot;
        let clicked_slot = slot_item._slot_index;

        if table_widget._drag_source_slot_index == INVALID_ITEM_INDEX {
            // Pick up / Start drag
            if slot_item._item_count > 0
                && slot_item._item_data_name != ITEM_NONE
                && slot_item._item_data_name != ITEM_HAND
                && slot_item._item_data_type != ItemDataType::Hand
            {
                table_widget._drag_source_is_table = clicked_is_table;
                table_widget._drag_source_slot_index = clicked_slot;
                table_widget._focused_is_table_slot = clicked_is_table;
                table_widget._focused_slot_index = clicked_slot;

                let material = if clicked_is_table {
                    if clicked_slot < table_widget._table_inventory_slots.len() {
                        table_widget._table_inventory_slots[clicked_slot]._material_instance.clone()
                    } else {
                        None
                    }
                } else {
                    let item_bar = get_game_ui_manager().get_item_bar_widget();
                    if clicked_slot < item_bar.get_total_inventory_slots() {
                        item_bar.get_inventory_slot_data(clicked_slot)._material_instance.clone()
                    } else {
                        None
                    }
                };

                let drag_ui = ptr_as_mut(table_widget._drag_widget.as_ref()).get_ui_component_mut();
                drag_ui.set_material_instance(material);
                if slot_item._item_count > 1 {
                    drag_ui.set_text(&format!("{}", slot_item._item_count));
                } else {
                    drag_ui.set_text("");
                }
                drag_ui.set_draggable(true);
                drag_ui.set_visible(true);

                let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
                let parent_area = ptr_as_ref(table_widget._layer.as_ref()).get_ui_component().get_ui_area();
                drag_ui.set_pos(
                    (touched_pos.x - parent_area.x) / dpi_scale - ITEM_UI_SIZE * 0.5,
                    (touched_pos.y - parent_area.y) / dpi_scale - ITEM_UI_SIZE * 0.5,
                );
            } else {
                table_widget._focused_is_table_slot = clicked_is_table;
                table_widget._focused_slot_index = clicked_slot;
            }
        } else {
            // Drop / Transfer / Swap
            let src_is_table = table_widget._drag_source_is_table;
            let src_slot = table_widget._drag_source_slot_index;

            // Retrieve source and destination item data to check for Hand item
            let (src_item_name, src_item_type) = if src_is_table {
                if src_slot < table_widget._table_inventory_slots.len() {
                    (
                        table_widget._table_inventory_slots[src_slot]._item_data_name.as_str(),
                        table_widget._table_inventory_slots[src_slot]._item_data_type,
                    )
                } else {
                    ("", ItemDataType::None)
                }
            } else {
                let item_bar = get_game_ui_manager().get_item_bar_widget();
                if src_slot < item_bar.get_total_inventory_slots() {
                    let d = item_bar.get_inventory_slot_data(src_slot);
                    (d._item_data_name.as_str(), d._item_data_type)
                } else {
                    ("", ItemDataType::None)
                }
            };

            let (dst_item_name, dst_item_type) = if clicked_is_table {
                if clicked_slot < table_widget._table_inventory_slots.len() {
                    (
                        table_widget._table_inventory_slots[clicked_slot]._item_data_name.as_str(),
                        table_widget._table_inventory_slots[clicked_slot]._item_data_type,
                    )
                } else {
                    ("", ItemDataType::None)
                }
            } else {
                let item_bar = get_game_ui_manager().get_item_bar_widget();
                if clicked_slot < item_bar.get_total_inventory_slots() {
                    let d = item_bar.get_inventory_slot_data(clicked_slot);
                    (d._item_data_name.as_str(), d._item_data_type)
                } else {
                    ("", ItemDataType::None)
                }
            };

            let is_src_hand = src_item_name == ITEM_HAND || src_item_type == ItemDataType::Hand;
            let is_dst_hand = dst_item_name == ITEM_HAND || dst_item_type == ItemDataType::Hand;

            if !is_src_hand && !is_dst_hand {
                if src_is_table == clicked_is_table {
                    // Same container swap
                    if src_is_table {
                        if src_slot != clicked_slot
                            && src_slot < table_widget._table_inventory_slots.len()
                            && clicked_slot < table_widget._table_inventory_slots.len()
                        {
                            table_widget._table_inventory_slots.swap(src_slot, clicked_slot);
                        }
                    } else {
                        if src_slot != clicked_slot {
                            get_game_ui_manager_mut().swap_inventory_slots(src_slot, clicked_slot);
                        }
                    }
                } else {
                    // Cross container transfer/swap! (Table <-> Player)
                    let (table_idx, player_idx) = if src_is_table {
                        (src_slot, clicked_slot)
                    } else {
                        (clicked_slot, src_slot)
                    };

                    if table_idx < table_widget._table_inventory_slots.len() {
                        let game_ui_manager = get_game_ui_manager_mut();
                        let item_bar = game_ui_manager.get_item_bar_widget_mut();
                        if player_idx < item_bar.get_total_inventory_slots() {
                            let table_slot_data = table_widget._table_inventory_slots[table_idx].clone();
                            let player_slot_data = item_bar.get_inventory_slot_data(player_idx).clone();

                            table_widget._table_inventory_slots[table_idx] = player_slot_data;
                            item_bar.set_inventory_slot_data(player_idx, &table_slot_data);
                        }
                    }
                }
            }

            table_widget._drag_source_slot_index = INVALID_ITEM_INDEX;
            table_widget._focused_is_table_slot = clicked_is_table;
            table_widget._focused_slot_index = clicked_slot;

            let drag_ui = ptr_as_mut(table_widget._drag_widget.as_ref()).get_ui_component_mut();
            drag_ui.set_draggable(false);
            drag_ui.set_visible(false);
        }

        table_widget.refresh_table_storage_widget();
        true
    }

    pub fn callback_slot_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ui_component.get_user_data();
        if user_data.is_null() {
            return false;
        }

        let slot_ptr = user_data as *const TableStorageSlotWidget<'a>;
        if !slot_ptr.is_null() {
            let slot_item = unsafe { &*slot_ptr };
            if !slot_item._table_storage_widget.is_null() {
                let table_widget = ptr_as_mut(slot_item._table_storage_widget);
                if slot_item._item_count > 0 && slot_item._item_data_name != ITEM_NONE {
                    table_widget._item_info_widget.show_item_info(
                        &slot_item._item_data_name,
                        &slot_item._item_name,
                        slot_item._item_count,
                        slot_item._widget.as_ref(),
                    );
                } else {
                    table_widget._item_info_widget.hide_item_info();
                }
            }
        }
        true
    }

    pub fn open_table_storage(&mut self) {
        if !self._is_opened {
            self._is_opened = true;
            self._focused_is_table_slot = true;
            self._focused_slot_index = 0;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self.refresh_table_storage_widget();
        }
    }

    pub fn close_table_storage(&mut self) {
        if self._is_opened {
            self._is_opened = false;
            self._nav_repeat_controller.reset();
            self._item_info_widget.hide_item_info();
            if self._drag_source_slot_index != INVALID_ITEM_INDEX {
                self._drag_source_slot_index = INVALID_ITEM_INDEX;
                let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
                drag_ui.set_draggable(false);
                drag_ui.set_visible(false);
            }
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
        }
    }

    pub fn is_opened_table_storage(&self) -> bool {
        self._is_opened
    }

    pub fn refresh_table_storage_widget(&mut self) {
        self.rebuild_grid();

        // Refresh Table Slots
        for slot_widget in self._table_slot_widgets.iter_mut() {
            let idx = slot_widget._slot_index;
            let is_selected = self._focused_is_table_slot && idx == self._focused_slot_index;

            if self._drag_source_is_table && idx == self._drag_source_slot_index {
                slot_widget.set_data("", ITEM_NONE, ItemDataType::None, None, 0, is_selected);
            } else if idx < self._table_inventory_slots.len() {
                let slot_data = &self._table_inventory_slots[idx];
                slot_widget.set_data(
                    &slot_data._item_name,
                    &slot_data._item_data_name,
                    slot_data._item_data_type,
                    slot_data._material_instance.clone(),
                    slot_data._item_count,
                    is_selected,
                );
            }
        }

        // Refresh Player Slots
        let item_bar = get_game_ui_manager().get_item_bar_widget();
        for slot_widget in self._player_slot_widgets.iter_mut() {
            let idx = slot_widget._slot_index;
            let is_selected = !self._focused_is_table_slot && idx == self._focused_slot_index;

            if !self._drag_source_is_table && idx == self._drag_source_slot_index {
                slot_widget.set_data("", ITEM_NONE, ItemDataType::None, None, 0, is_selected);
            } else if idx < item_bar.get_total_inventory_slots() {
                let slot_data = item_bar.get_inventory_slot_data(idx);
                slot_widget.set_data(
                    &slot_data._item_name,
                    &slot_data._item_data_name,
                    slot_data._item_data_type,
                    slot_data._material_instance.clone(),
                    slot_data._item_count,
                    is_selected,
                );
            }
        }
    }

    pub fn update_table_storage_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character<'a>>,
    ) {
        if !self._is_opened {
            return;
        }

        // ESC or Close key
        if keyboard_input_data.get_key_pressed(KeyCode::Escape) {
            self.close_table_storage();
            return;
        }

        // Update drag item position on mouse move
        if self._drag_source_slot_index != INVALID_ITEM_INDEX {
            let engine_core = get_engine_core();
            let mouse_pos = &engine_core._mouse_move_data._mouse_pos;
            let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
            let parent_area = ptr_as_ref(self._layer.as_ref()).get_ui_component().get_ui_area();
            let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
            drag_ui.set_pos(
                (mouse_pos.x as f32 - parent_area.x) / dpi_scale - ITEM_UI_SIZE * 0.5,
                (mouse_pos.y as f32 - parent_area.y) / dpi_scale - ITEM_UI_SIZE * 0.5,
            );
        }

        // Keyboard / Controller Nav
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move_slot, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if should_move_slot {
            let (dir_x, dir_y) = dir_opt.unwrap();
            let item_bar = get_game_ui_manager().get_item_bar_widget();
            let player_inv_rows = item_bar.get_inventory_rows();

            let mut is_table = self._focused_is_table_slot;
            let mut idx = self._focused_slot_index;

            if is_table {
                let cur_row = idx / TABLE_STORAGE_SLOTS_PER_ROW;
                let cur_col = idx % TABLE_STORAGE_SLOTS_PER_ROW;
                if dir_y < 0 {
                    if cur_row > 0 {
                        idx -= TABLE_STORAGE_SLOTS_PER_ROW;
                    }
                } else if dir_y > 0 {
                    if cur_row + 1 < TABLE_STORAGE_ROWS {
                        idx += TABLE_STORAGE_SLOTS_PER_ROW;
                    } else {
                        // Move from Table to Player
                        is_table = false;
                        idx = cur_col.min(SLOTS_PER_ROW - 1);
                    }
                } else if dir_x < 0 {
                    if cur_col > 0 {
                        idx -= 1;
                    }
                } else if dir_x > 0 {
                    if cur_col + 1 < TABLE_STORAGE_SLOTS_PER_ROW {
                        idx += 1;
                    }
                }
            } else {
                let cur_row = idx / SLOTS_PER_ROW;
                let cur_col = idx % SLOTS_PER_ROW;
                if dir_y < 0 {
                    if cur_row > 0 {
                        idx -= SLOTS_PER_ROW;
                    } else {
                        // Move from Player to Table
                        is_table = true;
                        idx = (TABLE_STORAGE_ROWS - 1) * TABLE_STORAGE_SLOTS_PER_ROW + cur_col;
                    }
                } else if dir_y > 0 {
                    if cur_row + 1 < player_inv_rows {
                        idx += SLOTS_PER_ROW;
                    }
                } else if dir_x < 0 {
                    if cur_col > 0 {
                        idx -= 1;
                    }
                } else if dir_x > 0 {
                    if cur_col + 1 < SLOTS_PER_ROW {
                        idx += 1;
                    }
                }
            }

            if is_table != self._focused_is_table_slot || idx != self._focused_slot_index {
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                self._focused_is_table_slot = is_table;
                self._focused_slot_index = idx;
                self.refresh_table_storage_widget();
            }
        }
    }

    pub fn get_table_storage_items(&self) -> &Vec<InventorySlotData<'a>> {
        &self._table_inventory_slots
    }

    pub fn set_table_storage_items(&mut self, items: &[InventorySlotData<'a>]) {
        let len = items.len().min(TOTAL_TABLE_STORAGE_SLOTS);
        for i in 0..len {
            self._table_inventory_slots[i] = items[i].clone();
        }
        if self._is_opened {
            self.refresh_table_storage_widget();
        }
    }

    pub fn get_table_storage_item_create_infos(&self) -> Vec<InventoryItemCreateInfo> {
        let mut list = Vec::new();
        for (idx, slot) in self._table_inventory_slots.iter().enumerate() {
            if slot._item_count > 0 && slot._item_data_name != ITEM_NONE {
                list.push(InventoryItemCreateInfo {
                    _item_data_name: slot._item_data_name.clone(),
                    _item_name: slot._item_name.clone(),
                    _item_data_type: slot._item_data_type,
                    _item_index: idx,
                    _row: idx / TABLE_STORAGE_SLOTS_PER_ROW,
                    _column: idx % TABLE_STORAGE_SLOTS_PER_ROW,
                    _item_count: slot._item_count,
                });
            }
        }
        list
    }

    pub fn load_table_storage_item_create_infos(&mut self, create_infos: &[InventoryItemCreateInfo]) {
        for slot in self._table_inventory_slots.iter_mut() {
            *slot = InventorySlotData::default();
        }
        let game_resources = get_game_resources();
        let engine_resources = rust_engine_3d::core::engine_service_locator::get_engine_resources();
        for info in create_infos {
            if info._item_index < TOTAL_TABLE_STORAGE_SLOTS {
                let item_data = game_resources.get_item_data(&info._item_data_name).borrow();
                let material = if !item_data._ui_material_instance.is_empty() {
                    Some(engine_resources.get_material_instance_data(item_data._ui_material_instance.as_str()).clone())
                } else {
                    None
                };

                self._table_inventory_slots[info._item_index] = InventorySlotData {
                    _item_data_name: info._item_data_name.clone(),
                    _item_name: item_data._name.clone(),
                    _item_data_type: item_data._item_type,
                    _material_instance: material,
                    _item_count: info._item_count,
                };
            }
        }
        if self._is_opened {
            self.refresh_table_storage_widget();
        }
    }
}
