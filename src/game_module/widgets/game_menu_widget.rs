use std::ffi::c_void;
use std::rc::Rc;
use nalgebra::Vector2;
use strum_macros::{Display, EnumString};
use winit::keyboard::KeyCode;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM};
use crate::game_module::game_resource::GameResources;

const ITEM_WIDTH: f32 = 250.0;
const ITEM_HEIGHT: f32 = 60.0;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy)]
pub enum GameMenuType {
    Resume,
    NewGame,
    LoadGame,
    SaveGame,
    Exit
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
        game_menu_type: GameMenuType
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
    pub _selected_menu_item_index: usize,
    pub _is_opened_game_menu: bool,
}

impl<'a> GameMenuWidget<'a> {
    pub fn callback_touch_over(ui_component: &UIComponentInstance<'a>, _touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let game_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameMenuItem<'a>);
        let game_menu_widget = ptr_as_ref(game_menu_item._game_menu_widget);
        let audio_manager = ptr_as_mut(game_menu_widget._audio_manager);
        audio_manager.play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_touch_down(ui_component: &UIComponentInstance<'a>, _touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let game_menu_item = ptr_as_ref(ui_component.get_user_data() as *const GameMenuItem<'a>);
        let game_menu_widget = ptr_as_mut(game_menu_item._game_menu_widget);
        let audio_manager = ptr_as_mut(game_menu_widget._audio_manager);
        audio_manager.play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        match game_menu_item._game_menu_type {
            GameMenuType::Resume => {
            }
            GameMenuType::NewGame => {
            }
            GameMenuType::LoadGame => {
                ptr_as_mut(game_menu_widget._game_client).load_game();
            }
            GameMenuType::SaveGame => {
                ptr_as_mut(game_menu_widget._game_client).save_game();
            }
            GameMenuType::Exit => {
                ptr_as_mut(game_menu_widget._game_client).exit_game();
            }
        }
        game_menu_widget.close_game_menu();
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
            _selected_menu_item_index: 0,
            _is_opened_game_menu: false,
        });

        let menu_items = vec![
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::Resume),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::NewGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::LoadGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::SaveGame),
            GameMenuItem::create_game_menu_item(game_menu_widget.as_ref(), layer_mut, GameMenuType::Exit)
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
        if self._is_opened_game_menu == false {
            ptr_as_mut(self._audio_manager).play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
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
    pub fn update_game_menu_widget(
        &mut self,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData
    ) {
        let _move_menu_up = keyboard_input_data.get_key_hold(KeyCode::ArrowUp) ||
            joystick_input_data._btn_up == ButtonState::Hold;
        let _move_menu_down = keyboard_input_data.get_key_hold(KeyCode::ArrowDown) ||
            joystick_input_data._btn_down == ButtonState::Hold;
        let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape) ||
            joystick_input_data._btn_b == ButtonState::Pressed;

        if close_game_menu {
            self.close_game_menu();
        }
    }
}
