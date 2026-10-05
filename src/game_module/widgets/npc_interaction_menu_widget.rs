use crate::game_module::actors::character::{Character, RequestType};
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
use crate::game_module::game_service_locator::{get_character_manager, get_game_client_mut};
use crate::game_module::widgets::key_binding_widget::KEY_BINDING_FONT_SIZE;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER_LEFT, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub const NPC_INTERACTION_MENU_WIDTH: f32 = 280.0;
pub const NPC_INTERACTION_MENU_BUTTON_HEIGHT: f32 = 36.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpcInteractionOption {
    Talk,
    Request,
    Dance,
    GiveItem,
    Close,
}

impl NpcInteractionOption {
    pub fn get_display_name(&self, request_name: Option<&str>, item_name: Option<&str>) -> String {
        match self {
            NpcInteractionOption::Talk => String::from("Talk"),
            NpcInteractionOption::Request => {
                if let Some(req) = request_name {
                    format!("{}", req)
                } else {
                    String::from("Request Menu")
                }
            }
            NpcInteractionOption::Dance => String::from("Dance"),
            NpcInteractionOption::GiveItem => {
                if let Some(name) = item_name {
                    format!("Give {}", name)
                } else {
                    String::from("Give Item")
                }
            }
            NpcInteractionOption::Close => String::from("Close"),
        }
    }

    pub fn get_key_hint(&self, index: usize) -> String {
        match self {
            NpcInteractionOption::Close => String::from("ESC"),
            _ => format!("{}", index + 1),
        }
    }
}

pub struct NpcInteractionButton<'a> {
    pub _option: NpcInteractionOption,
    pub _widget: Rc<WidgetDefault<'a>>,
}

pub struct NpcInteractionMenuWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _frame: Rc<WidgetDefault<'a>>,
    pub _button_container: Rc<WidgetDefault<'a>>,
    pub _buttons: Vec<NpcInteractionButton<'a>>,
    pub _target_npc: Option<RcRefCell<Character<'a>>>,
    pub _selected_index: usize,
    pub _is_opened: bool,
}

