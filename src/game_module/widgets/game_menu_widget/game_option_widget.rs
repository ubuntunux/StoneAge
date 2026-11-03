use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_input::UiInput;
use crate::game_module::game_option::{
    GameOptionData, MAX_MOUSE_SENSITIVITY, MAX_VOLUME, MIN_MOUSE_SENSITIVITY, MIN_VOLUME, MOUSE_SENSITIVITY_STEP,
    VOLUME_STEP,
};
use crate::game_module::game_service_locator::{get_game_resources, get_game_resources_mut};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter, EnumString, FromRepr};

const ROW_WIDTH: f32 = 680.0;
const ROW_HEIGHT: f32 = 42.0;
const LABEL_WIDTH: f32 = 280.0;
const STEPPER_BTN_SIZE: f32 = 34.0;
const VALUE_WIDTH: f32 = 140.0;
const TOGGLE_BTN_WIDTH: f32 = 100.0;
const ACTION_BTN_WIDTH: f32 = 160.0;
const ACTION_BTN_HEIGHT: f32 = 40.0;

const COLOR_ROW_BG_NORMAL: u32 = get_color32(245, 232, 205, 230);
const COLOR_ROW_BG_SELECTED: u32 = get_color32(255, 245, 220, 255);
const COLOR_ROW_BORDER_NORMAL: u32 = get_color32(200, 165, 125, 200);
const COLOR_ROW_BORDER_SELECTED: u32 = get_color32(195, 118, 58, 255);
const COLOR_TEXT_LABEL: u32 = get_color32(75, 45, 20, 255);
const COLOR_TEXT_VALUE: u32 = get_color32(50, 30, 10, 255);

const COLOR_BTN_STEPPER: u32 = get_color32(195, 140, 85, 230);
const COLOR_BTN_TOGGLE_ON: u32 = get_color32(80, 150, 80, 240);
const COLOR_BTN_TOGGLE_OFF: u32 = get_color32(165, 130, 90, 230);

const COLOR_BTN_SAVE: u32 = get_color32(76, 145, 65, 240);
const COLOR_BTN_RESET: u32 = get_color32(185, 95, 65, 240);
const COLOR_STATUS_TEXT: u32 = get_color32(50, 120, 50, 255);

const TOTAL_NAV_ITEMS: usize = OptionItemType::COUNT + 2;
const NAV_INDEX_RESET: usize = OptionItemType::COUNT;
const NAV_INDEX_SAVE: usize = OptionItemType::COUNT + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, FromRepr, EnumCount, EnumIter, EnumString)]
#[repr(usize)]
pub enum OptionItemType {
    MasterVolume,
    BgmVolume,
    SfxVolume,
    MuteAudio,
    MouseSensitivity,
    InvertMouseY,
    ShowDebugText,
}

impl OptionItemType {
    pub fn display_name(&self) -> &'static str {
        match self {
            OptionItemType::MasterVolume => "Master Volume",
            OptionItemType::BgmVolume => "BGM Volume",
            OptionItemType::SfxVolume => "SFX Volume",
            OptionItemType::MuteAudio => "Mute All Audio",
            OptionItemType::MouseSensitivity => "Mouse Sensitivity",
            OptionItemType::InvertMouseY => "Invert Mouse Y-Axis",
            OptionItemType::ShowDebugText => "Show Debug Text",
        }
    }

    pub fn is_toggle(&self) -> bool {
        matches!(
            self,
            OptionItemType::MuteAudio | OptionItemType::InvertMouseY | OptionItemType::ShowDebugText
        )
    }
}

pub struct OptionRowUserData<'a> {
    pub _widget_ptr: *const GameOptionWidget<'a>,
    pub _item_type: OptionItemType,
}

pub struct OptionRow<'a> {
    pub _row_widget: Rc<WidgetDefault<'a>>,
    pub _label_widget: Rc<WidgetDefault<'a>>,
    pub _value_widget: Option<Rc<WidgetDefault<'a>>>,
    pub _dec_btn: Option<Rc<WidgetDefault<'a>>>,
    pub _inc_btn: Option<Rc<WidgetDefault<'a>>>,
    pub _toggle_btn: Option<Rc<WidgetDefault<'a>>>,
    pub _item_type: OptionItemType,
    pub _user_data: Box<OptionRowUserData<'a>>,
}

