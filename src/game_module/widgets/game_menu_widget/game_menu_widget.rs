use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
use crate::game_module::widgets::game_menu_widget::friendly_npc_list_widget::FriendlyNpcListWidget;
use crate::game_module::widgets::game_menu_widget::game_debug_menu_widget::GameDebugMenuWidget;
use crate::game_module::widgets::game_menu_widget::inventory_widget::InventoryWidget;
use crate::game_module::widgets::game_menu_widget::player_records_widget::PlayerRecordsWidget;
use crate::game_module::widgets::game_menu_widget::save_load_widget::SaveLoadWidget;
use crate::game_module::widgets::game_menu_widget::taming_list_widget::TamingListWidget;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

const TAB_BUTTON_WIDTH: f32 = 100.0;
const TAB_BUTTON_HEIGHT: f32 = 35.0;
const TAB_BUTTON_COLOR_ACTIVE: u32 = get_color32(70, 130, 200, 255);
const TAB_BUTTON_COLOR_INACTIVE: u32 = get_color32(50, 50, 50, 255);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameMenuTab {
    Inventory,
    Records,
    TamingList,
    FriendlyNpcList,
    SaveLoad,
    DebugMenu,
}

pub struct GameMenuWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _inventory_tab_btn: Rc<WidgetDefault<'a>>,
    pub _records_tab_btn: Rc<WidgetDefault<'a>>,
    pub _taming_tab_btn: Rc<WidgetDefault<'a>>,
    pub _friendly_npc_tab_btn: Rc<WidgetDefault<'a>>,
    pub _saveload_tab_btn: Rc<WidgetDefault<'a>>,
    pub _debug_tab_btn: Rc<WidgetDefault<'a>>,
    pub _inventory_widget: Box<InventoryWidget<'a>>,
    pub _player_records_widget: Box<PlayerRecordsWidget<'a>>,
    pub _taming_list_widget: Box<TamingListWidget<'a>>,
    pub _friendly_npc_list_widget: Box<FriendlyNpcListWidget<'a>>,
    pub _save_load_widget: Box<SaveLoadWidget<'a>>,
    pub _game_debug_menu_widget: Box<GameDebugMenuWidget<'a>>,
    pub _active_tab: GameMenuTab,
    pub _is_opened_game_menu: bool,
}

