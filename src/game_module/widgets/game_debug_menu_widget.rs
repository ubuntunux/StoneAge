use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM};
use crate::game_module::game_service_locator::{get_application_mut, get_game_client_mut, get_game_scene_manager, get_game_scene_manager_mut};
use crate::game_module::game_weather::WeatherType;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::constants::{DEVELOPMENT, SHOW_DEBUG_TEXT};
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
use crate::game_module::game_client::GamePhase;

const ITEM_WIDTH: f32 = 250.0;
const ITEM_HEIGHT: f32 = 60.0;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, FromRepr, EnumCount, EnumIter, EnumString, Copy)]
#[repr(usize)]
pub enum GameDebugMenuType {
    ToggleGameMode,
    DebugText,
    RainTest,
    TimeTest,
}

pub struct GameDebugMenuItem<'a> {
    pub _game_debug_menu_widget: *const GameDebugMenuWidget<'a>,
    pub _game_debug_menu_type: GameDebugMenuType,
    pub _item_widget: Rc<WidgetDefault<'a>>,
}

impl<'a> GameDebugMenuItem<'a> {
    pub fn create_game_debug_menu_item(
        game_debug_menu_widget: &GameDebugMenuWidget<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        game_debug_menu_type: GameDebugMenuType,
    ) -> Box<GameDebugMenuItem<'a>> {
        let item_widget = UIManager::create_widget("game_debug_menu_item", UIWidgetTypes::Default);
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
        ui_component.set_text(game_debug_menu_type.to_string().as_str());
        ui_component.set_font_size(40.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        parent_widget.add_widget(&item_widget);

        let game_debug_menu_item = Box::new(GameDebugMenuItem {
            _game_debug_menu_widget: game_debug_menu_widget,
            _game_debug_menu_type: game_debug_menu_type,
            _item_widget: item_widget,
        });

        ui_component.set_touchable(true);
        ui_component.set_callback_touch_over(Some(Box::new(GameDebugMenuWidget::callback_touch_over)));
        ui_component.set_callback_touch_down(Some(Box::new(GameDebugMenuWidget::callback_touch_down)));
        ui_component.set_user_data(game_debug_menu_item.as_ref() as *const GameDebugMenuItem<'a> as *const c_void);

        game_debug_menu_item
    }
}

pub struct GameDebugMenuWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _menu_items: Vec<Box<GameDebugMenuItem<'a>>>,
    pub _selected_menu_item: GameDebugMenuType,
    pub _is_opened_game_debug_menu: bool,
}

