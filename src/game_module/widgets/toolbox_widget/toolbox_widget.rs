use crate::game_module::actors::character::Character;
use crate::game_module::widgets::toolbox_widget::item_tab_widget::{
    ToolboxIconType, ToolboxItemData, ToolboxTabWidget,
};
use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{
    ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData,
};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager,
    UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

const TAB_BUTTON_WIDTH: f32 = 100.0;
const TAB_BUTTON_HEIGHT: f32 = 36.0;
const TAB_ACTIVE_COLOR: u32 = get_color32(110, 110, 110, 255);
const TAB_INACTIVE_COLOR: u32 = get_color32(50, 50, 50, 255);

// ────────────────────────────────────────────────────────────────
// Tab enum
// ────────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolboxTab {
    Skill,
    Architecture,
    Cooking,
    ItemCraft,
    Vehicle,
    Weapon,
    Defense,
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
    pub _tab_btn_architecture: Rc<WidgetDefault<'a>>,
    pub _tab_btn_cooking: Rc<WidgetDefault<'a>>,
    pub _tab_btn_item_craft: Rc<WidgetDefault<'a>>,
    pub _tab_btn_vehicle: Rc<WidgetDefault<'a>>,
    pub _tab_btn_weapon: Rc<WidgetDefault<'a>>,
    pub _tab_btn_defense: Rc<WidgetDefault<'a>>,

    // Content panes
    pub _skill_tab: Box<ToolboxTabWidget<'a>>,
    pub _architecture_tab: Box<ToolboxTabWidget<'a>>,
    pub _cooking_tab: Box<ToolboxTabWidget<'a>>,
    pub _item_craft_tab: Box<ToolboxTabWidget<'a>>,
    pub _vehicle_tab: Box<ToolboxTabWidget<'a>>,
    pub _weapon_tab: Box<ToolboxTabWidget<'a>>,
    pub _defense_tab: Box<ToolboxTabWidget<'a>>,

    pub _active_tab: ToolboxTab,
}

impl<'a> ToolboxWidget<'a> {
    // ── Tab-button callbacks ──────────────────────────────────────

