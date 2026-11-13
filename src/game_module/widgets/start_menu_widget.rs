use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_input::UiInput;
use crate::game_module::game_localization::{TextId, get_text};
use crate::game_module::game_option::{Language, get_global_language};
use crate::game_module::game_service_locator::{get_game_client_mut, get_game_ui_manager_mut};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::game_menu_widget::game_option_widget::GameOptionWidget;
use crate::game_module::widgets::game_menu_widget::save_load_widget::SaveLoadWidget;
use crate::game_module::widgets::popup_widget::PopupWindowWidget;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager,
    UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

pub const START_MENU_PANEL_WIDTH: f32 = 360.0;
pub const START_MENU_PANEL_HEIGHT: f32 = 310.0;
pub const START_MENU_BTN_WIDTH: f32 = 280.0;
pub const START_MENU_BTN_HEIGHT: f32 = 50.0;
pub const START_MENU_BTN_FONT_SIZE: f32 = 24.0;
pub const POPUP_FRAME_WIDTH: f32 = 760.0;
pub const POPUP_FRAME_HEIGHT: f32 = 580.0;
pub const POPUP_HEADER_HEIGHT: f32 = 45.0;
pub const POPUP_TITLE_WIDTH: f32 = 680.0;
pub const POPUP_TITLE_FONT_SIZE: f32 = 24.0;
pub const POPUP_CLOSE_BTN_SIZE: f32 = 35.0;
pub const POPUP_CLOSE_BTN_FONT_SIZE: f32 = 26.0;

const COLOR_MENU_PANEL_BG: u32 = get_color32(240, 222, 186, 248);
const COLOR_MENU_PANEL_BORDER: u32 = get_color32(135, 78, 42, 255);
const COLOR_POPUP_BACKDROP: u32 = get_color32(0, 0, 0, 0);
const COLOR_POPUP_FRAME_BG: u32 = get_color32(240, 222, 186, 248);
const COLOR_POPUP_FRAME_BORDER: u32 = get_color32(135, 78, 42, 255);
const COLOR_POPUP_HEADER_BG: u32 = get_color32(215, 185, 140, 230);
const COLOR_POPUP_TITLE_TEXT: u32 = get_color32(75, 45, 20, 255);
const COLOR_POPUP_CONTENT_BG: u32 = get_color32(252, 245, 226, 230);
const COLOR_BTN_CLOSE: u32 = get_color32(200, 75, 60, 255);
const COLOR_BTN_TEXT: u32 = get_color32(255, 255, 255, 255);

const COLOR_BTN_NORMAL: u32 = get_color32(245, 232, 205, 240);
const COLOR_BTN_BORDER_NORMAL: u32 = get_color32(188, 148, 105, 230);
const COLOR_BTN_TEXT_NORMAL: u32 = get_color32(75, 45, 20, 255);

const COLOR_BTN_FOCUSED: u32 = get_color32(255, 220, 145, 255);
const COLOR_BTN_BORDER_FOCUSED: u32 = get_color32(255, 160, 40, 255);
const COLOR_BTN_TEXT_FOCUSED: u32 = get_color32(35, 18, 5, 255);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartMenuButton {
    NewGame,
    LoadGame,
    Options,
    Exit,
}