impl<'a> GameDebugMenuWidget<'a> {
    pub fn callback_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_debug_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameDebugMenuItem<'a>);
        let game_debug_menu_widget = ptr_as_mut(game_debug_menu_item._game_debug_menu_widget);
        game_debug_menu_widget.set_selected_menu_item(game_debug_menu_item._game_debug_menu_type, false);
        true
    }

    pub fn callback_touch_down(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_debug_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameDebugMenuItem<'a>);
        let game_debug_menu_widget = ptr_as_mut(game_debug_menu_item._game_debug_menu_widget);
        game_debug_menu_widget.press_game_debug_menu(game_debug_menu_item._game_debug_menu_type);
        true
    }

    pub fn create_game_debug_menu_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<GameDebugMenuWidget<'a>> {
        let layer = UIManager::create_widget("game_debug_menu_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        // ui_component.set_size_hint_x(Some(0.5));
        // ui_component.set_size_hint_y(Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_enable(false);
        parent_widget.add_widget(&layer);

        let mut game_debug_menu_widget = Box::new(GameDebugMenuWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _menu_items: Vec::new(),
            _selected_menu_item: GameDebugMenuType::DebugText,
            _is_opened_game_debug_menu: false,
        });

        let menu_items = vec![
            GameDebugMenuItem::create_game_debug_menu_item(game_debug_menu_widget.as_ref(), layer_mut, GameDebugMenuType::ToggleGameMode),
            GameDebugMenuItem::create_game_debug_menu_item(game_debug_menu_widget.as_ref(), layer_mut, GameDebugMenuType::DebugText),
            GameDebugMenuItem::create_game_debug_menu_item(game_debug_menu_widget.as_ref(), layer_mut, GameDebugMenuType::RainTest),
            GameDebugMenuItem::create_game_debug_menu_item(game_debug_menu_widget.as_ref(), layer_mut, GameDebugMenuType::TimeTest),
        ];

        game_debug_menu_widget.as_mut()._menu_items = menu_items;
        game_debug_menu_widget
    }
    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}
    pub fn is_opened_game_debug_menu(&self) -> bool {
        self._is_opened_game_debug_menu
    }
    pub fn open_game_debug_menu(&mut self) {
        if !self._is_opened_game_debug_menu {
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            self.set_selected_menu_item(self._selected_menu_item, true);
            self._is_opened_game_debug_menu = true;
        }
    }
    pub fn close_game_debug_menu(&mut self) {
        if self._is_opened_game_debug_menu {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened_game_debug_menu = false;
        }
    }
    pub fn set_selected_menu_item(&mut self, selected_menu_item: GameDebugMenuType, force: bool) -> bool {
        if self._selected_menu_item != selected_menu_item || force {
            let prev_menu_item = &self._menu_items[self._selected_menu_item as usize];
            let curr_menu_item = &self._menu_items[selected_menu_item as usize];
            ptr_as_mut(prev_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(false);
            ptr_as_mut(curr_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(true);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            self._selected_menu_item = selected_menu_item;
        }
        false
    }
    pub fn press_game_debug_menu(&mut self, selected_menu_item: GameDebugMenuType) {
        match selected_menu_item {
            GameDebugMenuType::ToggleGameMode => {
                get_application_mut().toggle_game_mode();
            }
            GameDebugMenuType::DebugText => {
                unsafe { SHOW_DEBUG_TEXT = !SHOW_DEBUG_TEXT };
            }
            GameDebugMenuType::RainTest => {
                let weather_type = get_game_scene_manager()._weather.get_weather_type();
                get_game_scene_manager_mut()._weather.set_next_weather(if weather_type == WeatherType::None {
                    WeatherType::Rain
                } else {
                    WeatherType::None
                });
            }
            GameDebugMenuType::TimeTest => {
                get_game_scene_manager_mut().set_time_of_day(get_game_scene_manager_mut().get_time_of_day() + 6.0);
            }
        }
        self.set_selected_menu_item(selected_menu_item, false);
        self.close_game_debug_menu();
    }
    pub fn update_game_debug_menu_widget(
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
        let press_game_debug_menu = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_x == ButtonState::Pressed
            || joystick_input_data._btn_a == ButtonState::Pressed;

        let is_toggle_game_mode_by_joystick = joystick_input_data._btn_left_trigger
            == ButtonState::Hold
            && joystick_input_data._btn_right_trigger == ButtonState::Hold
            && joystick_input_data._btn_left_shoulder == ButtonState::Hold
            && joystick_input_data._btn_right_shoulder == ButtonState::Hold;

        if move_menu_up {
            let selected_menu_item: usize = if self._selected_menu_item as usize == 0 {
                GameDebugMenuType::COUNT - 1
            } else {
                self._selected_menu_item as usize - 1
            };
            self.set_selected_menu_item(GameDebugMenuType::from_repr(selected_menu_item).unwrap(), false);
        } else if move_menu_down {
            let selected_menu_item: usize = if self._selected_menu_item as usize == (GameDebugMenuType::COUNT - 1) {
                0
            } else {
                self._selected_menu_item as usize + 1
            };
            self.set_selected_menu_item(GameDebugMenuType::from_repr(selected_menu_item).unwrap(), false);
        }

        if press_game_debug_menu {
            self.press_game_debug_menu(self._selected_menu_item);
        } else if keyboard_input_data.get_key_pressed(KeyCode::Backquote) || is_toggle_game_mode_by_joystick {
            self.close_game_debug_menu();
        }
    }
}
