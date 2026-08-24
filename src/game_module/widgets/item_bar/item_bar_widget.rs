use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::ITEM_NONE;
use crate::game_module::game_controller::KeyBindingType;
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_resources, get_game_ui_manager, get_item_manager_mut,
};
use crate::game_module::widgets::item_bar::{
    DEFAULT_INVENTORY_ROWS, EQUIPMENT_SLOT_START_INDEX, INVALID_ITEM_INDEX, ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM,
    ITEM_UI_SIZE, ITEM_WIDGET_UI_MARGIN, InventoryItemCreateInfo, InventoryItemCreateInfoList, InventorySlotData,
    ItemBarWidget, ItemSelectionWidget, ItemWidget, MAX_ITEM_COUNT, NUM_EQUIPMENT_SLOTS, SLOTS_PER_ROW,
};
use crate::game_module::widgets::key_binding_widget::{
    KEY_BINDING_FONT_SIZE, KEY_BINDING_ICON_MARGIN, KEY_BINDING_TEXT_MARGIN, KEY_BINDING_UI_SIZE, KeyBindingWidget,
    KeyBindingWidgetManager, KeyBindingWidgetMap,
};
use nalgebra::Vector2;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_BOTTOM_CENTER, PIVOT_CENTER_LEFT, PIVOT_CENTER_RIGHT, PIVOT_TOP_CENTER,
    UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

fn create_inventory_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_text: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
) -> KeyBindingWidget<'a> {
    let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
    let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_expandable_x(true);
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_round(10.0);
    ui_component.set_color(get_color32(0, 0, 0, 128));
    parent_widget.add_widget(&layout_widget);

    // icons
    let mut binding_icon_widgets: Vec<*const WidgetDefault> = Vec::new();
    let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
    ui_component.set_halign(HorizontalAlign::RIGHT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_margin_right(KEY_BINDING_ICON_MARGIN);
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    layout_widget_mut.add_widget(&binding_icon_widget);
    binding_icon_widgets.push(binding_icon_widget.as_ref());

    // text widget
    let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
    ui_component.set_expandable_x(true);
    ui_component.set_size_x(0.0);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_margin_left(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_margin_right(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_font_size(KEY_BINDING_FONT_SIZE);
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_color(get_color32(255, 255, 255, 0));
    ui_component.set_text(key_binding_text);
    layout_widget_mut.add_widget(&binding_name_widget);

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: binding_name_widget.as_ref(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons,
    }
}

fn create_quick_slot_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
) -> KeyBindingWidget<'a> {
    let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
    let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_pivot_preset(PIVOT_TOP_CENTER);
    ui_component.set_pos_hint(Some(0.5), Some(1.0));
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_round(10.0);
    ui_component.set_color(get_color32(0, 0, 0, 0));
    parent_widget.add_widget(&layout_widget);

    // icons
    let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
    let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    layout_widget_mut.add_widget(&binding_icon_widget);
    binding_icon_widgets.push(binding_icon_widget.as_ref());

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: std::ptr::null(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons,
    }
}

impl<'a> ItemBarWidget<'a> {
    pub fn create_item_bar_widget(
        key_binding_widget_manager: *const KeyBindingWidgetManager<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        window_size: &Vector2<i32>,
    ) -> ItemBarWidget<'a> {
        let layer = UIManager::create_widget("item_bar_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_round(5.0);
        ui_component.set_expandable(true);
        ui_component.set_pivot_preset(PIVOT_BOTTOM_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(1.0));
        ui_component.set_margin_bottom(ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM);
        parent_widget.add_widget(&layer);

        let selected_item_widget = UIManager::create_widget("selected_item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(selected_item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_visible(false);
        parent_widget.add_widget(&selected_item_widget);

        let inventory_rows = DEFAULT_INVENTORY_ROWS;
        let total_inv_slots = inventory_rows * SLOTS_PER_ROW;
        let total_storage_len = total_inv_slots.max(EQUIPMENT_SLOT_START_INDEX + NUM_EQUIPMENT_SLOTS);

        let mut inventory_slots = Vec::with_capacity(total_storage_len);
        for _ in 0..total_storage_len {
            inventory_slots.push(InventorySlotData::default());
        }

        let mut item_bar_widget = ItemBarWidget {
            _parent_widget: parent_widget,
            _layer: layer.as_ref(),
            _item_widgets: Vec::new(),
            _inventory_slots: inventory_slots,
            _inventory_rows: inventory_rows,
            _active_row_index: 0,
            _selected_item_widget: ItemSelectionWidget {
                _item_index: INVALID_ITEM_INDEX,
                _widget: selected_item_widget.as_ref(),
            },
            _selected_inventory_slot_index: INVALID_ITEM_INDEX,
            _item_count: 0,
            _max_item_count: total_storage_len,
            _inventory_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _quick_slot_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _window_size: *window_size,
        };

        for item_index in 0..MAX_ITEM_COUNT {
            let item_widget = ItemWidget::create_item_widget(ptr_as_mut(layer.as_ref()), item_index);
            item_bar_widget._item_widgets.push(item_widget);
        }

        item_bar_widget.register_item_bar_key_binding_widgets(key_binding_widget_manager);
        item_bar_widget.update_item_bar_widget_layout();
        item_bar_widget
    }

    pub fn get_inventory_key_binding_widget_map_mut(&mut self) -> &mut KeyBindingWidgetMap<'a> {
        ptr_as_mut(self._inventory_key_binding_widget_map.as_ref())
    }

