use crate::game_module::game_service_locator::get_game_scene_manager_mut;
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use winit::keyboard::KeyCode;
use crate::game_module::game_constants::AUDIO_SELECT_ITEM;

pub const WRAP_UP_POPUP_WIDTH: f32 = 320.0;
pub const WRAP_UP_POPUP_HEIGHT: f32 = 150.0;

pub const BUTTON_WIDTH: f32 = 140.0;
pub const BUTTON_HEIGHT: f32 = 48.0;

pub const COLOR_POPUP_BG: u32 = get_color32(28, 30, 38, 245);
pub const COLOR_POPUP_BORDER: u32 = get_color32(80, 85, 100, 255);
pub const COLOR_BTN_ACTIVE: u32 = get_color32(60, 120, 210, 255);
pub const COLOR_BTN_INACTIVE: u32 = get_color32(45, 48, 58, 255);
pub const COLOR_TEXT_ACTIVE: u32 = get_color32(255, 255, 255, 255);
pub const COLOR_TEXT_INACTIVE: u32 = get_color32(180, 185, 195, 255);

pub struct WrapUpPopupWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _dialog_panel: Rc<WidgetDefault<'a>>,
    pub _title_text: Rc<WidgetDefault<'a>>,
    pub _yes_btn: Rc<WidgetDefault<'a>>,
    pub _yes_text: Rc<WidgetDefault<'a>>,
    pub _no_btn: Rc<WidgetDefault<'a>>,
    pub _no_text: Rc<WidgetDefault<'a>>,
    pub _is_opened: bool,
    pub _selected_yes: bool,
}

impl<'a> WrapUpPopupWidget<'a> {
    pub fn callback_touch_down_yes(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const WrapUpPopupWidget<'a>);
        popup.confirm_yes();
        true
    }

    pub fn callback_touch_down_no(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const WrapUpPopupWidget<'a>);
        popup.confirm_no();
        true
    }

    pub fn callback_touch_over_yes(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const WrapUpPopupWidget<'a>);
        if !popup._selected_yes {
            popup._selected_yes = true;
            popup.refresh_button_styles();
        }
        true
    }

    pub fn callback_touch_over_no(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const WrapUpPopupWidget<'a>);
        if popup._selected_yes {
            popup._selected_yes = false;
            popup.refresh_button_styles();
        }
        true
    }

    pub fn confirm_yes(&mut self) {
        self.close_wrap_up_popup();
        get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
    }

    pub fn confirm_no(&mut self) {
        self.close_wrap_up_popup();
    }

