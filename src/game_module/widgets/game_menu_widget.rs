use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, DEFAULT_GAME_SAVE_DATA};
use crate::game_module::game_resource::GameResources;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PosHintX, PosHintY, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
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
pub enum GameMenuType {
    Resume,
    NewGame,
    LoadGame,
    SaveGame,
    Exit,
}

pub struct GameMenuItem<'a> {
    pub _game_menu_widget: *const GameMenuWidget<'a>,
    pub _game_menu_type: GameMenuType,
    pub _item_widget: Rc<WidgetDefault<'a>>,
}

impl<'a> GameMenuItem<'a> {
    pub fn create_game_menu_item(
        game_menu_widget: &GameMenuWidget<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        game_menu_type: GameMenuType,
    ) -> Box<GameMenuItem<'a>> {
        let item_widget = UIManager::create_widget("game_menu_item", UIWidgetTypes::Default);
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
        ui_component.set_text(game_menu_type.to_string().as_str());
        ui_component.set_font_size(40.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        parent_widget.add_widget(&item_widget);

        let game_menu_item = Box::new(GameMenuItem {
            _game_menu_widget: game_menu_widget,
            _game_menu_type: game_menu_type,
            _item_widget: item_widget,
        });

        ui_component.set_touchable(true);
        ui_component.set_callback_touch_over(Some(Box::new(GameMenuWidget::callback_touch_over)));
        ui_component.set_callback_touch_down(Some(Box::new(GameMenuWidget::callback_touch_down)));
        ui_component.set_user_data(game_menu_item.as_ref() as *const GameMenuItem<'a> as *const c_void);

        game_menu_item
    }
}

pub struct GameMenuWidget<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _menu_items: Vec<Box<GameMenuItem<'a>>>,
    pub _selected_menu_item: GameMenuType,
    pub _is_opened_game_menu: bool,
}

impl<'a> GameMenuWidget<'a> {
    pub fn callback_touch_over(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameMenuItem<'a>);
        let game_menu_widget = ptr_as_mut(game_menu_item._game_menu_widget);
        game_menu_widget.set_selected_menu_item(game_menu_item._game_menu_type, false);
        true
    }

    pub fn callback_touch_down(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameMenuItem<'a>);
        let game_menu_widget = ptr_as_mut(game_menu_item._game_menu_widget);
        game_menu_widget.press_game_menu(game_menu_item._game_menu_type);
        true
    }

    pub fn create_game_menu_widget(
        game_client: &GameClient<'a>,
        _game_resources: &GameResources<'a>,
        parent_widget: &mut WidgetDefault<'a>,
    ) -> Box<GameMenuWidget<'a>> {
        let layer = UIManager::create_widget("game_menu_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Center(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_enable(false);
        parent_widget.add_widget(&layer);

        let mut game_menu_widget = Box::new(GameMenuWidget {
            _game_client: game_client,
            _audio_manager: ptr_as_ref(game_client).get_game_scene_manager().get_audio_manager(),
            _parent_widget: parent_widget,
            _layer: layer,
            _menu_items: Vec::new(),
            _selected_menu_item: GameMenuType::Resume,
            _is_opened_game_menu: false,
        });

        let menu_items = vec![
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::Resume),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::NewGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::LoadGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::SaveGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::Exit),
        ];

        game_menu_widget.as_mut()._menu_items = menu_items;
        game_menu_widget
    }
    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        ui_component.set_size_hint_x(Some(0.5));
        ui_component.set_size_hint_y(Some(0.5));
    }
    pub fn is_opened_game_menu(&self) -> bool {
        self._is_opened_game_menu
    }
    pub fn open_game_menu(&mut self) {
        if !self._is_opened_game_menu {
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            self.set_selected_menu_item(self._selected_menu_item, true);
            self._is_opened_game_menu = true;
        }
    }
    pub fn close_game_menu(&mut self) {
        if self._is_opened_game_menu {
            ptr_as_mut(self._audio_manager).play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened_game_menu = false;
        }
    }
    pub fn set_selected_menu_item(&mut self, selected_menu_item: GameMenuType, force: bool) -> bool {
        if self._selected_menu_item != selected_menu_item || force {
            let prev_menu_item = &self._menu_items[self._selected_menu_item as usize];
            let curr_menu_item = &self._menu_items[selected_menu_item as usize];
            ptr_as_mut(prev_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(false);
            ptr_as_mut(curr_menu_item._item_widget.as_ref()).get_ui_component_mut().set_selected(true);
            ptr_as_mut(self._audio_manager).play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            self._selected_menu_item = selected_menu_item;
            return true;
        }
        false
    }
    pub fn press_game_menu(&mut self, selected_menu_item: GameMenuType) {
        let game_client = ptr_as_mut(self._game_client);
        match selected_menu_item {
            GameMenuType::Resume => {}
            GameMenuType::NewGame => {
                game_client.request_new_game();
            }
            GameMenuType::LoadGame => {
                game_client.request_load_game(DEFAULT_GAME_SAVE_DATA);
            }
            GameMenuType::SaveGame => {
                game_client.save_game(true);
            }
            GameMenuType::Exit => {
                game_client.exit_game();
            }
        }
        self.set_selected_menu_item(selected_menu_item, false);
        self.close_game_menu();
    }
    pub fn update_game_menu_widget(
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
            || joystick_input_data._btn_x == ButtonState::Pressed;
        let close_game_menu =
            keyboard_input_data.get_key_pressed(KeyCode::Escape) || joystick_input_data._btn_b == ButtonState::Pressed;

        if move_menu_up {
            let selected_menu_item: usize = if self._selected_menu_item as usize == 0 {
                GameMenuType::COUNT - 1
            } else {
                self._selected_menu_item as usize - 1
            };
            self.set_selected_menu_item(GameMenuType::from_repr(selected_menu_item).unwrap(), false);
        } else if move_menu_down {
            let selected_menu_item: usize = if self._selected_menu_item as usize == (GameMenuType::COUNT - 1) {
                0
            } else {
                self._selected_menu_item as usize + 1
            };
            self.set_selected_menu_item(GameMenuType::from_repr(selected_menu_item).unwrap(), false);
        }

        if press_game_menu {
            self.press_game_menu(self._selected_menu_item);
        } else if close_game_menu {
            self.close_game_menu();
        }
    }
}