impl<'a> NpcInteractionMenuWidget<'a> {
    pub fn create_npc_interaction_menu_widget(parent_widget: &mut WidgetDefault<'a>) -> NpcInteractionMenuWidget<'a> {
        let parent_ptr = parent_widget as *const WidgetDefault<'a>;

        let layer = UIManager::create_widget("npc_interaction_menu_layer", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_pivot_preset(PIVOT_CENTER_LEFT);
            ui_comp.set_renderable(false);
            ui_comp.set_expandable_y(true);
            ui_comp.set_enable(false);
        }

        let frame = UIManager::create_widget("npc_interaction_menu_frame", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(frame.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::LEFT);
            ui_comp.set_valign(VerticalAlign::TOP);
            ui_comp.set_expandable_y(true);
            ui_comp.set_padding(8.0);
            ui_comp.set_renderable(false);
        }
        ptr_as_mut(layer.as_ref()).add_widget(&frame);

        let button_container = UIManager::create_widget("npc_interaction_menu_btn_container", UIWidgetTypes::Default);
        {
            let ui_comp = ptr_as_mut(button_container.as_ref()).get_ui_component_mut();
            ui_comp.set_layout_type(UILayoutType::BoxLayout);
            ui_comp.set_layout_orientation(Orientation::VERTICAL);
            ui_comp.set_halign(HorizontalAlign::CENTER);
            ui_comp.set_valign(VerticalAlign::CENTER);
            ui_comp.set_margin(2.0);
            ui_comp.set_renderable(false);
            ui_comp.set_expandable_y(true);
        }
        ptr_as_mut(frame.as_ref()).add_widget(&button_container);

        NpcInteractionMenuWidget {
            _parent_widget: parent_ptr,
            _layer: layer,
            _frame: frame,
            _button_container: button_container,
            _buttons: Vec::new(),
            _target_npc: None,
            _selected_index: 0,
            _is_opened: false,
        }
    }

    pub fn is_opened_npc_interaction_menu(&self) -> bool {
        self._is_opened
    }

    pub fn open_npc_interaction_menu(&mut self, target_npc: RcRefCell<Character<'a>>) {
        let _npc_name = target_npc.borrow()._character_data.borrow()._name.clone();
        self._target_npc = Some(target_npc.clone());
        self._selected_index = 0;

        self.rebuild_buttons();
        self.update_selected_visuals();

        if !self._is_opened {
            self._is_opened = true;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(true);
            get_game_client_mut().set_next_game_phase(GamePhase::Interaction);
        }
    }

    pub fn close_npc_interaction_menu(&mut self) {
        if self._is_opened {
            self._is_opened = false;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            self._target_npc = None;
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
            if get_game_client_mut().is_game_phase(GamePhase::Interaction) {
                get_game_client_mut().set_next_game_phase(GamePhase::GamePlay);
            }
        }
    }

    fn update_selected_visuals(&mut self) {
        for (i, btn) in self._buttons.iter().enumerate() {
            let is_selected = i == self._selected_index;
            let ui_comp = ptr_as_mut(btn._widget.as_ref()).get_ui_component_mut();
            if is_selected {
                ui_comp.set_color(get_color32(40, 130, 220, 230));
                ui_comp.set_border_color(get_color32(255, 220, 100, 255));
                ui_comp.set_border(2.0);
            } else {
                ui_comp.set_color(get_color32(0, 0, 0, 160));
                ui_comp.set_border_color(get_color32(0, 0, 0, 0));
                ui_comp.set_border(0.0);
            }
        }
    }

    fn rebuild_buttons(&mut self) {
        let btn_container_mut = ptr_as_mut(self._button_container.as_ref());
        btn_container_mut.clear_widgets();
        self._buttons.clear();

        let target_npc_rc = match &self._target_npc {
            Some(npc) => npc.clone(),
            None => return,
        };

        let character_manager = get_character_manager();
        let player = if character_manager.is_valid_player() {
            Some(character_manager.get_player().borrow())
        } else {
            None
        };

        let target_npc_ref = target_npc_rc.borrow();
        let request_type = target_npc_ref.get_request_type();
        let request_type_str = match request_type {
            RequestType::Cooking => Some("Cooking"),
            RequestType::Craft => Some("Craft"),
            _ => None,
        };

        let mut options = Vec::new();
        options.push(NpcInteractionOption::Talk);

        if request_type_str.is_some() {
            options.push(NpcInteractionOption::Request);
        }

        options.push(NpcInteractionOption::Dance);

        if let Some(p) = &player {
            if p.get_attached_item_data_type().is_eatable() {
                options.push(NpcInteractionOption::GiveItem);
            }
        }

        options.push(NpcInteractionOption::Close);

        let eatable_item_name = player
            .as_ref()
            .and_then(|p| p.get_attached_item().as_ref().map(|item| item.borrow()._item_data.borrow()._name.clone()));

        for (index, option) in options.into_iter().enumerate() {
            let btn_widget = UIManager::create_widget(
                format!("npc_interact_btn_{:?}", option).as_str(),
                UIWidgetTypes::Default,
            );
            {
                let ui_comp = ptr_as_mut(btn_widget.as_ref()).get_ui_component_mut();
                ui_comp.set_layout_type(UILayoutType::BoxLayout);
                ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
                ui_comp.set_halign(HorizontalAlign::LEFT);
                ui_comp.set_valign(VerticalAlign::CENTER);
                ui_comp.set_size(NPC_INTERACTION_MENU_WIDTH, NPC_INTERACTION_MENU_BUTTON_HEIGHT);
                ui_comp.set_margin(3.0);
                ui_comp.set_color(get_color32(0, 0, 0, 160));
                ui_comp.set_round(10.0);
                ui_comp.set_touchable(true);

                // key hint icon / badge
                let hint_widget = UIManager::create_widget("btn_hint_badge", UIWidgetTypes::Default);
                {
                    let hint_ui = ptr_as_mut(hint_widget.as_ref()).get_ui_component_mut();
                    hint_ui.set_size(30.0, 30.0);
                    hint_ui.set_halign(HorizontalAlign::CENTER);
                    hint_ui.set_valign(VerticalAlign::CENTER);
                    hint_ui.set_margin_left(6.0);
                    hint_ui.set_color(get_color32(45, 120, 190, 220));
                    hint_ui.set_round(6.0);
                    hint_ui.set_text(option.get_key_hint(index).as_str());
                    hint_ui.set_font_size(KEY_BINDING_FONT_SIZE);
                    hint_ui.set_font_color(get_color32(255, 255, 255, 255));
                }
                ptr_as_mut(btn_widget.as_ref()).add_widget(&hint_widget);

                // text widget
                let text_widget = UIManager::create_widget("btn_text_label", UIWidgetTypes::Default);
                {
                    let text_ui = ptr_as_mut(text_widget.as_ref()).get_ui_component_mut();
                    text_ui.set_expandable_x(true);
                    text_ui.set_size_y(NPC_INTERACTION_MENU_BUTTON_HEIGHT);
                    text_ui.set_halign(HorizontalAlign::LEFT);
                    text_ui.set_valign(VerticalAlign::CENTER);
                    text_ui.set_margin_left(8.0);
                    text_ui.set_font_size(KEY_BINDING_FONT_SIZE);
                    text_ui.set_font_color(get_color32(255, 255, 255, 255));
                    text_ui.set_color(get_color32(0, 0, 0, 0));
                    text_ui.set_text(option.get_display_name(request_type_str, eatable_item_name.as_deref()).as_str());
                }
                ptr_as_mut(btn_widget.as_ref()).add_widget(&text_widget);

                let self_ptr = self as *mut Self as *const c_void;
                ui_comp.set_user_data(self_ptr);
                ui_comp.set_callback_touch_down(Some(Box::new(Self::on_click_option_button)));
            }

            btn_container_mut.add_widget(&btn_widget);
            self._buttons.push(NpcInteractionButton {
                _option: option,
                _widget: btn_widget,
            });
        }
    }

    fn on_click_option_button(
        widget: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _touch_pos: &Vector2<f32>,
        _touch_delta: &Vector2<f32>,
    ) -> bool {
        let user_data = widget.get_user_data();
        if user_data.is_null() {
            return false;
        }
        let menu_widget = unsafe { &mut *(user_data as *mut NpcInteractionMenuWidget<'a>) };
        let clicked_ptr = widget as *const rust_engine_3d::scene::ui::UIComponentInstance<'a>;

        let matched = menu_widget._buttons.iter().enumerate().find_map(|(idx, b)| {
            let comp_ptr = ptr_as_ref(b._widget.as_ref()).get_ui_component()
                as *const rust_engine_3d::scene::ui::UIComponentInstance<'a>;
            if comp_ptr == clicked_ptr {
                Some((idx, b._option))
            } else {
                None
            }
        });

        if let Some((idx, option)) = matched {
            menu_widget._selected_index = idx;
            menu_widget.update_selected_visuals();
            menu_widget.execute_option(option);
            true
        } else {
            false
        }
    }

    pub fn execute_option(&mut self, option: NpcInteractionOption) {
        let target_npc = match &self._target_npc {
            Some(npc) => npc.clone(),
            None => {
                self.close_npc_interaction_menu();
                return;
            }
        };

        self.close_npc_interaction_menu();

        let character_manager = get_character_manager();
        if !character_manager.is_valid_player() {
            return;
        }
        let mut player = character_manager.get_player().borrow_mut();

        match option {
            NpcInteractionOption::Talk => {
                player.execute_npc_talk(&target_npc);
            }
            NpcInteractionOption::Request => {
                player.execute_npc_request(&target_npc);
            }
            NpcInteractionOption::Dance => {
                player.execute_npc_dance(&target_npc);
            }
            NpcInteractionOption::GiveItem => {
                player.execute_npc_give_item(&target_npc);
            }
            NpcInteractionOption::Close => {}
        }
    }

    pub fn update(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
    ) {
        if !self._is_opened {
            return;
        }

        let target_npc = match &self._target_npc {
            Some(npc) => npc.clone(),
            None => {
                self.close_npc_interaction_menu();
                return;
            }
        };

        // Update screen position tracking based on NPC world position
        let main_camera = get_scene_manager().get_main_camera();
        let target_pos = *target_npc.borrow().get_position() + nalgebra::Vector3::new(0.0, 1.8, 0.0);
        let screen_pos =
            main_camera.convert_world_to_screen(&target_pos, true) / rust_engine_3d::scene::ui::get_global_dpi_scale();

        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_pos(screen_pos.x, screen_pos.y);

        let btn_count = self._buttons.len();
        if btn_count == 0 {
            return;
        }

        // Cancel (ESC, B key, Joystick Button B)
        let press_cancel = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyB)
            || joystick_input_data._btn_b == ButtonState::Pressed;

        if press_cancel {
            self.close_npc_interaction_menu();
            return;
        }

        // Navigation (Up/Down / WS / DPad)
        let press_up = keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || joystick_input_data._btn_up == ButtonState::Pressed;

        let press_down = keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || joystick_input_data._btn_down == ButtonState::Pressed;

        // Execution (Enter, Space, E key, Joystick Button A)
        let press_execute = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyE)
            || joystick_input_data._btn_a == ButtonState::Pressed;

        if press_up {
            if self._selected_index == 0 {
                self._selected_index = btn_count - 1;
            } else {
                self._selected_index -= 1;
            }
            self.update_selected_visuals();
        } else if press_down {
            self._selected_index = (self._selected_index + 1) % btn_count;
            self.update_selected_visuals();
        } else if press_execute {
            let option = self._buttons[self._selected_index]._option;
            self.execute_option(option);
            return;
        }

        // Digit shortcut selection (1, 2, 3, 4, etc.)
        let key_codes = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
        ];

        for (i, key_code) in key_codes.iter().enumerate() {
            if keyboard_input_data.get_key_pressed(*key_code) {
                if i < self._buttons.len() {
                    let option = self._buttons[i]._option;
                    self._selected_index = i;
                    self.update_selected_visuals();
                    self.execute_option(option);
                    return;
                }
            }
        }
    }
}