    pub fn create_wrap_up_popup_widget(root_widget: &mut WidgetDefault<'a>) -> Box<WrapUpPopupWidget<'a>> {
        let layer = UIManager::create_widget("wrap_up_popup_layer", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui = layer_mut.get_ui_component_mut();
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_renderable(true);
        ui.set_visible(false);
        ui.set_enable(false);
        root_widget.add_widget(&layer);

        let dialog_panel = UIManager::create_widget("wrap_up_dialog_panel", UIWidgetTypes::Default);
        let dialog_panel_mut = ptr_as_mut(dialog_panel.as_ref());
        let ui = dialog_panel_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_size(WRAP_UP_POPUP_WIDTH, WRAP_UP_POPUP_HEIGHT);
        ui.set_color(COLOR_POPUP_BG);
        ui.set_border_color(COLOR_POPUP_BORDER);
        ui.set_border(3.0);
        ui.set_round(12.0);
        ui.set_padding(20.0);
        ui.set_renderable(true);
        layer_mut.add_widget(&dialog_panel);

        let title_text = UIManager::create_widget("wrap_up_title_text", UIWidgetTypes::Default);
        let title_text_mut = ptr_as_mut(title_text.as_ref());
        let ui = title_text_mut.get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(60.0);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(24.0);
        ui.set_font_color(get_color32(240, 240, 245, 255));
        ui.set_text("Do you want to wrap up the day?");
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_renderable(true);
        dialog_panel_mut.add_widget(&title_text);

        let spacer = UIManager::create_widget("wrap_up_spacer", UIWidgetTypes::Default);
        let spacer_mut = ptr_as_mut(spacer.as_ref());
        let ui = spacer_mut.get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(20.0);
        ui.set_color(get_color32(0, 0, 0, 0));
        dialog_panel_mut.add_widget(&spacer);

        let btn_container = UIManager::create_widget("wrap_up_btn_container", UIWidgetTypes::Default);
        let btn_container_mut = ptr_as_mut(btn_container.as_ref());
        let ui = btn_container_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(60.0);
        ui.set_color(get_color32(0, 0, 0, 0));
        dialog_panel_mut.add_widget(&btn_container);

        let yes_btn = UIManager::create_widget("wrap_up_yes_btn", UIWidgetTypes::Default);
        let yes_btn_mut = ptr_as_mut(yes_btn.as_ref());
        let ui = yes_btn_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size(BUTTON_WIDTH, BUTTON_HEIGHT);
        ui.set_color(COLOR_BTN_ACTIVE);
        ui.set_round(8.0);
        ui.set_margin_right(20.0);
        ui.set_renderable(true);
        ui.set_touchable(true);
        btn_container_mut.add_widget(&yes_btn);

        let yes_text = UIManager::create_widget("wrap_up_yes_text", UIWidgetTypes::Default);
        let yes_text_mut = ptr_as_mut(yes_text.as_ref());
        let ui = yes_text_mut.get_ui_component_mut();
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(20.0);
        ui.set_font_color(COLOR_TEXT_ACTIVE);
        ui.set_text("Yes");
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_renderable(true);
        yes_btn_mut.add_widget(&yes_text);

        let no_btn = UIManager::create_widget("wrap_up_no_btn", UIWidgetTypes::Default);
        let no_btn_mut = ptr_as_mut(no_btn.as_ref());
        let ui = no_btn_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size(BUTTON_WIDTH, BUTTON_HEIGHT);
        ui.set_color(COLOR_BTN_INACTIVE);
        ui.set_round(8.0);
        ui.set_renderable(true);
        ui.set_touchable(true);
        btn_container_mut.add_widget(&no_btn);

        let no_text = UIManager::create_widget("wrap_up_no_text", UIWidgetTypes::Default);
        let no_text_mut = ptr_as_mut(no_text.as_ref());
        let ui = no_text_mut.get_ui_component_mut();
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(20.0);
        ui.set_font_color(COLOR_TEXT_INACTIVE);
        ui.set_text("No");
        ui.set_color(get_color32(0, 0, 0, 0));
        ui.set_renderable(true);
        no_btn_mut.add_widget(&no_text);

        let mut widget = Box::new(WrapUpPopupWidget {
            _parent_widget: root_widget as *const WidgetDefault,
            _layer: layer,
            _dialog_panel: dialog_panel,
            _title_text: title_text,
            _yes_btn: yes_btn,
            _yes_text: yes_text,
            _no_btn: no_btn,
            _no_text: no_text,
            _is_opened: false,
            _selected_yes: true,
        });

        let widget_ptr = widget.as_mut() as *mut WrapUpPopupWidget as *const c_void;
        ptr_as_mut(widget._yes_btn.as_ref()).get_ui_component_mut().set_user_data(widget_ptr);
        ptr_as_mut(widget._yes_btn.as_ref())
            .get_ui_component_mut()
            .set_callback_touch_down(Some(Box::new(Self::callback_touch_down_yes)));
        ptr_as_mut(widget._yes_btn.as_ref())
            .get_ui_component_mut()
            .set_callback_touch_over(Some(Box::new(Self::callback_touch_over_yes)));

        ptr_as_mut(widget._no_btn.as_ref()).get_ui_component_mut().set_user_data(widget_ptr);
        ptr_as_mut(widget._no_btn.as_ref())
            .get_ui_component_mut()
            .set_callback_touch_down(Some(Box::new(Self::callback_touch_down_no)));
        ptr_as_mut(widget._no_btn.as_ref())
            .get_ui_component_mut()
            .set_callback_touch_over(Some(Box::new(Self::callback_touch_over_no)));

        widget
    }

    pub fn open_wrap_up_popup(&mut self) {
        self._is_opened = true;
        self._selected_yes = true;
        self.refresh_button_styles();
        let ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui.set_visible(true);
        ui.set_enable(true);
    }

    pub fn close_wrap_up_popup(&mut self) {
        self._is_opened = false;
        let ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui.set_visible(false);
        ui.set_enable(false);
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn refresh_button_styles(&mut self) {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);

        let yes_btn_ui = ptr_as_mut(self._yes_btn.as_ref()).get_ui_component_mut();
        let yes_text_ui = ptr_as_mut(self._yes_text.as_ref()).get_ui_component_mut();
        let no_btn_ui = ptr_as_mut(self._no_btn.as_ref()).get_ui_component_mut();
        let no_text_ui = ptr_as_mut(self._no_text.as_ref()).get_ui_component_mut();

        if self._selected_yes {
            yes_btn_ui.set_color(COLOR_BTN_ACTIVE);
            yes_text_ui.set_font_color(COLOR_TEXT_ACTIVE);
            no_btn_ui.set_color(COLOR_BTN_INACTIVE);
            no_text_ui.set_font_color(COLOR_TEXT_INACTIVE);
        } else {
            yes_btn_ui.set_color(COLOR_BTN_INACTIVE);
            yes_text_ui.set_font_color(COLOR_TEXT_INACTIVE);
            no_btn_ui.set_color(COLOR_BTN_ACTIVE);
            no_text_ui.set_font_color(COLOR_TEXT_ACTIVE);
        }
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui.set_size(window_size.x as f32, window_size.y as f32);
    }

    pub fn update_wrap_up_popup(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if !self._is_opened {
            return;
        }

        let nav_left = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || joystick_input_data._btn_left == ButtonState::Pressed;
        let nav_right = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || joystick_input_data._btn_right == ButtonState::Pressed;

        if nav_left && !self._selected_yes {
            self._selected_yes = true;
            self.refresh_button_styles();
        } else if nav_right && self._selected_yes {
            self._selected_yes = false;
            self.refresh_button_styles();
        }

        let confirm = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed;

        let cancel = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_b == ButtonState::Pressed;

        if confirm {
            if self._selected_yes {
                self.confirm_yes();
            } else {
                self.confirm_no();
            }
        } else if cancel {
            self.confirm_no();
        }
    }
}
