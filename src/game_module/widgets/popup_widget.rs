use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_input::UiInput;
use crate::game_module::game_service_locator::get_game_ui_manager_mut;
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager,
    UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

const BTN_OK_COLOR: u32 = get_color32(118, 168, 68, 240);
const BTN_OK_FOCUSED_COLOR: u32 = get_color32(150, 210, 85, 255);
const BTN_OK_BORDER_COLOR: u32 = get_color32(72, 115, 38, 255);

const BTN_CANCEL_COLOR: u32 = get_color32(185, 95, 65, 240);
const BTN_CANCEL_FOCUSED_COLOR: u32 = get_color32(230, 115, 85, 255);
const BTN_CANCEL_BORDER_COLOR: u32 = get_color32(145, 65, 40, 200);

const BTN_FOCUSED_BORDER_COLOR: u32 = get_color32(255, 160, 40, 255);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupButtonFocus {
    Ok,
    Cancel,
}

impl PopupButtonFocus {
    pub fn from_horizontal_input(self, dir_x: i32, has_cancel: bool) -> Self {
        if !has_cancel {
            return PopupButtonFocus::Ok;
        }
        if dir_x < 0 {
            PopupButtonFocus::Ok
        } else if dir_x > 0 {
            PopupButtonFocus::Cancel
        } else {
            self
        }
    }
}

