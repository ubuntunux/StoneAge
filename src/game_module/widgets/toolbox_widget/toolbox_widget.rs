use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{
    get_game_resources, get_game_scene_manager, get_game_scene_manager_mut,
};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::toolbox_widget::item_tab_widget::{
    ToolboxIconType, ToolboxItemData, ToolboxItemState, ToolboxTabWidget,
};
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
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

const TAB_BUTTON_HEIGHT: f32 = 44.0;
const TAB_ACTIVE_COLOR: u32 = get_color32(195, 118, 58, 240);
const TAB_INACTIVE_COLOR: u32 = get_color32(165, 130, 90, 230);

// ────────────────────────────────────────────────────────────────
// Tab enum
// ────────────────────────────────────────────────────────────────
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ToolboxTab {
    #[default]
    Skill,
    ItemCraft,
    Npc,
    Teleport,
}

impl ToolboxTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolboxTab::Skill => "Skill",
            ToolboxTab::ItemCraft => "ItemCraft",
            ToolboxTab::Npc => "Npc",
            ToolboxTab::Teleport => "Teleport",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "ItemCraft" => ToolboxTab::ItemCraft,
            "Npc" => ToolboxTab::Npc,
            "Teleport" => ToolboxTab::Teleport,
            _ => ToolboxTab::Skill,
        }
    }
}

// ────────────────────────────────────────────────────────────────
// ToolboxWidget
// ────────────────────────────────────────────────────────────────
pub struct ToolboxWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _is_opened_toolbox: bool,

    // Tab buttons
    pub _tab_btn_skill: Rc<WidgetDefault<'a>>,
    pub _tab_btn_item_craft: Rc<WidgetDefault<'a>>,
    pub _tab_btn_npc: Rc<WidgetDefault<'a>>,
    pub _tab_btn_teleport: Rc<WidgetDefault<'a>>,
    pub _close_btn: Rc<WidgetDefault<'a>>,

    // Content panes
    pub _skill_tab: Box<ToolboxTabWidget<'a>>,
    pub _item_craft_tab: Box<ToolboxTabWidget<'a>>,
    pub _npc_tab: Box<ToolboxTabWidget<'a>>,
    pub _teleport_tab: Box<ToolboxTabWidget<'a>>,

    pub _active_tab: ToolboxTab,
    pub _last_opened_tab: ToolboxTab,
    pub _selected_item_index: usize,
    pub _last_lstick_y: i16,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

fn get_toolbox_item_data(icon_type: ToolboxIconType) -> ToolboxItemData {
    let resources = get_game_resources();
    let res_name = icon_type.toolbox_data_name();
    if resources.has_toolbox_item_data(res_name) {
        resources.get_toolbox_item_data(res_name).borrow().clone()
    } else {
        ToolboxItemData {
            _icon_type: icon_type,
            _description: "".to_string(),
            _item_data_type: ItemDataType::None,
            _item_data_count: 0,
        }
    }
}

impl<'a> ToolboxWidget<'a> {
    // ── Tab-button callbacks ──────────────────────────────────────

