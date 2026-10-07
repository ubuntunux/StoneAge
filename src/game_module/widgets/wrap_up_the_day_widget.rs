use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_QUEST_COMPLETE};
use crate::game_module::game_service_locator::{get_game_resources, get_game_scene_manager};
use crate::game_module::widgets::item_bar::InventorySlotData;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub const COLOR_PANEL_BG: u32 = get_color32(240, 222, 186, 248);
pub const COLOR_PANEL_BORDER: u32 = get_color32(135, 78, 42, 255);
pub const COLOR_TITLE_BG: u32 = get_color32(195, 118, 58, 240);
pub const COLOR_TITLE_BORDER: u32 = get_color32(115, 60, 28, 220);
pub const COLOR_TITLE_TEXT: u32 = get_color32(255, 250, 230, 255);
pub const COLOR_CONTAINER_BG: u32 = get_color32(252, 245, 226, 230);
pub const COLOR_CONTAINER_BORDER: u32 = get_color32(188, 148, 105, 200);
pub const COLOR_OK_BTN_BG: u32 = get_color32(118, 168, 68, 240);
pub const COLOR_OK_BTN_BORDER: u32 = get_color32(72, 115, 38, 255);
pub const COLOR_OK_BTN_TEXT: u32 = get_color32(255, 255, 245, 255);
pub const COLOR_ITEM_ROW_BG: u32 = get_color32(245, 232, 205, 230);
pub const COLOR_ITEM_ROW_BORDER: u32 = get_color32(200, 165, 125, 200);
pub const COLOR_ITEM_ROW_TEXT: u32 = get_color32(75, 45, 20, 255);
pub const COLOR_EMPTY_TEXT: u32 = get_color32(155, 120, 90, 255);

pub const ITEM_STAGGER_INTERVAL: f32 = 0.2;
pub const NUMBER_TICK_SPEED: f32 = 10.0;

pub struct SettlementItemAnimData<'a> {
    pub _display_name: String,
    pub _target_count: u32,
    pub _current_count: u32,
    pub _row_widget: Rc<WidgetDefault<'a>>,
    pub _info_widget: Rc<WidgetDefault<'a>>,
    pub _is_visible: bool,
}

pub struct WrapUpTheDayWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _panel_frame: Rc<WidgetDefault<'a>>,
    pub _title_text: Rc<WidgetDefault<'a>>,
    pub _items_container: Rc<WidgetDefault<'a>>,
    pub _ok_btn: Rc<WidgetDefault<'a>>,
    pub _is_opened: bool,
    pub _is_ok_clicked: bool,
    pub _is_settlement_started: bool,
    pub _item_widgets: Vec<Rc<WidgetDefault<'a>>>,
    pub _anim_timer: f32,
    pub _item_anim_data: Vec<SettlementItemAnimData<'a>>,
}

impl<'a> WrapUpTheDayWidget<'a> {
    pub fn create_wrap_up_the_day_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<WrapUpTheDayWidget<'a>> {
        // Root Layer
        let layer = UIManager::create_widget("daily_settlement_layer", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_pivot_preset(PIVOT_CENTER);
            ui_comp.set_pos_hint(Some(0.5), Some(0.5));
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_hint_y(Some(1.0));
            ui_comp.set_enable(false);
            ui_comp.set_renderable(false);
        }