pub struct PopupWindowWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _text_widget: Rc<WidgetDefault<'a>>,
    pub _ok_btn: Rc<WidgetDefault<'a>>,
    pub _cancel_btn: Option<Rc<WidgetDefault<'a>>>,
    pub _ok_callback: Option<(CallbackTouchEvent<'a>, *const c_void)>,
    pub _cancel_callback: Option<(CallbackTouchEvent<'a>, *const c_void)>,
    pub _focused_button: PopupButtonFocus,
    pub _is_opened: bool,
    pub _just_opened: bool,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> PopupWindowWidget<'a> {
    pub fn callback_noop_frame(
        _ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        true
    }

    pub fn callback_touch_down_backdrop(
        ui_component: &UIComponentInstance<'a>,
        pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const PopupWindowWidget<'a>);
        if popup._cancel_btn.is_some() {
            popup.execute_cancel(pos, delta)
        } else {
            popup.execute_ok(pos, delta)
        }
    }

    pub fn callback_touch_over_ok(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let popup = ptr_as_mut(ui_component.get_user_data() as *const PopupWindowWidget<'a>);
        if popup._is_opened {
            popup.set_focused_button(PopupButtonFocus::Ok, true);
        }
        true
    }

    pub fn callback_touch_over_cancel(
        ui_component: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let popup = ptr_as_mut(ui_component.get_user_data() as *const PopupWindowWidget<'a>);
        if popup._is_opened && popup._cancel_btn.is_some() {
            popup.set_focused_button(PopupButtonFocus::Cancel, true);
        }
        true
    }

    pub fn callback_touch_down_ok(
        ui_component: &UIComponentInstance<'a>,
        pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const PopupWindowWidget<'a>);
        popup.set_focused_button(PopupButtonFocus::Ok, false);
        popup.execute_ok(pos, delta)
    }

    pub fn callback_touch_down_cancel(
        ui_component: &UIComponentInstance<'a>,
        pos: &Vector2<f32>,
        delta: &Vector2<f32>,
    ) -> bool {
        let popup = ptr_as_mut(ui_component.get_user_data() as *const PopupWindowWidget<'a>);
        popup.set_focused_button(PopupButtonFocus::Cancel, false);
        popup.execute_cancel(pos, delta)
    }

    pub fn create_popup_widget(
        parent_widget: &mut WidgetDefault<'a>,
        popup_text: &str,
        ok_button_text: &str,
        cancel_button_text: Option<&str>,
        ok_callback: Option<(CallbackTouchEvent<'a>, *const c_void)>,
        cancel_callback: Option<(CallbackTouchEvent<'a>, *const c_void)>,
    ) -> RcRefCell<PopupWindowWidget<'a>> {
        let popup_layer = UIManager::create_widget("popup_window_layer", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(popup_layer.as_ref()).get_ui_component_mut();
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
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_backdrop)));
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
            ui_comp.set_color(get_color32(240, 222, 186, 248));
            ui_comp.set_border_color(get_color32(135, 78, 42, 255));
            ui_comp.set_border(2.5);
            ui_comp.set_round(10.0);
            ui_comp.set_ignore_parent_renderable_area(true);
            // ui_comp.set_touchable(true);
            // ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_noop_frame)));
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
            ui_comp.set_font_color(get_color32(75, 45, 20, 255));
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
            ui_comp.set_color(BTN_OK_FOCUSED_COLOR);
            ui_comp.set_border_color(BTN_FOCUSED_BORDER_COLOR);
            ui_comp.set_border(3.0);
            ui_comp.set_round(6.0);
            ui_comp.set_touchable(true);
            ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_ok)));
            ui_comp.set_callback_touch_over(Some(Box::new(Self::callback_touch_over_ok)));
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
                ui_comp.set_color(BTN_CANCEL_COLOR);
                ui_comp.set_border_color(BTN_CANCEL_BORDER_COLOR);
                ui_comp.set_border(2.0);
                ui_comp.set_round(6.0);
                ui_comp.set_touchable(true);
                ui_comp.set_callback_touch_down(Some(Box::new(Self::callback_touch_down_cancel)));
                ui_comp.set_callback_touch_over(Some(Box::new(Self::callback_touch_over_cancel)));
            }
            ptr_as_mut(btn_container.as_ref()).add_widget(&cancel_btn_widget);
            Some(cancel_btn_widget)
        } else {
            None
        };

        let popup_rc = newRcRefCell(PopupWindowWidget {
            _parent_widget: parent_widget,
            _layer: popup_layer,
            _text_widget: text_widget,
            _ok_btn: ok_btn,
            _cancel_btn: cancel_btn,
            _ok_callback: ok_callback,
            _cancel_callback: cancel_callback,
            _focused_button: PopupButtonFocus::Ok,
            _is_opened: false,
            _just_opened: false,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        });

        let popup_ptr = popup_rc.as_ptr() as *const c_void;
        {
            let guard = popup_rc.borrow();
            ptr_as_mut(guard._layer.as_ref()).get_ui_component_mut().set_user_data(popup_ptr);
            ptr_as_mut(guard._ok_btn.as_ref()).get_ui_component_mut().set_user_data(popup_ptr);
            if let Some(cancel_btn_ref) = guard._cancel_btn.as_ref() {
                ptr_as_mut(cancel_btn_ref.as_ref()).get_ui_component_mut().set_user_data(popup_ptr);
            }
        }

        popup_rc
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn get_focused_button(&self) -> PopupButtonFocus {
        self._focused_button
    }

    pub fn get_focused_button_widget(&self) -> &Rc<WidgetDefault<'a>> {
        match self._focused_button {
            PopupButtonFocus::Ok => &self._ok_btn,
            PopupButtonFocus::Cancel => self._cancel_btn.as_ref().unwrap_or(&self._ok_btn),
        }
    }

    pub fn set_focused_button(&mut self, focus: PopupButtonFocus, play_sound: bool) {
        let target_focus = if focus == PopupButtonFocus::Cancel && self._cancel_btn.is_none() {
            PopupButtonFocus::Ok
        } else {
            focus
        };

        if self._focused_button != target_focus {
            self._focused_button = target_focus;
            if play_sound {
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
            self.update_button_visuals();
        }
    }

    pub fn update_button_visuals(&mut self) {
        let is_ok_focused = self._focused_button == PopupButtonFocus::Ok || self._cancel_btn.is_none();
        {
            let ok_ui = ptr_as_mut(self._ok_btn.as_ref()).get_ui_component_mut();
            if is_ok_focused {
                ok_ui.set_color(BTN_OK_FOCUSED_COLOR);
                ok_ui.set_border_color(BTN_FOCUSED_BORDER_COLOR);
                ok_ui.set_border(3.0);
            } else {
                ok_ui.set_color(BTN_OK_COLOR);
                ok_ui.set_border_color(BTN_OK_BORDER_COLOR);
                ok_ui.set_border(2.0);
            }
        }

        if let Some(cancel_btn) = self._cancel_btn.as_ref() {
            let cancel_ui = ptr_as_mut(cancel_btn.as_ref()).get_ui_component_mut();
            if !is_ok_focused {
                cancel_ui.set_color(BTN_CANCEL_FOCUSED_COLOR);
                cancel_ui.set_border_color(BTN_FOCUSED_BORDER_COLOR);
                cancel_ui.set_border(3.0);
            } else {
                cancel_ui.set_color(BTN_CANCEL_COLOR);
                cancel_ui.set_border_color(BTN_CANCEL_BORDER_COLOR);
                cancel_ui.set_border(2.0);
            }
        }
    }

    pub fn execute_ok(&mut self, pos: &Vector2<f32>, delta: &Vector2<f32>) -> bool {
        let popup_ptr = self as *const Self as *const c_void;
        let ok_callback = self._ok_callback;
        let ok_btn = self._ok_btn.clone();
        self.close();
        if let Some((callback, user_data)) = ok_callback {
            let ok_ui_mut = ptr_as_mut(ok_btn.as_ref()).get_ui_component_mut();
            ok_ui_mut.set_user_data(user_data);
            let result = callback(ok_ui_mut, pos, delta);
            ok_ui_mut.set_user_data(popup_ptr);
            return result;
        }
        true
    }

    pub fn execute_cancel(&mut self, pos: &Vector2<f32>, delta: &Vector2<f32>) -> bool {
        let popup_ptr = self as *const Self as *const c_void;
        let cancel_callback = self._cancel_callback;
        let cancel_btn = self._cancel_btn.clone();
        self.close();
        if let Some(btn) = cancel_btn {
            if let Some((callback, user_data)) = cancel_callback {
                let cancel_ui_mut = ptr_as_mut(btn.as_ref()).get_ui_component_mut();
                cancel_ui_mut.set_user_data(user_data);
                let result = callback(cancel_ui_mut, pos, delta);
                cancel_ui_mut.set_user_data(popup_ptr);
                return result;
            }
        }
        true
    }

    pub fn execute_focused_button(&mut self) -> bool {
        let zero = Vector2::zeros();
        match self._focused_button {
            PopupButtonFocus::Ok => self.execute_ok(&zero, &zero),
            PopupButtonFocus::Cancel => {
                if self._cancel_btn.is_some() {
                    self.execute_cancel(&zero, &zero)
                } else {
                    self.execute_ok(&zero, &zero)
                }
            }
        }
    }

    pub fn open(&mut self, self_rc: &RcRefCell<PopupWindowWidget<'a>>) {
        if !self._is_opened {
            self._is_opened = true;
            self._just_opened = true;
            self._nav_repeat_controller.reset();
            self._focused_button = PopupButtonFocus::Ok;
            self.update_button_visuals();
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            get_game_ui_manager_mut().register_popup_widget(self_rc.clone());
            let target_btn = self.get_focused_button_widget().clone();
            get_game_ui_manager_mut().request_mouse_snap_to_widget(&target_btn);
        }
    }

    pub fn close(&mut self) {
        if self._is_opened {
            get_game_ui_manager_mut().clear_pending_mouse_snap();
            self._is_opened = false;
            self._just_opened = false;
            self._nav_repeat_controller.reset();
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            get_game_ui_manager_mut().unregister_popup_widget(self as *const Self);
        }
    }

    pub fn update(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) -> bool {
        if !self._is_opened {
            return false;
        }

        if self._just_opened {
            if UiInput::is_confirm_hold(keyboard_input_data, joystick_input_data)
                || UiInput::is_cancel(keyboard_input_data, joystick_input_data)
                || UiInput::is_delete(keyboard_input_data, joystick_input_data)
            {
                return true;
            }
            self._just_opened = false;
        }

        let press_cancel = UiInput::is_cancel(keyboard_input_data, joystick_input_data);
        let press_ok = UiInput::is_confirm(keyboard_input_data, joystick_input_data);

        if self._cancel_btn.is_some() {
            let delta_time = time_data._delta_time_with_scale as f32;
            let (should_move, dir_opt) =
                self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

            if should_move && let Some((dir_x, _dir_y)) = dir_opt {
                if dir_x != 0 {
                    let next_focus = self._focused_button.from_horizontal_input(dir_x, true);
                    self.set_focused_button(next_focus, true);
                    move_mouse_to_ui_component(self.get_focused_button_widget().get_ui_component());
                }
            }

            if press_cancel {
                self.execute_cancel(&Vector2::zeros(), &Vector2::zeros());
                return true;
            } else if press_ok {
                self.execute_focused_button();
                return true;
            }
        } else {
            let close_popup = press_cancel || press_ok || UiInput::is_delete(keyboard_input_data, joystick_input_data);

            if close_popup {
                self.execute_ok(&Vector2::zeros(), &Vector2::zeros());
                return true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_button_focus_horizontal_navigation() {
        assert_eq!(
            PopupButtonFocus::Ok.from_horizontal_input(1, true),
            PopupButtonFocus::Cancel
        );
        assert_eq!(
            PopupButtonFocus::Cancel.from_horizontal_input(-1, true),
            PopupButtonFocus::Ok
        );
        assert_eq!(
            PopupButtonFocus::Ok.from_horizontal_input(-1, true),
            PopupButtonFocus::Ok
        );
        assert_eq!(
            PopupButtonFocus::Cancel.from_horizontal_input(1, true),
            PopupButtonFocus::Cancel
        );
        assert_eq!(
            PopupButtonFocus::Ok.from_horizontal_input(0, true),
            PopupButtonFocus::Ok
        );
        assert_eq!(
            PopupButtonFocus::Cancel.from_horizontal_input(0, true),
            PopupButtonFocus::Cancel
        );
        assert_eq!(
            PopupButtonFocus::Ok.from_horizontal_input(1, false),
            PopupButtonFocus::Ok
        );
    }
}
