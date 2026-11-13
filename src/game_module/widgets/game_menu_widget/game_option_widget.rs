use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_input::UiInput;
use crate::game_module::game_option::{
    CAMERA_ROTATION_SPEED_STEP, FONT_SCALE_STEP, FontType, GameOptionData, Language, MAX_CAMERA_ROTATION_SPEED,
    MAX_FONT_SCALE, MAX_UI_SCALE, MAX_VOLUME, MIN_CAMERA_ROTATION_SPEED, MIN_FONT_SCALE, MIN_UI_SCALE, MIN_VOLUME,
    UI_SCALE_STEP, VOLUME_STEP,
};
use crate::game_module::game_service_locator::{get_game_resources, get_game_resources_mut, get_game_ui_manager_mut};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::drop_down_combo_box_widget::DropDownComboBoxWidget;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::WindowMode;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_core};
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::renderer::renderer_data::RenderQualityLevel;
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

const ROW_WIDTH: f32 = 660.0;
const ROW_HEIGHT: f32 = 42.0;
const LABEL_WIDTH: f32 = 280.0;
const STEPPER_BTN_SIZE: f32 = 34.0;
const VALUE_WIDTH: f32 = 140.0;
const COMBO_BOX_WIDTH: f32 = 320.0;
const TOGGLE_BTN_WIDTH: f32 = 100.0;
const ACTION_BTN_WIDTH: f32 = 160.0;
const ACTION_BTN_HEIGHT: f32 = 38.0;
const ROWS_CONTAINER_HEIGHT: f32 = 350.0;

const TITLE_FONT_SIZE: f32 = 26.0;
const LABEL_FONT_SIZE: f32 = 22.0;
const VALUE_FONT_SIZE: f32 = 22.0;
const STEPPER_BTN_FONT_SIZE: f32 = 24.0;
const TOGGLE_BTN_FONT_SIZE: f32 = 20.0;
const ACTION_BTN_FONT_SIZE: f32 = 20.0;
const STATUS_FONT_SIZE: f32 = 18.0;

const LANGUAGE_ITEMS: [Language; 2] = [Language::English, Language::Korean];
const LANGUAGE_NAMES: [&str; 2] = ["English", "한글"];

const FONT_ITEMS: [FontType; 4] = [
    FontType::UbuntuSansMono,
    FontType::D2Coding,
    FontType::NanumGothicCoding,
    FontType::Pretendard,
];
const FONT_NAMES: [&str; 4] = ["Ubuntu Sans Mono", "D2Coding", "Nanum Gothic Coding", "Pretendard"];

const WINDOW_MODE_ITEMS: [WindowMode; 3] = [
    WindowMode::WindowMode,
    WindowMode::FullScreenExclusiveMode,
    WindowMode::FullScreenBorderlessMode,
];

const WINDOW_MODE_NAMES: [&str; 3] = ["Windowed", "Full Screen (Exclusive)", "Full Screen (Borderless)"];

const RENDER_QUALITY_LEVEL_ITEMS: [RenderQualityLevel; 3] = [
    RenderQualityLevel::Low,
    RenderQualityLevel::Medium,
    RenderQualityLevel::High,
];

const RENDER_QUALITY_LEVEL_NAMES: [&str; 3] = ["Low", "Medium", "High"];

const COLOR_ROW_BG_NORMAL: u32 = get_color32(245, 232, 205, 230);
const COLOR_ROW_BG_SELECTED: u32 = get_color32(255, 220, 145, 255);
const COLOR_ROW_BG_DISABLED: u32 = get_color32(225, 215, 200, 180);
const COLOR_ROW_BORDER_NORMAL: u32 = get_color32(200, 165, 125, 200);
const COLOR_ROW_BORDER_SELECTED: u32 = get_color32(255, 160, 40, 255);
const COLOR_ROW_BORDER_DISABLED: u32 = get_color32(185, 170, 150, 160);
const COLOR_TEXT_LABEL: u32 = get_color32(75, 45, 20, 255);
const COLOR_TEXT_LABEL_SELECTED: u32 = get_color32(35, 18, 5, 255);
const COLOR_TEXT_LABEL_DISABLED: u32 = get_color32(140, 120, 100, 180);
const COLOR_TEXT_VALUE: u32 = get_color32(50, 30, 10, 255);
const COLOR_TEXT_VALUE_SELECTED: u32 = get_color32(160, 60, 10, 255);

const COLOR_BTN_STEPPER: u32 = get_color32(195, 140, 85, 230);
const COLOR_BTN_STEPPER_SELECTED: u32 = get_color32(220, 145, 75, 255);
const COLOR_BTN_STEPPER_DISABLED: u32 = get_color32(180, 165, 145, 140);
const COLOR_BTN_TEXT_ENABLED: u32 = get_color32(255, 255, 255, 255);
const COLOR_BTN_TEXT_DISABLED: u32 = get_color32(220, 215, 205, 150);
const COLOR_BTN_TOGGLE_ON: u32 = get_color32(80, 150, 80, 240);
const COLOR_BTN_TOGGLE_OFF: u32 = get_color32(165, 130, 90, 230);

const COLOR_BTN_RESET: u32 = get_color32(185, 95, 65, 240);
const COLOR_BTN_RESET_FOCUSED: u32 = get_color32(235, 95, 65, 255);
const COLOR_BTN_RESET_BORDER: u32 = get_color32(145, 65, 40, 200);
const COLOR_BTN_BORDER_FOCUSED: u32 = get_color32(255, 240, 120, 255);
const COLOR_BTN_TEXT_FOCUSED: u32 = get_color32(255, 255, 220, 255);
const COLOR_STATUS_TEXT: u32 = get_color32(50, 120, 50, 255);

const ROW_BORDER_WIDTH: f32 = 2.0;
const ACTION_BTN_BORDER_WIDTH: f32 = 2.5;

pub const TOTAL_NAV_ITEMS: usize = OptionItemType::COUNT + 1;
pub const NAV_INDEX_RESET: usize = OptionItemType::COUNT;

pub fn is_resolution_selectable(window_mode: WindowMode) -> bool {
    window_mode == WindowMode::WindowMode
}

