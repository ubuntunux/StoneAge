use crate::game_module::widgets::game_menu_widget::save_load_slot_widget::SaveLoadSlotWidget;
use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::WidgetDefault;

pub struct SaveLoadWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _save_load_slot_widget: Box<SaveLoadSlotWidget<'a>>,
}

impl<'a> SaveLoadWidget<'a> {
    pub fn create_save_load_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<SaveLoadWidget<'a>> {
        let save_load_slot_widget = SaveLoadSlotWidget::create_save_load_slot_widget(parent_widget);
        Box::new(SaveLoadWidget {
            _parent_widget: parent_widget,
            _save_load_slot_widget: save_load_slot_widget,
        })
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        self._save_load_slot_widget.changed_window_size(window_size);
    }

    pub fn is_opened_save_load_widget(&self) -> bool {
        self._save_load_slot_widget.is_opened()
    }

    pub fn open_save_load_widget(&mut self) {
        self._save_load_slot_widget.open_slot_widget();
    }

    pub fn close_save_load_widget(&mut self) {
        self._save_load_slot_widget.close_slot_widget();
    }

    pub fn update_save_load_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        self._save_load_slot_widget.update_slot_widget(time_data, joystick_input_data, keyboard_input_data);
    }
}
