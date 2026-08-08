use crate::game_module::game_service_locator::get_game_resources;
use crate::game_module::widgets::item_acquire_notification::{ItemAcquireEntry, ItemAcquireNotificationWidget, ItemAcquireSlot, MAX_NOTIFICATION_SLOTS, NOTIFICATION_DISPLAY_DURATION, NOTIFICATION_FADE_DURATION, NOTIFICATION_FONT_SIZE, NOTIFICATION_ICON_SIZE, NOTIFICATION_ICON_TEXT_MARGIN, NOTIFICATION_LAYOUT_WIDTH, NOTIFICATION_MARGIN_LEFT, NOTIFICATION_MARGIN_TOP, NOTIFICATION_ROW_MARGIN};
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_TOP_LEFT, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

impl<'a> ItemAcquireNotificationWidget<'a> {
    pub fn create(parent_widget: &mut WidgetDefault<'a>) -> Box<ItemAcquireNotificationWidget<'a>> {
        let container = UIManager::create_widget("item_acquire_notification_container", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(container.as_ref()).get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::VERTICAL);
            ui.set_pivot_preset(PIVOT_TOP_LEFT);
            ui.set_pos(NOTIFICATION_MARGIN_LEFT, NOTIFICATION_MARGIN_TOP);
            ui.set_expandable(true);
            ui.set_renderable(false);
            ui.set_color(get_color32(0, 0, 0, 0));
        }
        parent_widget.add_widget(&container);

        let mut slots: Vec<ItemAcquireSlot<'a>> = Vec::with_capacity(MAX_NOTIFICATION_SLOTS);
        let container_mut = ptr_as_mut(container.as_ref());

        for i in 0..MAX_NOTIFICATION_SLOTS {
            let slot = Self::create_slot(container_mut, i);
            slots.push(slot);
        }

        let slot_height = NOTIFICATION_ICON_SIZE + NOTIFICATION_ROW_MARGIN;

        let mut widget = Box::new(ItemAcquireNotificationWidget {
            _container_widget: container.as_ref(),
            _slots: slots,
            _entries: Vec::new(),
            _slot_height: slot_height,
        });

        widget.hide_all_slots();

        widget
    }

    fn create_slot(parent: &mut WidgetDefault<'a>, _index: usize) -> ItemAcquireSlot<'a> {
        let row = UIManager::create_widget("notification_row", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(row.as_ref()).get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_x(NOTIFICATION_LAYOUT_WIDTH);
            ui.set_size_y(NOTIFICATION_ICON_SIZE + NOTIFICATION_ICON_TEXT_MARGIN);
            ui.set_expandable_x(true);
            ui.set_margin_bottom(NOTIFICATION_ROW_MARGIN);
            ui.set_round(6.0);
            ui.set_color(get_color32(0, 0, 0, 160));
            ui.set_visible(false);
        }
        parent.add_widget(&row);
        let row_mut = ptr_as_mut(row.as_ref());

        let icon = UIManager::create_widget("notification_icon", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
            ui.set_size(NOTIFICATION_ICON_SIZE, NOTIFICATION_ICON_SIZE);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin(NOTIFICATION_ICON_TEXT_MARGIN * 0.5);
            ui.set_color(get_color32(255, 255, 255, 255));
        }
        row_mut.add_widget(&icon);

        let name = UIManager::create_widget("notification_name", UIWidgetTypes::Default);
        {
            let ui = ptr_as_mut(name.as_ref()).get_ui_component_mut();
            ui.set_expandable_x(true);
            ui.set_size(0.0, NOTIFICATION_ICON_SIZE);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin(NOTIFICATION_ICON_TEXT_MARGIN * 0.5);
            ui.set_font_size(NOTIFICATION_FONT_SIZE);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_color(get_color32(255, 255, 255, 0));
            ui.set_text("");
        }
        row_mut.add_widget(&name);

        ItemAcquireSlot {
            _root_widget: row.as_ref(),
            _icon_widget: icon.as_ref(),
            _name_widget: name.as_ref(),
        }
    }

    fn hide_all_slots(&mut self) {
        for slot in self._slots.iter() {
            let ui = ptr_as_mut(slot._root_widget).get_ui_component_mut();
            ui.set_visible(false);
        }
    }

    pub fn notify_item_acquired(&mut self, item_data_name: &str) {
        let (item_name, material_instance) = {
            let game_resources = get_game_resources();
            let item_data = game_resources.get_item_data(item_data_name).borrow();
            let name = item_data._name.clone();
            let material = get_engine_resources()
                .get_material_instance_data(item_data._ui_material_instance.as_str())
                .clone();
            (name, material)
        };

        // if let Some(entry) = self._entries.iter_mut().find(|e| e._item_data_name == item_data_name) {
        //     entry._remaining_time = NOTIFICATION_DISPLAY_DURATION;
        //     return;
        // }

        if self._entries.len() >= MAX_NOTIFICATION_SLOTS {
            self._entries.remove(0);
        }

        self._entries.push(ItemAcquireEntry {
            _item_data_name: item_data_name.to_string(),
            _item_name: item_name.to_string(),
            _remaining_time: NOTIFICATION_DISPLAY_DURATION,
        });

        let slot_index = self._entries.len() - 1;
        let slot = &self._slots[slot_index];
        let icon_ui = ptr_as_mut(slot._icon_widget).get_ui_component_mut();
        icon_ui.set_material_instance(Some(material_instance));
    }

    pub fn update(&mut self, delta_time: f32) {
        for entry in self._entries.iter_mut() {
            entry._remaining_time -= delta_time;
        }

        self._entries.retain(|e| e._remaining_time > 0.0);

        let entry_count = self._entries.len();
        for (i, slot) in self._slots.iter().enumerate() {
            let root_ui = ptr_as_mut(slot._root_widget).get_ui_component_mut();

            if i < entry_count {
                let entry = &self._entries[i];
                root_ui.set_visible(true);

                let alpha = if entry._remaining_time < NOTIFICATION_FADE_DURATION {
                    (entry._remaining_time / NOTIFICATION_FADE_DURATION).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let alpha_u8 = (alpha * 160.0) as u8;
                root_ui.set_color(get_color32(0, 0, 0, alpha_u8.into()));

                let text_alpha = (alpha * 255.0) as u8;
                let name_ui = ptr_as_mut(slot._name_widget).get_ui_component_mut();
                name_ui.set_font_color(get_color32(255, 255, 255, text_alpha.into()));
                name_ui.set_text(entry._item_name.as_str());
            } else {
                root_ui.set_visible(false);
                let name_ui = ptr_as_mut(slot._name_widget).get_ui_component_mut();
                name_ui.set_text("");
            }
        }
    }
}