pub fn get_next_nav_index(
    current_index: usize,
    _dir_x: i32,
    dir_y: i32,
    is_nav_index_enabled: impl Fn(usize) -> bool,
) -> Option<usize> {
    if dir_y > 0 {
        if current_index >= OptionItemType::COUNT {
            // 하단 버튼(Reset)에서 아래로 더 가면 첫 번째 활성화된 옵션 아이템으로 순환
            for idx in 0..OptionItemType::COUNT {
                if is_nav_index_enabled(idx) {
                    return Some(idx);
                }
            }
            Some(NAV_INDEX_RESET)
        } else {
            // 옵션 목록 내에서 다음 활성화된 아이템 탐색
            for idx in (current_index + 1)..OptionItemType::COUNT {
                if is_nav_index_enabled(idx) {
                    return Some(idx);
                }
            }
            // 마지막 옵션 아이템에서 아래로 더 가면 Reset Defaults 버튼으로 이동
            if is_nav_index_enabled(NAV_INDEX_RESET) {
                Some(NAV_INDEX_RESET)
            } else {
                None
            }
        }
    } else if dir_y < 0 {
        if current_index >= OptionItemType::COUNT {
            // 하단 버튼(Reset)에서 위로 가면 마지막 활성화된 옵션 아이템으로 이동
            for idx in (0..OptionItemType::COUNT).rev() {
                if is_nav_index_enabled(idx) {
                    return Some(idx);
                }
            }
            Some(NAV_INDEX_RESET)
        } else {
            // 옵션 목록 내에서 이전 활성화된 아이템 탐색
            for idx in (0..current_index).rev() {
                if is_nav_index_enabled(idx) {
                    return Some(idx);
                }
            }
            // 최상단 옵션 아이템에서 위로 더 가면 Reset Defaults 버튼으로 순환
            if is_nav_index_enabled(NAV_INDEX_RESET) {
                Some(NAV_INDEX_RESET)
            } else {
                None
            }
        }
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, FromRepr, EnumCount, EnumIter, EnumString)]
#[repr(usize)]
pub enum OptionItemType {
    Language,
    Font,
    WindowMode,
    Monitor,
    Resolution,
    RenderQualityLevel,
    Vsync,
    MasterVolume,
    BgmVolume,
    SfxVolume,
    MuteAudio,
    CameraRotationSpeed,
    InvertMouseY,
    UiScale,
    FontScale,
}

impl OptionItemType {
    pub fn display_name(&self) -> &'static str {
        match self {
            OptionItemType::Language => "Language",
            OptionItemType::Font => "Font",
            OptionItemType::WindowMode => "Window Mode",
            OptionItemType::Monitor => "Display Monitor",
            OptionItemType::Resolution => "Resolution",
            OptionItemType::RenderQualityLevel => "Render Quality",
            OptionItemType::Vsync => "VSync",
            OptionItemType::MasterVolume => "Master Volume",
            OptionItemType::BgmVolume => "BGM Volume",
            OptionItemType::SfxVolume => "SFX Volume",
            OptionItemType::MuteAudio => "Mute All Audio",
            OptionItemType::CameraRotationSpeed => "Camera Rotation Speed",
            OptionItemType::InvertMouseY => "Invert Mouse Y-Axis",
            OptionItemType::UiScale => "UI Scale",
            OptionItemType::FontScale => "Font Scale",
        }
    }

    pub fn is_toggle(&self) -> bool {
        matches!(
            self,
            OptionItemType::Vsync | OptionItemType::MuteAudio | OptionItemType::InvertMouseY
        )
    }

    pub fn is_combo(&self) -> bool {
        matches!(
            self,
            OptionItemType::Language
                | OptionItemType::Font
                | OptionItemType::WindowMode
                | OptionItemType::Monitor
                | OptionItemType::Resolution
                | OptionItemType::RenderQualityLevel
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
    pub _combo_box: Option<Box<DropDownComboBoxWidget<'a>>>,
    pub _item_type: OptionItemType,
    pub _user_data: Box<OptionRowUserData<'a>>,
}

pub struct GameOptionWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _overlay_root: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _title_widget: Rc<WidgetDefault<'a>>,
    pub _rows_container: Rc<WidgetDefault<'a>>,
    pub _rows: Vec<OptionRow<'a>>,
    pub _reset_btn: Rc<WidgetDefault<'a>>,
    pub _status_label: Rc<WidgetDefault<'a>>,
    pub _selected_index: usize,
    pub _game_option_data: GameOptionData,
    pub _is_opened: bool,
}

impl<'a> GameOptionWidget<'a> {
    pub fn is_item_enabled(&self, item_type: OptionItemType) -> bool {
        match item_type {
            OptionItemType::Resolution => is_resolution_selectable(self._game_option_data._window_mode),
            _ => true,
        }
    }

    pub fn is_nav_index_enabled(&self, index: usize) -> bool {
        if let Some(item_type) = OptionItemType::from_repr(index) {
            self.is_item_enabled(item_type)
        } else {
            index < TOTAL_NAV_ITEMS
        }
    }

    pub fn callback_dec(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        if !widget.is_item_enabled(user_data._item_type) {
            return false;
        }
        widget.decrement_option(user_data._item_type);
        true
    }

    pub fn callback_inc(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        if !widget.is_item_enabled(user_data._item_type) {
            return false;
        }
        widget.increment_option(user_data._item_type);
        true
    }

    pub fn callback_toggle(ui_component: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        if !widget.is_item_enabled(user_data._item_type) {
            return false;
        }
        widget.toggle_option(user_data._item_type);
        true
    }

