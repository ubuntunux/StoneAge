use crate::game_module::actors::character::{Character, CharacterDataType, RequestType};
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_QUEST_COMPLETE, AUDIO_SELECT_ITEM, ITEM_NONE};
use crate::game_module::game_input::UiInput;
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_client_mut, get_game_scene_manager, get_game_scene_manager_mut,
    get_game_ui_manager, get_item_manager_mut, get_prop_manager_mut,
};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::scenario::scenario::ScenarioType;
use crate::game_module::widgets::key_binding_widget::KEY_BINDING_FONT_SIZE;
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::core::input::{JoystickInputData, KeyboardInputData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER_LEFT, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

pub const NPC_INTERACTION_MENU_WIDTH: f32 = 280.0;
pub const NPC_INTERACTION_MENU_BUTTON_HEIGHT: f32 = 36.0;

#[derive(Clone, Default)]
pub enum InteractionMenuTarget<'a> {
    #[default]
    None,
    Npc(RcRefCell<Character<'a>>),
    Bed(RcRefCell<Prop<'a>>),
    Table(RcRefCell<Prop<'a>>),
    Prop(RcRefCell<Prop<'a>>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpcInteractionOption {
    Talk,
    Request,
    Cooking,
    StorageTable,
    GiveAllResources,
    WrapUpTheDay,
    Dance,
    GiveItem,
    Sleep,
    CookItem(usize),
    Retrieve,
    Close,
}

impl NpcInteractionOption {
    pub fn get_display_name(&self, request_name: Option<&str>, item_name: Option<&str>) -> String {
        match self {
            NpcInteractionOption::Talk => String::from("Talk"),
            NpcInteractionOption::Request => {
                if let Some(req) = request_name {
                    req.to_string()
                } else {
                    String::from("Request Menu")
                }
            }
            NpcInteractionOption::Cooking => String::from("Cooking"),
            NpcInteractionOption::StorageTable => String::from("Supply Ingredients"),
            NpcInteractionOption::GiveAllResources => String::from("Give All Resources"),
            NpcInteractionOption::WrapUpTheDay => String::from("Wrap Up the Day"),
            NpcInteractionOption::Dance => String::from("Dance"),
            NpcInteractionOption::GiveItem => {
                if let Some(name) = item_name {
                    format!("Give {}", name)
                } else {
                    String::from("Give Item")
                }
            }
            NpcInteractionOption::Sleep => String::from("Sleep"),
            NpcInteractionOption::CookItem(slot_idx) => {
                let item_name = get_game_ui_manager()
                    .get_table_storage_widget()
                    .and_then(|w| w._table_inventory_slots.get(*slot_idx))
                    .map(|slot| slot._item_name.as_str())
                    .unwrap_or("Food");
                format!("Cook {}", item_name)
            }
            NpcInteractionOption::Retrieve => String::from("Retrieve to Quick Slot"),
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
    pub _target: InteractionMenuTarget<'a>,
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
            ui_comp.set_margin_left(150.0);
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
            _target: InteractionMenuTarget::None,
            _selected_index: 0,
            _is_opened: false,
        }
    }

    pub fn is_opened_npc_interaction_menu(&self) -> bool {
        self._is_opened
    }

    fn open_with_target(&mut self, target: InteractionMenuTarget<'a>) {
        self._target = target;
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

    pub fn open_npc_interaction_menu(&mut self, target_npc: RcRefCell<Character<'a>>) {
        let character_manager = get_character_manager();
        if let Some(player) = character_manager.get_maybe_player() {
            let mut player_mut = player.borrow_mut();
            let mut npc_mut = target_npc.borrow_mut();
            player_mut.look_at(npc_mut.get_position());
            player_mut.set_move_idle();
            npc_mut.look_at(player_mut.get_position());
            npc_mut.set_move_idle();
            npc_mut.set_is_interacting(true);
            npc_mut.set_next_behavior(BehaviorState::Idle, true);
        }
        self.open_with_target(InteractionMenuTarget::Npc(target_npc));
    }

    pub fn open_bed_interaction_menu(&mut self, target_bed: RcRefCell<Prop<'a>>) {
        self.open_with_target(InteractionMenuTarget::Bed(target_bed));
    }

    pub fn open_table_interaction_menu(&mut self, target_table: RcRefCell<Prop<'a>>) {
        if get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay)
            && !get_game_ui_manager().has_eatable_table_storage_item()
        {
            return;
        }
        self.open_with_target(InteractionMenuTarget::Table(target_table));
    }

    pub fn open_prop_interaction_menu(&mut self, target_prop: RcRefCell<Prop<'a>>) {
        self.open_with_target(InteractionMenuTarget::Prop(target_prop));
    }

    pub fn close_npc_interaction_menu(&mut self) {
        if self._is_opened {
            self._is_opened = false;
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
            ptr_as_mut(self._layer.as_ref()).get_ui_component_mut().set_enable(false);
            if let InteractionMenuTarget::Npc(npc) = &self._target {
                npc.borrow_mut().set_is_interacting(false);
            }
            self._target = InteractionMenuTarget::None;
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

        let character_manager = get_character_manager();
        let player = if character_manager.is_valid_player() {
            Some(character_manager.get_player().borrow())
        } else {
            None
        };

        let mut request_type_str: Option<&str> = None;
        let mut options = Vec::new();

        match &self._target {
            InteractionMenuTarget::Bed(prop) => {
                options.push(NpcInteractionOption::Sleep);
                if prop.borrow().is_deployed_prop() {
                    options.push(NpcInteractionOption::Retrieve);
                }
                options.push(NpcInteractionOption::Close);
            }
            InteractionMenuTarget::Table(prop) => {
                if let Some(table_storage_widget) = get_game_ui_manager().get_table_storage_widget() {
                    let table_slots = &table_storage_widget._table_inventory_slots;
                    let mut seen_items = std::collections::HashSet::new();
                    for (slot_idx, slot) in table_slots.iter().enumerate() {
                        if slot._item_count > 0
                            && slot._item_data_name != ITEM_NONE
                            && slot._item_data_type.is_eatable()
                            && seen_items.insert(slot._item_data_name.clone())
                        {
                            options.push(NpcInteractionOption::CookItem(slot_idx));
                        }
                    }
                }
                if prop.borrow().is_deployed_prop() {
                    options.push(NpcInteractionOption::Retrieve);
                }
                options.push(NpcInteractionOption::Close);
            }
            InteractionMenuTarget::Prop(prop) => {
                if prop.borrow().is_deployed_prop() {
                    options.push(NpcInteractionOption::Retrieve);
                }
                options.push(NpcInteractionOption::Close);
            }
            InteractionMenuTarget::Npc(target_npc_rc) => {
                let target_npc_ref = target_npc_rc.borrow();
                let is_wife = target_npc_ref.get_character_type() == CharacterDataType::Wife;
                let request_type = target_npc_ref.get_request_type();
                request_type_str = match request_type {
                    RequestType::Cooking => Some("Cooking"),
                    RequestType::Craft => Some("Craft"),
                    _ => None,
                };

                options.push(NpcInteractionOption::Talk);

                if request_type_str.is_some() {
                    options.push(NpcInteractionOption::Request);
                }

                if is_wife {
                    options.push(NpcInteractionOption::StorageTable);
                    options.push(NpcInteractionOption::GiveAllResources);
                }

                options.push(NpcInteractionOption::Dance);

                if let Some(p) = &player
                    && p.get_attached_item_data_type().is_eatable()
                {
                    options.push(NpcInteractionOption::GiveItem);
                }

                options.push(NpcInteractionOption::Close);
            }
            InteractionMenuTarget::None => {
                return;
            }
        }

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
                ui_comp.set_callback_touch_over(Some(Box::new(Self::on_touch_over_option_button)));
            }

            btn_container_mut.add_widget(&btn_widget);
            self._buttons.push(NpcInteractionButton {
                _option: option,
                _widget: btn_widget,
            });
        }
    }

    fn find_button_index_by_ui_component(
        &self,
        clicked_comp: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
    ) -> Option<usize> {
        let clicked_ptr = clicked_comp as *const rust_engine_3d::scene::ui::UIComponentInstance<'a>;
        self._buttons.iter().enumerate().find_map(|(idx, b)| {
            let comp_ptr = ptr_as_ref(b._widget.as_ref()).get_ui_component()
                as *const rust_engine_3d::scene::ui::UIComponentInstance<'a>;
            if comp_ptr == clicked_ptr { Some(idx) } else { None }
        })
    }

    fn on_touch_over_option_button(
        widget: &rust_engine_3d::scene::ui::UIComponentInstance<'a>,
        _touch_pos: &Vector2<f32>,
        _touch_delta: &Vector2<f32>,
    ) -> bool {
        let user_data = widget.get_user_data();
        if user_data.is_null() {
            return false;
        }
        let menu_widget = unsafe { &mut *(user_data as *mut NpcInteractionMenuWidget<'a>) };

        if let Some(idx) = menu_widget.find_button_index_by_ui_component(widget) {
            if menu_widget._selected_index != idx {
                menu_widget._selected_index = idx;
                menu_widget.update_selected_visuals();
            }
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            true
        } else {
            false
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

        if let Some(idx) = menu_widget.find_button_index_by_ui_component(widget) {
            let option = menu_widget._buttons[idx]._option;
            menu_widget._selected_index = idx;
            menu_widget.update_selected_visuals();
            menu_widget.execute_option(option);
            true
        } else {
            false
        }
    }

    pub fn execute_option(&mut self, option: NpcInteractionOption) {
        let previous_target = self._target.clone();
        self.close_npc_interaction_menu();

        let character_manager = get_character_manager();
        if !character_manager.is_valid_player() {
            return;
        }

        let target_npc = match &previous_target {
            InteractionMenuTarget::Npc(npc) => Some(npc.clone()),
            _ => None,
        };
        let target_table = match &previous_target {
            InteractionMenuTarget::Table(table) => Some(table.clone()),
            _ => None,
        };
        let target_prop = match &previous_target {
            InteractionMenuTarget::Bed(prop)
            | InteractionMenuTarget::Table(prop)
            | InteractionMenuTarget::Prop(prop) => Some(prop.clone()),
            _ => None,
        };

        match option {
            NpcInteractionOption::Cooking => {
                get_game_client_mut().set_next_game_phase(GamePhase::OpenCooking);
            }
            NpcInteractionOption::Talk => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_talk(target_npc);
                }
            }
            NpcInteractionOption::Request => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_request(target_npc);
                }
            }
            NpcInteractionOption::StorageTable => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_table_storage(target_npc);
                } else if target_table.is_some() {
                    get_game_client_mut().set_next_game_phase(GamePhase::OpenTableStorage);
                }
            }
            NpcInteractionOption::GiveAllResources => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_give_all_materials(target_npc);
                }
            }
            NpcInteractionOption::WrapUpTheDay => {
                if let Some(player) = character_manager.get_maybe_player() {
                    player.borrow_mut().set_move_idle();
                }
                get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                if let Some(scenario) =
                    get_game_scene_manager_mut().get_game_scenario(ScenarioType::ScenarioIntro_Intro)
                {
                    scenario.borrow_mut().complete_sub_quest_wrap_up_the_day();
                }
                get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
                get_game_client_mut().set_next_game_phase(GamePhase::WrapUpTheDay);
            }
            NpcInteractionOption::Dance => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_dance(target_npc);
                }
            }
            NpcInteractionOption::GiveItem => {
                if let Some(target_npc) = &target_npc {
                    character_manager.get_player().borrow_mut().execute_npc_give_item(target_npc);
                }
            }
            NpcInteractionOption::Sleep => {
                get_game_scene_manager_mut().request_open_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
                get_game_client_mut().set_next_game_phase(GamePhase::WrapUpTheDay);
            }
            NpcInteractionOption::CookItem(slot_idx) => {
                get_game_scene_manager()
                    .get_item_manager_mut()
                    .place_table_storage_item_on_table(target_table, slot_idx);
            }
            NpcInteractionOption::Retrieve => {
                if let Some(target_prop) = target_prop {
                    let deployed_item_name = target_prop.borrow()._deployed_item_data_name.clone();
                    if !deployed_item_name.is_empty() {
                        let success = get_item_manager_mut().pick_item(&deployed_item_name, 1);
                        if success {
                            let mut player = character_manager.get_player().borrow_mut();
                            player
                                ._controller
                                .remove_interaction_object(InteractionObject::PropDeployed(target_prop.clone()));
                            player
                                ._controller
                                .remove_interaction_object(InteractionObject::PropTable(target_prop.clone()));
                            player
                                ._controller
                                .remove_interaction_object(InteractionObject::PropBed(target_prop.clone()));
                            get_prop_manager_mut().remove_prop(&target_prop);
                        }
                    }
                }
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

        if let InteractionMenuTarget::Npc(npc) = &self._target {
            if let Some(player) = get_character_manager().get_maybe_player() {
                let mut npc_mut = npc.borrow_mut();
                npc_mut.look_at(player.borrow().get_position());
                if !npc_mut.is_move_stop() {
                    npc_mut.set_move_idle();
                }
            }
        }

        let target_pos = match &self._target {
            InteractionMenuTarget::Npc(npc) => *npc.borrow().get_position() + Vector3::new(0.0, 1.8, 0.0),
            InteractionMenuTarget::Bed(prop)
            | InteractionMenuTarget::Table(prop)
            | InteractionMenuTarget::Prop(prop) => {
                let prop_borrow = prop.borrow();
                *prop_borrow.get_position() + Vector3::new(0.0, 1.0, 0.0)
            }
            InteractionMenuTarget::None => {
                self.close_npc_interaction_menu();
                return;
            }
        };

        // Update screen position tracking based on target world position
        let main_camera = get_scene_manager().get_main_camera();
        let screen_pos =
            main_camera.convert_world_to_screen(&target_pos, true) / rust_engine_3d::scene::ui::get_global_dpi_scale();

        let layer_ui = ptr_as_mut(self._layer.as_ref()).get_ui_component_mut();
        layer_ui.set_pos(screen_pos.x, screen_pos.y);

        let btn_count = self._buttons.len();
        if btn_count == 0 {
            return;
        }

        // Cancel (ESC, B key, Joystick Button B)
        if UiInput::is_cancel(keyboard_input_data, joystick_input_data) {
            self.close_npc_interaction_menu();
            return;
        }

        // Navigation (Up/Down / WS / DPad)
        let press_up = UiInput::is_up(keyboard_input_data, joystick_input_data);
        let press_down = UiInput::is_down(keyboard_input_data, joystick_input_data);

        // Execution (Enter, Space, Joystick Button A)
        let press_execute = UiInput::is_confirm(keyboard_input_data, joystick_input_data);

        if press_up {
            if self._selected_index == 0 {
                self._selected_index = btn_count - 1;
            } else {
                self._selected_index -= 1;
            }
            self.update_selected_visuals();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            if let Some(btn) = self._buttons.get(self._selected_index) {
                move_mouse_to_ui_component(btn._widget.get_ui_component());
            }
        } else if press_down {
            self._selected_index = (self._selected_index + 1) % btn_count;
            self.update_selected_visuals();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            if let Some(btn) = self._buttons.get(self._selected_index) {
                move_mouse_to_ui_component(btn._widget.get_ui_component());
            }
        } else if press_execute {
            let option = self._buttons[self._selected_index]._option;
            self.execute_option(option);
            return;
        }

        // Digit shortcut selection (1, 2, 3, 4, etc.)
        if let Some(digit) = UiInput::get_pressed_digit(keyboard_input_data) {
            let i = digit.saturating_sub(1);
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