    pub fn callback_tab_skill(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Skill);
        true
    }
    pub fn callback_tab_item_craft(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::ItemCraft);
        true
    }
    pub fn callback_tab_npc(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Npc);
        true
    }
    pub fn callback_tab_teleport(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Teleport);
        true
    }

    pub fn callback_close(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).close_toolbox();
        true
    }

    pub fn callback_tab_touch_over(_ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    // ── Tab button helper ─────────────────────────────────────────

    fn create_tab_button(
        name: &str,
        label: &str,
        callback: rust_engine_3d::scene::ui::CallbackTouchEvent<'a>,
        header: &mut WidgetDefault<'a>,
    ) -> Rc<WidgetDefault<'a>> {
        let btn = UIManager::create_widget(name, UIWidgetTypes::Default);
        let ui = ptr_as_mut(btn.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin(3.0);
        ui.set_text(label);
        ui.set_font_size(24.0);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_round(6.0);
        ui.set_color(TAB_INACTIVE_COLOR);
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(callback)));
        ui.set_callback_touch_over(Some(Box::new(Self::callback_tab_touch_over)));
        header.add_widget(&btn);
        btn
    }

    // ── Constructor ───────────────────────────────────────────────

    pub fn create_toolbox_widget(parent_widget: &mut WidgetDefault<'a>) -> ToolboxWidget<'a> {
        // ── Root layer (Neutral dark gray) ──────────────────────────
        let layer = UIManager::create_widget("toolbox_widget", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui = layer_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_size(760.0, 580.0);
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
        ui.set_color(get_color32(240, 222, 186, 248));
        ui.set_border_color(get_color32(135, 78, 42, 255));
        ui.set_border(2.0);
        ui.set_round(10.0);
        ui.set_padding(8.0);

        // ── Main Body Container (Vertical: Top Header + Bottom Content) ──
        let body = UIManager::create_widget("toolbox_body", UIWidgetTypes::Default);
        let body_mut = ptr_as_mut(body.as_ref());
        let ui = body_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_renderable(false);
        layer_mut.add_widget(&body);

        // ── Horizontal Tab Header Bar (Dark gray) ─────────────────
        let header = UIManager::create_widget("toolbox_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header.as_ref());
        let ui = header_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(TAB_BUTTON_HEIGHT + 8.0);
        ui.set_color(get_color32(215, 185, 140, 230));
        ui.set_border_color(get_color32(160, 115, 70, 220));
        ui.set_border(1.0);
        ui.set_round(6.0);
        ui.set_margin(4.0);
        ui.set_padding(4.0);
        body_mut.add_widget(&header);

        let tab_btn_skill = Self::create_tab_button("tb_skill", "Skill", Self::callback_tab_skill, header_mut);
        let tab_btn_item_craft =
            Self::create_tab_button("tb_item_craft", "Craft", Self::callback_tab_item_craft, header_mut);
        let tab_btn_npc = Self::create_tab_button("tb_npc", "NPC", Self::callback_tab_npc, header_mut);
        let tab_btn_teleport =
            Self::create_tab_button("tb_teleport", "Teleport", Self::callback_tab_teleport, header_mut);

        let close_btn = UIManager::create_widget("close_btn", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(close_btn.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_size(35.0, 35.0);
        ui_component.set_margin(3.0);
        ui_component.set_text("X");
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(6.0);
        ui_component.set_color(get_color32(200, 75, 60, 255));
        ui_component.set_touchable(true);
        ui_component.set_callback_touch_down(Some(Box::new(Self::callback_close)));
        ui_component.set_callback_touch_over(Some(Box::new(Self::callback_tab_touch_over)));
        header_mut.add_widget(&close_btn);

        // ── Content area (Dark gray) ────────────────────────────────
        let content = UIManager::create_widget("toolbox_content", UIWidgetTypes::Default);
        let content_mut = ptr_as_mut(content.as_ref());
        let ui = content_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
        ui.set_color(get_color32(252, 245, 226, 230));
        ui.set_round(6.0);
        ui.set_margin(4.0);
        body_mut.add_widget(&content);

        // Build content panes for each tab with items requiring EnergyBall
        let skill_tab = ToolboxTabWidget::create_toolbox_tab_widget(
            "skill",
            content_mut,
            vec![
                get_toolbox_item_data(ToolboxIconType::HandSkill),
                get_toolbox_item_data(ToolboxIconType::QuickGather),
            ],
        );
        let item_craft_tab = ToolboxTabWidget::create_toolbox_tab_widget(
            "item_craft",
            content_mut,
            vec![
                get_toolbox_item_data(ToolboxIconType::WoodenClub),
                get_toolbox_item_data(ToolboxIconType::StoneAxe),
                get_toolbox_item_data(ToolboxIconType::FlintSpear),
                get_toolbox_item_data(ToolboxIconType::HuntingBow),
                get_toolbox_item_data(ToolboxIconType::LeatherArmor),
                get_toolbox_item_data(ToolboxIconType::BoneShield),
                get_toolbox_item_data(ToolboxIconType::Campfire),
                get_toolbox_item_data(ToolboxIconType::Worktable),
            ],
        );
        let npc_tab = ToolboxTabWidget::create_toolbox_tab_widget(
            "npc",
            content_mut,
            vec![
                get_toolbox_item_data(ToolboxIconType::NpcGatherer),
                get_toolbox_item_data(ToolboxIconType::NpcCrafter),
                get_toolbox_item_data(ToolboxIconType::NpcGuard),
                get_toolbox_item_data(ToolboxIconType::NpcHunter),
            ],
        );

        let teleport_tab = ToolboxTabWidget::create_toolbox_tab_widget(
            "teleport",
            content_mut,
            vec![
                get_toolbox_item_data(ToolboxIconType::MapHome),
                get_toolbox_item_data(ToolboxIconType::MapForest),
                get_toolbox_item_data(ToolboxIconType::MapCave),
                get_toolbox_item_data(ToolboxIconType::MapUfo),
            ],
        );

        let widget = ToolboxWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _is_opened_toolbox: false,
            _tab_btn_skill: tab_btn_skill,
            _tab_btn_item_craft: tab_btn_item_craft,
            _tab_btn_npc: tab_btn_npc,
            _tab_btn_teleport: tab_btn_teleport,
            _close_btn: close_btn,
            _skill_tab: skill_tab,
            _item_craft_tab: item_craft_tab,
            _npc_tab: npc_tab,
            _teleport_tab: teleport_tab,
            _active_tab: ToolboxTab::Skill,
            _last_opened_tab: ToolboxTab::Skill,
            _selected_item_index: 0,
            _last_lstick_y: 0,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        };

        // Wire user_data on tab buttons → they need *const ToolboxWidget
        // (safe because ToolboxWidget is stored in a Box in GameUIManager)
        let self_ptr = &widget as *const ToolboxWidget<'a> as *const c_void;
        ptr_as_mut(widget._tab_btn_skill.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_item_craft.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_npc.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_teleport.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._close_btn.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

        widget
    }

    // ── Tab switching ─────────────────────────────────────────────

    fn all_tab_buttons(&self) -> [&Rc<WidgetDefault<'a>>; 4] {
        [
            &self._tab_btn_skill,
            &self._tab_btn_item_craft,
            &self._tab_btn_npc,
            &self._tab_btn_teleport,
        ]
    }

    pub fn set_active_tab(&mut self, tab: ToolboxTab) {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);

        self._active_tab = tab;
        self._last_opened_tab = tab;

        // Reset all buttons to inactive colour
        for btn in self.all_tab_buttons() {
            ptr_as_mut(btn.as_ref()).get_ui_component_mut().set_color(TAB_INACTIVE_COLOR);
        }

        // Close all panes
        self._skill_tab.close();
        self._item_craft_tab.close();
        self._npc_tab.close();
        self._teleport_tab.close();

        // Activate selected tab
        let (active_btn, open_fn): (&Rc<WidgetDefault<'a>>, Box<dyn FnOnce(&mut ToolboxWidget<'a>)>) = match tab {
            ToolboxTab::Skill => (
                &self._tab_btn_skill,
                Box::new(|w: &mut ToolboxWidget<'a>| w._skill_tab.open()),
            ),
            ToolboxTab::ItemCraft => (
                &self._tab_btn_item_craft,
                Box::new(|w: &mut ToolboxWidget<'a>| w._item_craft_tab.open()),
            ),
            ToolboxTab::Npc => (
                &self._tab_btn_npc,
                Box::new(|w: &mut ToolboxWidget<'a>| w._npc_tab.open()),
            ),
            ToolboxTab::Teleport => (
                &self._tab_btn_teleport,
                Box::new(|w: &mut ToolboxWidget<'a>| {
                    let current_stage_name = get_game_scene_manager().get_current_game_scene_data_name().clone();
                    if !current_stage_name.is_empty()
                        && get_game_scene_manager().get_discovered_world_data(&current_stage_name).is_none()
                    {
                        get_game_scene_manager_mut().inspect_and_register_discovered_world_data();
                    }
                    w._teleport_tab.open();
                }),
            ),
        };

        ptr_as_mut(active_btn.as_ref()).get_ui_component_mut().set_color(TAB_ACTIVE_COLOR);
        open_fn(self);

        self._selected_item_index = 0;
        self.update_item_selection();
    }

    pub fn get_active_tab_mut(&mut self) -> &mut ToolboxTabWidget<'a> {
        match self._active_tab {
            ToolboxTab::Skill => &mut self._skill_tab,
            ToolboxTab::ItemCraft => &mut self._item_craft_tab,
            ToolboxTab::Npc => &mut self._npc_tab,
            ToolboxTab::Teleport => &mut self._teleport_tab,
        }
    }

    pub fn update_item_selection(&mut self) {
        let selected_idx = self._selected_item_index;
        let active_tab = self.get_active_tab_mut();
        active_tab.select_item(selected_idx);
    }

    // ── Open / Close ──────────────────────────────────────────────

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

    pub fn is_opened_toolbox(&self) -> bool {
        self._is_opened_toolbox
    }

    pub fn open_toolbox(&mut self) {
        if !self._is_opened_toolbox {
            ptr_as_mut(self._parent_widget).add_widget(&self._layer);
            self._is_opened_toolbox = true;

            // Update self_ptr on all tab buttons after being placed in its final location
            let self_ptr = self as *const ToolboxWidget<'a> as *const c_void;
            ptr_as_mut(self._tab_btn_skill.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_item_craft.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_npc.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_teleport.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._close_btn.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

            // Restore last opened tab
            let last_tab = self._last_opened_tab;
            self.set_active_tab(last_tab);
        }
    }

    pub fn close_toolbox(&mut self) {
        if self._is_opened_toolbox {
            self._teleport_tab.close();
            ptr_as_mut(self._parent_widget).remove_widget(self._layer.as_ref());
            self._is_opened_toolbox = false;
        }
    }

    // ── Per-frame update ──────────────────────────────────────────

    pub fn update_toolbox_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character>,
    ) {
        if !self.is_opened_toolbox() {
            return;
        }

        // Refresh material counts for active tab
        self.get_active_tab_mut().update_detail_panel();

        // Tab navigation (Keyboard Tab / Shift+Tab, Joystick LB / RB)
        let tab_pressed = keyboard_input_data.get_key_pressed(KeyCode::Tab);
        let is_shift = keyboard_input_data.get_key_hold(KeyCode::ShiftLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ShiftRight);

        let switch_tab_next =
            (tab_pressed && !is_shift) || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let switch_tab_prev =
            (tab_pressed && is_shift) || joystick_input_data._btn_left_shoulder == ButtonState::Pressed;

        if switch_tab_next {
            let next_tab = match self._active_tab {
                ToolboxTab::Skill => ToolboxTab::ItemCraft,
                ToolboxTab::ItemCraft => ToolboxTab::Npc,
                ToolboxTab::Npc => ToolboxTab::Teleport,
                ToolboxTab::Teleport => ToolboxTab::Skill,
            };
            self.set_active_tab(next_tab);
        } else if switch_tab_prev {
            let prev_tab = match self._active_tab {
                ToolboxTab::Skill => ToolboxTab::Teleport,
                ToolboxTab::ItemCraft => ToolboxTab::Skill,
                ToolboxTab::Npc => ToolboxTab::ItemCraft,
                ToolboxTab::Teleport => ToolboxTab::Npc,
            };
            self.set_active_tab(prev_tab);
        }

        // Item navigation (with hold repeat)
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        let item_count = self.get_active_tab_mut()._items.len();
        if should_move && item_count > 0 {
            let (_dir_x, dir_y) = dir_opt.unwrap();
            if dir_y < 0 {
                if self._selected_item_index == 0 {
                    self._selected_item_index = item_count - 1;
                } else {
                    self._selected_item_index -= 1;
                }
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                self.update_item_selection();
            } else if dir_y > 0 {
                if self._selected_item_index + 1 >= item_count {
                    self._selected_item_index = 0;
                } else {
                    self._selected_item_index += 1;
                }
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                self.update_item_selection();
            }
            let sel_idx = self._selected_item_index;
            let active_tab = self.get_active_tab_mut();
            if let Some(item) = active_tab._items.get(sel_idx) {
                move_mouse_to_ui_component(item._layout.get_ui_component());
            }
        }

        // Action / Unlock / Teleport confirm (Keyboard Enter/Space, Joystick A/X)
        let action_pressed = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed
            || joystick_input_data._btn_x == ButtonState::Pressed;

        if action_pressed {
            self.get_active_tab_mut().execute_selected_action();
        }

        let close =
            keyboard_input_data.get_key_pressed(KeyCode::Escape) || joystick_input_data._btn_b == ButtonState::Pressed;

        if close {
            self.close_toolbox();
        }
    }

    pub fn get_unlocked_items(&self) -> HashSet<ToolboxIconType> {
        let mut unlocked = HashSet::new();
        let tabs = [
            &self._skill_tab,
            &self._item_craft_tab,
            &self._npc_tab,
            &self._teleport_tab,
        ];
        for tab in tabs {
            for item in &tab._items {
                if item._state == ToolboxItemState::Unlocked {
                    unlocked.insert(item._data._icon_type);
                }
            }
        }
        unlocked
    }

    pub fn load_unlocked_items(&mut self, unlocked_set: &HashSet<ToolboxIconType>) {
        let tabs = [
            &mut self._skill_tab,
            &mut self._item_craft_tab,
            &mut self._npc_tab,
            &mut self._teleport_tab,
        ];
        for tab in tabs {
            for item in &mut tab._items {
                if unlocked_set.contains(&item._data._icon_type) {
                    item._state = ToolboxItemState::Unlocked;
                } else {
                    item._state = ToolboxItemState::Locked;
                }
            }
            tab.update_detail_panel();
        }
    }

    pub fn get_last_opened_tab(&self) -> ToolboxTab {
        self._last_opened_tab
    }

    pub fn set_last_opened_tab(&mut self, tab: ToolboxTab) {
        self._last_opened_tab = tab;
        self._active_tab = self._last_opened_tab;
    }
}