    pub fn callback_row_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        if !widget.is_item_enabled(user_data._item_type) {
            return false;
        }
        widget.set_selected_index(user_data._item_type as usize, false);
        true
    }

    pub fn callback_row_click(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let user_data = ptr_as_ref(ui_component.get_user_data() as *const OptionRowUserData<'a>);
        let widget = ptr_as_mut(user_data._widget_ptr);
        if !widget.is_item_enabled(user_data._item_type) {
            return false;
        }
        widget.set_selected_index(user_data._item_type as usize, true);
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
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let widget = ptr_as_mut(ui_component.get_user_data() as *const GameOptionWidget<'a>);
        widget.set_selected_index(NAV_INDEX_RESET, false);
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

    pub fn callback_language_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        if index < LANGUAGE_ITEMS.len() {
            widget._game_option_data.set_language(LANGUAGE_ITEMS[index]);
            widget._game_option_data.apply();
            widget.refresh_all_values();
            widget.clear_status_message();
        }
    }

    pub fn callback_font_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        if index < FONT_ITEMS.len() {
            widget._game_option_data.set_font(FONT_ITEMS[index]);
            widget._game_option_data.apply();
            widget.refresh_all_values();
            widget.clear_status_message();
        }
    }

    pub fn callback_window_mode_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        if index < WINDOW_MODE_ITEMS.len() {
            widget._game_option_data.set_window_mode(WINDOW_MODE_ITEMS[index]);
            widget._game_option_data.apply();
            widget.refresh_all_values();
            widget.clear_status_message();
        }
    }

    pub fn callback_monitor_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        widget._game_option_data.set_monitor_index(Some(index));
        widget.update_resolution_combo_items();
        widget._game_option_data.apply();
        widget.refresh_all_values();
        widget.clear_status_message();
    }

    pub fn callback_resolution_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        if !widget.is_item_enabled(OptionItemType::Resolution) {
            return;
        }
        let resolutions = get_engine_core().get_available_resolutions(widget._game_option_data._monitor_index);
        if let Some(res) = resolutions.get(index) {
            widget._game_option_data.set_resolution(*res);
            widget._game_option_data.apply();
            widget.refresh_all_values();
            widget.clear_status_message();
        }
    }

    pub fn callback_render_quality_changed(user_data: *const c_void, index: usize) {
        let widget = ptr_as_mut(user_data as *const GameOptionWidget<'a>);
        if index < RENDER_QUALITY_LEVEL_ITEMS.len() {
            widget._game_option_data.set_render_quality_level(RENDER_QUALITY_LEVEL_ITEMS[index]);
            widget._game_option_data.apply();
            widget.refresh_all_values();
            widget.clear_status_message();
        }
    }

    pub fn update_monitor_combo_items(&mut self) {
        let monitors = get_engine_core().get_monitor_names();
        let primary_idx = get_engine_core().get_primary_monitor_index();
        let max_idx = monitors.len().saturating_sub(1);
        let monitor_idx = self._game_option_data._monitor_index.unwrap_or(primary_idx).min(max_idx);
        if let Some(saved_idx) = self._game_option_data._monitor_index {
            if !monitors.is_empty() && saved_idx >= monitors.len() {
                self._game_option_data.set_monitor_index(Some(monitor_idx));
            }
        }
        if let Some(row) = self._rows.iter_mut().find(|r| r._item_type == OptionItemType::Monitor) {
            if let Some(combo) = row._combo_box.as_mut() {
                combo.set_items(monitors, monitor_idx);
            }
        }
    }

    pub fn update_resolution_combo_items(&mut self) {
        let resolutions = get_engine_core().get_available_resolutions(self._game_option_data._monitor_index);
        let res_names: Vec<String> = resolutions.iter().map(|r| format!("{} x {}", r.x, r.y)).collect();
        let current_res = self._game_option_data._resolution;
        let res_idx = resolutions.iter().position(|r| *r == current_res).unwrap_or(0);
        if let Some(res) = resolutions.get(res_idx) {
            self._game_option_data.set_resolution(*res);
        }
        if let Some(row) = self._rows.iter_mut().find(|r| r._item_type == OptionItemType::Resolution) {
            if let Some(combo) = row._combo_box.as_mut() {
                combo.set_items(res_names, res_idx);
            }
        }
    }

    pub fn create_game_option_widget(
        parent_widget: &mut WidgetDefault<'a>,
        overlay_root: *const WidgetDefault<'a>,
    ) -> Box<GameOptionWidget<'a>> {
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
        ui_comp.set_padding(6.0);
        ui_comp.set_color(get_color32(0, 0, 0, 0));

        // Title Header
        let title_widget = UIManager::create_widget("option_title", UIWidgetTypes::Default);
        let title_ui = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
        title_ui.set_halign(HorizontalAlign::CENTER);
        title_ui.set_valign(VerticalAlign::CENTER);
        title_ui.set_size_hint_x(Some(1.0));
        title_ui.set_size_y(30.0);
        title_ui.set_text("GAME SETTINGS");
        title_ui.set_font_size(TITLE_FONT_SIZE);
        title_ui.set_font_color(get_color32(110, 60, 25, 255));
        title_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&title_widget);

        // Rows Container with scroll_y enabled
        let rows_container = UIManager::create_widget("option_rows_container", UIWidgetTypes::Default);
        let rows_container_mut = ptr_as_mut(rows_container.as_ref());
        let rows_ui = rows_container_mut.get_ui_component_mut();
        rows_ui.set_layout_type(UILayoutType::BoxLayout);
        rows_ui.set_layout_orientation(Orientation::VERTICAL);
        rows_ui.set_halign(HorizontalAlign::CENTER);
        rows_ui.set_valign(VerticalAlign::TOP);
        rows_ui.set_expandable(false);
        rows_ui.set_scroll_y(true);
        rows_ui.set_enable_renderable_area(true);
        rows_ui.set_touchable(false);
        rows_ui.set_size_hint_x(Some(1.0));
        rows_ui.set_size_y(ROWS_CONTAINER_HEIGHT);
        rows_ui.set_margin(3.0);
        rows_ui.set_padding(4.0);
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
        footer_ui.set_size_y(46.0);
        footer_ui.set_margin(4.0);
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
        reset_ui.set_font_size(ACTION_BTN_FONT_SIZE);
        reset_ui.set_font_color(get_color32(255, 255, 255, 255));
        reset_ui.set_round(5.0);
        reset_ui.set_border(ACTION_BTN_BORDER_WIDTH);
        reset_ui.set_border_color(COLOR_BTN_RESET_BORDER);
        reset_ui.set_color(COLOR_BTN_RESET);
        reset_ui.set_touchable(true);
        reset_ui.set_callback_touch_down(Some(Box::new(Self::callback_reset_click)));
        reset_ui.set_callback_touch_over(Some(Box::new(Self::callback_reset_touch_over)));
        footer_mut.add_widget(&reset_btn);

        // Status Label
        let status_label = UIManager::create_widget("option_status_label", UIWidgetTypes::Default);
        let status_ui = ptr_as_mut(status_label.as_ref()).get_ui_component_mut();
        status_ui.set_halign(HorizontalAlign::CENTER);
        status_ui.set_valign(VerticalAlign::CENTER);
        status_ui.set_size_hint_x(Some(1.0));
        status_ui.set_size_y(24.0);
        status_ui.set_text("");
        status_ui.set_font_size(STATUS_FONT_SIZE);
        status_ui.set_font_color(COLOR_STATUS_TEXT);
        status_ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&status_label);

        let initial_option = get_game_resources().get_game_option_data().borrow().clone();

        let mut widget = Box::new(GameOptionWidget {
            _parent_widget: parent_widget,
            _overlay_root: overlay_root,
            _layer: layer,
            _title_widget: title_widget,
            _rows_container: rows_container,
            _rows: Vec::new(),
            _reset_btn: reset_btn,
            _status_label: status_label,
            _selected_index: 0,
            _game_option_data: initial_option.clone(),
            _is_opened: false,
        });

        let widget_ptr = widget.as_ref() as *const GameOptionWidget<'a>;
        ptr_as_mut(widget._reset_btn.as_ref()).get_ui_component_mut().set_user_data(widget_ptr as *const c_void);

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
            row_ui.set_border(ROW_BORDER_WIDTH);
            row_ui.set_border_color(COLOR_ROW_BORDER_NORMAL);
            row_ui.set_round(5.0);
            row_ui.set_touchable(true);
            row_ui.set_callback_touch_over(Some(Box::new(Self::callback_row_touch_over)));
            row_ui.set_callback_touch_down(Some(Box::new(Self::callback_row_click)));
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
            label_ui.set_font_size(LABEL_FONT_SIZE);
            label_ui.set_font_color(COLOR_TEXT_LABEL);
            label_ui.set_color(get_color32(0, 0, 0, 0));
            row_mut.add_widget(&label_widget);

            if item_type.is_combo() {
                let combo_box = match item_type {
                    OptionItemType::Language => {
                        let mut combo = DropDownComboBoxWidget::create_from_enum(
                            row_mut,
                            overlay_root,
                            "language_combo",
                            &LANGUAGE_ITEMS,
                            &LANGUAGE_NAMES,
                            initial_option._language,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_language_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    OptionItemType::Font => {
                        let mut combo = DropDownComboBoxWidget::create_from_enum(
                            row_mut,
                            overlay_root,
                            "font_combo",
                            &FONT_ITEMS,
                            &FONT_NAMES,
                            initial_option._font,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_font_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    OptionItemType::WindowMode => {
                        let mut combo = DropDownComboBoxWidget::create_from_enum(
                            row_mut,
                            overlay_root,
                            "window_mode_combo",
                            &WINDOW_MODE_ITEMS,
                            &WINDOW_MODE_NAMES,
                            initial_option._window_mode,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_window_mode_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    OptionItemType::Monitor => {
                        let monitors = get_engine_core().get_monitor_names();
                        let primary_idx = get_engine_core().get_primary_monitor_index();
                        let initial_idx = initial_option
                            ._monitor_index
                            .unwrap_or(primary_idx)
                            .min(if monitors.is_empty() { 0 } else { monitors.len() - 1 });
                        let mut combo = DropDownComboBoxWidget::create_drop_down_combo_box(
                            row_mut,
                            overlay_root,
                            "monitor_combo",
                            monitors,
                            initial_idx,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_monitor_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    OptionItemType::Resolution => {
                        let resolutions = get_engine_core().get_available_resolutions(initial_option._monitor_index);
                        let res_names: Vec<String> = resolutions.iter().map(|r| format!("{} x {}", r.x, r.y)).collect();
                        let initial_idx =
                            resolutions.iter().position(|r| *r == initial_option._resolution).unwrap_or(0);
                        let mut combo = DropDownComboBoxWidget::create_drop_down_combo_box(
                            row_mut,
                            overlay_root,
                            "resolution_combo",
                            res_names,
                            initial_idx,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_resolution_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    OptionItemType::RenderQualityLevel => {
                        let mut combo = DropDownComboBoxWidget::create_from_enum(
                            row_mut,
                            overlay_root,
                            "render_quality_combo",
                            &RENDER_QUALITY_LEVEL_ITEMS,
                            &RENDER_QUALITY_LEVEL_NAMES,
                            initial_option._render_quality_level,
                            COMBO_BOX_WIDTH,
                            STEPPER_BTN_SIZE,
                        );
                        combo.set_callback_selection_changed(Some((
                            Self::callback_render_quality_changed,
                            widget_ptr as *const c_void,
                        )));
                        combo
                    }
                    _ => unreachable!(),
                };

                widget._rows.push(OptionRow {
                    _row_widget: row_widget,
                    _label_widget: label_widget,
                    _value_widget: None,
                    _dec_btn: None,
                    _inc_btn: None,
                    _toggle_btn: None,
                    _combo_box: Some(combo_box),
                    _item_type: item_type,
                    _user_data: user_data,
                });
            } else if item_type.is_toggle() {
                // Toggle Button
                let toggle_btn = UIManager::create_widget(&format!("toggle_{:?}", item_type), UIWidgetTypes::Default);
                let toggle_ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                toggle_ui.set_halign(HorizontalAlign::CENTER);
                toggle_ui.set_valign(VerticalAlign::CENTER);
                toggle_ui.set_size(TOGGLE_BTN_WIDTH, STEPPER_BTN_SIZE);
                toggle_ui.set_margin(4.0);
                toggle_ui.set_text("OFF");
                toggle_ui.set_font_size(TOGGLE_BTN_FONT_SIZE);
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
                    _combo_box: None,
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
                dec_ui.set_font_size(STEPPER_BTN_FONT_SIZE);
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
                val_ui.set_font_size(VALUE_FONT_SIZE);
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
                inc_ui.set_font_size(STEPPER_BTN_FONT_SIZE);
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
                    _combo_box: None,
                    _item_type: item_type,
                    _user_data: user_data,
                });
            }
        }

        widget.refresh_all_values();
        widget.update_selection_visuals();
        widget
    }

    pub fn close_all_dropdowns(&mut self) {
        for row in self._rows.iter_mut() {
            if let Some(combo) = row._combo_box.as_mut() {
                combo.close_dropdown();
            }
        }
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
        self.close_all_dropdowns();
    }

    pub fn is_opened_game_option_widget(&self) -> bool {
        self._is_opened
    }

    pub fn open_game_option_widget(&mut self) {
        if !self._is_opened {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self._game_option_data = get_game_resources().get_game_option_data().borrow().clone();
            self.update_monitor_combo_items();
            self.update_resolution_combo_items();
            self.refresh_all_values();
            self.clear_status_message();
            self._selected_index = 0;
            ptr_as_mut(self._rows_container.as_ref()).get_ui_component_mut().set_scroll_pos_y(0.0);
            self.update_selection_visuals();
            self._is_opened = true;
            if let Some(first_row) = self._rows.first() {
                get_game_ui_manager_mut().request_mouse_snap_to_widget(&first_row._row_widget);
            }
        }
    }

    pub fn close_game_option_widget(&mut self) {
        if self._is_opened {
            get_game_ui_manager_mut().clear_pending_mouse_snap();
            self.close_all_dropdowns();
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self.clear_status_message();
            self._is_opened = false;
        }
    }

    pub fn set_selected_index(&mut self, index: usize, play_audio: bool) {
        if index < TOTAL_NAV_ITEMS && self.is_nav_index_enabled(index) && self._selected_index != index {
            self._selected_index = index;
            self.update_selection_visuals();
            if play_audio {
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
        }
    }

    fn move_mouse_to_selected_item(&self) {
        if self._selected_index < self._rows.len() {
            move_mouse_to_ui_component(self._rows[self._selected_index]._row_widget.get_ui_component());
        } else if self._selected_index == NAV_INDEX_RESET {
            move_mouse_to_ui_component(self._reset_btn.get_ui_component());
        }
    }

    fn update_selection_visuals(&mut self) {
        let res_enabled = is_resolution_selectable(self._game_option_data._window_mode);
        for (i, row) in self._rows.iter_mut().enumerate() {
            let is_enabled = match row._item_type {
                OptionItemType::Resolution => res_enabled,
                _ => true,
            };
            let is_selected = is_enabled && i == self._selected_index;
            let row_ui = ptr_as_mut(row._row_widget.as_ref()).get_ui_component_mut();
            row_ui.set_color(if !is_enabled {
                COLOR_ROW_BG_DISABLED
            } else if is_selected {
                COLOR_ROW_BG_SELECTED
            } else {
                COLOR_ROW_BG_NORMAL
            });
            row_ui.set_border_color(if !is_enabled {
                COLOR_ROW_BORDER_DISABLED
            } else if is_selected {
                COLOR_ROW_BORDER_SELECTED
            } else {
                COLOR_ROW_BORDER_NORMAL
            });

            let label_ui = ptr_as_mut(row._label_widget.as_ref()).get_ui_component_mut();
            label_ui.set_font_color(if !is_enabled {
                COLOR_TEXT_LABEL_DISABLED
            } else if is_selected {
                COLOR_TEXT_LABEL_SELECTED
            } else {
                COLOR_TEXT_LABEL
            });

            if let Some(val_widget) = row._value_widget.as_ref() {
                let val_ui = ptr_as_mut(val_widget.as_ref()).get_ui_component_mut();
                val_ui.set_font_color(if is_selected {
                    COLOR_TEXT_VALUE_SELECTED
                } else {
                    COLOR_TEXT_VALUE
                });
            }

            if is_enabled {
                let stepper_color = if is_selected {
                    COLOR_BTN_STEPPER_SELECTED
                } else {
                    COLOR_BTN_STEPPER
                };
                if let Some(dec_btn) = row._dec_btn.as_ref() {
                    ptr_as_mut(dec_btn.as_ref()).get_ui_component_mut().set_color(stepper_color);
                }
                if let Some(inc_btn) = row._inc_btn.as_ref() {
                    ptr_as_mut(inc_btn.as_ref()).get_ui_component_mut().set_color(stepper_color);
                }
            }
        }

        // Scroll into view if an option row is selected, or scroll to bottom if action buttons are selected
        if self._selected_index < self._rows.len() {
            let container_mut = ptr_as_mut(self._rows_container.as_ref());
            let target_row = &self._rows[self._selected_index]._row_widget;
            let target_row_ui = ptr_as_mut(target_row.as_ref()).get_ui_component_mut();
            container_mut.get_ui_component_mut().scroll_into_view(target_row_ui);
        } else if let Some(last_row) = self._rows.last() {
            let container_mut = ptr_as_mut(self._rows_container.as_ref());
            let target_row_ui = ptr_as_mut(last_row._row_widget.as_ref()).get_ui_component_mut();
            container_mut.get_ui_component_mut().scroll_into_view(target_row_ui);
        }

        let is_reset_selected = self._selected_index == NAV_INDEX_RESET;

        let reset_ui = ptr_as_mut(self._reset_btn.as_ref()).get_ui_component_mut();
        reset_ui.set_color(if is_reset_selected {
            COLOR_BTN_RESET_FOCUSED
        } else {
            COLOR_BTN_RESET
        });
        reset_ui.set_border_color(if is_reset_selected {
            COLOR_BTN_BORDER_FOCUSED
        } else {
            COLOR_BTN_RESET_BORDER
        });
        reset_ui.set_font_color(if is_reset_selected {
            COLOR_BTN_TEXT_FOCUSED
        } else {
            COLOR_BTN_TEXT_ENABLED
        });
    }

    pub fn decrement_option(&mut self, item_type: OptionItemType) {
        if !self.is_item_enabled(item_type) {
            return;
        }
        match item_type {
            OptionItemType::Language => {
                let current_idx =
                    LANGUAGE_ITEMS.iter().position(|&l| l == self._game_option_data._language).unwrap_or(0);
                let prev_idx = if current_idx == 0 {
                    LANGUAGE_ITEMS.len() - 1
                } else {
                    current_idx - 1
                };
                self._game_option_data.set_language(LANGUAGE_ITEMS[prev_idx]);
            }
            OptionItemType::Font => {
                let current_idx = FONT_ITEMS.iter().position(|&f| f == self._game_option_data._font).unwrap_or(0);
                let prev_idx = if current_idx == 0 {
                    FONT_ITEMS.len() - 1
                } else {
                    current_idx - 1
                };
                self._game_option_data.set_font(FONT_ITEMS[prev_idx]);
            }
            OptionItemType::WindowMode => {
                let current_idx =
                    WINDOW_MODE_ITEMS.iter().position(|&m| m == self._game_option_data._window_mode).unwrap_or(0);
                let prev_idx = if current_idx == 0 {
                    WINDOW_MODE_ITEMS.len() - 1
                } else {
                    current_idx - 1
                };
                self._game_option_data.set_window_mode(WINDOW_MODE_ITEMS[prev_idx]);
            }
            OptionItemType::Monitor => {
                let monitor_count = get_engine_core().get_monitor_names().len().max(1);
                let current_idx = self
                    ._game_option_data
                    ._monitor_index
                    .unwrap_or_else(|| get_engine_core().get_primary_monitor_index());
                let prev_idx = if current_idx == 0 {
                    monitor_count - 1
                } else {
                    (current_idx - 1).min(monitor_count - 1)
                };
                self._game_option_data.set_monitor_index(Some(prev_idx));
                self.update_resolution_combo_items();
            }
            OptionItemType::Resolution => {
                let resolutions = get_engine_core().get_available_resolutions(self._game_option_data._monitor_index);
                if !resolutions.is_empty() {
                    let current_idx =
                        resolutions.iter().position(|r| *r == self._game_option_data._resolution).unwrap_or(0);
                    let prev_idx = if current_idx == 0 {
                        resolutions.len() - 1
                    } else {
                        current_idx - 1
                    };
                    self._game_option_data.set_resolution(resolutions[prev_idx]);
                }
            }
            OptionItemType::RenderQualityLevel => {
                let current_idx = RENDER_QUALITY_LEVEL_ITEMS
                    .iter()
                    .position(|&q| q == self._game_option_data._render_quality_level)
                    .unwrap_or(0);
                let prev_idx = if current_idx == 0 {
                    RENDER_QUALITY_LEVEL_ITEMS.len() - 1
                } else {
                    current_idx - 1
                };
                self._game_option_data.set_render_quality_level(RENDER_QUALITY_LEVEL_ITEMS[prev_idx]);
            }
            OptionItemType::Vsync => {
                self._game_option_data.set_vsync(!self._game_option_data._vsync);
            }
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
            OptionItemType::CameraRotationSpeed => {
                let val = (self._game_option_data._camera_rotation_speed - CAMERA_ROTATION_SPEED_STEP)
                    .max(MIN_CAMERA_ROTATION_SPEED);
                self._game_option_data.set_camera_rotation_speed(val);
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
            }
            OptionItemType::UiScale => {
                let val = (self._game_option_data._ui_scale - UI_SCALE_STEP).max(MIN_UI_SCALE);
                self._game_option_data.set_ui_scale(val);
            }
            OptionItemType::FontScale => {
                let val = (self._game_option_data._font_scale - FONT_SCALE_STEP).max(MIN_FONT_SCALE);
                self._game_option_data.set_font_scale(val);
            }
        }
        self._game_option_data.apply();
        self.refresh_all_values();
        self.clear_status_message();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
    }

    pub fn increment_option(&mut self, item_type: OptionItemType) {
        if !self.is_item_enabled(item_type) {
            return;
        }
        match item_type {
            OptionItemType::Language => {
                let current_idx =
                    LANGUAGE_ITEMS.iter().position(|&l| l == self._game_option_data._language).unwrap_or(0);
                let next_idx = (current_idx + 1) % LANGUAGE_ITEMS.len();
                self._game_option_data.set_language(LANGUAGE_ITEMS[next_idx]);
            }
            OptionItemType::Font => {
                let current_idx = FONT_ITEMS.iter().position(|&f| f == self._game_option_data._font).unwrap_or(0);
                let next_idx = (current_idx + 1) % FONT_ITEMS.len();
                self._game_option_data.set_font(FONT_ITEMS[next_idx]);
            }
            OptionItemType::WindowMode => {
                let current_idx =
                    WINDOW_MODE_ITEMS.iter().position(|&m| m == self._game_option_data._window_mode).unwrap_or(0);
                let next_idx = (current_idx + 1) % WINDOW_MODE_ITEMS.len();
                self._game_option_data.set_window_mode(WINDOW_MODE_ITEMS[next_idx]);
            }
            OptionItemType::Monitor => {
                let monitor_count = get_engine_core().get_monitor_names().len().max(1);
                let current_idx = self
                    ._game_option_data
                    ._monitor_index
                    .unwrap_or_else(|| get_engine_core().get_primary_monitor_index());
                let next_idx = (current_idx + 1) % monitor_count;
                self._game_option_data.set_monitor_index(Some(next_idx));
                self.update_resolution_combo_items();
            }
            OptionItemType::Resolution => {
                let resolutions = get_engine_core().get_available_resolutions(self._game_option_data._monitor_index);
                if !resolutions.is_empty() {
                    let current_idx =
                        resolutions.iter().position(|r| *r == self._game_option_data._resolution).unwrap_or(0);
                    let next_idx = (current_idx + 1) % resolutions.len();
                    self._game_option_data.set_resolution(resolutions[next_idx]);
                }
            }
            OptionItemType::RenderQualityLevel => {
                let current_idx = RENDER_QUALITY_LEVEL_ITEMS
                    .iter()
                    .position(|&q| q == self._game_option_data._render_quality_level)
                    .unwrap_or(0);
                let next_idx = (current_idx + 1) % RENDER_QUALITY_LEVEL_ITEMS.len();
                self._game_option_data.set_render_quality_level(RENDER_QUALITY_LEVEL_ITEMS[next_idx]);
            }
            OptionItemType::Vsync => {
                self._game_option_data.set_vsync(!self._game_option_data._vsync);
            }
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
            OptionItemType::CameraRotationSpeed => {
                let val = (self._game_option_data._camera_rotation_speed + CAMERA_ROTATION_SPEED_STEP)
                    .min(MAX_CAMERA_ROTATION_SPEED);
                self._game_option_data.set_camera_rotation_speed(val);
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
            }
            OptionItemType::UiScale => {
                let val = (self._game_option_data._ui_scale + UI_SCALE_STEP).min(MAX_UI_SCALE);
                self._game_option_data.set_ui_scale(val);
            }
            OptionItemType::FontScale => {
                let val = (self._game_option_data._font_scale + FONT_SCALE_STEP).min(MAX_FONT_SCALE);
                self._game_option_data.set_font_scale(val);
            }
        }
        self._game_option_data.apply();
        self.refresh_all_values();
        self.clear_status_message();
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
    }

    pub fn toggle_option(&mut self, item_type: OptionItemType) {
        if !self.is_item_enabled(item_type) {
            return;
        }
        match item_type {
            OptionItemType::Language
            | OptionItemType::Font
            | OptionItemType::WindowMode
            | OptionItemType::Monitor
            | OptionItemType::Resolution
            | OptionItemType::RenderQualityLevel => {
                for row in self._rows.iter_mut() {
                    if let Some(combo) = row._combo_box.as_mut() {
                        if row._item_type == item_type {
                            combo.toggle_dropdown();
                        } else {
                            combo.close_dropdown();
                        }
                    }
                }
                return;
            }
            OptionItemType::Vsync => {
                self._game_option_data.set_vsync(!self._game_option_data._vsync);
            }
            OptionItemType::MuteAudio => {
                self._game_option_data._is_mute = !self._game_option_data._is_mute;
            }
            OptionItemType::InvertMouseY => {
                self._game_option_data._invert_mouse_y = !self._game_option_data._invert_mouse_y;
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
        let res_enabled = is_resolution_selectable(self._game_option_data._window_mode);
        for row in self._rows.iter_mut() {
            match row._item_type {
                OptionItemType::Language => {
                    if let Some(combo) = row._combo_box.as_mut() {
                        let current_idx =
                            LANGUAGE_ITEMS.iter().position(|&l| l == self._game_option_data._language).unwrap_or(0);
                        combo.set_selected_index(current_idx);
                    }
                }
                OptionItemType::Font => {
                    if let Some(combo) = row._combo_box.as_mut() {
                        let current_idx =
                            FONT_ITEMS.iter().position(|&f| f == self._game_option_data._font).unwrap_or(0);
                        combo.set_selected_index(current_idx);
                    }
                }
                OptionItemType::WindowMode => {
                    if let Some(combo) = row._combo_box.as_mut() {
                        let current_idx = WINDOW_MODE_ITEMS
                            .iter()
                            .position(|&m| m == self._game_option_data._window_mode)
                            .unwrap_or(0);
                        combo.set_selected_index(current_idx);
                    }
                }
                OptionItemType::Monitor => {
                    if let Some(combo) = row._combo_box.as_mut() {
                        let primary_idx = get_engine_core().get_primary_monitor_index();
                        let target_idx = self._game_option_data._monitor_index.unwrap_or(primary_idx);
                        combo.set_selected_index(target_idx);
                    }
                }
                OptionItemType::Resolution => {
                    ptr_as_mut(row._row_widget.as_ref()).get_ui_component_mut().set_touchable(res_enabled);
                    ptr_as_mut(row._label_widget.as_ref()).get_ui_component_mut().set_font_color(if res_enabled {
                        COLOR_TEXT_LABEL
                    } else {
                        COLOR_TEXT_LABEL_DISABLED
                    });
                    if let Some(dec_btn) = row._dec_btn.as_ref() {
                        let dec_ui = ptr_as_mut(dec_btn.as_ref()).get_ui_component_mut();
                        dec_ui.set_touchable(res_enabled);
                        dec_ui.set_color(if res_enabled {
                            COLOR_BTN_STEPPER
                        } else {
                            COLOR_BTN_STEPPER_DISABLED
                        });
                        dec_ui.set_font_color(if res_enabled {
                            COLOR_BTN_TEXT_ENABLED
                        } else {
                            COLOR_BTN_TEXT_DISABLED
                        });
                    }
                    if let Some(inc_btn) = row._inc_btn.as_ref() {
                        let inc_ui = ptr_as_mut(inc_btn.as_ref()).get_ui_component_mut();
                        inc_ui.set_touchable(res_enabled);
                        inc_ui.set_color(if res_enabled {
                            COLOR_BTN_STEPPER
                        } else {
                            COLOR_BTN_STEPPER_DISABLED
                        });
                        inc_ui.set_font_color(if res_enabled {
                            COLOR_BTN_TEXT_ENABLED
                        } else {
                            COLOR_BTN_TEXT_DISABLED
                        });
                    }
                    if let Some(combo) = row._combo_box.as_mut() {
                        combo.set_enabled(res_enabled);
                        let resolutions =
                            get_engine_core().get_available_resolutions(self._game_option_data._monitor_index);
                        let current_idx = if res_enabled {
                            resolutions.iter().position(|r| *r == self._game_option_data._resolution).unwrap_or(0)
                        } else {
                            0
                        };
                        combo.set_selected_index(current_idx);
                    }
                }
                OptionItemType::RenderQualityLevel => {
                    if let Some(combo) = row._combo_box.as_mut() {
                        let current_idx = RENDER_QUALITY_LEVEL_ITEMS
                            .iter()
                            .position(|&q| q == self._game_option_data._render_quality_level)
                            .unwrap_or(0);
                        combo.set_selected_index(current_idx);
                    }
                }
                OptionItemType::Vsync => {
                    if let Some(toggle_btn) = row._toggle_btn.as_ref() {
                        let is_on = self._game_option_data._vsync;
                        let ui = ptr_as_mut(toggle_btn.as_ref()).get_ui_component_mut();
                        ui.set_text(if is_on { "ON" } else { "OFF" });
                        ui.set_color(if is_on {
                            COLOR_BTN_TOGGLE_ON
                        } else {
                            COLOR_BTN_TOGGLE_OFF
                        });
                    }
                }
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
                OptionItemType::CameraRotationSpeed => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.1}x", self._game_option_data._camera_rotation_speed);
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
                OptionItemType::UiScale => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.0}%", self._game_option_data._ui_scale * 100.0);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
                OptionItemType::FontScale => {
                    if let Some(val_widget) = row._value_widget.as_ref() {
                        let text = format!("{:.0}%", self._game_option_data._font_scale * 100.0);
                        ptr_as_mut(val_widget.as_ref()).get_ui_component_mut().set_text(&text);
                    }
                }
            }
        }
        if !self.is_nav_index_enabled(self._selected_index) {
            self._selected_index = OptionItemType::Language as usize;
        }
        self.update_selection_visuals();
    }

    pub fn save_options(&mut self) {
        get_game_resources_mut().save_game_option_data(&self._game_option_data);
    }

    pub fn reset_to_defaults(&mut self) {
        self.close_all_dropdowns();
        self._game_option_data = GameOptionData::default();
        self.update_monitor_combo_items();
        self.update_resolution_combo_items();
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
    ) -> bool {
        // If any combo box dropdown is opened, handle modal dropdown input
        for row in self._rows.iter_mut() {
            if let Some(combo) = row._combo_box.as_mut() {
                if combo.is_opened() {
                    if UiInput::is_up(keyboard_input_data, joystick_input_data) {
                        combo.move_hover_prev();
                    } else if UiInput::is_down(keyboard_input_data, joystick_input_data) {
                        combo.move_hover_next();
                    } else if UiInput::is_confirm(keyboard_input_data, joystick_input_data) {
                        combo.confirm_hovered();
                    } else if UiInput::is_cancel(keyboard_input_data, joystick_input_data) {
                        combo.close_dropdown();
                    }
                    return true;
                }
            }
        }

        if UiInput::is_up(keyboard_input_data, joystick_input_data) {
            if let Some(next_idx) =
                get_next_nav_index(self._selected_index, 0, -1, |idx| self.is_nav_index_enabled(idx))
            {
                self.set_selected_index(next_idx, true);
                self.move_mouse_to_selected_item();
            }
        } else if UiInput::is_down(keyboard_input_data, joystick_input_data) {
            if let Some(next_idx) = get_next_nav_index(self._selected_index, 0, 1, |idx| self.is_nav_index_enabled(idx))
            {
                self.set_selected_index(next_idx, true);
                self.move_mouse_to_selected_item();
            }
        } else if UiInput::is_left(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.decrement_option(item_type);
                }
            } else if let Some(next_idx) =
                get_next_nav_index(self._selected_index, -1, 0, |idx| self.is_nav_index_enabled(idx))
            {
                self.set_selected_index(next_idx, true);
                self.move_mouse_to_selected_item();
            }
        } else if UiInput::is_right(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.increment_option(item_type);
                }
            } else if let Some(next_idx) =
                get_next_nav_index(self._selected_index, 1, 0, |idx| self.is_nav_index_enabled(idx))
            {
                self.set_selected_index(next_idx, true);
                self.move_mouse_to_selected_item();
            }
        } else if UiInput::is_confirm(keyboard_input_data, joystick_input_data) {
            if self._selected_index < OptionItemType::COUNT {
                if let Some(item_type) = OptionItemType::from_repr(self._selected_index) {
                    self.toggle_option(item_type);
                }
            } else if self._selected_index == NAV_INDEX_RESET {
                self.reset_to_defaults();
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_item_types() {
        assert_eq!(OptionItemType::from_repr(0), Some(OptionItemType::Language));
        assert_eq!(OptionItemType::Language.display_name(), "Language");
        assert!(OptionItemType::Language.is_combo());
        assert!(!OptionItemType::Language.is_toggle());

        assert_eq!(OptionItemType::from_repr(1), Some(OptionItemType::Font));
        assert_eq!(OptionItemType::Font.display_name(), "Font");
        assert!(OptionItemType::Font.is_combo());
        assert!(!OptionItemType::Font.is_toggle());

        assert_eq!(OptionItemType::from_repr(2), Some(OptionItemType::WindowMode));
        assert_eq!(OptionItemType::WindowMode.display_name(), "Window Mode");
        assert!(OptionItemType::WindowMode.is_combo());
        assert!(!OptionItemType::WindowMode.is_toggle());

        assert_eq!(OptionItemType::from_repr(3), Some(OptionItemType::Monitor));
        assert_eq!(OptionItemType::Monitor.display_name(), "Display Monitor");
        assert!(OptionItemType::Monitor.is_combo());
        assert!(!OptionItemType::Monitor.is_toggle());

        assert_eq!(OptionItemType::from_repr(4), Some(OptionItemType::Resolution));
        assert_eq!(OptionItemType::Resolution.display_name(), "Resolution");
        assert!(OptionItemType::Resolution.is_combo());
        assert!(!OptionItemType::Resolution.is_toggle());

        assert_eq!(OptionItemType::from_repr(5), Some(OptionItemType::RenderQualityLevel));
        assert_eq!(OptionItemType::RenderQualityLevel.display_name(), "Render Quality");
        assert!(OptionItemType::RenderQualityLevel.is_combo());
        assert!(!OptionItemType::RenderQualityLevel.is_toggle());

        assert_eq!(OptionItemType::from_repr(6), Some(OptionItemType::Vsync));
        assert_eq!(OptionItemType::Vsync.display_name(), "VSync");
        assert!(!OptionItemType::Vsync.is_combo());
        assert!(OptionItemType::Vsync.is_toggle());

        assert!(OptionItemType::MuteAudio.is_toggle());
        assert!(!OptionItemType::MuteAudio.is_combo());

        assert_eq!(OptionItemType::from_repr(13), Some(OptionItemType::UiScale));
        assert_eq!(OptionItemType::UiScale.display_name(), "UI Scale");
        assert!(!OptionItemType::UiScale.is_combo());
        assert!(!OptionItemType::UiScale.is_toggle());

        assert_eq!(OptionItemType::from_repr(14), Some(OptionItemType::FontScale));
        assert_eq!(OptionItemType::FontScale.display_name(), "Font Scale");
        assert!(!OptionItemType::FontScale.is_combo());
        assert!(!OptionItemType::FontScale.is_toggle());
    }

    #[test]
    fn test_font_items_consistency() {
        assert_eq!(FONT_ITEMS.len(), FONT_NAMES.len());
        assert_eq!(FONT_ITEMS.len(), 4);
        assert_eq!(FONT_NAMES[0], "Ubuntu Sans Mono");
        assert_eq!(FONT_NAMES[1], "D2Coding");
        assert_eq!(FONT_NAMES[2], "Nanum Gothic Coding");
        assert_eq!(FONT_NAMES[3], "Pretendard");

        assert_eq!(FontType::UbuntuSansMono.display_name(), "Ubuntu Sans Mono");
        assert_eq!(FontType::D2Coding.display_name(), "D2Coding");
        assert_eq!(FontType::NanumGothicCoding.display_name(), "Nanum Gothic Coding");
        assert_eq!(FontType::Pretendard.display_name(), "Pretendard");
    }

    #[test]
    fn test_language_items_consistency() {
        assert_eq!(LANGUAGE_ITEMS.len(), LANGUAGE_NAMES.len());
        assert_eq!(LANGUAGE_ITEMS.len(), 2);
        assert_eq!(LANGUAGE_NAMES[0], "English");
        assert_eq!(LANGUAGE_NAMES[1], "한글");

        assert_eq!(Language::English.display_name(), "English");
        assert_eq!(Language::Korean.display_name(), "한글");
    }

    #[test]
    fn test_render_quality_level_items_consistency() {
        assert_eq!(RENDER_QUALITY_LEVEL_ITEMS.len(), RENDER_QUALITY_LEVEL_NAMES.len());
        assert_eq!(RENDER_QUALITY_LEVEL_ITEMS.len(), 3);
        assert_eq!(RENDER_QUALITY_LEVEL_NAMES[0], "Low");
        assert_eq!(RENDER_QUALITY_LEVEL_NAMES[1], "Medium");
        assert_eq!(RENDER_QUALITY_LEVEL_NAMES[2], "High");

        assert_eq!(RenderQualityLevel::Low.display_name(), "Low");
        assert_eq!(RenderQualityLevel::Medium.display_name(), "Medium");
        assert_eq!(RenderQualityLevel::High.display_name(), "High");
    }

    #[test]
    fn test_window_mode_items_consistency() {
        assert_eq!(WINDOW_MODE_ITEMS.len(), WINDOW_MODE_NAMES.len());
        assert_eq!(WINDOW_MODE_ITEMS.len(), 3);
        assert_eq!(WINDOW_MODE_NAMES[0], "Windowed");
        assert_eq!(WINDOW_MODE_NAMES[1], "Full Screen (Exclusive)");
        assert_eq!(WINDOW_MODE_NAMES[2], "Full Screen (Borderless)");

        assert_eq!(WindowMode::WindowMode.display_name(), "Windowed");
        assert_eq!(
            WindowMode::FullScreenExclusiveMode.display_name(),
            "Full Screen (Exclusive)"
        );
        assert_eq!(
            WindowMode::FullScreenBorderlessMode.display_name(),
            "Full Screen (Borderless)"
        );

        assert!(is_resolution_selectable(WindowMode::WindowMode));
        assert!(!is_resolution_selectable(WindowMode::FullScreenExclusiveMode));
        assert!(!is_resolution_selectable(WindowMode::FullScreenBorderlessMode));
    }

    #[test]
    fn test_option_navigation_vertical_and_horizontal() {
        let is_all_enabled = |_idx: usize| true;

        // Down through all option items sequentially without skipping or premature cycling
        let last_option_idx = OptionItemType::COUNT - 1;
        let mut curr = 0;
        for expected_next in 1..=last_option_idx {
            let next = get_next_nav_index(curr, 0, 1, is_all_enabled);
            assert_eq!(next, Some(expected_next));
            curr = next.unwrap();
        }

        // Down from the last option item moves to Reset Default button
        assert_eq!(
            get_next_nav_index(last_option_idx, 0, 1, is_all_enabled),
            Some(NAV_INDEX_RESET)
        );

        // Down from Reset button wraps back to the first option item (0)
        assert_eq!(get_next_nav_index(NAV_INDEX_RESET, 0, 1, is_all_enabled), Some(0));

        // Up from Reset button moves up to the last option item
        assert_eq!(
            get_next_nav_index(NAV_INDEX_RESET, 0, -1, is_all_enabled),
            Some(last_option_idx)
        );

        // Up through all option items sequentially
        let mut curr_up = last_option_idx;
        for expected_prev in (0..last_option_idx).rev() {
            let prev = get_next_nav_index(curr_up, 0, -1, is_all_enabled);
            assert_eq!(prev, Some(expected_prev));
            curr_up = prev.unwrap();
        }

        // Up from the first option item wraps around to Reset Default button
        assert_eq!(get_next_nav_index(0, 0, -1, is_all_enabled), Some(NAV_INDEX_RESET));

        // Horizontal navigation returns None (values change instead or only 1 bottom button exists)
        assert_eq!(get_next_nav_index(NAV_INDEX_RESET, 1, 0, is_all_enabled), None);
        assert_eq!(get_next_nav_index(NAV_INDEX_RESET, -1, 0, is_all_enabled), None);
        assert_eq!(get_next_nav_index(0, 1, 0, is_all_enabled), None);
        assert_eq!(get_next_nav_index(0, -1, 0, is_all_enabled), None);

        // Navigation skips disabled option item
        let is_res_disabled = |idx: usize| idx != OptionItemType::Resolution as usize;
        let monitor_idx = OptionItemType::Monitor as usize;
        let quality_idx = OptionItemType::RenderQualityLevel as usize;
        assert_eq!(
            get_next_nav_index(monitor_idx, 0, 1, is_res_disabled),
            Some(quality_idx)
        );
        assert_eq!(
            get_next_nav_index(quality_idx, 0, -1, is_res_disabled),
            Some(monitor_idx)
        );
    }
}
