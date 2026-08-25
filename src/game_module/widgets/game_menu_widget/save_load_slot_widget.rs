use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{
    get_game_client_mut, get_game_resources, get_game_resources_mut, get_game_ui_manager_mut,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub struct SaveLoadSlotItem<'a> {
    pub _slot_widget: *const SaveLoadSlotWidget<'a>,
    pub _slot_index: usize,
    pub _slot_name: String,
    pub _item_widget: Rc<WidgetDefault<'a>>,
    pub _load_btn: Rc<WidgetDefault<'a>>,
    pub _save_btn: Rc<WidgetDefault<'a>>,
}

pub struct SaveLoadSlotWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _new_game_btn: Rc<WidgetDefault<'a>>,
    pub _exit_game_btn: Rc<WidgetDefault<'a>>,
    pub _slot_header_label: Rc<WidgetDefault<'a>>,
    pub _slot_container: Rc<WidgetDefault<'a>>,
    pub _add_slot_btn: Rc<WidgetDefault<'a>>,
    pub _slot_items: Vec<Box<SaveLoadSlotItem<'a>>>,
    pub _slot_names: Vec<String>,
    pub _selected_slot_index: usize,
    pub _is_opened: bool,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> SaveLoadSlotWidget<'a> {
    pub fn callback_touch_over_slot(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        slot_widget.set_selected_slot(slot_item._slot_index, false);
        true
    }

    pub fn callback_touch_down_slot_load(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        slot_widget.load_slot(slot_item._slot_index);
        true
    }

    pub fn callback_touch_down_slot_save(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        slot_widget.save_slot(slot_item._slot_index);
        true
    }

    pub fn callback_touch_down_new_game(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.new_game();
        true
    }

    pub fn callback_touch_down_exit_game(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.exit_game();
        true
    }

    pub fn callback_touch_down_add_slot(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.add_new_slot();
        true
    }

    pub fn callback_touch_down_close(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.close_slot_widget();
        get_game_ui_manager_mut().close_game_menu();
        true
    }

    pub fn create_save_load_slot_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<SaveLoadSlotWidget<'a>> {
        let layer = UIManager::create_widget("save_load_slot_layer", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        {
            let ui_component = layer_mut.get_ui_component_mut();
            ui_component.set_layout_type(UILayoutType::BoxLayout);
            ui_component.set_layout_orientation(Orientation::VERTICAL);
            ui_component.set_halign(HorizontalAlign::CENTER);
            ui_component.set_valign(VerticalAlign::CENTER);
            ui_component.set_pivot_preset(PIVOT_CENTER);
            ui_component.set_expandable(false);
            ui_component.set_size_hint_x(Some(1.0));
            ui_component.set_size_hint_y(Some(1.0));
            ui_component.set_padding(14.0);
            ui_component.set_color(get_color32(25, 30, 40, 245));
            ui_component.set_border_color(get_color32(70, 110, 160, 255));
            ui_component.set_round(8.0);
        }

        // --- 1. Top Header Area: Prominent [New Game] and [Exit Game] buttons + Close [X] ---
        let header_layout = UIManager::create_widget("slot_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header_layout.as_ref());
        {
            let ui_comp = header_mut.get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_y(52.0);
            ui_comp.set_margin(4.0);
            ui_comp.set_color(get_color32(18, 22, 30, 220));
            ui_comp.set_round(6.0);
        }
        layer_mut.add_widget(&header_layout);

        // Header 1: [New Game] Button (Enlarged)
        let new_game_btn = UIManager::create_widget("header_new_game_btn", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(new_game_btn.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(240.0, 42.0);
            ui_comp.set_margin(10.0);
            ui_comp.set_text("New Game");
            ui_comp.set_font_size(22.0);
            ui_comp.set_font_color(get_color32(255, 255, 255, 255));
            ui_comp.set_color(get_color32(40, 130, 190, 255));
            ui_comp.set_round(5.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_new_game)));
        }
        header_mut.add_widget(&new_game_btn);

        // Header 2: [Exit Game] Button (Enlarged)
        let exit_game_btn = UIManager::create_widget("header_exit_game_btn", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(exit_game_btn.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(240.0, 42.0);
            ui_comp.set_margin(10.0);
            ui_comp.set_text("Exit Game");
            ui_comp.set_font_size(22.0);
            ui_comp.set_font_color(get_color32(255, 255, 255, 255));
            ui_comp.set_color(get_color32(180, 55, 55, 255));
            ui_comp.set_round(5.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_exit_game)));
        }
        header_mut.add_widget(&exit_game_btn);

        // --- 2. Slot Section Header: LOAD / SAVE Section Title Bar ---
        let slot_header_label = UIManager::create_widget("slot_section_header", UIWidgetTypes::Default);
        {
            let label_ui = ptr_as_mut(slot_header_label.as_ref()).get_ui_component_mut();
            label_ui.set_halign(HorizontalAlign::CENTER);
            label_ui.set_valign(VerticalAlign::CENTER);
            label_ui.set_size_hint_x(Some(1.0));
            label_ui.set_size_y(32.0);
            label_ui.set_margin(3.0);
            label_ui.set_text("LOAD / SAVE");
            label_ui.set_font_size(22.0);
            label_ui.set_font_color(get_color32(220, 230, 245, 255));
            label_ui.set_color(get_color32(18, 24, 34, 180));
            label_ui.set_round(4.0);
        }
        layer_mut.add_widget(&slot_header_label);

        // --- 3. Body Area: Vertical Slot List Container ---
        let slot_container = UIManager::create_widget("slot_container", UIWidgetTypes::Default);
        {
            let slot_container_mut = ptr_as_mut(slot_container.as_ref());
            let ui_comp = slot_container_mut.get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::TOP);
            ui_comp.set_expandable(false);
            ui_comp.set_scroll_y(true);
            ui_comp.set_enable_renderable_area(true);
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_hint_y(Some(1.0));
            ui_comp.set_margin(4.0);
            ui_comp.set_padding(8.0);
            ui_comp.set_color(get_color32(15, 20, 28, 200));
            ui_comp.set_round(6.0);
        }
        layer_mut.add_widget(&slot_container);

        // --- 4. Footer Area: [+ Add New Slot] Button ---
        let footer_layout = UIManager::create_widget("slot_footer", UIWidgetTypes::Default);
        let footer_mut = ptr_as_mut(footer_layout.as_ref());
        {
            let ui_comp = footer_mut.get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_y(48.0);
            ui_comp.set_margin(4.0);
            ui_comp.set_color(get_color32(18, 22, 30, 220));
            ui_comp.set_round(5.0);
        }
        layer_mut.add_widget(&footer_layout);

        let add_slot_btn = UIManager::create_widget("add_slot_btn", UIWidgetTypes::Default);
        {
            let add_ui = ptr_as_mut(add_slot_btn.as_ref()).get_ui_component_mut();
            add_ui.set_halign(HorizontalAlign::CENTER);
            add_ui.set_valign(VerticalAlign::CENTER);
            add_ui.set_size(300.0, 38.0);
            add_ui.set_margin(6.0);
            add_ui.set_text("+ Add New Slot");
            add_ui.set_font_size(20.0);
            add_ui.set_font_color(get_color32(255, 255, 255, 255));
            add_ui.set_color(get_color32(45, 140, 75, 255));
            add_ui.set_round(5.0);
            add_ui.set_touchable(true);
            add_ui.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_add_slot)));
        }
        footer_mut.add_widget(&add_slot_btn);

        let initial_slot_names = vec![
            "save_data/00".to_string(),
            "save_data/01".to_string(),
            "save_data/02".to_string(),
        ];

        let slot_widget = Box::new(SaveLoadSlotWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _new_game_btn: new_game_btn,
            _exit_game_btn: exit_game_btn,
            _slot_header_label: slot_header_label,
            _slot_container: slot_container,
            _add_slot_btn: add_slot_btn,
            _slot_items: Vec::new(),
            _slot_names: initial_slot_names,
            _selected_slot_index: 0,
            _is_opened: false,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        });

        let ptr_self = slot_widget.as_ref() as *const SaveLoadSlotWidget<'a> as *const c_void;
        ptr_as_mut(slot_widget._new_game_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(slot_widget._exit_game_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(slot_widget._add_slot_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);

        slot_widget
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn open_slot_widget(&mut self) {
        if !self._is_opened {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._is_opened = true;
            self._selected_slot_index = 0;
            self.refresh_slot_list();
        }
    }

    pub fn close_slot_widget(&mut self) {
        if self._is_opened {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened = false;
        }
    }

    pub fn refresh_slot_list(&mut self) {
        let container_mut = ptr_as_mut(self._slot_container.as_ref());
        container_mut.clear_widgets();
        self._slot_items.clear();

        for (index, slot_name) in self._slot_names.iter().enumerate() {
            let slot_card = UIManager::create_widget("slot_card", UIWidgetTypes::Default);

            let is_selected = index == self._selected_slot_index;
            let card_color = if is_selected {
                get_color32(60, 90, 130, 240)
            } else {
                get_color32(40, 48, 60, 220)
            };

            {
                let card_ui = ptr_as_mut(slot_card.as_ref()).get_ui_component_mut();
                card_ui.set_layout_type(UILayoutType::BoxLayout);
                card_ui.set_layout_orientation(Orientation::HORIZONTAL);
                card_ui.set_halign(HorizontalAlign::CENTER);
                card_ui.set_valign(VerticalAlign::CENTER);
                card_ui.set_size(660.0, 52.0);
                card_ui.set_margin(3.0);
                card_ui.set_padding(6.0);
                card_ui.set_round(5.0);
                card_ui.set_color(card_color);
                card_ui.set_border_color(get_color32(80, 100, 125, 255));
            }

            let has_save_data = get_game_resources().has_game_save_data(slot_name);

            let slot_info_text = if has_save_data {
                let save_data_ref = get_game_resources_mut().get_game_save_data(slot_name);
                let scene_name = &save_data_ref.borrow()._last_game_scene_data_name;
                let date = save_data_ref.borrow()._date;
                format!(
                    "Slot {}: [{}] | Scene: {} | Day: {}",
                    index + 1,
                    slot_name,
                    if scene_name.is_empty() { "default" } else { scene_name },
                    date
                )
            } else {
                format!("Slot {}: [{}] | [ Empty Slot ]", index + 1, slot_name)
            };

            let text_widget = UIManager::create_widget("slot_text", UIWidgetTypes::Default);
            {
                let text_ui = ptr_as_mut(text_widget.as_ref()).get_ui_component_mut();
                text_ui.set_halign(HorizontalAlign::LEFT);
                text_ui.set_valign(VerticalAlign::CENTER);
                text_ui.set_size(400.0, 40.0);
                text_ui.set_margin(4.0);
                text_ui.set_text(&slot_info_text);
                text_ui.set_font_size(19.0);
                text_ui.set_font_color(get_color32(255, 255, 255, 255));
                text_ui.set_color(get_color32(0, 0, 0, 0));
            }
            ptr_as_mut(slot_card.as_ref()).add_widget(&text_widget);

            // LOAD Button on slot card
            let load_btn = UIManager::create_widget("slot_load_btn", UIWidgetTypes::Default);
            {
                let action_ui = ptr_as_mut(load_btn.as_ref()).get_ui_component_mut();
                action_ui.set_halign(HorizontalAlign::CENTER);
                action_ui.set_valign(VerticalAlign::CENTER);
                action_ui.set_size(95.0, 36.0);
                action_ui.set_margin(4.0);
                action_ui.set_round(4.0);
                action_ui.set_text("LOAD");
                action_ui.set_font_size(20.0);
                action_ui.set_font_color(get_color32(255, 255, 255, 255));

                if has_save_data {
                    action_ui.set_color(get_color32(40, 110, 190, 255));
                    action_ui.set_touchable(true);
                } else {
                    action_ui.set_color(get_color32(70, 75, 85, 180));
                    action_ui.set_touchable(false);
                }
            }
            ptr_as_mut(slot_card.as_ref()).add_widget(&load_btn);

            // SAVE Button on slot card
            let save_btn = UIManager::create_widget("slot_save_btn", UIWidgetTypes::Default);
            {
                let action_ui = ptr_as_mut(save_btn.as_ref()).get_ui_component_mut();
                action_ui.set_halign(HorizontalAlign::CENTER);
                action_ui.set_valign(VerticalAlign::CENTER);
                action_ui.set_size(95.0, 36.0);
                action_ui.set_margin(4.0);
                action_ui.set_round(4.0);
                action_ui.set_text("SAVE");
                action_ui.set_font_size(20.0);
                action_ui.set_font_color(get_color32(255, 255, 255, 255));
                action_ui.set_color(get_color32(45, 140, 65, 255));
                action_ui.set_touchable(true);
            }
            ptr_as_mut(slot_card.as_ref()).add_widget(&save_btn);

            container_mut.add_widget(&slot_card);

            let slot_item = Box::new(SaveLoadSlotItem {
                _slot_widget: self,
                _slot_index: index,
                _slot_name: slot_name.clone(),
                _item_widget: slot_card.clone(),
                _load_btn: load_btn.clone(),
                _save_btn: save_btn.clone(),
            });

            let item_ptr = slot_item.as_ref() as *const SaveLoadSlotItem<'a> as *const c_void;

            {
                let card_ui = ptr_as_mut(slot_card.as_ref()).get_ui_component_mut();
                card_ui.set_touchable(true);
                card_ui.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_slot)));
                card_ui.set_user_data(item_ptr);
            }

            {
                let load_ui = ptr_as_mut(load_btn.as_ref()).get_ui_component_mut();
                load_ui.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_slot_load)));
                load_ui.set_user_data(item_ptr);
            }

            {
                let save_ui = ptr_as_mut(save_btn.as_ref()).get_ui_component_mut();
                save_ui.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_slot_save)));
                save_ui.set_user_data(item_ptr);
            }

            self._slot_items.push(slot_item);
        }
    }

    pub fn set_selected_slot(&mut self, index: usize, force: bool) {
        if index < self._slot_items.len() && (self._selected_slot_index != index || force) {
            let prev_index = self._selected_slot_index;
            self._selected_slot_index = index;

            if prev_index < self._slot_items.len() {
                let prev_card = &self._slot_items[prev_index]._item_widget;
                ptr_as_mut(prev_card.as_ref()).get_ui_component_mut().set_color(get_color32(40, 48, 60, 220));
            }

            let curr_card = &self._slot_items[index]._item_widget;
            let curr_card_mut = ptr_as_mut(curr_card.as_ref());
            curr_card_mut.get_ui_component_mut().set_color(get_color32(60, 90, 130, 240));

            let container_mut = ptr_as_mut(self._slot_container.as_ref());
            container_mut.get_ui_component_mut().scroll_into_view(curr_card_mut.get_ui_component());

            if !force {
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            }
        }
    }

    pub fn add_new_slot(&mut self) {
        let new_index = self._slot_names.len();
        let new_slot_name = format!("save_data/{:02}", new_index);
        self._slot_names.push(new_slot_name);
        self._selected_slot_index = new_index;
        self.refresh_slot_list();
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
    }

    pub fn new_game(&mut self) {
        let game_client = get_game_client_mut();
        game_client.request_new_game();
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        get_game_ui_manager_mut().close_game_menu();
    }

    pub fn exit_game(&mut self) {
        let game_client = get_game_client_mut();
        game_client.exit_game();
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        get_game_ui_manager_mut().close_game_menu();
    }

    pub fn load_slot(&mut self, slot_index: usize) {
        if slot_index < self._slot_names.len() {
            let slot_name = &self._slot_names[slot_index];
            if get_game_resources().has_game_save_data(slot_name) {
                let game_client = get_game_client_mut();
                game_client.request_load_game(slot_name);
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                get_game_ui_manager_mut().close_game_menu();
            }
        }
    }

    pub fn save_slot(&mut self, slot_index: usize) {
        if slot_index < self._slot_names.len() {
            let slot_name = self._slot_names[slot_index].clone();
            let game_client = get_game_client_mut();
            game_client._game_save_data_name = slot_name;
            game_client.save_game(true);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            self.refresh_slot_list();
        }
    }

    pub fn update_slot_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if !self._is_opened || self._slot_items.is_empty() {
            return;
        }

        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if should_move {
            let (_dir_x, dir_y) = dir_opt.unwrap();
            if dir_y < 0 {
                let next_index = self._selected_slot_index.saturating_sub(1);
                self.set_selected_slot(next_index, false);
            } else if dir_y > 0 {
                let max_index = self._slot_items.len().saturating_sub(1);
                let next_index = (self._selected_slot_index + 1).min(max_index);
                self.set_selected_slot(next_index, false);
            }
        }

        let close_widget =
            keyboard_input_data.get_key_pressed(KeyCode::Escape) || joystick_input_data._btn_b == ButtonState::Pressed;

        if close_widget {
            self.close_slot_widget();
            get_game_ui_manager_mut().close_game_menu();
        }
    }
}