    pub fn get_quick_slot_key_binding_widget_map_mut(&mut self) -> &mut KeyBindingWidgetMap<'a> {
        ptr_as_mut(self._quick_slot_key_binding_widget_map.as_ref())
    }

    pub fn register_item_bar_key_binding_widgets(
        &mut self,
        key_binding_widget_manager: *const KeyBindingWidgetManager<'a>,
    ) {
        let engine_resources = get_engine_resources();

        let key_binding_widget_manager = ptr_as_mut(key_binding_widget_manager);
        key_binding_widget_manager.register_key_binding_widget_map(&self._inventory_key_binding_widget_map);
        key_binding_widget_manager.register_key_binding_widget_map(&self._quick_slot_key_binding_widget_map);

        // inventory
        let inventory_key_binding_widget_map = ptr_as_mut(self._inventory_key_binding_widget_map.as_ref());

        let select_prev_widget = create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::SelectPrevItem,
            "select_prev_item_key_binding",
            "Prev",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_q").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_left").clone()],
        );
        let ui_component = ptr_as_mut(select_prev_widget._layout_widget).get_ui_component_mut();
        ui_component.set_pivot_preset(PIVOT_CENTER_RIGHT);
        ui_component.set_pos_hint(Some(0.5), Some(1.0));
        ui_component.set_pos_x(-ItemBarWidget::get_item_bar_width() * 0.5 - KEY_BINDING_TEXT_MARGIN);
        ui_component.set_pos_y(-ItemBarWidget::get_item_bar_center_y());
        inventory_key_binding_widget_map.register_key_binding_widget(select_prev_widget);

        let select_next_widget = create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::SelectNextItem,
            "select_next_item_key_binding",
            "Next",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_e").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_right").clone()],
        );
        let ui_component = ptr_as_mut(select_next_widget._layout_widget).get_ui_component_mut();
        ui_component.set_pivot_preset(PIVOT_CENTER_LEFT);
        ui_component.set_pos_hint(Some(0.5), Some(1.0));
        ui_component.set_pos_x(ItemBarWidget::get_item_bar_width() * 0.5 + KEY_BINDING_TEXT_MARGIN);
        ui_component.set_pos_y(-ItemBarWidget::get_item_bar_center_y());
        inventory_key_binding_widget_map.register_key_binding_widget(select_next_widget);

        // quick slot
        let quick_slot_key_binding_widget_map = ptr_as_mut(self._quick_slot_key_binding_widget_map.as_ref());
        let key_binding_types = [
            KeyBindingType::SelectItem01,
            KeyBindingType::SelectItem02,
            KeyBindingType::SelectItem03,
            KeyBindingType::SelectItem04,
            KeyBindingType::SelectItem05,
            KeyBindingType::SelectItem06,
            KeyBindingType::SelectItem07,
            KeyBindingType::SelectItem08,
            KeyBindingType::SelectItem09,
            KeyBindingType::SelectItem10,
        ];
        let key_codes = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        for i in 0..MAX_ITEM_COUNT {
            quick_slot_key_binding_widget_map.register_key_binding_widget(create_quick_slot_key_binding_widget(
                ptr_as_mut(self.get_item_widget(i)._widget),
                key_binding_types[i],
                &format!("select_item{:02}_key_binding", i + 1),
                vec![
                    engine_resources
                        .get_material_instance_data(&format!("ui/controller/keycode_{}", key_codes[i]))
                        .clone(),
                ],
                vec![],
            ));
        }
    }

    pub fn get_item_bar_width() -> f32 {
        (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 2.0) * MAX_ITEM_COUNT as f32
    }

    pub fn get_item_bar_pos_top() -> f32 {
        ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM + ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 3.0
    }

    pub fn get_item_bar_center_y() -> f32 {
        ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM + (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN) * 0.5
    }

    pub fn get_selected_item_pos_left(item_index: usize) -> f32 {
        (item_index as f32 - MAX_ITEM_COUNT as f32 / 2.0) * (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 2.0)
    }

    pub fn get_selected_item_widget(&self) -> &ItemSelectionWidget<'a> {
        &self._selected_item_widget
    }

    pub fn update_item_bar_widget_layout(&mut self) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_size_y(ITEM_UI_SIZE);
        self.update_quick_slot_widgets();
    }

    pub fn update_quick_slot_widgets(&mut self) {
        let start_slot = self._active_row_index * SLOTS_PER_ROW;
        for i in 0..SLOTS_PER_ROW {
            let slot_idx = start_slot + i;
            let slot_data = &self._inventory_slots[slot_idx];
            self._item_widgets[i].set_item_data(
                &slot_data._item_name,
                &slot_data._item_data_name,
                slot_data._item_data_type,
                slot_data._material_instance.clone(),
                slot_data._item_count,
            );
        }

        let selected_slot = self._selected_inventory_slot_index;
        if selected_slot != INVALID_ITEM_INDEX
            && selected_slot >= start_slot
            && selected_slot < start_slot + SLOTS_PER_ROW
        {
            let quick_idx = selected_slot - start_slot;
            self._selected_item_widget.update_selected_item_widget(quick_idx, Some(&self._item_widgets[quick_idx]));
        } else {
            self._selected_item_widget.update_selected_item_widget(INVALID_ITEM_INDEX, None);
        }
    }

    pub fn get_inventory_rows(&self) -> usize {
        self._inventory_rows
    }

    pub fn get_total_inventory_slots(&self) -> usize {
        self._inventory_rows * SLOTS_PER_ROW
    }

    pub fn get_total_slots_storage_len(&self) -> usize {
        self.get_total_inventory_slots().max(EQUIPMENT_SLOT_START_INDEX + NUM_EQUIPMENT_SLOTS)
    }

    pub fn is_valid_slot_index(&self, slot_index: usize) -> bool {
        slot_index < self.get_total_inventory_slots()
            || (slot_index >= EQUIPMENT_SLOT_START_INDEX && slot_index < EQUIPMENT_SLOT_START_INDEX + NUM_EQUIPMENT_SLOTS)
    }

    pub fn get_total_slots_with_equipment(&self) -> usize {
        self.get_total_inventory_slots() + NUM_EQUIPMENT_SLOTS
    }

    pub fn set_inventory_rows(&mut self, rows: usize) {
        let rows = rows.max(1);
        self._inventory_rows = rows;
        let target_len = self.get_total_slots_storage_len();
        if self._inventory_slots.len() < target_len {
            self._inventory_slots.resize_with(target_len, InventorySlotData::default);
        }
        self._max_item_count = target_len;
        self.update_quick_slot_widgets();
    }

    pub fn switch_active_row(&mut self) -> usize {
        self._active_row_index = (self._active_row_index + 1) % self._inventory_rows;
        self.update_quick_slot_widgets();
        self._active_row_index
    }

    pub fn set_active_row_index(&mut self, row_index: usize) {
        self._active_row_index = row_index % self._inventory_rows;
        self.update_quick_slot_widgets();
    }

    pub fn get_active_row_index(&self) -> usize {
        self._active_row_index
    }

    pub fn get_inventory_slot_data(&self, slot_index: usize) -> &InventorySlotData<'a> {
        &self._inventory_slots[slot_index]
    }

    pub fn swap_inventory_slots(&mut self, src_slot_index: usize, dst_slot_index: usize) -> bool {
        if self.is_valid_slot_index(src_slot_index)
            && self.is_valid_slot_index(dst_slot_index)
            && src_slot_index != dst_slot_index
        {
            self._inventory_slots.swap(src_slot_index, dst_slot_index);

            if self._selected_inventory_slot_index == src_slot_index {
                self._selected_inventory_slot_index = dst_slot_index;
            } else if self._selected_inventory_slot_index == dst_slot_index {
                self._selected_inventory_slot_index = src_slot_index;
            }

            self.update_quick_slot_widgets();
            return true;
        }
        false
    }

    pub fn get_selected_inventory_slot_index(&self) -> usize {
        self._selected_inventory_slot_index
    }

    pub fn get_item_widget(&self, index: usize) -> &ItemWidget<'a> {
        &self._item_widgets[index]
    }

    pub fn get_item_widget_mut(&mut self, index: usize) -> &mut ItemWidget<'a> {
        &mut self._item_widgets[index]
    }

    pub fn find_item_widget(&self, item_data_name: &str) -> Option<&ItemWidget<'a>> {
        self._item_widgets.iter().find(|item_widget| item_widget._item_data_name == item_data_name)
    }

    pub fn find_item_widget_mut(&mut self, item_data_name: &str) -> Option<&mut ItemWidget<'a>> {
        self._item_widgets.iter_mut().find(|item_widget| item_widget._item_data_name.as_str() == item_data_name)
    }

    pub fn get_item_count(&self, item_data_name: &str) -> usize {
        let mut total = 0;
        for slot in self._inventory_slots.iter() {
            if slot._item_data_name == item_data_name {
                total += slot._item_count;
            }
        }
        total
    }

    pub fn get_selected_item_data_name(&self) -> &str {
        if self._selected_inventory_slot_index != INVALID_ITEM_INDEX
            && self.is_valid_slot_index(self._selected_inventory_slot_index)
        {
            return self._inventory_slots[self._selected_inventory_slot_index]._item_data_name.as_str();
        }
        ITEM_NONE
    }

    pub fn get_selected_item_name(&self) -> &str {
        if self._selected_inventory_slot_index != INVALID_ITEM_INDEX
            && self.is_valid_slot_index(self._selected_inventory_slot_index)
        {
            return self._inventory_slots[self._selected_inventory_slot_index]._item_name.as_str();
        }
        ITEM_NONE
    }

    pub fn get_selected_item_data_type(&self) -> ItemDataType {
        if self._selected_inventory_slot_index != INVALID_ITEM_INDEX
            && self.is_valid_slot_index(self._selected_inventory_slot_index)
        {
            return self._inventory_slots[self._selected_inventory_slot_index]._item_data_type;
        }
        ItemDataType::None
    }

    pub fn get_selected_item_index(&self) -> usize {
        self._selected_inventory_slot_index
    }

    pub fn get_inventory_item_create_infos(&self) -> InventoryItemCreateInfoList {
        let mut inventory_item_create_info_list = InventoryItemCreateInfoList::new();

        for row in 0..self._inventory_rows {
            let mut create_infos = Vec::new();
            for col in 0..SLOTS_PER_ROW {
                let slot_idx = row * SLOTS_PER_ROW + col;
                if slot_idx < self._inventory_slots.len() {
                    let slot = &self._inventory_slots[slot_idx];
                    if 0 < slot._item_count && slot._item_data_name != ITEM_NONE {
                        create_infos.push(InventoryItemCreateInfo {
                            _item_data_name: slot._item_data_name.clone(),
                            _item_name: slot._item_name.clone(),
                            _item_data_type: slot._item_data_type,
                            _item_index: slot_idx,
                            _row: row,
                            _column: col,
                            _item_count: slot._item_count,
                        });
                    }
                }
            }
            if !create_infos.is_empty() {
                inventory_item_create_info_list.insert(row, create_infos);
            }
        }

        let mut equip_create_infos = Vec::new();
        let equip_row = EQUIPMENT_SLOT_START_INDEX;
        for col in 0..NUM_EQUIPMENT_SLOTS {
            let slot_idx = EQUIPMENT_SLOT_START_INDEX + col;
            if slot_idx < self._inventory_slots.len() {
                let slot = &self._inventory_slots[slot_idx];
                if 0 < slot._item_count && slot._item_data_name != ITEM_NONE {
                    equip_create_infos.push(InventoryItemCreateInfo {
                        _item_data_name: slot._item_data_name.clone(),
                        _item_name: slot._item_name.clone(),
                        _item_data_type: slot._item_data_type,
                        _item_index: slot_idx,
                        _row: equip_row,
                        _column: col,
                        _item_count: slot._item_count,
                    });
                }
            }
        }
        if !equip_create_infos.is_empty() {
            inventory_item_create_info_list.insert(equip_row, equip_create_infos);
        }
        inventory_item_create_info_list
    }

    pub fn get_selected_quick_slot_row_col(&self) -> Option<(usize, usize)> {
        let selected_slot = self._selected_inventory_slot_index;
        if selected_slot != INVALID_ITEM_INDEX && selected_slot < self.get_total_inventory_slots() {
            Some((selected_slot / SLOTS_PER_ROW, selected_slot % SLOTS_PER_ROW))
        } else {
            None
        }
    }

    pub fn add_item_at_slot(&mut self, slot_index: usize, item_data_name: &str, item_count: usize) -> bool {
        if self.is_valid_slot_index(slot_index) && item_data_name != ITEM_NONE {
            let item_data = get_game_resources().get_item_data(item_data_name).borrow();
            let material = get_engine_resources().get_material_instance_data(item_data._ui_material_instance.as_str());
            if slot_index >= self._inventory_slots.len() {
                self._inventory_slots.resize_with(slot_index + 1, InventorySlotData::default);
            }
            let slot = &mut self._inventory_slots[slot_index];
            slot._item_name = item_data._name.clone();
            slot._item_data_name = item_data_name.to_string();
            slot._item_data_type = item_data._item_type;
            slot._material_instance = Some(material.clone());
            slot._item_count = item_count;
            self._item_count += 1;
            self.update_quick_slot_widgets();
            return true;
        }
        false
    }

    pub fn clear_item_bar_widget(&mut self) {
        for slot in self._inventory_slots.iter_mut() {
            slot._item_data_name = String::from(ITEM_NONE);
            slot._item_name = String::from(ITEM_NONE);
            slot._item_data_type = ItemDataType::None;
            slot._material_instance = None;
            slot._item_count = 0;
        }
        self._item_count = 0;
        self.select_item(INVALID_ITEM_INDEX);
    }

    pub fn add_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        if item_data_name != ITEM_NONE && item_count > 0 {
            let was_empty_item = self.get_selected_item_data_name() == ITEM_NONE;
            let mut target_slot_index = INVALID_ITEM_INDEX;

            // 1. If item already exists in inventory, stack count to existing slot
            for (idx, slot) in self._inventory_slots.iter_mut().enumerate() {
                if slot._item_data_name == item_data_name && slot._item_count > 0 {
                    slot._item_count += item_count;
                    target_slot_index = idx;
                    break;
                }
            }

            // 2. If new item, search empty slot in current quick slot row first, then anywhere in main inventory
            let total_inv_slots = self.get_total_inventory_slots();
            if target_slot_index == INVALID_ITEM_INDEX {
                let current_row_start = self._active_row_index * SLOTS_PER_ROW;
                let current_row_end = (current_row_start + SLOTS_PER_ROW).min(total_inv_slots);

                for idx in current_row_start..current_row_end {
                    let slot = &self._inventory_slots[idx];
                    if slot._item_count == 0 || slot._item_data_name == ITEM_NONE || slot._item_data_name.is_empty() {
                        target_slot_index = idx;
                        break;
                    }
                }

                if target_slot_index == INVALID_ITEM_INDEX {
                    for idx in 0..total_inv_slots {
                        let slot = &self._inventory_slots[idx];
                        if slot._item_count == 0 || slot._item_data_name == ITEM_NONE || slot._item_data_name.is_empty()
                        {
                            target_slot_index = idx;
                            break;
                        }
                    }
                }

                if target_slot_index != INVALID_ITEM_INDEX {
                    let item_data = get_game_resources().get_item_data(item_data_name).borrow();
                    let material =
                        get_engine_resources().get_material_instance_data(item_data._ui_material_instance.as_str());
                    let slot = &mut self._inventory_slots[target_slot_index];
                    slot._item_name = item_data._name.clone();
                    slot._item_data_name = item_data_name.to_string();
                    slot._item_data_type = item_data._item_type;
                    slot._material_instance = Some(material.clone());
                    slot._item_count = item_count;
                    self._item_count += 1;
                }
            }

            if target_slot_index != INVALID_ITEM_INDEX {
                if was_empty_item {
                    self.select_item(target_slot_index);
                }
                self.update_quick_slot_widgets();
                return true;
            }
        }
        false
    }

    pub fn remove_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        if item_data_name == ITEM_NONE {
            return false;
        }

        for (idx, slot) in self._inventory_slots.iter_mut().enumerate() {
            if slot._item_data_name == item_data_name && slot._item_count > 0 {
                if item_count <= slot._item_count {
                    slot._item_count -= item_count;
                } else {
                    slot._item_count = 0;
                }

                if slot._item_count == 0 {
                    slot._item_data_name = String::from(ITEM_NONE);
                    slot._item_name = String::from(ITEM_NONE);
                    slot._item_data_type = ItemDataType::None;
                    slot._material_instance = None;
                    self._item_count = self._item_count.saturating_sub(1);

                    if self._selected_inventory_slot_index == idx {
                        let player = ptr_as_mut(get_character_manager().get_player().as_ptr());
                        get_item_manager_mut().detach_item(player);
                        self._selected_inventory_slot_index = INVALID_ITEM_INDEX;
                    }
                }
                self.update_quick_slot_widgets();
                return true;
            }
        }
        false
    }

    pub fn select_item(&mut self, slot_index: usize) {
        if let Some(player) = get_character_manager().get_maybe_player() {
            let player = ptr_as_mut(player.as_ptr());
            let total_inv_slots = self.get_total_inventory_slots();
            if slot_index < total_inv_slots {
                self._active_row_index = slot_index / SLOTS_PER_ROW;
            }
            if self.is_valid_slot_index(slot_index)
                && self._inventory_slots[slot_index]._item_data_name != ITEM_NONE
                && self._inventory_slots[slot_index]._item_count > 0
            {
                self._selected_inventory_slot_index = slot_index;
                get_item_manager_mut().attach_item(player, self.get_selected_item_data_name());
            } else {
                self._selected_inventory_slot_index = INVALID_ITEM_INDEX;
                get_item_manager_mut().detach_item(player);
            }
            self.update_quick_slot_widgets();
        }
    }

    pub fn select_quick_slot(&mut self, quick_index: usize) {
        if quick_index < SLOTS_PER_ROW {
            let slot_index = self._active_row_index * SLOTS_PER_ROW + quick_index;
            self.select_item(slot_index);
        }
    }

    pub fn select_next_item(&mut self) {
        let mut target_slot = INVALID_ITEM_INDEX;
        let total_inv_slots = self.get_total_inventory_slots();

        let curr_slot = if self._selected_inventory_slot_index < total_inv_slots {
            self._selected_inventory_slot_index
        } else {
            self._active_row_index * SLOTS_PER_ROW + SLOTS_PER_ROW - 1
        };

        let start_slot = (curr_slot + 1) % total_inv_slots;

        for step in 0..total_inv_slots {
            let slot_idx = (start_slot + step) % total_inv_slots;
            if self._inventory_slots[slot_idx]._item_data_name != ITEM_NONE
                && self._inventory_slots[slot_idx]._item_count > 0
            {
                target_slot = slot_idx;
                break;
            }
        }

        if target_slot == INVALID_ITEM_INDEX {
            target_slot = start_slot;
        }

        self.select_item(target_slot);
    }

    pub fn select_previous_item(&mut self) {
        let mut target_slot = INVALID_ITEM_INDEX;
        let total_inv_slots = self.get_total_inventory_slots();

        let curr_slot = if self._selected_inventory_slot_index < total_inv_slots {
            self._selected_inventory_slot_index
        } else {
            self._active_row_index * SLOTS_PER_ROW
        };

        let start_slot = (curr_slot + total_inv_slots - 1) % total_inv_slots;

        for step in 0..total_inv_slots {
            let slot_idx = (start_slot + total_inv_slots - step) % total_inv_slots;
            if self._inventory_slots[slot_idx]._item_data_name != ITEM_NONE
                && self._inventory_slots[slot_idx]._item_count > 0
            {
                target_slot = slot_idx;
                break;
            }
        }

        if target_slot == INVALID_ITEM_INDEX {
            target_slot = start_slot;
        }

        self.select_item(target_slot);
    }

    pub fn update_selected_item_helper_widget(&mut self, force_update: bool) {
        let selected_item_index = get_game_ui_manager().get_selected_inventory_item_index();
        if self._selected_inventory_slot_index != selected_item_index || force_update {
            self._selected_inventory_slot_index = selected_item_index;
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        self._window_size = *window_size;
        self.update_selected_item_helper_widget(true);
        self.select_item(self.get_selected_item_index());
    }

    pub fn update_item_bar_widget(&mut self) {
        self.update_selected_item_helper_widget(true);
    }
}