pub struct GameOptionWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _title_widget: Rc<WidgetDefault<'a>>,
    pub _rows_container: Rc<WidgetDefault<'a>>,
    pub _rows: Vec<OptionRow<'a>>,
    pub _reset_btn: Rc<WidgetDefault<'a>>,
    pub _save_btn: Rc<WidgetDefault<'a>>,
    pub _status_label: Rc<WidgetDefault<'a>>,
    pub _selected_index: usize,
    pub _game_option_data: GameOptionData,
    pub _is_opened: bool,
}

impl<'a> GameOptionWidget<'a> {
    pub fn callback_dec(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        widget.decrement_option(user_data._item_type);
        true
    }

    pub fn callback_inc(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        widget.increment_option(user_data._item_type);
        true
    }

    pub fn callback_toggle(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        widget.toggle_option(user_data._item_type);
        true
    }

    pub fn callback_row_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        widget.set_selected_index(user_data._item_type as usize, false);
        true
    }

    pub fn callback_btn_touch_over(
        _ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_reset_touch_over(
        ui_component: &UIComponentInstance<'a>,
        pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        let widget = ptr_as_mut(ui_component.get_user_data() as *const GameOptionWidget<'a>);
        widget.set_selected_index(NAV_INDEX_RESET, false);
        Self::callback_btn_touch_over(ui_component, pos, delta)
    }

    pub fn callback_save_touch_over(
        ui_component: &UIComponentInstance<'a>,
        pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        let widget = ptr_as_mut(ui_component.get_user_data() as *const GameOptionWidget<'a>);
        widget.set_selected_index(NAV_INDEX_SAVE, false);
        Self::callback_btn_touch_over(ui_component, pos, delta)
    }

    pub fn callback_reset_click(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let widget = ptr_as_mut(ui_component.get_user_data() as *const GameOptionWidget<'a>);
        widget.reset_to_defaults();
        true
    }

    pub fn callback_save_click(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let widget = ptr_as_mut(ui_component.get_user_data() as *const GameOptionWidget<'a>);
        widget.save_options();
        true
    }

    pub fn create_game_option_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<GameOptionWidget<'a>> {
        let layer = UIManager::create_widget("game_option_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_comp = layer_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_pivot_preset(PIVOT_CENTER);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_padding(8.0);
        ui_comp.set_color(get_color32(0, 0, 0, 0));

        // Title Header
        let title_widget = UIManager::create_widget("option_title", UIWidgetTypes::Default);
        let title_ui = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        title_ui.set_halign(HorizontalAlign::CENTER);
        title_ui.set_valign(VerticalAlign::CENTER);
        title_ui.set_size_hint_x(Some(1.0));
        title_ui.set_size_y(30.0);
        title_ui.set_text("GAME SETTINGS");
        title_ui.set_font_size(24.0);
        title_ui.set_font_color(get_color32(110, 60, 25, 255));
        title_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&title_widget);

        // Rows Container
        let rows_container = UIManager::create_widget("option_rows_container", UIWidgetTypes::Default);
        let rows_container_mut = ptr_as_mut(rows_container.as_ref());
        let rows_ui = rows_container_mut.get_ui_component_mut();
        rows_ui.set_layout_type(UILayoutType::BoxLayout);
        rows_ui.set_layout_orientation(Orientation::VERTICAL);
        rows_ui.set_halign(HorizontalAlign::CENTER);
        rows_ui.set_valign(VerticalAlign::TOP);
        rows_ui.set_size_hint_x(Some(1.0));
        rows_ui.set_size_y(350.0);
        rows_ui.set_margin(4.0);
        rows_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&rows_container);

        // Footer Action Layout
        let footer_layout = UIManager::create_widget("option_footer", UIWidgetTypes::Default);
        let footer_mut = ptr_as_mut(footer_layout.as_ref());
        let footer_ui = footer_mut.get_ui_component_mut();
        footer_ui.set_layout_type(UILayoutType::BoxLayout);
        footer_ui.set_layout_orientation(Orientation::HORIZONTAL);
        footer_ui.set_halign(HorizontalAlign::CENTER);
        footer_ui.set_valign(VerticalAlign::CENTER);
        footer_ui.set_size_hint_x(Some(1.0));
        footer_ui.set_size_y(50.0);
        footer_ui.set_margin(5.0);
        footer_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&footer_layout);

        // Reset Button
        let reset_btn = UIManager::create_widget("option_reset_btn", UIWidgetTypes::Default);
        let reset_ui = ptr_as_mut(reset_btn.as_ref()).get_ui_component_mut();
        reset_ui.set_halign(HorizontalAlign::CENTER);
        reset_ui.set_valign(VerticalAlign::CENTER);
        reset_ui.set_size(ACTION_BTN_WIDTH, ACTION_BTN_HEIGHT);
        reset_ui.set_margin(8.0);
        reset_ui.set_text("Reset Defaults");
        reset_ui.set_font_size(20.0);
        reset_ui.set_font_color(get_color32(255, 255, 255, 255));
        reset_ui.set_round(5.0);
        reset_ui.set_color(COLOR_BTN_RESET);
        reset_ui.set_touchable(true);
        reset_ui.set_callback_touch_down(Some(Box::new(Self::callback_reset_click)));
        reset_ui.set_callback_touch_over(Some(Box::new(Self::callback_reset_touch_over)));
        footer_mut.add_widget(&reset_btn);

        // Save Button
        let save_btn = UIManager::create_widget("option_save_btn", UIWidgetTypes::Default);
        let save_ui = ptr_as_mut(save_btn.as_ref()).get_ui_component_mut();
        save_ui.set_halign(HorizontalAlign::CENTER);
        save_ui.set_valign(VerticalAlign::CENTER);
        save_ui.set_size(ACTION_BTN_WIDTH, ACTION_BTN_HEIGHT);
        save_ui.set_margin(8.0);
        save_ui.set_text("Save Options");
        save_ui.set_font_size(20.0);
        save_ui.set_font_color(get_color32(255, 255, 255, 255));
        save_ui.set_round(5.0);
        save_ui.set_color(COLOR_BTN_SAVE);
        save_ui.set_touchable(true);
        save_ui.set_callback_touch_down(Some(Box::new(Self::callback_save_click)));
        save_ui.set_callback_touch_over(Some(Box::new(Self::callback_save_touch_over)));
        footer_mut.add_widget(&save_btn);

        // Status Label
        let status_label = UIManager::create_widget("option_status_label", UIWidgetTypes::Default);
        let status_ui = ptr_as_mut(status_label.as_ref()).get_ui_component_mut();
        status_ui.set_halign(HorizontalAlign::CENTER);
        status_ui.set_valign(VerticalAlign::CENTER);
        status_ui.set_size_hint_x(Some(1.0));
        status_ui.set_size_y(24.0);
        status_ui.set_text("");
        status_ui.set_font_size(18.0);
        status_ui.set_font_color(COLOR_STATUS_TEXT);
        status_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&status_label);

        let initial_option = get_game_resources().get_game_option_data().borrow().clone();

        let mut widget = Box::new(GameOptionWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _title_widget: title_widget,
            _rows_container: rows_container,
            _rows: Vec::new(),
            _reset_btn: reset_btn,
            _save_btn: save_btn,
            _status_label: status_label,
            _selected_index: 0,
            _game_option_data: initial_option,
            _is_opened: false,
        });

        let widget_ptr = widget.as_ref() as *const GameOptionWidget<'a>;
        ptr_as_mut(widget._reset_btn.as_ref()).get_ui_component_mut().set_user_data(widget_ptr as *const c_void);
        ptr_as_mut(widget._save_btn.as_ref()).get_ui_component_mut().set_user_data(widget_ptr as *const c_void);

        // Build option rows
        let rows_container_mut = ptr_as_mut(widget._rows_container.as_ref());
        for item_type in OptionItemType::iter() {
            let user_data = Box::new(OptionRowUserData {
                _widget_ptr: widget_ptr,
                _item_type: item_type,
            });

            let row_widget = UIManager::create_widget(&format!("row_{:?}", item_type), UIWidgetTypes::Default);
            let row_mut = ptr_as_mut(row_widget.as_ref());
            let row_ui = row_mut.get_ui_component_mut();
            row_ui.set_layout_type(UILayoutType::BoxLayout);
            row_ui.set_layout_orientation(Orientation::HORIZONTAL);
            row_ui.set_halign(HorizontalAlign::CENTER);
            row_ui.set_valign(VerticalAlign::CENTER);
            row_ui.set_size(ROW_WIDTH, ROW_HEIGHT);
            row_ui.set_margin(3.0);
            row_ui.set_padding(4.0);
            row_ui.set_color(COLOR_ROW_BG_NORMAL);
            row_ui.set_border_color(COLOR_ROW_BORDER_NORMAL);
            row_ui.set_round(5.0);
            row_ui.set_touchable(true);
            row_ui.set_callback_touch_over(Some(Box::new(Self::callback_row_touch_over)));
            row_ui.set_user_data(user_data.as_ref() as *const OptionRowUserData<'a> as *const c_void);
            rows_container_mut.add_widget(&row_widget);

            // Label
            let label_widget = UIManager::create_widget(&format!("label_{:?}", item_type), UIWidgetTypes::Default);
            let label_ui = ptr_as_mut(label_widget.as_ref()).get_ui_component_mut();
            label_ui.set_halign(HorizontalAlign::LEFT);
            label_ui.set_valign(VerticalAlign::CENTER);
            label_ui.set_size(LABEL_WIDTH, ROW_HEIGHT - 8.0);
            label_ui.set_margin(4.0);
            label_ui.set_text(item_type.display_name());
            label_ui.set_font_size(20.0);
            label_ui.set_font_color(COLOR_TEXT_LABEL);
            label_ui.set_color(get_color32(0, 0, 0, 0));
            row_mut.add_widget(&label_widget);

            if item_type.is_toggle() {
                // Toggle Button
                let toggle_btn = UIManager::create_widget(&format!("toggle_{:?}", item_type), UIWidgetTypes::Default);
                let toggle_ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                toggle_ui.set_halign(HorizontalAlign::CENTER);
                toggle_ui.set_valign(VerticalAlign::CENTER);
                toggle_ui.set_size(TOGGLE_BTN_WIDTH, STEPPER_BTN_SIZE);
                toggle_ui.set_margin(4.0);
                toggle_ui.set_text("OFF");
                toggle_ui.set_font_size(18.0);
                toggle_ui.set_font_color(get_color32(255, 255, 255, 255));
                toggle_ui.set_round(4.0);
                toggle_ui.set_color(COLOR_BTN_TOGGLE_OFF);
                toggle_ui.set_touchable(true);
                toggle_ui.set_callback_touch_down(Some(Box::new(Self::callback_toggle)));
                toggle_ui.set_callback_touch_over(Some(Box::new(Self::callback_btn_touch_over)));
                toggle_ui.set_user_data(user_data.as_ref() as *const OptionRowUserData<'a> as *const c_void);
                row_mut.add_widget(&toggle_btn);

                widget._rows.push(OptionRow {
                    _row_widget: row_widget,
                    _label_widget: label_widget,
                    _value_widget: None,
                    _dec_btn: None,
                    _inc_btn: None,
                    _toggle_btn: Some(toggle_btn),
                    _item_type: item_type,
                    _user_data: user_data,
                });
            } else {
                // Stepper: Dec, Value, Inc
                let dec_btn = UIManager::create_widget(&format!("dec_{:?}", item_type), UIWidgetTypes::Default);
                let dec_ui = ptr_as_mut(dec_btn.as_ref()).get_ui_component_mut();
                dec_ui.set_halign(HorizontalAlign::CENTER);
                dec_ui.set_valign(VerticalAlign::CENTER);
                dec_ui.set_size(STEPPER_BTN_SIZE, STEPPER_BTN_SIZE);
                dec_ui.set_margin(4.0);
                dec_ui.set_text("<");
                dec_ui.set_font_size(22.0);
                dec_ui.set_font_color(get_color32(255, 255, 255, 255));
                dec_ui.set_round(4.0);
                dec_ui.set_color(COLOR_BTN_STEPPER);
                dec_ui.set_touchable(true);
                dec_ui.set_callback_touch_down(Some(Box::new(Self::callback_dec)));
                dec_ui.set_callback_touch_over(Some(Box::new(Self::callback_btn_touch_over)));
                dec_ui.set_user_data(user_data.as_ref() as *const OptionRowUserData<'a> as *const c_void);
                row_mut.add_widget(&dec_btn);

                let value_widget = UIManager::create_widget(&format!("value_{:?}", item_type), UIWidgetTypes::Default);
                let val_ui = ptr_as_mut(value_widget.as_ref()).get_ui_component_mut();
                val_ui.set_halign(HorizontalAlign::CENTER);
                val_ui.set_valign(VerticalAlign::CENTER);
                val_ui.set_size(VALUE_WIDTH, ROW_HEIGHT - 8.0);
                val_ui.set_margin(4.0);
                val_ui.set_text("100%");
                val_ui.set_font_size(20.0);
                val_ui.set_font_color(COLOR_TEXT_VALUE);
                val_ui.set_color(get_color32(0, 0, 0, 0));
                row_mut.add_widget(&value_widget);

                let inc_btn = UIManager::create_widget(&format!("inc_{:?}", item_type), UIWidgetTypes::Default);
                let inc_ui = ptr_as_mut(inc_btn.as_ref()).get_ui_component_mut();
                inc_ui.set_halign(HorizontalAlign::CENTER);
                inc_ui.set_valign(VerticalAlign::CENTER);
                inc_ui.set_size(STEPPER_BTN_SIZE, STEPPER_BTN_SIZE);
                inc_ui.set_margin(4.0);
                inc_ui.set_text(">");
                inc_ui.set_font_size(22.0);
                inc_ui.set_font_color(get_color32(255, 255, 255, 255));
                inc_ui.set_round(4.0);
                inc_ui.set_color(COLOR_BTN_STEPPER);
                inc_ui.set_touchable(true);
                inc_ui.set_callback_touch_down(Some(Box::new(Self::callback_inc)));
                inc_ui.set_callback_touch_over(Some(Box::new(Self::callback_btn_touch_over)));
                inc_ui.set_user_data(user_data.as_ref() as *const OptionRowUserData<'a> as *const c_void);
                row_mut.add_widget(&inc_btn);

                widget._rows.push(OptionRow {
                    _row_widget: row_widget,
                    _label_widget: label_widget,
                    _value_widget: Some(value_widget),
                    _dec_btn: Some(dec_btn),
                    _inc_btn: Some(inc_btn),
                    _toggle_btn: None,
                    _item_type: item_type,
                    _user_data: user_data,
                });
            }
        }

        widget.refresh_all_values();
        widget.update_selection_visuals();
        widget
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_game_option_widget(&self) -> bool {
        self._is_opened
    }

    pub fn open_game_option_widget(&mut self) {
        if !self._is_opened {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._game_option_data = get_game_resources().get_game_option_data().borrow().clone();
            self.refresh_all_values();
            self.clear_status_message();
            self.update_selection_visuals();
            self._is_opened = true;
        }
    }

    pub fn close_game_option_widget(&mut self) {
        if self._is_opened {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self.clear_status_message();
            self._is_opened = false;
        }
    }

    pub fn set_selected_index(&mut self, index: usize, play_audio: bool) {
        if index < TOTAL_NAV_ITEMS && self._selected_index != index {
            self._selected_index = index;
            self.update_selection_visuals();
            if play_audio {
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
        }
    }

    fn update_selection_visuals(&mut self) {
        for (i, row) in self._rows.iter_mut().enumerate() {
            let is_selected = i == self._selected_index;
            let row_ui = ptr_as_mut(row._row_widget.as_ref()).get_ui_component_mut();
            row_ui.set_color(if is_selected {
                COLOR_ROW_BG_SELECTED
            } else {
                COLOR_ROW_BG_NORMAL
            });
            row_ui.set_border_color(if is_selected {
                COLOR_ROW_BORDER_SELECTED
            } else {
                COLOR_ROW_BORDER_NORMAL
            });
        }

        let is_reset_selected = self._selected_index == NAV_INDEX_RESET;
        let is_save_selected = self._selected_index == NAV_INDEX_SAVE;

        let reset_ui = ptr_as_mut(self._reset_btn.as_ref()).get_ui_component_mut();
        reset_ui.set_border_color(if is_reset_selected {
            COLOR_ROW_BORDER_SELECTED
        } else {
            get_color32(0, 0, 0, 0)
        });

        let save_ui = ptr_as_mut(self._save_btn.as_ref()).get_ui_component_mut();
        save_ui.set_border_color(if is_save_selected {
            COLOR_ROW_BORDER_SELECTED
        } else {
            get_color32(0, 0, 0, 0)
        });
    }

    pub fn decrement_option(&mut self, item_type: OptionItemType) {
        match item_type {
            OptionItemType::MasterVolume => {
                let val = (self._game_option_data._master_volume - VOLUME_STEP).max(MIN_VOLUME);
                self._game_option_data.set_master_volume(val);
            }
            OptionItemType::BgmVolume => {
                let val = (self._game_option_data._bgm_volume - VOLUME_STEP).max(MIN_VOLUME);
                self._game_option_data.set_bgm_volume(val);
            }
            OptionItemType::SfxVolume => {
                let val = (self._game_option_data._sfx_volume - VOLUME_STEP).max(MIN_VOLUME);
                self._game_option_data.set_sfx_volume(val);
            }
            OptionItemType::MuteAudio => {
                self._game_option_data._is_mute = !self._game_option_data._is_mute;
            }
            OptionItemType::MouseSensitivity => {
                let val =
                    (self._game_option_data._mouse_sensitivity - MOUSE_SENSITIVITY_STEP).max(MIN_MOUSE_SENSITIVITY);
                self._game_option_data.set_mouse_sensitivity(val);
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
            }
            OptionItemType::ShowDebugText => {
                self._game_option_data._show_debug_text = !self._game_option_data._show_debug_text;
            }
        }
        self._game_option_data.apply();
        self.refresh_all_values();
        self.clear_status_message();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
    }

    pub fn increment_option(&mut self, item_type: OptionItemType) {
        match item_type {
            OptionItemType::MasterVolume => {
                let val = (self._game_option_data._master_volume + VOLUME_STEP).min(MAX_VOLUME);
                self._game_option_data.set_master_volume(val);
            }
            OptionItemType::BgmVolume => {
                let val = (self._game_option_data._bgm_volume + VOLUME_STEP).min(MAX_VOLUME);
                self._game_option_data.set_bgm_volume(val);
            }
            OptionItemType::SfxVolume => {
                let val = (self._game_option_data._sfx_volume + VOLUME_STEP).min(MAX_VOLUME);
                self._game_option_data.set_sfx_volume(val);
            }
            OptionItemType::MuteAudio => {
                self._game_option_data._is_mute = !self._game_option_data._is_mute;
            }
            OptionItemType::MouseSensitivity => {
                let val =
                    (self._game_option_data._mouse_sensitivity + MOUSE_SENSITIVITY_STEP).min(MAX_MOUSE_SENSITIVITY);
                self._game_option_data.set_mouse_sensitivity(val);
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
            }
            OptionItemType::ShowDebugText => {
                self._game_option_data._show_debug_text = !self._game_option_data._show_debug_text;
            }
        }
        self._game_option_data.apply();
        self.refresh_all_values();
        self.clear_status_message();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
    }

    pub fn toggle_option(&mut self, item_type: OptionItemType) {
        match item_type {
            OptionItemType::MuteAudio => {
                self._game_option_data._is_mute = !self._game_option_data._is_mute;
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
            }
            OptionItemType::ShowDebugText => {
                self._game_option_data._show_debug_text = !self._game_option_data._show_debug_text;
            }
            _ => {
                self.increment_option(item_type);
                return;
            }
        }
        self._game_option_data.apply();
        self.refresh_all_values();
        self.clear_status_message();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
    }

    pub fn refresh_all_values(&mut self) {
        for row in self._rows.iter_mut() {
            match row._item_type {
                OptionItemType::MasterVolume => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.0}%", self._game_option_data._master_volume * 100.0);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
                OptionItemType::BgmVolume => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.0}%", self._game_option_data._bgm_volume * 100.0);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
                OptionItemType::SfxVolume => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.0}%", self._game_option_data._sfx_volume * 100.0);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
                OptionItemType::MuteAudio => {
                    if let Some(toggle_btn) = row._toggle_btn.as_ref() {
                        let is_on = self._game_option_data._is_mute;
                        let ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                        ui.set_text(if is_on { "ON" } else { "OFF" });
                        ui.set_color(if is_on {
                            COLOR_BTN_TOGGLE_ON
                        } else {
                            COLOR_BTN_TOGGLE_OFF
                        });
                    }
                }
                OptionItemType::MouseSensitivity => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.1}x", self._game_option_data._mouse_sensitivity);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
                OptionItemType::InvertMouseY => {
                    if let Some(toggle_btn) = row._toggle_btn.as_ref() {
                        let is_on = self._game_option_data._invert_mouse_y;
                        let ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                        ui.set_text(if is_on { "ON" } else { "OFF" });
                        ui.set_color(if is_on {
                            COLOR_BTN_TOGGLE_ON
                        } else {
                            COLOR_BTN_TOGGLE_OFF
                        });
                    }
                }
                OptionItemType::ShowDebugText => {
                    if let Some(toggle_btn) = row._toggle_btn.as_ref() {
                        let is_on = self._game_option_data._show_debug_text;
                        let ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                        ui.set_text(if is_on { "ON" } else { "OFF" });
                        ui.set_color(if is_on {
                            COLOR_BTN_TOGGLE_ON
                        } else {
                            COLOR_BTN_TOGGLE_OFF
                        });
                    }
                }
            }
        }
    }

    pub fn save_options(&mut self) {
        get_game_resources_mut().save_game_option_data(&self._game_option_data);
        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        status_ui.set_text("Options Saved Successfully!");
        status_ui.set_font_color(COLOR_STATUS_TEXT);
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
    }

    pub fn reset_to_defaults(&mut self) {
        self._game_option_data = GameOptionData::default();
        self._game_option_data.apply();
        self.refresh_all_values();
        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        status_ui.set_text("Reset to Default Settings.");
        status_ui.set_font_color(COLOR_STATUS_TEXT);
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
    }

    fn clear_status_message(&mut self) {
        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        status_ui.set_text("");
    }

    pub fn update_game_option_widget(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if UiInput::is_up(keyboard_input_data, joystick_input_data) {
            let next_index = if self._selected_index == 0 {
                TOTAL_NAV_ITEMS - 1
            } else {
                self._selected_index - 1
            };
            self.set_selected_index(next_index, true);
        } else if UiInput::is_down(keyboard_input_data, joystick_input_data) {
            let next_index = if self._selected_index + 1 >= TOTAL_NAV_ITEMS {
                0
            } else {
                self._selected_index + 1
            };
            self.set_selected_index(next_index, true);
        } else if UiInput::is_left(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.decrement_option(item_type);
                }
            } else if self._selected_index == NAV_INDEX_SAVE {
                self.set_selected_index(NAV_INDEX_RESET, true);
            }
        } else if UiInput::is_right(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.increment_option(item_type);
                }
            } else if self._selected_index == NAV_INDEX_RESET {
                self.set_selected_index(NAV_INDEX_SAVE, true);
            }
        } else if UiInput::is_confirm(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.toggle_option(item_type);
                }
            } else if self._selected_index == NAV_INDEX_RESET {
                self.reset_to_defaults();
            } else if self._selected_index == NAV_INDEX_SAVE {
                self.save_options();
            }
        }
    }
}
