use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_input::UiInput;
use crate::game_module::game_service_locator::{
    get_game_client_mut, get_game_resources, get_game_resources_mut, get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::popup_widget::PopupWindowWidget;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

const BTN_DISABLED_COLOR: u32 = get_color32(210, 185, 150, 255);
const BTN_LOAD_COLOR: u32 = get_color32(70, 135, 200, 240);
const BTN_LOAD_FOCUSED_COLOR: u32 = get_color32(100, 175, 245, 255);
const BTN_SAVE_COLOR: u32 = get_color32(118, 168, 68, 240);
const BTN_SAVE_FOCUSED_COLOR: u32 = get_color32(150, 210, 85, 255);
const BTN_DELETE_COLOR: u32 = get_color32(200, 75, 60, 255);
const BTN_DELETE_FOCUSED_COLOR: u32 = get_color32(240, 105, 90, 255);
const BTN_NEW_GAME_COLOR: u32 = get_color32(118, 168, 68, 240);
const BTN_NEW_GAME_FOCUSED_COLOR: u32 = get_color32(150, 210, 85, 255);
const BTN_EXIT_GAME_COLOR: u32 = get_color32(200, 75, 60, 255);
const BTN_EXIT_GAME_FOCUSED_COLOR: u32 = get_color32(230, 100, 85, 255);
const SLOT_CARD_NORMAL_COLOR: u32 = get_color32(245, 232, 205, 230);
const SLOT_CARD_SELECTED_COLOR: u32 = get_color32(235, 205, 155, 240);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotButtonFocus {
    Load,
    Save,
    Delete,
}

impl SlotButtonFocus {
    pub fn next(self, has_save_data: bool) -> Self {
        if !has_save_data {
            return SlotButtonFocus::Save;
        }
        match self {
            SlotButtonFocus::Load => SlotButtonFocus::Save,
            SlotButtonFocus::Save => SlotButtonFocus::Delete,
            SlotButtonFocus::Delete => SlotButtonFocus::Delete,
        }
    }

    pub fn prev(self, has_save_data: bool) -> Self {
        if !has_save_data {
            return SlotButtonFocus::Save;
        }
        match self {
            SlotButtonFocus::Load => SlotButtonFocus::Load,
            SlotButtonFocus::Save => SlotButtonFocus::Load,
            SlotButtonFocus::Delete => SlotButtonFocus::Save,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveLoadFocusTarget {
    Slot(usize, SlotButtonFocus),
    NewGame,
    ExitGame,
}

fn is_popup_opened(popup: Option<&RcRefCell<PopupWindowWidget<'_>>>) -> bool {
    if let Some(p) = popup {
        if let Ok(guard) = p.try_borrow() {
            return guard.is_opened();
        }
        return true;
    }
    false
}

fn set_button_enabled<'a>(button: &Rc<WidgetDefault<'a>>, enabled: bool, active_color: u32, disabled_color: u32) {
    let ui = ptr_as_mut::<WidgetDefault<'a>>(button.as_ref()).get_ui_component_mut();
    ui.set_color(if enabled { active_color } else { disabled_color });
    ui.set_touchable(enabled);
}

pub struct SaveLoadSlotItem<'a> {
    pub _slot_widget: *const SaveLoadSlotWidget<'a>,
    pub _slot_index: usize,
    pub _slot_name: String,
    pub _item_widget: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _load_btn: Rc<WidgetDefault<'a>>,
    pub _save_btn: Rc<WidgetDefault<'a>>,
    pub _delete_btn: Rc<WidgetDefault<'a>>,
}

pub struct SaveLoadSlotWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _new_game_btn: Rc<WidgetDefault<'a>>,
    pub _exit_game_btn: Rc<WidgetDefault<'a>>,
    pub _slot_header_label: Rc<WidgetDefault<'a>>,
    pub _slot_container: Rc<WidgetDefault<'a>>,
    pub _save_complete_popup: Option<RcRefCell<PopupWindowWidget<'a>>>,
    pub _delete_confirm_popup: Option<RcRefCell<PopupWindowWidget<'a>>>,
    pub _pending_delete_slot_index: Option<usize>,
    pub _slot_items: Vec<Box<SaveLoadSlotItem<'a>>>,
    pub _slot_names: Vec<String>,
    pub _selected_slot_index: usize,
    pub _focus_target: SaveLoadFocusTarget,
    pub _is_opened: bool,
    pub _nav_repeat_controller: WidgetNavRepeatController,
    pub _need_rebuild_slot_list: bool,
}

impl<'a> SaveLoadSlotWidget<'a> {
    pub fn callback_touch_over_slot(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        if slot_widget._selected_slot_index != slot_item._slot_index {
            slot_widget.set_selected_slot(slot_item._slot_index, false);
        }
        true
    }

    pub fn callback_button_touch_over(
        _ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_touch_over_slot_load(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        if slot_widget.has_save_data_at(slot_item._slot_index) {
            slot_widget.set_focus_target(
                SaveLoadFocusTarget::Slot(slot_item._slot_index, SlotButtonFocus::Load),
                false,
            );
        }
        true
    }

    pub fn callback_touch_over_slot_save(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        slot_widget.set_focus_target(
            SaveLoadFocusTarget::Slot(slot_item._slot_index, SlotButtonFocus::Save),
            false,
        );
        true
    }

    pub fn callback_touch_over_slot_delete(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        if slot_widget.has_save_data_at(slot_item._slot_index) {
            slot_widget.set_focus_target(
                SaveLoadFocusTarget::Slot(slot_item._slot_index, SlotButtonFocus::Delete),
                false,
            );
        }
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

    pub fn callback_touch_down_slot_delete(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadSlotItem<'a>);
        let slot_widget = ptr_as_mut(slot_item._slot_widget);
        slot_widget.request_delete_slot(slot_item._slot_index);
        true
    }

    pub fn callback_touch_down_confirm_delete_slot(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        if let Some(slot_index) = slot_widget._pending_delete_slot_index {
            slot_widget.delete_slot(slot_index);
            slot_widget._pending_delete_slot_index = None;
        }
        if let Some(popup) = slot_widget._delete_confirm_popup.as_ref()
            && let Ok(mut popup_guard) = popup.try_borrow_mut()
        {
            popup_guard.close();
        }
        true
    }

    pub fn callback_touch_down_cancel_delete_slot(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget._pending_delete_slot_index = None;
        if let Some(popup) = slot_widget._delete_confirm_popup.as_ref()
            && let Ok(mut popup_guard) = popup.try_borrow_mut()
        {
            popup_guard.close();
        }
        true
    }

    pub fn callback_touch_down_close_save_popup(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        if let Some(popup) = slot_widget._save_complete_popup.as_ref()
            && let Ok(mut popup_guard) = popup.try_borrow_mut()
        {
            popup_guard.close();
        }
        true
    }

    pub fn callback_touch_over_new_game(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.set_focus_target(SaveLoadFocusTarget::NewGame, false);
        true
    }

    pub fn callback_touch_over_exit_game(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let slot_widget = ptr_as_mut(ui_component.get_user_data() as *const SaveLoadSlotWidget<'a>);
        slot_widget.set_focus_target(SaveLoadFocusTarget::ExitGame, false);
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

    pub fn create_save_load_slot_widget(
        parent_widget: &mut WidgetDefault<'a>,
        root_widget: &mut WidgetDefault<'a>,
    ) -> Box<SaveLoadSlotWidget<'a>> {
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
            ui_component.set_color(get_color32(240, 222, 186, 248));
            ui_component.set_border_color(get_color32(135, 78, 42, 255));
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
            ui_comp.set_color(get_color32(215, 185, 140, 230));
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
            ui_comp.set_color(BTN_NEW_GAME_COLOR);
            ui_comp.set_round(5.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_new_game)));
            ui_comp.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_new_game)));
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
            ui_comp.set_color(BTN_EXIT_GAME_COLOR);
            ui_comp.set_round(5.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_exit_game)));
            ui_comp.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_exit_game)));
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
            label_ui.set_font_color(get_color32(255, 250, 230, 255));
            label_ui.set_color(get_color32(195, 118, 58, 240));
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
            ui_comp.set_color(get_color32(252, 245, 226, 230));
            ui_comp.set_round(6.0);
        }
        layer_mut.add_widget(&slot_container);

        let mut slot_widget = Box::new(SaveLoadSlotWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _new_game_btn: new_game_btn,
            _exit_game_btn: exit_game_btn,
            _slot_header_label: slot_header_label,
            _slot_container: slot_container,
            _save_complete_popup: None,
            _delete_confirm_popup: None,
            _pending_delete_slot_index: None,
            _slot_items: Vec::new(),
            _slot_names: Vec::new(),
            _selected_slot_index: 0,
            _focus_target: SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save),
            _is_opened: false,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
            _need_rebuild_slot_list: false,
        });

        let ptr_self = slot_widget.as_ref() as *const SaveLoadSlotWidget<'a> as *const c_void;
        ptr_as_mut::<WidgetDefault<'a>>(slot_widget._new_game_btn.as_ref())
            .get_ui_component_mut()
            .set_user_data(ptr_self);
        ptr_as_mut::<WidgetDefault<'a>>(slot_widget._exit_game_btn.as_ref())
            .get_ui_component_mut()
            .set_user_data(ptr_self);

        // --- 5. Save Complete Popup ---
        slot_widget._save_complete_popup = Some(PopupWindowWidget::create_popup_widget(
            root_widget,
            "Game Saved Successfully!",
            "OK",
            None,
            Some((SaveLoadSlotWidget::callback_touch_down_close_save_popup, ptr_self)),
            None,
        ));

        // --- 6. Delete Confirmation Popup ---
        slot_widget._delete_confirm_popup = Some(PopupWindowWidget::create_popup_widget(
            root_widget,
            "Are you sure you want to delete?",
            "Delete",
            Some("Cancel"),
            Some((SaveLoadSlotWidget::callback_touch_down_confirm_delete_slot, ptr_self)),
            Some((SaveLoadSlotWidget::callback_touch_down_cancel_delete_slot, ptr_self)),
        ));

        slot_widget
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn open_slot_widget(&mut self) {
        if !self._is_opened {
            let parent_mut = ptr_as_mut::<WidgetDefault<'a>>(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._is_opened = true;
            self._nav_repeat_controller.reset();
            self._selected_slot_index = 0;
            if self._slot_items.is_empty() {
                self.init_slot_list();
            } else {
                self.refresh_slot_states();
            }
            let default_btn = if self.has_save_data_at(0) {
                SlotButtonFocus::Load
            } else {
                SlotButtonFocus::Save
            };
            self.set_focus_target(SaveLoadFocusTarget::Slot(0, default_btn), true);
        }
    }

    pub fn close_slot_widget(&mut self) {
        if self._is_opened {
            if let Some(popup) = self._save_complete_popup.as_ref()
                && let Ok(mut popup_guard) = popup.try_borrow_mut()
            {
                popup_guard.close();
            }
            if let Some(popup) = self._delete_confirm_popup.as_ref()
                && let Ok(mut popup_guard) = popup.try_borrow_mut()
            {
                popup_guard.close();
            }
            let parent_mut = ptr_as_mut::<WidgetDefault<'a>>(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened = false;
        }
    }

    fn create_slot_item_widget(&mut self, index: usize, slot_name: &str) {
        let container_mut = ptr_as_mut::<WidgetDefault<'a>>(self._slot_container.as_ref());
        let slot_card = UIManager::create_widget("slot_card", UIWidgetTypes::Default);

        let is_selected = matches!(self._focus_target, SaveLoadFocusTarget::Slot(idx, _) if idx == index);
        let card_color = if is_selected {
            SLOT_CARD_SELECTED_COLOR
        } else {
            SLOT_CARD_NORMAL_COLOR
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
            card_ui.set_border_color(get_color32(200, 165, 125, 200));
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
            text_ui.set_font_size(22.0);
            text_ui.set_font_color(get_color32(75, 45, 20, 255));
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
            action_ui.set_font_size(22.0);
            action_ui.set_font_color(get_color32(255, 255, 255, 255));

            if has_save_data {
                action_ui.set_color(get_color32(70, 135, 200, 240));
                action_ui.set_touchable(true);
            } else {
                action_ui.set_color(get_color32(210, 185, 150, 255));
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
            action_ui.set_size(85.0, 36.0);
            action_ui.set_margin(4.0);
            action_ui.set_round(4.0);
            action_ui.set_text("SAVE");
            action_ui.set_font_size(22.0);
            action_ui.set_font_color(get_color32(255, 255, 255, 255));
            action_ui.set_color(get_color32(118, 168, 68, 240));
            action_ui.set_touchable(true);
        }
        ptr_as_mut(slot_card.as_ref()).add_widget(&save_btn);

        // DELETE Button on slot card
        let delete_btn = UIManager::create_widget("slot_delete_btn", UIWidgetTypes::Default);
        {
            let action_ui = ptr_as_mut(delete_btn.as_ref()).get_ui_component_mut();
            action_ui.set_halign(HorizontalAlign::CENTER);
            action_ui.set_valign(VerticalAlign::CENTER);
            action_ui.set_size(36.0, 36.0);
            action_ui.set_margin(4.0);
            action_ui.set_round(4.0);
            action_ui.set_text("X");
            action_ui.set_font_size(22.0);
            action_ui.set_font_color(get_color32(255, 255, 255, 255));

            if has_save_data {
                action_ui.set_color(get_color32(200, 75, 60, 255));
                action_ui.set_touchable(true);
            } else {
                action_ui.set_color(get_color32(210, 185, 150, 255));
                action_ui.set_touchable(false);
            }
        }
        ptr_as_mut(slot_card.as_ref()).add_widget(&delete_btn);

        container_mut.add_widget(&slot_card);

        let slot_item = Box::new(SaveLoadSlotItem {
            _slot_widget: self,
            _slot_index: index,
            _slot_name: slot_name.to_string(),
            _item_widget: slot_card.clone(),
            _text_widget: text_widget.clone(),
            _load_btn: load_btn.clone(),
            _save_btn: save_btn.clone(),
            _delete_btn: delete_btn.clone(),
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
            load_ui.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_slot_load)));
            load_ui.set_user_data(item_ptr);
        }

        {
            let save_ui = ptr_as_mut(save_btn.as_ref()).get_ui_component_mut();
            save_ui.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_slot_save)));
            save_ui.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_slot_save)));
            save_ui.set_user_data(item_ptr);
        }

        {
            let delete_ui = ptr_as_mut(delete_btn.as_ref()).get_ui_component_mut();
            delete_ui.set_callback_touch_down(Some(Box::new(SaveLoadSlotWidget::callback_touch_down_slot_delete)));
            delete_ui.set_callback_touch_over(Some(Box::new(SaveLoadSlotWidget::callback_touch_over_slot_delete)));
            delete_ui.set_user_data(item_ptr);
        }

        self._slot_items.push(slot_item);
    }

    pub fn get_max_save_data_index(&self) -> Option<usize> {
        let mut max_idx = None;
        for idx in 0..100 {
            let slot_name = format!("save_data/{:02}", idx);
            if get_game_resources().has_game_save_data(&slot_name) {
                max_idx = Some(idx);
            }
        }
        max_idx
    }

    pub fn init_slot_list(&mut self) {
        let container_mut = ptr_as_mut::<WidgetDefault<'a>>(self._slot_container.as_ref());
        container_mut.clear_widgets();
        self._slot_items.clear();
        self._slot_names.clear();

        let max_save_idx = self.get_max_save_data_index();
        let target_max = match max_save_idx {
            Some(m) => m + 1,
            None => 0,
        };

        for idx in 0..=target_max {
            let slot_name = format!("save_data/{:02}", idx);
            self._slot_names.push(slot_name.clone());
            self.create_slot_item_widget(idx, &slot_name);
        }
    }

    pub fn ensure_empty_slot_exists(&mut self) {
        if let Some(last_slot_name) = self._slot_names.last() {
            let last_slot_name = last_slot_name.clone();
            if get_game_resources().has_game_save_data(&last_slot_name) {
                let new_index = self._slot_names.len();
                let new_slot_name = format!("save_data/{:02}", new_index);
                self._slot_names.push(new_slot_name.clone());
                self.create_slot_item_widget(new_index, &new_slot_name);
            }
        } else {
            let new_slot_name = "save_data/00".to_string();
            self._slot_names.push(new_slot_name.clone());
            self.create_slot_item_widget(0, &new_slot_name);
        }
    }

    pub fn refresh_slot_states(&mut self) {
        for idx in 0..self._slot_items.len() {
            self.update_slot_item_ui(idx);
        }
        self.ensure_empty_slot_exists();
    }

    pub fn update_slot_item_ui(&mut self, slot_index: usize) {
        if slot_index < self._slot_items.len() {
            let slot_name = &self._slot_names[slot_index];
            let has_save_data = get_game_resources().has_game_save_data(slot_name);

            let slot_info_text = if has_save_data {
                let save_data_ref = get_game_resources_mut().get_game_save_data(slot_name);
                let scene_name = &save_data_ref.borrow()._last_game_scene_data_name;
                let date = save_data_ref.borrow()._date;
                format!(
                    "Slot {}: [{}] | Scene: {} | Day: {}",
                    slot_index + 1,
                    slot_name,
                    if scene_name.is_empty() { "default" } else { scene_name },
                    date
                )
            } else {
                format!("Slot {}: [{}] | [ Empty Slot ]", slot_index + 1, slot_name)
            };

            let item = &self._slot_items[slot_index];

            let text_ui = ptr_as_mut::<WidgetDefault<'a>>(item._text_widget.as_ref()).get_ui_component_mut();
            text_ui.set_text(&slot_info_text);

            set_button_enabled(&item._load_btn, has_save_data, BTN_LOAD_COLOR, BTN_DISABLED_COLOR);
            set_button_enabled(&item._delete_btn, has_save_data, BTN_DELETE_COLOR, BTN_DISABLED_COLOR);
        }
    }

    pub fn has_save_data_at(&self, slot_index: usize) -> bool {
        if slot_index < self._slot_names.len() {
            get_game_resources().has_game_save_data(&self._slot_names[slot_index])
        } else {
            false
        }
    }

    fn update_slot_buttons_highlight(&self, slot_index: usize, focused_btn: Option<SlotButtonFocus>) {
        if slot_index < self._slot_items.len() && slot_index < self._slot_names.len() {
            let has_save_data = self.has_save_data_at(slot_index);
            let item = &self._slot_items[slot_index];

            let load_color = if !has_save_data {
                BTN_DISABLED_COLOR
            } else if focused_btn == Some(SlotButtonFocus::Load) {
                BTN_LOAD_FOCUSED_COLOR
            } else {
                BTN_LOAD_COLOR
            };
            ptr_as_mut::<WidgetDefault<'a>>(item._load_btn.as_ref()).get_ui_component_mut().set_color(load_color);

            let save_color = if focused_btn == Some(SlotButtonFocus::Save) {
                BTN_SAVE_FOCUSED_COLOR
            } else {
                BTN_SAVE_COLOR
            };
            ptr_as_mut::<WidgetDefault<'a>>(item._save_btn.as_ref()).get_ui_component_mut().set_color(save_color);

            let delete_color = if !has_save_data {
                BTN_DISABLED_COLOR
            } else if focused_btn == Some(SlotButtonFocus::Delete) {
                BTN_DELETE_FOCUSED_COLOR
            } else {
                BTN_DELETE_COLOR
            };
            ptr_as_mut::<WidgetDefault<'a>>(item._delete_btn.as_ref()).get_ui_component_mut().set_color(delete_color);
        }
    }

    pub fn set_focus_target(&mut self, target: SaveLoadFocusTarget, force: bool) {
        if self._focus_target != target || force {
            let prev_target = self._focus_target;
            self._focus_target = target;

            match prev_target {
                SaveLoadFocusTarget::Slot(prev_index, _) => {
                    if prev_index < self._slot_items.len() {
                        let prev_card = &self._slot_items[prev_index]._item_widget;
                        ptr_as_mut::<WidgetDefault<'a>>(prev_card.as_ref())
                            .get_ui_component_mut()
                            .set_color(SLOT_CARD_NORMAL_COLOR);
                        self.update_slot_buttons_highlight(prev_index, None);
                    }
                }
                SaveLoadFocusTarget::NewGame => {
                    ptr_as_mut::<WidgetDefault<'a>>(self._new_game_btn.as_ref())
                        .get_ui_component_mut()
                        .set_color(BTN_NEW_GAME_COLOR);
                }
                SaveLoadFocusTarget::ExitGame => {
                    ptr_as_mut::<WidgetDefault<'a>>(self._exit_game_btn.as_ref())
                        .get_ui_component_mut()
                        .set_color(BTN_EXIT_GAME_COLOR);
                }
            }

            match target {
                SaveLoadFocusTarget::Slot(index, btn) => {
                    if index < self._slot_items.len() {
                        self._selected_slot_index = index;
                        let curr_card = &self._slot_items[index]._item_widget;
                        let curr_card_mut = ptr_as_mut::<WidgetDefault<'a>>(curr_card.as_ref());
                        curr_card_mut.get_ui_component_mut().set_color(SLOT_CARD_SELECTED_COLOR);

                        let container_mut = ptr_as_mut::<WidgetDefault<'a>>(self._slot_container.as_ref());
                        container_mut.get_ui_component_mut().scroll_into_view(curr_card_mut.get_ui_component());

                        self.update_slot_buttons_highlight(index, Some(btn));
                    }
                }
                SaveLoadFocusTarget::NewGame => {
                    let btn_mut = ptr_as_mut::<WidgetDefault<'a>>(self._new_game_btn.as_ref());
                    btn_mut.get_ui_component_mut().set_color(BTN_NEW_GAME_FOCUSED_COLOR);
                }
                SaveLoadFocusTarget::ExitGame => {
                    let btn_mut = ptr_as_mut::<WidgetDefault<'a>>(self._exit_game_btn.as_ref());
                    btn_mut.get_ui_component_mut().set_color(BTN_EXIT_GAME_FOCUSED_COLOR);
                }
            }

            if !force {
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
        }
    }

    fn move_mouse_to_focus_target(&self, target: SaveLoadFocusTarget) {
        match target {
            SaveLoadFocusTarget::Slot(index, btn) => {
                if let Some(item) = self._slot_items.get(index) {
                    let target_widget = match btn {
                        SlotButtonFocus::Load => &item._load_btn,
                        SlotButtonFocus::Save => &item._save_btn,
                        SlotButtonFocus::Delete => &item._delete_btn,
                    };
                    move_mouse_to_ui_component(target_widget.get_ui_component());
                }
            }
            SaveLoadFocusTarget::NewGame => {
                move_mouse_to_ui_component(self._new_game_btn.get_ui_component());
            }
            SaveLoadFocusTarget::ExitGame => {
                move_mouse_to_ui_component(self._exit_game_btn.get_ui_component());
            }
        }
    }

    pub fn set_selected_slot(&mut self, index: usize, force: bool) {
        if index < self._slot_items.len() {
            let default_btn = if self.has_save_data_at(index) {
                SlotButtonFocus::Load
            } else {
                SlotButtonFocus::Save
            };
            self.set_focus_target(SaveLoadFocusTarget::Slot(index, default_btn), force);
        }
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

    pub fn request_delete_slot(&mut self, slot_index: usize) {
        if slot_index < self._slot_names.len() {
            let slot_name = &self._slot_names[slot_index];
            if get_game_resources().has_game_save_data(slot_name) {
                self._pending_delete_slot_index = Some(slot_index);
                if let Some(popup) = self._delete_confirm_popup.as_ref()
                    && let Ok(mut popup_guard) = popup.try_borrow_mut()
                {
                    popup_guard.open(popup);
                }
            }
        }
    }

    pub fn delete_slot(&mut self, slot_index: usize) {
        if slot_index < self._slot_names.len() {
            let slot_name = self._slot_names[slot_index].clone();
            if get_game_resources().has_game_save_data(&slot_name) {
                let game_res = get_game_resources_mut();
                game_res.remove_game_save_data(&slot_name);

                let mut idx = slot_index + 1;
                loop {
                    let from_name = format!("save_data/{:02}", idx);
                    let to_name = format!("save_data/{:02}", idx - 1);
                    if game_res.has_game_save_data(&from_name) {
                        game_res.rename_game_save_data(&from_name, &to_name);
                        idx += 1;
                    } else {
                        break;
                    }
                }

                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

                if slot_index < self._slot_items.len() {
                    let card_ui = ptr_as_mut::<WidgetDefault<'a>>(self._slot_items[slot_index]._item_widget.as_ref())
                        .get_ui_component_mut();
                    card_ui.set_enable(false);
                }

                self._need_rebuild_slot_list = true;
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
            self.update_slot_item_ui(slot_index);
            self.ensure_empty_slot_exists();
            self.set_focus_target(SaveLoadFocusTarget::Slot(slot_index, SlotButtonFocus::Save), true);
            if let Some(popup) = self._save_complete_popup.as_ref()
                && let Ok(mut popup_guard) = popup.try_borrow_mut()
            {
                popup_guard.open(popup);
            }
        }
    }

    pub fn update_slot_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if !self._is_opened {
            return;
        }

        if is_popup_opened(self._save_complete_popup.as_ref()) || is_popup_opened(self._delete_confirm_popup.as_ref()) {
            return;
        }

        if self._need_rebuild_slot_list {
            self._need_rebuild_slot_list = false;
            self.init_slot_list();
            let max_idx = self._slot_items.len().saturating_sub(1);
            let target_idx = self._selected_slot_index.min(max_idx);
            self.set_selected_slot(target_idx, true);
        }

        if self._slot_items.is_empty() {
            return;
        }

        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if should_move && let Some((dir_x, dir_y)) = dir_opt {
            let next_target =
                Self::compute_next_focus_target(self._focus_target, dir_x, dir_y, self._slot_items.len(), |idx| {
                    self.has_save_data_at(idx)
                });

            if let Some(target) = next_target {
                self.set_focus_target(target, false);
                self.move_mouse_to_focus_target(target);
            }
        }

        if UiInput::is_confirm(keyboard_input_data, joystick_input_data) {
            match self._focus_target {
                SaveLoadFocusTarget::NewGame => {
                    self.new_game();
                }
                SaveLoadFocusTarget::ExitGame => {
                    self.exit_game();
                }
                SaveLoadFocusTarget::Slot(idx, btn) => match btn {
                    SlotButtonFocus::Load => {
                        if self.has_save_data_at(idx) {
                            self.load_slot(idx);
                        }
                    }
                    SlotButtonFocus::Save => {
                        self.save_slot(idx);
                    }
                    SlotButtonFocus::Delete => {
                        if self.has_save_data_at(idx) {
                            self.request_delete_slot(idx);
                        }
                    }
                },
            }
        }

        if UiInput::is_cancel(keyboard_input_data, joystick_input_data) {
            self.close_slot_widget();
            get_game_ui_manager_mut().close_game_menu();
        }
    }

    pub fn compute_next_focus_target(
        curr_target: SaveLoadFocusTarget,
        dir_x: i32,
        dir_y: i32,
        slot_count: usize,
        has_save_data: impl Fn(usize) -> bool,
    ) -> Option<SaveLoadFocusTarget> {
        match curr_target {
            SaveLoadFocusTarget::NewGame => {
                if dir_y > 0 {
                    let btn = if has_save_data(0) {
                        SlotButtonFocus::Load
                    } else {
                        SlotButtonFocus::Save
                    };
                    Some(SaveLoadFocusTarget::Slot(0, btn))
                } else if dir_x != 0 {
                    Some(SaveLoadFocusTarget::ExitGame)
                } else {
                    None
                }
            }
            SaveLoadFocusTarget::ExitGame => {
                if dir_y > 0 {
                    let btn = if has_save_data(0) {
                        SlotButtonFocus::Load
                    } else {
                        SlotButtonFocus::Save
                    };
                    Some(SaveLoadFocusTarget::Slot(0, btn))
                } else if dir_x != 0 {
                    Some(SaveLoadFocusTarget::NewGame)
                } else {
                    None
                }
            }
            SaveLoadFocusTarget::Slot(idx, btn) => {
                if dir_y < 0 {
                    if idx == 0 {
                        Some(SaveLoadFocusTarget::NewGame)
                    } else {
                        let target_idx = idx - 1;
                        let next_btn = if has_save_data(target_idx) {
                            btn
                        } else {
                            SlotButtonFocus::Save
                        };
                        Some(SaveLoadFocusTarget::Slot(target_idx, next_btn))
                    }
                } else if dir_y > 0 {
                    let max_index = slot_count.saturating_sub(1);
                    let target_idx = (idx + 1).min(max_index);
                    let next_btn = if has_save_data(target_idx) {
                        btn
                    } else {
                        SlotButtonFocus::Save
                    };
                    Some(SaveLoadFocusTarget::Slot(target_idx, next_btn))
                } else if dir_x != 0 {
                    let has_data = has_save_data(idx);
                    let next_btn = if dir_x > 0 {
                        btn.next(has_data)
                    } else {
                        btn.prev(has_data)
                    };
                    Some(SaveLoadFocusTarget::Slot(idx, next_btn))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_load_navigation_vertical_and_horizontal() {
        let slot_count = 3;
        let has_save_data = |idx| idx == 0; // slot 0만 세이브 데이터 있음

        // 1. 슬롯 최상단(0)에서 위쪽 방향키 -> New Game으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load),
            0,
            -1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::NewGame));

        // 2. 슬롯 1번에서 위쪽 방향키 -> 슬롯 0번으로 이동 (0번은 데이터가 있으므로 포커스 버튼 유지)
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(1, SlotButtonFocus::Save),
            0,
            -1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save)));

        // 3. New Game에서 아래쪽 방향키 -> 슬롯 0번으로 이동 (데이터가 있으므로 기본 Load)
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::NewGame,
            0,
            1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load)));

        // 4. Exit Game에서 아래쪽 방향키 -> 슬롯 0번으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::ExitGame,
            0,
            1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load)));

        // 5. New Game에서 오른쪽 방향키 -> Exit Game으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::NewGame,
            1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::ExitGame));

        // 6. New Game에서 왼쪽 방향키 -> Exit Game으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::NewGame,
            -1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::ExitGame));

        // 7. Exit Game에서 왼쪽 방향키 -> New Game으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::ExitGame,
            -1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::NewGame));

        // 8. Exit Game에서 오른쪽 방향키 -> New Game으로 이동
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::ExitGame,
            1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::NewGame));

        // 9. 슬롯 0번에서 아래쪽 방향키 -> 슬롯 1번으로 이동 (슬롯 1번은 데이터가 없으므로 Save로 조정됨)
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load),
            0,
            1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(1, SlotButtonFocus::Save)));

        // 10. 슬롯 최하단(2)에서 아래쪽 방향키 -> 최하단 유지
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(2, SlotButtonFocus::Save),
            0,
            1,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(2, SlotButtonFocus::Save)));
    }

    #[test]
    fn test_slot_button_focus_horizontal() {
        let slot_count = 3;

        // 세이브 데이터가 있는 슬롯 (Load <-> Save <-> Delete 양 끝 멈춤)
        let has_save_data = |_idx| true;

        // Load -> Save -> Delete -> Delete 유지 (오른쪽 이동)
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load),
            1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save)));

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save),
            1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Delete)));

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Delete),
            1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Delete)));

        // Delete -> Save -> Load -> Load 유지 (왼쪽 이동)
        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Delete),
            -1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save)));

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save),
            -1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load)));

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load),
            -1,
            0,
            slot_count,
            has_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Load)));

        // 세이브 데이터가 없는 빈 슬롯 (Save 버튼만 활성화되므로 좌우 이동해도 Save 유지)
        let has_no_save_data = |_idx| false;

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save),
            1,
            0,
            slot_count,
            has_no_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save)));

        let next = SaveLoadSlotWidget::compute_next_focus_target(
            SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save),
            -1,
            0,
            slot_count,
            has_no_save_data,
        );
        assert_eq!(next, Some(SaveLoadFocusTarget::Slot(0, SlotButtonFocus::Save)));
    }
}
