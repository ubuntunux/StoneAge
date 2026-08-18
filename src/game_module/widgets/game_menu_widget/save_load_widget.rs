use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, DEFAULT_GAME_SAVE_DATA};
use crate::game_module::game_service_locator::get_game_client_mut;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
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
use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumIter, EnumString, FromRepr};
use winit::keyboard::KeyCode;

const ITEM_WIDTH: f32 = 250.0;
const ITEM_HEIGHT: f32 = 60.0;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, FromRepr, EnumCount, EnumIter, EnumString, Copy)]
#[repr(usize)]
pub enum SaveLoadType {
    Resume,
    NewGame,
    LoadGame,
    SaveGame,
    Exit,
}

pub struct SaveLoadMenuItem<'a> {
    pub _save_load_widget: *const SaveLoadWidget<'a>,
    pub _save_load_type: SaveLoadType,
    pub _item_widget: Rc<WidgetDefault<'a>>,
}

impl<'a> SaveLoadMenuItem<'a> {
    pub fn create_save_load_menu_item(
        save_load_widget: &SaveLoadWidget<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        save_load_type: SaveLoadType,
    ) -> Box<SaveLoadMenuItem<'a>> {
        let item_widget = UIManager::create_widget("save_load_menu_item", UIWidgetTypes::Default);
        let item_widget_mut = ptr_as_mut(item_widget.as_ref());
        let ui_component = item_widget_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_x(ITEM_WIDTH);
        ui_component.set_size_y(ITEM_HEIGHT);
        ui_component.set_color(get_color32(50, 50, 50, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_margin(10.0);
        ui_component.set_round(5.0);
        ui_component.set_text(save_load_type.to_string().as_str());
        ui_component.set_font_size(40.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        parent_widget.add_widget(&item_widget);

        let save_load_menu_item = Box::new(SaveLoadMenuItem {
            _save_load_widget: save_load_widget,
            _save_load_type: save_load_type,
            _item_widget: item_widget,
        });

        ui_component.set_touchable(true);
        ui_component.set_callback_touch_over(Some(Box::new(SaveLoadWidget::callback_touch_over)));
        ui_component.set_callback_touch_down(Some(Box::new(SaveLoadWidget::callback_touch_down)));
        ui_component.set_user_data(save_load_menu_item.as_ref() as *const SaveLoadMenuItem<'a> as *const c_void);

        save_load_menu_item
    }
}

pub struct SaveLoadWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _menu_items: Vec<Box<SaveLoadMenuItem<'a>>>,
    pub _selected_menu_item: SaveLoadType,
    pub _is_opened_save_load_widget: bool,
}

impl<'a> SaveLoadWidget<'a> {
    pub fn callback_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let save_load_menu_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadMenuItem<'a>);
        let save_load_widget = ptr_as_mut(save_load_menu_item._save_load_widget);
        save_load_widget.set_selected_menu_item(save_load_menu_item._save_load_type, false);
        true
    }

    pub fn callback_touch_down(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let save_load_menu_item = ptr_as_ref(ui_component.get_user_data() as *const SaveLoadMenuItem<'a>);
        let save_load_widget = ptr_as_mut(save_load_menu_item._save_load_widget);
        save_load_widget.press_save_load_menu(save_load_menu_item._save_load_type);
        true
    }

    pub fn create_save_load_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<SaveLoadWidget<'a>> {
        let layer = UIManager::create_widget("save_load_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(100, 100, 100, 200));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);

        let mut save_load_widget = Box::new(SaveLoadWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _menu_items: Vec::new(),
            _selected_menu_item: SaveLoadType::Resume,
            _is_opened_save_load_widget: false,
        });

        let menu_items = vec![
            SaveLoadMenuItem::create_save_load_menu_item(save_load_widget.as_ref(), layer_mut, SaveLoadType::Resume),
            SaveLoadMenuItem::create_save_load_menu_item(save_load_widget.as_ref(), layer_mut, SaveLoadType::NewGame),
            SaveLoadMenuItem::create_save_load_menu_item(save_load_widget.as_ref(), layer_mut, SaveLoadType::LoadGame),
            SaveLoadMenuItem::create_save_load_menu_item(save_load_widget.as_ref(), layer_mut, SaveLoadType::SaveGame),
            SaveLoadMenuItem::create_save_load_menu_item(save_load_widget.as_ref(), layer_mut, SaveLoadType::Exit),
        ];

        save_load_widget.as_mut()._menu_items = menu_items;
        save_load_widget
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_save_load_widget(&self) -> bool {
        self._is_opened_save_load_widget
    }

    pub fn open_save_load_widget(&mut self) {
        if !self._is_opened_save_load_widget {
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self.set_selected_menu_item(self._selected_menu_item, true);
            self._is_opened_save_load_widget = true;
        }
    }

    pub fn close_save_load_widget(&mut self) {
        if self._is_opened_save_load_widget {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            self._is_opened_save_load_widget = false;
        }
    }

    pub fn set_selected_menu_item(&mut self, selected_menu_item: SaveLoadType, force: bool) -> bool {
        if self._selected_menu_item != selected_menu_item || force {
            let prev_menu_item = &self._menu_items[self._selected_menu_item as usize];
            let curr_menu_item = &self._menu_items[selected_menu_item as usize];
            ptr_as_mut(prev_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(false);
            ptr_as_mut(curr_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(true);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            self._selected_menu_item = selected_menu_item;
            return true;
        }
        false
    }

    pub fn press_save_load_menu(&mut self, selected_menu_item: SaveLoadType) {
        let game_client = get_game_client_mut();
        match selected_menu_item {
            SaveLoadType::Resume => {}
            SaveLoadType::NewGame => {
                game_client.request_new_game();
            }
            SaveLoadType::LoadGame => {
                game_client.request_load_game(DEFAULT_GAME_SAVE_DATA);
            }
            SaveLoadType::SaveGame => {
                game_client.save_game(true);
            }
            SaveLoadType::Exit => {
                game_client.exit_game();
            }
        }
        self.set_selected_menu_item(selected_menu_item, false);
        self.close_save_load_widget();
    }

    pub fn update_save_load_widget(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        let move_menu_up = keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || joystick_input_data._btn_up == ButtonState::Pressed;
        let move_menu_down = keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || joystick_input_data._btn_down == ButtonState::Pressed;
        let press_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_x == ButtonState::Pressed
            || joystick_input_data._btn_a == ButtonState::Pressed;

        if move_menu_up {
            let selected_menu_item: usize = if self._selected_menu_item as usize == 0 {
                SaveLoadType::COUNT - 1
            } else {
                self._selected_menu_item as usize - 1
            };
            self.set_selected_menu_item(SaveLoadType::from_repr(selected_menu_item).unwrap(), false);
        } else if move_menu_down {
            let selected_menu_item: usize = if self._selected_menu_item as usize == (SaveLoadType::COUNT - 1) {
                0
            } else {
                self._selected_menu_item as usize + 1
            };
            self.set_selected_menu_item(SaveLoadType::from_repr(selected_menu_item).unwrap(), false);
        }

        if press_game_menu {
            self.press_save_load_menu(self._selected_menu_item);
        }
    }
}
