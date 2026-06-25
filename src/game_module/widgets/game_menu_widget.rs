use std::rc::Rc;
use nalgebra::Vector2;
use strum_macros::{Display, EnumString};
use winit::keyboard::KeyCode;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
use crate::game_module::game_resource::GameResources;

const ITEM_WIDTH: f32 = 250.0;
const ITEM_HEIGHT: f32 = 100.0;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy)]
pub enum GameMenuType {
    NewGame,
    LoadGame,
    SaveGame,
    Exit
}

pub struct GameMenuItem<'a> {
    pub _game_menu_type: GameMenuType,
    pub _item_widget: Rc<WidgetDefault<'a>>,
}

impl<'a> GameMenuItem<'a> {
    pub fn create_game_menu_item(
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

        Box::new(GameMenuItem {
            _game_menu_type: game_menu_type,
            _item_widget: item_widget,
        })
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
    pub fn create_game_menu_widget(
        game_client: &GameClient<'a>,
        _game_resources: &GameResources<'a>,
        parent_widget: &mut WidgetDefault<'a>,
    ) -> GameMenuWidget<'a> {
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

        let menu_items = vec![
            GameMenuItem::create_game_menu_item(layer_mut, GameMenuType::NewGame),
            GameMenuItem::create_game_menu_item(layer_mut, GameMenuType::LoadGame),
            GameMenuItem::create_game_menu_item(layer_mut, GameMenuType::SaveGame),
            GameMenuItem::create_game_menu_item(layer_mut, GameMenuType::Exit)
        ];

        GameMenuWidget {
            _game_client: game_client,
            _audio_manager: ptr_as_ref(game_client).get_game_scene_manager().get_audio_manager(),
            _parent_widget: parent_widget,
            _layer: layer,
            _menu_items: menu_items,
            _selected_menu_item_index: 0,
            _is_opened_game_menu: false,
        }
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