        // Panel Frame
        let panel_frame = UIManager::create_widget("daily_settlement_frame", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(panel_frame.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_pivot_preset(PIVOT_CENTER);
            ui_comp.set_pos_hint(Some(0.5), Some(0.5));
            ui_comp.set_size(520.0, 500.0);
            ui_comp.set_padding(20.0);
            ui_comp.set_color(COLOR_PANEL_BG);
            ui_comp.set_border_color(COLOR_PANEL_BORDER);
            ui_comp.set_border(3.0);
            ui_comp.set_round(14.0);
        }
        ptr_as_mut(layer.as_ref()).add_widget(&panel_frame);

        // Header Title
        let title_text = UIManager::create_widget("daily_settlement_title", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(title_text.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(480.0, 50.0);
            ui_comp.set_margin(8.0);
            ui_comp.set_text("DAY 1 - WRAP UP THE DAY");
            ui_comp.set_font_size(24.0);
            ui_comp.set_font_color(COLOR_TITLE_TEXT);
            ui_comp.set_color(COLOR_TITLE_BG);
            ui_comp.set_border_color(COLOR_TITLE_BORDER);
            ui_comp.set_border(1.0);
            ui_comp.set_round(8.0);
        }
        ptr_as_mut(panel_frame.as_ref()).add_widget(&title_text);

        // Items Container List
        let items_container = UIManager::create_widget("daily_settlement_items_container", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(items_container.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::TOP);
            ui_comp.set_size(480.0, 330.0);
            ui_comp.set_margin(10.0);
            ui_comp.set_padding(10.0);
            ui_comp.set_color(COLOR_CONTAINER_BG);
            ui_comp.set_border_color(COLOR_CONTAINER_BORDER);
            ui_comp.set_border(1.0);
            ui_comp.set_round(8.0);
            ui_comp.set_scroll_y(true);
            ui_comp.set_enable_renderable_area(true);
        }
        ptr_as_mut(panel_frame.as_ref()).add_widget(&items_container);

        // OK Button (Store In Chest)
        let ok_btn = UIManager::create_widget("daily_settlement_ok_btn", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(ok_btn.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(240.0, 44.0);
            ui_comp.set_margin(10.0);
            ui_comp.set_text("STORE IN CHEST");
            ui_comp.set_font_size(20.0);
            ui_comp.set_font_color(COLOR_OK_BTN_TEXT);
            ui_comp.set_color(COLOR_OK_BTN_BG);
            ui_comp.set_border_color(COLOR_OK_BTN_BORDER);
            ui_comp.set_border(2.0);
            ui_comp.set_round(8.0);
            ui_comp.set_touchable(true);
        }
        ptr_as_mut(panel_frame.as_ref()).add_widget(&ok_btn);

        parent_widget.add_widget(&layer);

        let widget = Box::new(WrapUpTheDayWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _panel_frame: panel_frame,
            _title_text: title_text,
            _items_container: items_container,
            _ok_btn: ok_btn,
            _is_opened: false,
            _is_ok_clicked: false,
            _is_settlement_started: false,
            _item_widgets: Vec::new(),
            _anim_timer: 0.0,
            _item_anim_data: Vec::new(),
        });

        let ok_btn_comp = ptr_as_mut(widget._ok_btn.as_ref()).get_ui_component_mut();
        ok_btn_comp.set_callback_touch_down(Some(Box::new(WrapUpTheDayWidget::callback_ok_click)));
        ok_btn_comp.set_user_data(widget.as_ref() as *const WrapUpTheDayWidget<'a> as *const c_void);

        widget
    }

    pub fn open_daily_settlement(&mut self, transferred_items: &[InventorySlotData<'a>]) {
        self._is_opened = true;
        self._is_ok_clicked = false;
        self._is_settlement_started = false;
        self._anim_timer = 0.0;
        self._item_anim_data.clear();

        // Title update with Game Date
        let date = get_game_scene_manager().get_date();
        let title_text_str = format!("DAY {} - WRAP UP THE DAY", date);
        let title_comp = ptr_as_mut(self._title_text.as_ref()).get_ui_component_mut();
        title_comp.set_text(&title_text_str);

        let ui_comp = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(true);

        let ui_comp = ptr_as_mut(self._panel_frame.as_ref()).get_ui_component_mut();
        ui_comp.set_opacity(1.0);

        // Clear existing item row widgets
        let items_container_mut = ptr_as_mut(self._items_container.as_ref());
        items_container_mut.clear_widgets();
        self._item_widgets.clear();

        let widget_heights = 50.0;

        if transferred_items.is_empty() {
            let empty_widget = UIManager::create_widget("settlement_item_empty", UIWidgetTypes::Default);
            let ui_comp = ptr_as_mut(empty_widget.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::LEFT);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_hint_y(Some(1.0));
            ui_comp.set_size_y(widget_heights);
            ui_comp.set_margin(2.0);
            ui_comp.set_text("No resources collected today.");
            ui_comp.set_font_size(24.0);
            ui_comp.set_font_color(COLOR_EMPTY_TEXT);
            ui_comp.set_color(get_color32(0, 0, 0, 0));
            items_container_mut.add_widget(&empty_widget);
            self._item_widgets.push(empty_widget);
        } else {
            let engine_resources = get_engine_resources();
            for (idx, slot) in transferred_items.iter().enumerate() {
                let row_widget =
                    UIManager::create_widget(&format!("settlement_item_row_{}", idx), UIWidgetTypes::Default);
                let ui_comp = ptr_as_mut(row_widget.as_ref()).get_ui_component_mut();
                ui_comp.set_layout_type(UILayoutType::BoxLayout);
                ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
                ui_comp.set_halign(HorizontalAlign::LEFT);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_size_hint_x(Some(1.0));
                ui_comp.set_size_y(widget_heights);
                ui_comp.set_margin(2.0);
                ui_comp.set_color(get_color32(0, 0, 0, 0));
                ui_comp.set_opacity(0.0);
                items_container_mut.add_widget(&row_widget);

                // Material instance icon
                let icon_widget =
                    UIManager::create_widget(&format!("settlement_item_icon_{}", idx), UIWidgetTypes::Default);
                let ui_comp = ptr_as_mut(icon_widget.as_ref()).get_ui_component_mut();
                ui_comp.set_layout_type(UILayoutType::BoxLayout);
                ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
                ui_comp.set_halign(HorizontalAlign::LEFT);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_size(widget_heights, widget_heights);
                if !slot._item_data_name.is_empty() {
                    let item_data = get_game_resources().get_item_data(&slot._item_data_name).borrow();
                    if !item_data._ui_material_instance.is_empty() {
                        let mat_inst = engine_resources.get_material_instance_data(&item_data._ui_material_instance);
                        ui_comp.set_material_instance(Some(mat_inst.clone()));
                    }
                    ptr_as_mut(row_widget.as_ref()).add_widget(&icon_widget);
                }

                let item_info_widget =
                    UIManager::create_widget(&format!("settlement_item_info_{}", idx), UIWidgetTypes::Default);
                let ui_comp = ptr_as_mut(item_info_widget.as_ref()).get_ui_component_mut();
                ui_comp.set_layout_type(UILayoutType::BoxLayout);
                ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
                ui_comp.set_halign(HorizontalAlign::LEFT);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_color(get_color32(0, 0, 0, 0));
                ui_comp.set_expandable_x(true);
                ui_comp.set_size_hint_y(Some(1.0));
                ui_comp.set_margin_left(4.0);
                let display_name = if !slot._item_name.is_empty() {
                    slot._item_name.clone()
                } else {
                    slot._item_data_name.clone()
                };
                let initial_text = format!("{} x 0", display_name);
                ui_comp.set_text(&initial_text);
                ui_comp.set_font_size(24.0);
                ui_comp.set_font_color(COLOR_ITEM_ROW_TEXT);
                ptr_as_mut(row_widget.as_ref()).add_widget(&item_info_widget);

                self._item_anim_data.push(SettlementItemAnimData {
                    _display_name: display_name,
                    _target_count: slot._item_count as u32,
                    _current_count: 0,
                    _row_widget: row_widget.clone(),
                    _info_widget: item_info_widget,
                    _is_visible: false,
                });

                self._item_widgets.push(row_widget);
            }
        }
    }

    pub fn start_daily_settlement(&mut self) {
        if !self._is_settlement_started {
            self._is_settlement_started = true;
            self._anim_timer = 0.0;
        }
    }

    pub fn close_daily_settlement(&mut self) {
        self._is_opened = false;
        self._is_settlement_started = false;
        let ui_comp = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(false);
    }

    pub fn is_opened_daily_settlement(&self) -> bool {
        self._is_opened
    }

    pub fn is_settlement_started(&self) -> bool {
        self._is_settlement_started
    }

    pub fn is_ok_clicked(&self) -> bool {
        self._is_ok_clicked
    }

    pub fn callback_ok_click(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ui_component.get_user_data();
        if !user_data.is_null() {
            let widget = unsafe { &*(user_data as *const WrapUpTheDayWidget<'a>) };
            let widget_mut = ptr_as_mut(widget);
            widget_mut._is_ok_clicked = true;
            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
            true
        } else {
            false
        }
    }

    pub fn update_wrap_up_the_day_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
    ) {
        if !self._is_opened {
            return;
        }

        if self._is_ok_clicked {
            let ui_comp = ptr_as_mut(self._panel_frame.as_ref()).get_ui_component_mut();
            let opacity = 0f32.max(ui_comp.get_opacity() - time_data._delta_time as f32 * 5.0);
            ui_comp.set_opacity(opacity);
            if opacity <= 0.0 {
                self.close_daily_settlement();
            }
            return;
        }

        if !self._is_settlement_started {
            return;
        }

        // Staggered fade-in & count ticker update
        self._anim_timer += time_data._delta_time as f32;
        for (idx, anim_data) in self._item_anim_data.iter_mut().enumerate() {
            let start_time = idx as f32 * ITEM_STAGGER_INTERVAL;
            if start_time <= self._anim_timer {
                if !anim_data._is_visible {
                    anim_data._is_visible = true;
                    let ui_comp = ptr_as_mut(anim_data._row_widget.as_ref()).get_ui_component_mut();
                    ui_comp.set_opacity(1.0);
                    get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                }

                if anim_data._current_count < anim_data._target_count {
                    let elapsed = self._anim_timer - start_time;
                    let calculated_count = (elapsed * NUMBER_TICK_SPEED) as u32;
                    let new_count = calculated_count.min(anim_data._target_count);
                    if new_count != anim_data._current_count {
                        anim_data._current_count = new_count;
                        let text = format!("{} x {}", anim_data._display_name, anim_data._current_count);
                        let ui_comp = ptr_as_mut(anim_data._info_widget.as_ref()).get_ui_component_mut();
                        ui_comp.set_text(&text);
                        //get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                    }
                }
            }
        }

        // Key interactions (Confirm with Space, Return/Enter, KeyE, or Joypad Button A)
        if keyboard_input_data.get_key_pressed(KeyCode::Space)
            || keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::NumpadEnter)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyE)
            || joystick_input_data._btn_a == ButtonState::Pressed
        {
            self._is_ok_clicked = true;
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
    }
}