    pub fn callback_tab_skill(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Skill);
        true
    }
    pub fn callback_tab_architecture(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Architecture);
        true
    }
    pub fn callback_tab_cooking(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Cooking);
        true
    }
    pub fn callback_tab_item_craft(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::ItemCraft);
        true
    }
    pub fn callback_tab_vehicle(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Vehicle);
        true
    }
    pub fn callback_tab_weapon(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Weapon);
        true
    }
    pub fn callback_tab_defense(
        ui: &UIComponentInstance<'a>,
        _pos: &Vector2<f32>,
        _delta: &Vector2<f32>,
    ) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>)
            .set_active_tab(ToolboxTab::Defense);
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
        ui.set_size(TAB_BUTTON_WIDTH, TAB_BUTTON_HEIGHT);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin(4.0);
        ui.set_text(label);
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_round(6.0);
        ui.set_color(TAB_INACTIVE_COLOR);
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(callback)));
        header.add_widget(&btn);
        btn
    }

    // ── Constructor ───────────────────────────────────────────────

    pub fn create_toolbox_widget(
        parent_widget: &mut WidgetDefault<'a>,
    ) -> ToolboxWidget<'a> {
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
        ui.set_size_hint_x(Some(0.6));
        ui.set_size_hint_y(Some(0.65));
        ui.set_color(get_color32(35, 35, 35, 230));
        ui.set_border_color(get_color32(90, 90, 90, 255));
        ui.set_border(2.0);
        ui.set_round(10.0);
        ui.set_padding(8.0);

        // ── Title bar (Dark gray) ───────────────────────────────────
        let title_bar = UIManager::create_widget("toolbox_title_bar", UIWidgetTypes::Default);
        let title_mut = ptr_as_mut(title_bar.as_ref());
        let ui = title_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(44.0);
        ui.set_color(get_color32(25, 25, 25, 255));
        ui.set_border_color(get_color32(70, 70, 70, 255));
        ui.set_border(1.0);
        ui.set_round(8.0);
        ui.set_margin(4.0);
        layer_mut.add_widget(&title_bar);

        let title_label = UIManager::create_widget("toolbox_title", UIWidgetTypes::Default);
        let ui = ptr_as_mut(title_label.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(44.0);
        ui.set_text("Monolith Toolbox");
        ui.set_font_size(26.0);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_renderable(false);
        title_mut.add_widget(&title_label);

        // ── Tab header (Dark gray) ──────────────────────────────────
        let header = UIManager::create_widget("toolbox_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header.as_ref());
        let ui = header_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(TAB_BUTTON_HEIGHT + 12.0);
        ui.set_color(get_color32(25, 25, 25, 200));
        ui.set_border_color(get_color32(70, 70, 70, 200));
        ui.set_border(1.0);
        ui.set_round(6.0);
        ui.set_margin(4.0);
        ui.set_padding(4.0);
        layer_mut.add_widget(&header);

        let tab_btn_skill =
            Self::create_tab_button("tb_skill", "Skill", Self::callback_tab_skill, header_mut);
        let tab_btn_architecture = Self::create_tab_button(
            "tb_architecture",
            "Architecture",
            Self::callback_tab_architecture,
            header_mut,
        );
        let tab_btn_cooking =
            Self::create_tab_button("tb_cooking", "Cooking", Self::callback_tab_cooking, header_mut);
        let tab_btn_item_craft = Self::create_tab_button(
            "tb_item_craft",
            "Item Craft",
            Self::callback_tab_item_craft,
            header_mut,
        );
        let tab_btn_vehicle =
            Self::create_tab_button("tb_vehicle", "Vehicle", Self::callback_tab_vehicle, header_mut);
        let tab_btn_weapon =
            Self::create_tab_button("tb_weapon", "Weapon", Self::callback_tab_weapon, header_mut);
        let tab_btn_defense =
            Self::create_tab_button("tb_defense", "Defense", Self::callback_tab_defense, header_mut);

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
        ui.set_color(get_color32(30, 30, 30, 220));
        ui.set_round(6.0);
        ui.set_margin(4.0);
        layer_mut.add_widget(&content);

        // Build content panes for each tab with items requiring EnergyBall
        let skill_tab = ToolboxTabWidget::create(
            "skill",
            "Skill List",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "skill_hand".to_string(),
                    icon_type: ToolboxIconType::HandSkill,
                    description: "Passive hand-based crafting skill".to_string(),
                    energy_cost: 0,
                },
                ToolboxItemData {
                    id: "skill_gather".to_string(),
                    icon_type: ToolboxIconType::QuickGather,
                    description: "Increases resource gathering speed".to_string(),
                    energy_cost: 1,
                },
            ],
        );
        let architecture_tab = ToolboxTabWidget::create(
            "architecture",
            "Architecture Structures",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "arch_shelter".to_string(),
                    icon_type: ToolboxIconType::StoneShelter,
                    description: "Basic stone shelter structure for protection".to_string(),
                    energy_cost: 2,
                },
                ToolboxItemData {
                    id: "arch_tower".to_string(),
                    icon_type: ToolboxIconType::Watchtower,
                    description: "Provides high elevation view of surrounding area".to_string(),
                    energy_cost: 3,
                },
            ],
        );
        let cooking_tab = ToolboxTabWidget::create(
            "cooking",
            "Cooking Recipes",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "cook_roast".to_string(),
                    icon_type: ToolboxIconType::RoastMeat,
                    description: "Restores HP and Stamina when consumed".to_string(),
                    energy_cost: 1,
                },
                ToolboxItemData {
                    id: "cook_soup".to_string(),
                    icon_type: ToolboxIconType::FishSoup,
                    description: "Nutritious soup offering temporary stat buff".to_string(),
                    energy_cost: 1,
                },
            ],
        );
        let item_craft_tab = ToolboxTabWidget::create(
            "item_craft",
            "Item Crafting Recipes",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "craft_axe".to_string(),
                    icon_type: ToolboxIconType::StoneAxe,
                    description: "Essential harvesting tool for wood and stone".to_string(),
                    energy_cost: 1,
                },
                ToolboxItemData {
                    id: "craft_table".to_string(),
                    icon_type: ToolboxIconType::Worktable,
                    description: "Unlocks advanced recipe crafting".to_string(),
                    energy_cost: 2,
                },
            ],
        );
        let vehicle_tab = ToolboxTabWidget::create(
            "vehicle",
            "Vehicles & Mounts",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "vehicle_cart".to_string(),
                    icon_type: ToolboxIconType::WoodenCart,
                    description: "Transport vehicle that increases movement speed".to_string(),
                    energy_cost: 2,
                },
                ToolboxItemData {
                    id: "vehicle_mammoth".to_string(),
                    icon_type: ToolboxIconType::RidingMammoth,
                    description: "Heavy mount capable of carrying large loads".to_string(),
                    energy_cost: 5,
                },
            ],
        );
        let weapon_tab = ToolboxTabWidget::create(
            "weapon",
            "Weapons",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "weapon_spear".to_string(),
                    icon_type: ToolboxIconType::FlintSpear,
                    description: "Sharp melee weapon for hunting and battle".to_string(),
                    energy_cost: 2,
                },
                ToolboxItemData {
                    id: "weapon_bow".to_string(),
                    icon_type: ToolboxIconType::HuntingBow,
                    description: "Ranged weapon for distant targets".to_string(),
                    energy_cost: 3,
                },
            ],
        );
        let defense_tab = ToolboxTabWidget::create(
            "defense",
            "Defense Gear",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "defense_armor".to_string(),
                    icon_type: ToolboxIconType::LeatherArmor,
                    description: "Protective armor that increases defense stat".to_string(),
                    energy_cost: 2,
                },
                ToolboxItemData {
                    id: "defense_shield".to_string(),
                    icon_type: ToolboxIconType::BoneShield,
                    description: "Sturdy shield for blocking physical attacks".to_string(),
                    energy_cost: 2,
                },
            ],
        );

        let widget = ToolboxWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _is_opened_toolbox: false,
            _tab_btn_skill: tab_btn_skill,
            _tab_btn_architecture: tab_btn_architecture,
            _tab_btn_cooking: tab_btn_cooking,
            _tab_btn_item_craft: tab_btn_item_craft,
            _tab_btn_vehicle: tab_btn_vehicle,
            _tab_btn_weapon: tab_btn_weapon,
            _tab_btn_defense: tab_btn_defense,
            _skill_tab: skill_tab,
            _architecture_tab: architecture_tab,
            _cooking_tab: cooking_tab,
            _item_craft_tab: item_craft_tab,
            _vehicle_tab: vehicle_tab,
            _weapon_tab: weapon_tab,
            _defense_tab: defense_tab,
            _active_tab: ToolboxTab::Skill,
        };

        // Wire user_data on tab buttons → they need *const ToolboxWidget
        // (safe because ToolboxWidget is stored in a Box in GameUIManager)
        let self_ptr = &widget as *const ToolboxWidget<'a> as *const c_void;
        ptr_as_mut(widget._tab_btn_skill.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_architecture.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_cooking.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_item_craft.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_vehicle.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_weapon.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
        ptr_as_mut(widget._tab_btn_defense.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

        widget
    }

    // ── Tab switching ─────────────────────────────────────────────

    fn all_tab_buttons(&self) -> [&Rc<WidgetDefault<'a>>; 7] {
        [
            &self._tab_btn_skill,
            &self._tab_btn_architecture,
            &self._tab_btn_cooking,
            &self._tab_btn_item_craft,
            &self._tab_btn_vehicle,
            &self._tab_btn_weapon,
            &self._tab_btn_defense,
        ]
    }

    pub fn set_active_tab(&mut self, tab: ToolboxTab) {
        self._active_tab = tab;

        // Reset all buttons to inactive colour
        for btn in self.all_tab_buttons() {
            ptr_as_mut(btn.as_ref())
                .get_ui_component_mut()
                .set_color(TAB_INACTIVE_COLOR);
        }

        // Close all panes
        self._skill_tab.close();
        self._architecture_tab.close();
        self._cooking_tab.close();
        self._item_craft_tab.close();
        self._vehicle_tab.close();
        self._weapon_tab.close();
        self._defense_tab.close();

        // Activate selected tab
        let (active_btn, open_fn): (&Rc<WidgetDefault<'a>>, Box<dyn FnOnce(&mut ToolboxWidget<'a>)>) =
            match tab {
                ToolboxTab::Skill => (
                    &self._tab_btn_skill,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._skill_tab.open()),
                ),
                ToolboxTab::Architecture => (
                    &self._tab_btn_architecture,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._architecture_tab.open()),
                ),
                ToolboxTab::Cooking => (
                    &self._tab_btn_cooking,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._cooking_tab.open()),
                ),
                ToolboxTab::ItemCraft => (
                    &self._tab_btn_item_craft,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._item_craft_tab.open()),
                ),
                ToolboxTab::Vehicle => (
                    &self._tab_btn_vehicle,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._vehicle_tab.open()),
                ),
                ToolboxTab::Weapon => (
                    &self._tab_btn_weapon,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._weapon_tab.open()),
                ),
                ToolboxTab::Defense => (
                    &self._tab_btn_defense,
                    Box::new(|w: &mut ToolboxWidget<'a>| w._defense_tab.open()),
                ),
            };

        ptr_as_mut(active_btn.as_ref())
            .get_ui_component_mut()
            .set_color(TAB_ACTIVE_COLOR);
        open_fn(self);
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
            ptr_as_mut(self._tab_btn_architecture.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_cooking.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_item_craft.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_vehicle.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_weapon.as_ref()).get_ui_component_mut().set_user_data(self_ptr);
            ptr_as_mut(self._tab_btn_defense.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

            // Default to Skill tab
            self.set_active_tab(ToolboxTab::Skill);
        }
    }

    pub fn close_toolbox(&mut self) {
        if self._is_opened_toolbox {
            ptr_as_mut(self._parent_widget).remove_widget(self._layer.as_ref());
            self._is_opened_toolbox = false;
        }
    }

    // ── Per-frame update ──────────────────────────────────────────

    pub fn update_toolbox_widget(
        &mut self,
        _time_data: &TimeData,
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

        // Tab navigation (Keyboard Tab / Shift+Tab, Joystick LB / RB)
        let tab_pressed = keyboard_input_data.get_key_pressed(KeyCode::Tab);
        let is_shift = keyboard_input_data.get_key_hold(KeyCode::ShiftLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ShiftRight);

        let switch_tab_next = (tab_pressed && !is_shift)
            || joystick_input_data._btn_right_shoulder == ButtonState::Pressed;
        let switch_tab_prev = (tab_pressed && is_shift)
            || joystick_input_data._btn_left_shoulder == ButtonState::Pressed;

        if switch_tab_next {
            let next_tab = match self._active_tab {
                ToolboxTab::Skill => ToolboxTab::Architecture,
                ToolboxTab::Architecture => ToolboxTab::Cooking,
                ToolboxTab::Cooking => ToolboxTab::ItemCraft,
                ToolboxTab::ItemCraft => ToolboxTab::Vehicle,
                ToolboxTab::Vehicle => ToolboxTab::Weapon,
                ToolboxTab::Weapon => ToolboxTab::Defense,
                ToolboxTab::Defense => ToolboxTab::Skill,
            };
            self.set_active_tab(next_tab);
        } else if switch_tab_prev {
            let prev_tab = match self._active_tab {
                ToolboxTab::Skill => ToolboxTab::Defense,
                ToolboxTab::Architecture => ToolboxTab::Skill,
                ToolboxTab::Cooking => ToolboxTab::Architecture,
                ToolboxTab::ItemCraft => ToolboxTab::Cooking,
                ToolboxTab::Vehicle => ToolboxTab::ItemCraft,
                ToolboxTab::Weapon => ToolboxTab::Vehicle,
                ToolboxTab::Defense => ToolboxTab::Weapon,
            };
            self.set_active_tab(prev_tab);
        }

        let close = keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || joystick_input_data._btn_b == ButtonState::Pressed;

        if close {
            self.close_toolbox();
        }
    }
}
