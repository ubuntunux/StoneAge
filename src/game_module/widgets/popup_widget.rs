use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub struct PopupWindowWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _ok_btn: Rc<WidgetDefault<'a>>,
    pub _cancel_btn: Option<Rc<WidgetDefault<'a>>>,
    pub _is_opened: bool,
}

impl<'a> PopupWindowWidget<'a> {
    pub fn create_popup_widget(
        parent_widget: &mut WidgetDefault<'a>,
        popup_text: &str,
        ok_button_text: &str,
        cancel_button_text: Option<&str>,
        ok_callback: Option<CallbackTouchEvent<'a>>,
        cancel_callback: Option<CallbackTouchEvent<'a>>,
    ) -> Box<PopupWindowWidget<'a>> {
        let popup_layer = UIManager::create_widget("popup_window_layer", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(popup_layer.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_pivot_preset(PIVOT_CENTER);
            ui_comp.set_pos_hint(Some(0.5), Some(0.5));
            ui_comp.set_pos_hint(Some(0.5), Some(0.5));
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_hint_y(Some(1.0));
            ui_comp.set_enable(false);
            ui_comp.set_renderable(false);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(ok_callback.map(Box::new));
        }

        let popup_layer_frame = UIManager::create_widget("popup_window_frame", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(popup_layer_frame.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_pivot_preset(PIVOT_CENTER);
            ui_comp.set_pos_hint(Some(0.5), Some(0.5));
            ui_comp.set_size(440.0, 210.0);
            ui_comp.set_padding(16.0);
            ui_comp.set_color(get_color32(20, 26, 36, 250));
            ui_comp.set_border_color(get_color32(70, 140, 210, 255));
            ui_comp.set_border(2.0);
            ui_comp.set_round(10.0);
            ui_comp.set_ignore_parent_renderable_area(true);
        }
        ptr_as_mut(popup_layer.as_ref()).add_widget(&popup_layer_frame);

        let text_widget = UIManager::create_widget("popup_window_text", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(text_widget.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(400.0, 60.0);
            ui_comp.set_margin(10.0);
            ui_comp.set_text(popup_text);
            ui_comp.set_font_size(24.0);
            ui_comp.set_font_color(get_color32(240, 245, 255, 255));
            ui_comp.set_color(get_color32(0, 0, 0, 0));
        }
        ptr_as_mut(popup_layer_frame.as_ref()).add_widget(&text_widget);

        let btn_container = UIManager::create_widget("popup_btn_container", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(btn_container.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size_hint_x(Some(1.0));
            ui_comp.set_size_y(50.0);
            ui_comp.set_color(get_color32(0, 0, 0, 0));
        }
        ptr_as_mut(popup_layer_frame.as_ref()).add_widget(&btn_container);

        let is_two_button_style = cancel_button_text.is_some();

        let ok_btn = UIManager::create_widget("popup_ok_btn", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(ok_btn.as_ref()).get_ui_component_mut();
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_size(if is_two_button_style { 140.0 } else { 160.0 }, 42.0);
            ui_comp.set_margin(8.0);
            ui_comp.set_text(ok_button_text);
            ui_comp.set_font_size(22.0);
            ui_comp.set_font_color(get_color32(255, 255, 255, 255));
            ui_comp.set_color(get_color32(40, 130, 190, 255));
            ui_comp.set_round(6.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(ok_callback.map(Box::new));
        }
        ptr_as_mut(btn_container.as_ref()).add_widget(&ok_btn);

        let cancel_btn = if let Some(cancel_text) = cancel_button_text {
            let cancel_btn_widget = UIManager::create_widget("popup_cancel_btn", UIWidgetTypes::Default);
            {
                let ui_comp = ptr_as_mut(cancel_btn_widget.as_ref()).get_ui_component_mut();
                ui_comp.set_halign(HorizontalAlign::CENTER);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_size(140.0, 42.0);
                ui_comp.set_margin(8.0);
                ui_comp.set_text(cancel_text);
                ui_comp.set_font_size(22.0);
                ui_comp.set_font_color(get_color32(255, 255, 255, 255));
                ui_comp.set_color(get_color32(100, 105, 115, 255));
                ui_comp.set_round(6.0);
                ui_comp.set_touchable(true);
                ui_comp.set_callback_touch_down(cancel_callback.map(Box::new));
            }
            ptr_as_mut(btn_container.as_ref()).add_widget(&cancel_btn_widget);
            Some(cancel_btn_widget)
        } else {
            None
        };

        Box::new(PopupWindowWidget {
            _parent_widget: parent_widget,
            _layer: popup_layer,
            _text_widget: text_widget,
            _ok_btn: ok_btn,
            _cancel_btn: cancel_btn,
            _is_opened: false,
        })
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn open(&mut self) {
        if !self._is_opened {
            self._is_opened = true;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
        }
    }

    pub fn close(&mut self) {
        if self._is_opened {
            self._is_opened = false;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
    }

    pub fn update(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) -> bool {
        if !self._is_opened {
            return false;
        }

        let close_popup = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || joystick_input_data._btn_a == ButtonState::Pressed
            || joystick_input_data._btn_b == ButtonState::Pressed
            || joystick_input_data._btn_x == ButtonState::Pressed;

        if close_popup {
            self.close();
        }
        true
    }
}