impl<'a> GameMenuWidget<'a> {
    pub fn callback_tab_inventory(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::Inventory);
        true
    }

    pub fn callback_tab_records(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::Records);
        true
    }

    pub fn callback_tab_taming(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::TamingList);
        true
    }

    pub fn callback_tab_friendly_npc(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::FriendlyNpcList);
        true
    }

    pub fn callback_tab_saveload(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::SaveLoad);
        true
    }

    pub fn callback_tab_debug(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.set_active_tab(GameMenuTab::DebugMenu);
        true
    }

    pub fn callback_close(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_menu_widget = ptr_as_mut(ui_component.get_user_data() as *const GameMenuWidget<'a>);
        game_menu_widget.close_game_menu();
        true
    }

    pub fn create_tab_button(
        widget_name: &str,
        text: &str,
        callback: rust_engine_3d::scene::ui::CallbackTouchEvent<'a>,
        parent_widget: &mut WidgetDefault<'a>,
    ) -> Rc<WidgetDefault<'a>> {
        let tab_btn = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(tab_btn.as_ref()).get_ui_component_mut();
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_size(TAB_BUTTON_WIDTH, TAB_BUTTON_HEIGHT);
        ui_component.set_margin(4.0);
        ui_component.set_text(text);
        ui_component.set_font_size(22.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(5.0);
        ui_component.set_color(TAB_BUTTON_COLOR_INACTIVE);
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(callback)));
        parent_widget.add_widget(&tab_btn);
        tab_btn
    }

    pub fn create_game_menu_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<GameMenuWidget<'a>> {
        let layer = UIManager::create_widget("game_menu_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_size(760.0, 580.0);
        ui_component.set_expandable(false);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(100, 100, 100, 220));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_enable(false);
        parent_widget.add_widget(&layer);

        // Header Layout with 6 Tab Buttons
        let header_layout = UIManager::create_widget("menu_header", UIWidgetTypes::Default);
        let header_layout_mut = ptr_as_mut(header_layout.as_ref());
        let ui_component = header_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(false);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(45.0);
        ui_component.set_margin(5.0);
        ui_component.set_padding(5.0);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_round(5.0);
        layer_mut.add_widget(&header_layout);

        let inventory_tab_btn = Self::create_tab_button(
            "inventory_tab_btn",
            "Inventory",
            GameMenuWidget::callback_tab_inventory,
            header_layout_mut,
        );

        let records_tab_btn = Self::create_tab_button(
            "records_tab_btn",
            "Records",
            GameMenuWidget::callback_tab_records,
            header_layout_mut,
        );

        let taming_tab_btn = Self::create_tab_button(
            "taming_tab_btn",
            "Taming",
            GameMenuWidget::callback_tab_taming,
            header_layout_mut,
        );

        let friendly_npc_tab_btn = Self::create_tab_button(
            "friendly_npc_tab_btn",
            "NPC Friends",
            GameMenuWidget::callback_tab_friendly_npc,
            header_layout_mut,
        );

        let saveload_tab_btn = Self::create_tab_button(
            "saveload_tab_btn",
            "Save / Load",
            GameMenuWidget::callback_tab_saveload,
            header_layout_mut,
        );

        let debug_tab_btn = Self::create_tab_button(
            "debug_tab_btn",
            "Debug",
            GameMenuWidget::callback_tab_debug,
            header_layout_mut,
        );

        let close_btn = UIManager::create_widget("close_btn", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(close_btn.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size(35.0, 35.0);
        ui_component.set_margin(5.0);
        ui_component.set_text("X");
        ui_component.set_font_size(26.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(5.0);
        ui_component.set_color(get_color32(180, 50, 50, 255));
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(GameMenuWidget::callback_close)));
        header_layout_mut.add_widget(&close_btn);

        let content_layout = UIManager::create_widget("menu_content", UIWidgetTypes::Default);
        let content_layout_mut = ptr_as_mut(content_layout.as_ref());
        let ui_component = content_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(false);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_margin(5.0);
        ui_component.set_padding(5.0);
        ui_component.set_color(get_color32(0, 0, 0, 128));
        ui_component.set_round(5.0);
        layer_mut.add_widget(&content_layout);

        // Create sub-widgets
        let inventory_widget = InventoryWidget::create_inventory_widget(content_layout_mut);
        let player_records_widget = PlayerRecordsWidget::create_player_records_widget(content_layout_mut);
        let taming_list_widget = TamingListWidget::create_taming_list_widget(content_layout_mut);
        let friendly_npc_list_widget = FriendlyNpcListWidget::create_friendly_npc_list_widget(content_layout_mut);
        let save_load_widget = SaveLoadWidget::create_save_load_widget(content_layout_mut);
        let game_debug_menu_widget = GameDebugMenuWidget::create_game_debug_menu_widget(content_layout_mut);

        let game_menu_widget = Box::new(GameMenuWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _inventory_tab_btn: inventory_tab_btn,
            _records_tab_btn: records_tab_btn,
            _taming_tab_btn: taming_tab_btn,
            _friendly_npc_tab_btn: friendly_npc_tab_btn,
            _saveload_tab_btn: saveload_tab_btn,
            _debug_tab_btn: debug_tab_btn,
            _inventory_widget: inventory_widget,
            _player_records_widget: player_records_widget,
            _taming_list_widget: taming_list_widget,
            _friendly_npc_list_widget: friendly_npc_list_widget,
            _save_load_widget: save_load_widget,
            _game_debug_menu_widget: game_debug_menu_widget,
            _active_tab: GameMenuTab::SaveLoad,
            _is_opened_game_menu: false,
        });

        // Set user_data on header buttons to point to game_menu_widget instance
        let ptr_self = game_menu_widget.as_ref() as *const GameMenuWidget<'a> as *const c_void;
        ptr_as_mut(game_menu_widget._inventory_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(game_menu_widget._records_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(game_menu_widget._taming_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(game_menu_widget._friendly_npc_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(game_menu_widget._saveload_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(game_menu_widget._debug_tab_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);
        ptr_as_mut(close_btn.as_ref()).get_ui_component_mut().set_user_data(ptr_self);

        game_menu_widget
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        self._inventory_widget.changed_window_size(window_size);
        self._player_records_widget.changed_window_size(window_size);
        self._taming_list_widget.changed_window_size(window_size);
        self._friendly_npc_list_widget.changed_window_size(window_size);
        self._save_load_widget.changed_window_size(window_size);
        self._game_debug_menu_widget.changed_window_size(window_size);
    }

    pub fn is_opened_game_menu(&self) -> bool {
        self._is_opened_game_menu
    }

    pub fn get_active_tab(&self) -> GameMenuTab {
        self._active_tab
    }

    pub fn set_active_tab(&mut self, tab: GameMenuTab) {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        self._active_tab = tab;

        let inv_tab_ui = ptr_as_mut(self._inventory_tab_btn.as_ref()).get_ui_component_mut();
        let records_tab_ui = ptr_as_mut(self._records_tab_btn.as_ref()).get_ui_component_mut();
        let taming_tab_ui = ptr_as_mut(self._taming_tab_btn.as_ref()).get_ui_component_mut();
        let friendly_npc_tab_ui = ptr_as_mut(self._friendly_npc_tab_btn.as_ref()).get_ui_component_mut();
        let saveload_tab_ui = ptr_as_mut(self._saveload_tab_btn.as_ref()).get_ui_component_mut();
        let debug_tab_ui = ptr_as_mut(self._debug_tab_btn.as_ref()).get_ui_component_mut();

        // Close all sub-widgets first
        self._inventory_widget.close_inventory();
        self._player_records_widget.close_player_records_widget();
        self._taming_list_widget.close_taming_list_widget();
        self._friendly_npc_list_widget.close_friendly_npc_list_widget();
        self._save_load_widget.close_save_load_widget();
        self._game_debug_menu_widget.close_game_debug_menu();

        match tab {
            GameMenuTab::Inventory => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                self._inventory_widget.open_inventory();
            }
            GameMenuTab::Records => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                self._player_records_widget.open_player_records_widget();
            }
            GameMenuTab::TamingList => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                self._taming_list_widget.open_taming_list_widget();
            }
            GameMenuTab::FriendlyNpcList => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                self._friendly_npc_list_widget.open_friendly_npc_list_widget();
            }
            GameMenuTab::SaveLoad => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                self._save_load_widget.open_save_load_widget();
            }
            GameMenuTab::DebugMenu => {
                inv_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                records_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                taming_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                friendly_npc_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                saveload_tab_ui.set_color(TAB_BUTTON_COLOR_INACTIVE);
                debug_tab_ui.set_color(TAB_BUTTON_COLOR_ACTIVE);
                self._game_debug_menu_widget.open_game_debug_menu();
            }
        }
    }

    pub fn open_game_menu(&mut self, tab: Option<GameMenuTab>) {
        let target_tab = tab.unwrap_or(self._active_tab);
        if !self._is_opened_game_menu {
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            self._is_opened_game_menu = true;
            self.set_active_tab(target_tab);
        } else if self._active_tab != target_tab {
            self.set_active_tab(target_tab);
        }
    }

    pub fn close_game_menu(&mut self) {
        if self._is_opened_game_menu {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            self._inventory_widget.close_inventory();
            self._player_records_widget.close_player_records_widget();
            self._taming_list_widget.close_taming_list_widget();
            self._friendly_npc_list_widget.close_friendly_npc_list_widget();
            self._save_load_widget.close_save_load_widget();
            self._game_debug_menu_widget.close_game_debug_menu();
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_opened_game_menu = false;
        }
    }

    pub fn update_game_menu_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        player: &RcRefCell<Character>,
    ) {
        // Tab switching via Tab key or Joystick Shoulders (LB / RB)
        let switch_tab_next = keyboard_input_data.get_key_pressed(KeyCode::Tab)
            || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let switch_tab_prev = joystick_input_data._btn_left_shoulder == ButtonState::Pressed;

        if switch_tab_next {
            let next_tab = match self._active_tab {
                GameMenuTab::Inventory => GameMenuTab::Records,
                GameMenuTab::Records => GameMenuTab::TamingList,
                GameMenuTab::TamingList => GameMenuTab::FriendlyNpcList,
                GameMenuTab::FriendlyNpcList => GameMenuTab::SaveLoad,
                GameMenuTab::SaveLoad => GameMenuTab::DebugMenu,
                GameMenuTab::DebugMenu => GameMenuTab::Inventory,
            };
            self.set_active_tab(next_tab);
        } else if switch_tab_prev {
            let prev_tab = match self._active_tab {
                GameMenuTab::Inventory => GameMenuTab::DebugMenu,
                GameMenuTab::Records => GameMenuTab::Inventory,
                GameMenuTab::TamingList => GameMenuTab::Records,
                GameMenuTab::FriendlyNpcList => GameMenuTab::TamingList,
                GameMenuTab::SaveLoad => GameMenuTab::FriendlyNpcList,
                GameMenuTab::DebugMenu => GameMenuTab::SaveLoad,
            };
            self.set_active_tab(prev_tab);
        }

        match self._active_tab {
            GameMenuTab::Records => {
                self._player_records_widget.update_player_records_widget(joystick_input_data, keyboard_input_data);
                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
            GameMenuTab::TamingList => {
                self._taming_list_widget.update_taming_list_widget(joystick_input_data, keyboard_input_data);
                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
            GameMenuTab::FriendlyNpcList => {
                self._friendly_npc_list_widget
                    .update_friendly_npc_list_widget(joystick_input_data, keyboard_input_data);
                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
            GameMenuTab::SaveLoad => {
                self._save_load_widget.update_save_load_widget(time_data, joystick_input_data, keyboard_input_data);
                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
            GameMenuTab::Inventory => {
                self._inventory_widget.update_inventory_widget(
                    time_data,
                    joystick_input_data,
                    keyboard_input_data,
                    mouse_move_data,
                    mouse_input_data,
                    mouse_delta,
                    player,
                );

                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
            GameMenuTab::DebugMenu => {
                self._game_debug_menu_widget.update_game_debug_menu_widget(joystick_input_data, keyboard_input_data);
                let close_game_menu = keyboard_input_data.get_key_pressed(KeyCode::Escape)
                    || joystick_input_data._btn_b == ButtonState::Pressed;
                if close_game_menu {
                    self.close_game_menu();
                }
            }
        }
    }
}