impl StartMenuButton {
    pub fn next(self) -> Self {
        match self {
            StartMenuButton::NewGame => StartMenuButton::LoadGame,
            StartMenuButton::LoadGame => StartMenuButton::Options,
            StartMenuButton::Options => StartMenuButton::Exit,
            StartMenuButton::Exit => StartMenuButton::NewGame,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            StartMenuButton::NewGame => StartMenuButton::Exit,
            StartMenuButton::LoadGame => StartMenuButton::NewGame,
            StartMenuButton::Options => StartMenuButton::LoadGame,
            StartMenuButton::Exit => StartMenuButton::Options,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartMenuPopup {
    None,
    LoadGame,
    Options,
}

pub struct StartMenuWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _menu_panel: Rc<WidgetDefault<'a>>,
    pub _new_game_btn: Rc<WidgetDefault<'a>>,
    pub _load_game_btn: Rc<WidgetDefault<'a>>,
    pub _options_btn: Rc<WidgetDefault<'a>>,
    pub _exit_btn: Rc<WidgetDefault<'a>>,
    pub _load_game_popup_layer: Rc<WidgetDefault<'a>>,
    pub _load_game_popup_frame: Rc<WidgetDefault<'a>>,
    pub _load_game_close_btn: Rc<WidgetDefault<'a>>,
    pub _load_game_title: Rc<WidgetDefault<'a>>,
    pub _save_load_widget: Box<SaveLoadWidget<'a>>,
    pub _options_popup_layer: Rc<WidgetDefault<'a>>,
    pub _options_popup_frame: Rc<WidgetDefault<'a>>,
    pub _options_close_btn: Rc<WidgetDefault<'a>>,
    pub _options_title: Rc<WidgetDefault<'a>>,
    pub _game_option_widget: Box<GameOptionWidget<'a>>,
    pub _exit_confirm_popup: Option<RcRefCell<PopupWindowWidget<'a>>>,
    pub _is_opened: bool,
    pub _active_popup: StartMenuPopup,
    pub _selected_button: StartMenuButton,
    pub _current_language: Language,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> StartMenuWidget<'a> {
    fn create_menu_button(
        widget_name: &str,
        text: &str,
        callback_down: CallbackTouchEvent<'a>,
        callback_over: CallbackTouchEvent<'a>,
        parent_widget: &mut WidgetDefault<'a>,
    ) -> Rc<WidgetDefault<'a>> {
        let btn = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn.as_ref()).get_ui_component_mut();
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_size(START_MENU_BTN_WIDTH, START_MENU_BTN_HEIGHT);
        ui_component.set_margin(5.0);
        ui_component.set_text(text);
        ui_component.set_font_size(START_MENU_BTN_FONT_SIZE);
        ui_component.set_font_color(COLOR_BTN_TEXT_NORMAL);
        ui_component.set_round(6.0);
        ui_component.set_border(2.0);
        ui_component.set_border_color(COLOR_BTN_BORDER_NORMAL);
        ui_component.set_color(COLOR_BTN_NORMAL);
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(callback_down)));
        ui_component.set_callback_touch_over(Some(Box::new(callback_over)));
        parent_widget.add_widget(&btn);
        btn
    }

    pub fn callback_touch_frame_noop(
        _ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        true
    }

    pub fn callback_touch_down_new_game(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            start_menu.start_new_game();
        }
        true
    }

    pub fn callback_touch_over_new_game(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            if start_menu._selected_button != StartMenuButton::NewGame {
                start_menu.set_selected_button(StartMenuButton::NewGame, true);
            }
        }
        true
    }

    pub fn callback_touch_down_load_game(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            start_menu.open_load_game_popup();
        }
        true
    }

    pub fn callback_touch_over_load_game(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            if start_menu._selected_button != StartMenuButton::LoadGame {
                start_menu.set_selected_button(StartMenuButton::LoadGame, true);
            }
        }
        true
    }

    pub fn callback_touch_down_options(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            start_menu.open_options_popup();
        }
        true
    }

    pub fn callback_touch_over_options(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            if start_menu._selected_button != StartMenuButton::Options {
                start_menu.set_selected_button(StartMenuButton::Options, true);
            }
        }
        true
    }

    pub fn callback_touch_down_exit(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            start_menu.open_exit_confirm_popup();
        }
        true
    }

    pub fn callback_touch_over_exit(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        if start_menu._active_popup == StartMenuPopup::None && !start_menu.is_exit_popup_opened() {
            if start_menu._selected_button != StartMenuButton::Exit {
                start_menu.set_selected_button(StartMenuButton::Exit, true);
            }
        }
        true
    }

    pub fn callback_touch_down_close_load_game(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        start_menu.close_load_game_popup();
        true
    }

    pub fn callback_touch_down_close_options(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        start_menu.close_options_popup();
        true
    }

    pub fn callback_exit_confirm(
        _ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        get_game_client_mut().exit_game();
        true
    }

    pub fn callback_exit_cancel(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        let start_menu = ptr_as_mut(ui_component.get_user_data() as *const StartMenuWidget<'a>);
        start_menu.close_exit_confirm_popup();
        true
    }

    pub fn create_start_menu_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<StartMenuWidget<'a>> {
        let layer = UIManager::create_widget("start_menu_layer", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_comp = layer_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::FloatLayout);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_renderable(false);
        ui_comp.set_enable(false);
        parent_widget.add_widget(&layer);

        // Menu panel container placed in lower-center
        let menu_panel = UIManager::create_widget("start_menu_panel", UIWidgetTypes::Default);
        let menu_panel_mut = ptr_as_mut(menu_panel.as_ref());
        let ui_comp = menu_panel_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_pivot_preset(PIVOT_CENTER);
        ui_comp.set_pos_hint(Some(0.5), Some(0.68));
        ui_comp.set_size(START_MENU_PANEL_WIDTH, START_MENU_PANEL_HEIGHT);
        ui_comp.set_padding(14.0);
        ui_comp.set_color(COLOR_MENU_PANEL_BG);
        ui_comp.set_border_color(COLOR_MENU_PANEL_BORDER);
        ui_comp.set_border(3.0);
        ui_comp.set_round(10.0);
        layer_mut.add_widget(&menu_panel);

        let new_game_btn = Self::create_menu_button(
            "start_menu_new_game_btn",
            &get_text(TextId::StartMenuNewGame),
            Self::callback_touch_down_new_game,
            Self::callback_touch_over_new_game,
            menu_panel_mut,
        );

        let load_game_btn = Self::create_menu_button(
            "start_menu_load_game_btn",
            &get_text(TextId::StartMenuLoadGame),
            Self::callback_touch_down_load_game,
            Self::callback_touch_over_load_game,
            menu_panel_mut,
        );

        let options_btn = Self::create_menu_button(
            "start_menu_options_btn",
            &get_text(TextId::StartMenuOptions),
            Self::callback_touch_down_options,
            Self::callback_touch_over_options,
            menu_panel_mut,
        );

        let exit_btn = Self::create_menu_button(
            "start_menu_exit_btn",
            &get_text(TextId::StartMenuExit),
            Self::callback_touch_down_exit,
            Self::callback_touch_over_exit,
            menu_panel_mut,
        );

        // Load Game Popup Layer
        let load_game_popup_layer = UIManager::create_widget("load_game_popup_layer", UIWidgetTypes::Default);
        let load_popup_layer_mut = ptr_as_mut(load_game_popup_layer.as_ref());
        let ui_comp = load_popup_layer_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::FloatLayout);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_color(COLOR_POPUP_BACKDROP);
        ui_comp.set_enable(false);
        ui_comp.set_visible(false);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_close_load_game)));
        layer_mut.add_widget(&load_game_popup_layer);

        let load_game_popup_frame = UIManager::create_widget("load_game_popup_frame", UIWidgetTypes::Default);
        let load_frame_mut = ptr_as_mut(load_game_popup_frame.as_ref());
        let ui_comp = load_frame_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_pivot_preset(PIVOT_CENTER);
        ui_comp.set_pos_hint(Some(0.5), Some(0.5));
        ui_comp.set_size(POPUP_FRAME_WIDTH, POPUP_FRAME_HEIGHT);
        ui_comp.set_expandable(false);
        ui_comp.set_padding(10.0);
        ui_comp.set_color(COLOR_POPUP_FRAME_BG);
        ui_comp.set_border_color(COLOR_POPUP_FRAME_BORDER);
        ui_comp.set_border(2.0);
        ui_comp.set_round(5.0);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_frame_noop)));
        load_popup_layer_mut.add_widget(&load_game_popup_frame);

        let load_header = UIManager::create_widget("load_game_popup_header", UIWidgetTypes::Default);
        let load_header_mut = ptr_as_mut(load_header.as_ref());
        let ui_comp = load_header_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_y(POPUP_HEADER_HEIGHT);
        ui_comp.set_margin(5.0);
        ui_comp.set_padding(5.0);
        ui_comp.set_color(COLOR_POPUP_HEADER_BG);
        ui_comp.set_round(5.0);
        load_frame_mut.add_widget(&load_header);

        let load_game_title = UIManager::create_widget("load_game_popup_title", UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(load_game_title.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(POPUP_TITLE_WIDTH, POPUP_HEADER_HEIGHT - 10.0);
        ui_comp.set_text(&get_text(TextId::StartMenuLoadGame));
        ui_comp.set_font_size(POPUP_TITLE_FONT_SIZE);
        ui_comp.set_font_color(COLOR_POPUP_TITLE_TEXT);
        ui_comp.set_color(get_color32(0, 0, 0, 0));
        load_header_mut.add_widget(&load_game_title);

        let load_game_close_btn = UIManager::create_widget("load_game_close_btn", UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(load_game_close_btn.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(POPUP_CLOSE_BTN_SIZE, POPUP_CLOSE_BTN_SIZE);
        ui_comp.set_margin(5.0);
        ui_comp.set_text("X");
        ui_comp.set_font_size(POPUP_CLOSE_BTN_FONT_SIZE);
        ui_comp.set_font_color(COLOR_BTN_TEXT);
        ui_comp.set_round(5.0);
        ui_comp.set_color(COLOR_BTN_CLOSE);
        ui_comp.set_touchable(true);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_close_load_game)));
        load_header_mut.add_widget(&load_game_close_btn);

        let load_content = UIManager::create_widget("load_game_popup_content", UIWidgetTypes::Default);
        let load_content_mut = ptr_as_mut(load_content.as_ref());
        let ui_comp = load_content_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_margin(5.0);
        ui_comp.set_padding(5.0);
        ui_comp.set_color(COLOR_POPUP_CONTENT_BG);
        ui_comp.set_round(5.0);
        load_frame_mut.add_widget(&load_content);

        let save_load_widget = SaveLoadWidget::create_save_load_widget(load_content_mut, layer_mut);

        // Options Popup Layer
        let options_popup_layer = UIManager::create_widget("options_popup_layer", UIWidgetTypes::Default);
        let options_popup_layer_mut = ptr_as_mut(options_popup_layer.as_ref());
        let ui_comp = options_popup_layer_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::FloatLayout);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_color(COLOR_POPUP_BACKDROP);
        ui_comp.set_enable(false);
        ui_comp.set_visible(false);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_close_options)));
        layer_mut.add_widget(&options_popup_layer);

        let options_popup_frame = UIManager::create_widget("options_popup_frame", UIWidgetTypes::Default);
        let options_frame_mut = ptr_as_mut(options_popup_frame.as_ref());
        let ui_comp = options_frame_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::VERTICAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::TOP);
        ui_comp.set_pivot_preset(PIVOT_CENTER);
        ui_comp.set_pos_hint(Some(0.5), Some(0.5));
        ui_comp.set_size(POPUP_FRAME_WIDTH, POPUP_FRAME_HEIGHT);
        ui_comp.set_expandable(false);
        ui_comp.set_padding(10.0);
        ui_comp.set_color(COLOR_POPUP_FRAME_BG);
        ui_comp.set_border_color(COLOR_POPUP_FRAME_BORDER);
        ui_comp.set_border(2.0);
        ui_comp.set_round(5.0);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_frame_noop)));
        options_popup_layer_mut.add_widget(&options_popup_frame);

        let options_header = UIManager::create_widget("options_popup_header", UIWidgetTypes::Default);
        let options_header_mut = ptr_as_mut(options_header.as_ref());
        let ui_comp = options_header_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_y(POPUP_HEADER_HEIGHT);
        ui_comp.set_margin(5.0);
        ui_comp.set_padding(5.0);
        ui_comp.set_color(COLOR_POPUP_HEADER_BG);
        ui_comp.set_round(5.0);
        options_frame_mut.add_widget(&options_header);

        let options_title = UIManager::create_widget("options_popup_title", UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(options_title.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(POPUP_TITLE_WIDTH, POPUP_HEADER_HEIGHT - 10.0);
        ui_comp.set_text(&get_text(TextId::StartMenuOptions));
        ui_comp.set_font_size(POPUP_TITLE_FONT_SIZE);
        ui_comp.set_font_color(COLOR_POPUP_TITLE_TEXT);
        ui_comp.set_color(get_color32(0, 0, 0, 0));
        options_header_mut.add_widget(&options_title);

        let options_close_btn = UIManager::create_widget("options_close_btn", UIWidgetTypes::Default);
        let ui_comp = ptr_as_mut(options_close_btn.as_ref()).get_ui_component_mut();
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_size(POPUP_CLOSE_BTN_SIZE, POPUP_CLOSE_BTN_SIZE);
        ui_comp.set_margin(5.0);
        ui_comp.set_text("X");
        ui_comp.set_font_size(POPUP_CLOSE_BTN_FONT_SIZE);
        ui_comp.set_font_color(COLOR_BTN_TEXT);
        ui_comp.set_round(5.0);
        ui_comp.set_color(COLOR_BTN_CLOSE);
        ui_comp.set_touchable(true);
        ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_close_options)));
        options_header_mut.add_widget(&options_close_btn);

        let options_content = UIManager::create_widget("options_popup_content", UIWidgetTypes::Default);
        let options_content_mut = ptr_as_mut(options_content.as_ref());
        let ui_comp = options_content_mut.get_ui_component_mut();
        ui_comp.set_layout_type(UILayoutType::BoxLayout);
        ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
        ui_comp.set_halign(HorizontalAlign::CENTER);
        ui_comp.set_valign(VerticalAlign::CENTER);
        ui_comp.set_expandable(false);
        ui_comp.set_size_hint_x(Some(1.0));
        ui_comp.set_size_hint_y(Some(1.0));
        ui_comp.set_margin(5.0);
        ui_comp.set_padding(5.0);
        ui_comp.set_color(COLOR_POPUP_CONTENT_BG);
        ui_comp.set_round(5.0);
        options_frame_mut.add_widget(&options_content);

        let game_option_widget =
            GameOptionWidget::create_game_option_widget(options_content_mut, layer_mut as *const WidgetDefault<'a>);

        let mut start_menu = Box::new(StartMenuWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _menu_panel: menu_panel,
            _new_game_btn: new_game_btn,
            _load_game_btn: load_game_btn,
            _options_btn: options_btn,
            _exit_btn: exit_btn,
            _load_game_popup_layer: load_game_popup_layer,
            _load_game_popup_frame: load_game_popup_frame,
            _load_game_close_btn: load_game_close_btn,
            _load_game_title: load_game_title,
            _save_load_widget: save_load_widget,
            _options_popup_layer: options_popup_layer,
            _options_popup_frame: options_popup_frame,
            _options_close_btn: options_close_btn,
            _options_title: options_title,
            _game_option_widget: game_option_widget,
            _exit_confirm_popup: None,
            _is_opened: false,
            _active_popup: StartMenuPopup::None,
            _selected_button: StartMenuButton::NewGame,
            _current_language: get_global_language(),
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        });

        let ptr_self = start_menu.as_ref() as *const StartMenuWidget<'a> as *const c_void;
        ptr_as_mut(start_menu._new_game_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._load_game_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._options_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._exit_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._load_game_popup_layer.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._load_game_close_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._options_popup_layer.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(start_menu._options_close_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);

        // Exit confirmation popup
        start_menu._exit_confirm_popup = Some(PopupWindowWidget::create_popup_widget(
            layer_mut,
            &get_text(TextId::ExitConfirmPrompt),
            &get_text(TextId::PopupYes),
            Some(&get_text(TextId::PopupNo)),
            Some((StartMenuWidget::callback_exit_confirm, ptr_self)),
            Some((StartMenuWidget::callback_exit_cancel, ptr_self)),
        ));

        start_menu
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn is_exit_popup_opened(&self) -> bool {
        if let Some(popup) = self._exit_confirm_popup.as_ref() {
            if let Ok(guard) = popup.try_borrow() {
                return guard.is_opened();
            }
        }
        false
    }

    pub fn get_selected_button_widget(&self) -> &Rc<WidgetDefault<'a>> {
        match self._selected_button {
            StartMenuButton::NewGame => &self._new_game_btn,
            StartMenuButton::LoadGame => &self._load_game_btn,
            StartMenuButton::Options => &self._options_btn,
            StartMenuButton::Exit => &self._exit_btn,
        }
    }

    pub fn set_selected_button(&mut self, button: StartMenuButton, play_sound: bool) {
        if self._selected_button != button {
            self._selected_button = button;
            if play_sound {
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
            self.update_button_visuals();
        }
    }

    pub fn update_button_visuals(&mut self) {
        let is_menu_active = self._active_popup == StartMenuPopup::None && !self.is_exit_popup_opened();

        let set_btn = |btn: &Rc<WidgetDefault<'a>>, text_id: TextId, is_focused: bool| {
            let ui = ptr_as_mut(btn.as_ref()).get_ui_component_mut();
            let base_text = get_text(text_id);
            ui.set_text(&base_text);
            if is_focused && is_menu_active {
                ui.set_color(COLOR_BTN_FOCUSED);
                ui.set_border_color(COLOR_BTN_BORDER_FOCUSED);
                ui.set_border(3.0);
                ui.set_font_color(COLOR_BTN_TEXT_FOCUSED);
            } else {
                ui.set_color(COLOR_BTN_NORMAL);
                ui.set_border_color(COLOR_BTN_BORDER_NORMAL);
                ui.set_border(2.0);
                ui.set_font_color(COLOR_BTN_TEXT_NORMAL);
            }
        };

        set_btn(
            &self._new_game_btn,
            TextId::StartMenuNewGame,
            self._selected_button == StartMenuButton::NewGame,
        );
        set_btn(
            &self._load_game_btn,
            TextId::StartMenuLoadGame,
            self._selected_button == StartMenuButton::LoadGame,
        );
        set_btn(
            &self._options_btn,
            TextId::StartMenuOptions,
            self._selected_button == StartMenuButton::Options,
        );
        set_btn(
            &self._exit_btn,
            TextId::StartMenuExit,
            self._selected_button == StartMenuButton::Exit,
        );
    }

    pub fn refresh_exit_popup_texts(&mut self) {
        if let Some(popup) = self._exit_confirm_popup.as_ref() {
            if let Ok(guard) = popup.try_borrow() {
                ptr_as_mut(guard._text_widget.as_ref())
                    .get_ui_component_mut()
                    .set_text(&get_text(TextId::ExitConfirmPrompt));
                ptr_as_mut(guard._ok_btn.as_ref()).get_ui_component_mut().set_text(&get_text(TextId::PopupYes));
                if let Some(cancel_btn) = guard._cancel_btn.as_ref() {
                    ptr_as_mut(cancel_btn.as_ref()).get_ui_component_mut().set_text(&get_text(TextId::PopupNo));
                }
            }
        }
    }

    pub fn refresh_texts(&mut self) {
        self.update_button_visuals();
        ptr_as_mut(self._load_game_title.as_ref())
            .get_ui_component_mut()
            .set_text(&get_text(TextId::StartMenuLoadGame));
        ptr_as_mut(self._options_title.as_ref()).get_ui_component_mut().set_text(&get_text(TextId::StartMenuOptions));
        self.refresh_exit_popup_texts();
    }

    pub fn open_start_menu(&mut self) {
        let global_language = get_global_language();
        if self._current_language != global_language {
            self._current_language = global_language;
            self.refresh_texts();
        }
        if !self._is_opened {
            self._is_opened = true;
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            self.close_all_popups();
            self.set_selected_button(StartMenuButton::NewGame, false);
            self.update_button_visuals();
            let new_game_widget = self._new_game_btn.clone();
            get_game_ui_manager_mut().request_mouse_snap_to_widget(&new_game_widget);
        }
    }

    pub fn close_start_menu(&mut self) {
        if self._is_opened {
            get_game_ui_manager_mut().clear_pending_mouse_snap();
            self.close_all_popups();
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened = false;
        }
    }

    pub fn close_all_popups(&mut self) {
        if self._active_popup == StartMenuPopup::LoadGame {
            self._save_load_widget.close_save_load_widget();
            let ui_comp = ptr_as_mut(self._load_game_popup_layer.as_ref()).get_ui_component_mut();
            ui_comp.set_enable(false);
            ui_comp.set_visible(false);
            self._active_popup = StartMenuPopup::None;
        }
        if self._active_popup == StartMenuPopup::Options {
            self._game_option_widget.close_game_option_widget();
            let ui_comp = ptr_as_mut(self._options_popup_layer.as_ref()).get_ui_component_mut();
            ui_comp.set_enable(false);
            ui_comp.set_visible(false);
            self._active_popup = StartMenuPopup::None;
        }
        self.close_exit_confirm_popup();
    }

    pub fn open_load_game_popup(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self.close_all_popups();
        self._active_popup = StartMenuPopup::LoadGame;
        let ui_comp = ptr_as_mut(self._load_game_popup_layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(true);
        ui_comp.set_visible(true);
        self._save_load_widget.open_save_load_widget();
        self.update_button_visuals();
    }

    pub fn close_load_game_popup(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self._save_load_widget.close_save_load_widget();
        let ui_comp = ptr_as_mut(self._load_game_popup_layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(false);
        ui_comp.set_visible(false);
        self._active_popup = StartMenuPopup::None;
        self.update_button_visuals();
        let load_btn_widget = self._load_game_btn.clone();
        get_game_ui_manager_mut().request_mouse_snap_to_widget(&load_btn_widget);
    }

    pub fn open_options_popup(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self.close_all_popups();
        self._active_popup = StartMenuPopup::Options;
        let ui_comp = ptr_as_mut(self._options_popup_layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(true);
        ui_comp.set_visible(true);
        self._game_option_widget.open_game_option_widget();
        self.update_button_visuals();
    }

    pub fn close_options_popup(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self._game_option_widget.close_game_option_widget();
        let ui_comp = ptr_as_mut(self._options_popup_layer.as_ref()).get_ui_component_mut();
        ui_comp.set_enable(false);
        ui_comp.set_visible(false);
        self._active_popup = StartMenuPopup::None;
        self.refresh_texts();
        self.update_button_visuals();
        let options_btn_widget = self._options_btn.clone();
        get_game_ui_manager_mut().request_mouse_snap_to_widget(&options_btn_widget);
    }

    pub fn open_exit_confirm_popup(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        if let Some(popup) = self._exit_confirm_popup.as_ref() {
            let popup_clone = popup.clone();
            if let Ok(mut guard) = popup.try_borrow_mut() {
                guard.open(&popup_clone);
            }
        }
        self.update_button_visuals();
    }

    pub fn close_exit_confirm_popup(&mut self) {
        if let Some(popup) = self._exit_confirm_popup.as_ref() {
            if let Ok(mut guard) = popup.try_borrow_mut() {
                guard.close();
            }
        }
        self.update_button_visuals();
    }

    pub fn start_new_game(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        self.close_start_menu();
        get_game_client_mut().request_new_game();
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        if window_size.x <= 0 || window_size.y <= 0 {
            return;
        }
        self._save_load_widget.changed_window_size(window_size);
        self._game_option_widget.changed_window_size(window_size);
    }

    pub fn update(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
    ) {
        if !self._is_opened {
            return;
        }

        let global_language = get_global_language();
        if self._current_language != global_language {
            self._current_language = global_language;
            self.refresh_texts();
        }

        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if self.is_exit_popup_opened() {
            if should_move && let Some((dir_x, _dir_y)) = dir_opt {
                if let Some(popup) = self._exit_confirm_popup.as_ref() {
                    if let Ok(guard) = popup.try_borrow_mut() {
                        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                        if dir_x < 0 {
                            move_mouse_to_ui_component(guard._ok_btn.as_ref().get_ui_component());
                        } else if 0 < dir_x {
                            move_mouse_to_ui_component(guard._cancel_btn.as_ref().unwrap().as_ref().get_ui_component());
                        }
                    }
                }
            }
        } else if self._active_popup == StartMenuPopup::LoadGame {
            self._save_load_widget.update_save_load_widget(time_data, joystick_input_data, keyboard_input_data);
            if !self._save_load_widget.is_opened_save_load_widget() {
                self.close_load_game_popup();
            }
        } else if self._active_popup == StartMenuPopup::Options {
            let modal_consumed =
                self._game_option_widget.update_game_option_widget(joystick_input_data, keyboard_input_data);
            if !modal_consumed && UiInput::is_cancel(keyboard_input_data, joystick_input_data) {
                self.close_options_popup();
            }
        }
        else
        {
            if should_move && let Some((_dir_x, dir_y)) = dir_opt {
                if dir_y < 0 {
                    self.set_selected_button(self._selected_button.prev(), true);
                    move_mouse_to_ui_component(self.get_selected_button_widget().get_ui_component());
                } else if dir_y > 0 {
                    self.set_selected_button(self._selected_button.next(), true);
                    move_mouse_to_ui_component(self.get_selected_button_widget().get_ui_component());
                }
            }

            if UiInput::is_confirm(keyboard_input_data, joystick_input_data) {
                match self._selected_button {
                    StartMenuButton::NewGame => self.start_new_game(),
                    StartMenuButton::LoadGame => self.open_load_game_popup(),
                    StartMenuButton::Options => self.open_options_popup(),
                    StartMenuButton::Exit => self.open_exit_confirm_popup(),
                }
            } else if UiInput::is_cancel(keyboard_input_data, joystick_input_data) {
                self.open_exit_confirm_popup();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_menu_button_cycling() {
        assert_eq!(StartMenuButton::NewGame.next(), StartMenuButton::LoadGame);
        assert_eq!(StartMenuButton::LoadGame.next(), StartMenuButton::Options);
        assert_eq!(StartMenuButton::Options.next(), StartMenuButton::Exit);
        assert_eq!(StartMenuButton::Exit.next(), StartMenuButton::NewGame);

        assert_eq!(StartMenuButton::NewGame.prev(), StartMenuButton::Exit);
        assert_eq!(StartMenuButton::Exit.prev(), StartMenuButton::Options);
        assert_eq!(StartMenuButton::Options.prev(), StartMenuButton::LoadGame);
        assert_eq!(StartMenuButton::LoadGame.prev(), StartMenuButton::NewGame);
    }

    #[test]
    fn test_start_menu_localization_texts() {
        assert_eq!(
            TextId::StartMenuNewGame.get_default_text_by_language(Language::English),
            "New Game"
        );
        assert_eq!(
            TextId::StartMenuNewGame.get_default_text_by_language(Language::Korean),
            "새 게임"
        );
        assert_eq!(
            TextId::StartMenuLoadGame.get_default_text_by_language(Language::English),
            "Load Game"
        );
        assert_eq!(
            TextId::StartMenuLoadGame.get_default_text_by_language(Language::Korean),
            "불러오기"
        );
        assert_eq!(
            TextId::StartMenuOptions.get_default_text_by_language(Language::English),
            "Options"
        );
        assert_eq!(
            TextId::StartMenuOptions.get_default_text_by_language(Language::Korean),
            "옵션"
        );
        assert_eq!(
            TextId::StartMenuExit.get_default_text_by_language(Language::English),
            "Exit"
        );
        assert_eq!(
            TextId::StartMenuExit.get_default_text_by_language(Language::Korean),
            "종료"
        );
        assert_eq!(
            TextId::ExitConfirmPrompt.get_default_text_by_language(Language::English),
            "Are you sure you want to exit?"
        );
        assert_eq!(
            TextId::ExitConfirmPrompt.get_default_text_by_language(Language::Korean),
            "게임을 종료하시겠습니까?"
        );
        assert_eq!(TextId::PopupYes.get_default_text_by_language(Language::English), "Yes");
        assert_eq!(TextId::PopupYes.get_default_text_by_language(Language::Korean), "예");
        assert_eq!(TextId::PopupNo.get_default_text_by_language(Language::English), "No");
        assert_eq!(TextId::PopupNo.get_default_text_by_language(Language::Korean), "아니오");
    }

    #[test]
    fn test_start_menu_popup_variants() {
        let none = StartMenuPopup::None;
        let load = StartMenuPopup::LoadGame;
        let options = StartMenuPopup::Options;
        assert_ne!(none, load);
        assert_ne!(load, options);
        assert_ne!(none, options);
    }
}
