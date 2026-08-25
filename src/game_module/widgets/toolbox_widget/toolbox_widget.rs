use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::AUDIO_PICKUP_ITEM;
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
use std::collections::HashSet;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

const TAB_BUTTON_WIDTH: f32 = 50.0;
const TAB_BUTTON_HEIGHT: f32 = 40.0;
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
    Npc,
}

impl ToolboxTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolboxTab::Skill => "Skill",
            ToolboxTab::Architecture => "Architecture",
            ToolboxTab::Cooking => "Cooking",
            ToolboxTab::ItemCraft => "ItemCraft",
            ToolboxTab::Vehicle => "Vehicle",
            ToolboxTab::Weapon => "Weapon",
            ToolboxTab::Defense => "Defense",
            ToolboxTab::Npc => "Npc",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Architecture" => ToolboxTab::Architecture,
            "Cooking" => ToolboxTab::Cooking,
            "ItemCraft" => ToolboxTab::ItemCraft,
            "Vehicle" => ToolboxTab::Vehicle,
            "Weapon" => ToolboxTab::Weapon,
            "Defense" => ToolboxTab::Defense,
            "Npc" => ToolboxTab::Npc,
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
    pub _tab_btn_architecture: Rc<WidgetDefault<'a>>,
    pub _tab_btn_cooking: Rc<WidgetDefault<'a>>,
    pub _tab_btn_item_craft: Rc<WidgetDefault<'a>>,
    pub _tab_btn_vehicle: Rc<WidgetDefault<'a>>,
    pub _tab_btn_weapon: Rc<WidgetDefault<'a>>,
    pub _tab_btn_defense: Rc<WidgetDefault<'a>>,
    pub _tab_btn_npc: Rc<WidgetDefault<'a>>,

    // Content panes
    pub _skill_tab: Box<ToolboxTabWidget<'a>>,
    pub _architecture_tab: Box<ToolboxTabWidget<'a>>,
    pub _cooking_tab: Box<ToolboxTabWidget<'a>>,
    pub _item_craft_tab: Box<ToolboxTabWidget<'a>>,
    pub _vehicle_tab: Box<ToolboxTabWidget<'a>>,
    pub _weapon_tab: Box<ToolboxTabWidget<'a>>,
    pub _defense_tab: Box<ToolboxTabWidget<'a>>,
    pub _npc_tab: Box<ToolboxTabWidget<'a>>,

    pub _active_tab: ToolboxTab,
    pub _last_opened_tab: ToolboxTab,
    pub _selected_item_index: usize,
    pub _last_lstick_y: i16,
}

impl<'a> ToolboxWidget<'a> {
    // ── Tab-button callbacks ──────────────────────────────────────

    pub fn callback_tab_skill(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Skill);
        true
    }
    pub fn callback_tab_architecture(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Architecture);
        true
    }
    pub fn callback_tab_cooking(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Cooking);
        true
    }
    pub fn callback_tab_item_craft(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::ItemCraft);
        true
    }
    pub fn callback_tab_vehicle(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Vehicle);
        true
    }
    pub fn callback_tab_weapon(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Weapon);
        true
    }
    pub fn callback_tab_defense(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Defense);
        true
    }
    pub fn callback_tab_npc(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        ptr_as_mut(ui.get_user_data() as *const ToolboxWidget<'a>).set_active_tab(ToolboxTab::Npc);
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
        ui.set_margin(3.0);
        ui.set_text(label);
        ui.set_font_size(20.0);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_round(6.0);
        ui.set_color(TAB_INACTIVE_COLOR);
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(callback)));
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
        ui.set_size_hint_x(Some(0.6));
        ui.set_size_hint_y(Some(0.65));
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
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

        // ── Main Body Container (Horizontal: Left Header + Right Content) ──
        let body = UIManager::create_widget("toolbox_body", UIWidgetTypes::Default);
        let body_mut = ptr_as_mut(body.as_ref());
        let ui = body_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_renderable(false);
        layer_mut.add_widget(&body);

        // ── Vertical Tab Header Sidebar (Dark gray) ─────────────────
        let header = UIManager::create_widget("toolbox_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header.as_ref());
        let ui = header_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_x(TAB_BUTTON_WIDTH + 10.0);
        ui.set_size_hint_y(Some(1.0));
        ui.set_color(get_color32(25, 25, 25, 200));
        ui.set_border_color(get_color32(70, 70, 70, 200));
        ui.set_border(1.0);
        ui.set_round(6.0);
        ui.set_margin(4.0);
        ui.set_padding(4.0);
        body_mut.add_widget(&header);

        let tab_btn_skill = Self::create_tab_button("tb_skill", "Skill", Self::callback_tab_skill, header_mut);
        let tab_btn_architecture =
            Self::create_tab_button("tb_architecture", "Arch", Self::callback_tab_architecture, header_mut);
        let tab_btn_cooking = Self::create_tab_button("tb_cooking", "Cook", Self::callback_tab_cooking, header_mut);
        let tab_btn_item_craft =
            Self::create_tab_button("tb_item_craft", "Craft", Self::callback_tab_item_craft, header_mut);
        let tab_btn_vehicle = Self::create_tab_button("tb_vehicle", "Vehicle", Self::callback_tab_vehicle, header_mut);
        let tab_btn_weapon = Self::create_tab_button("tb_weapon", "Weapon", Self::callback_tab_weapon, header_mut);
        let tab_btn_defense = Self::create_tab_button("tb_defense", "Defense", Self::callback_tab_defense, header_mut);
        let tab_btn_npc = Self::create_tab_button("tb_npc", "NPC", Self::callback_tab_npc, header_mut);

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
        ui.set_color(get_color32(30, 30, 30, 220));
        ui.set_round(6.0);
        ui.set_margin(4.0);
        body_mut.add_widget(&content);

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
                ToolboxItemData {
                    id: "craft_campfire".to_string(),
                    icon_type: ToolboxIconType::Campfire,
                    description: "Provides light, warmth, and cooking capability".to_string(),
                    energy_cost: 1,
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
        let npc_tab = ToolboxTabWidget::create(
            "npc",
            "NPC Recruitment",
            content_mut,
            vec![
                ToolboxItemData {
                    id: "npc_gatherer".to_string(),
                    icon_type: ToolboxIconType::NpcGatherer,
                    description: "Collects wood and wild plants for the village".to_string(),
                    energy_cost: 2,
                },
                ToolboxItemData {
                    id: "npc_crafter".to_string(),
                    icon_type: ToolboxIconType::NpcCrafter,
                    description: "Crafts tools and building items automatically".to_string(),
                    energy_cost: 3,
                },
                ToolboxItemData {
                    id: "npc_guard".to_string(),
                    icon_type: ToolboxIconType::NpcGuard,
                    description: "Defends the base against wild beasts and threats".to_string(),
                    energy_cost: 4,
                },
                ToolboxItemData {
                    id: "npc_hunter".to_string(),
                    icon_type: ToolboxIconType::NpcHunter,
                    description: "Hunts animals and gathers meat and leather".to_string(),
                    energy_cost: 3,
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
            _tab_btn_npc: tab_btn_npc,
            _skill_tab: skill_tab,
            _architecture_tab: architecture_tab,
            _cooking_tab: cooking_tab,
            _item_craft_tab: item_craft_tab,
            _vehicle_tab: vehicle_tab,
            _weapon_tab: weapon_tab,
            _defense_tab: defense_tab,
            _npc_tab: npc_tab,
            _active_tab: ToolboxTab::Skill,
            _last_opened_tab: ToolboxTab::Skill,
            _selected_item_index: 0,
            _last_lstick_y: 0,
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
        ptr_as_mut(widget._tab_btn_npc.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

        widget
    }

    // ── Tab switching ─────────────────────────────────────────────

    fn all_tab_buttons(&self) -> [&Rc<WidgetDefault<'a>>; 8] {
        [
            &self._tab_btn_skill,
            &self._tab_btn_architecture,
            &self._tab_btn_cooking,
            &self._tab_btn_item_craft,
            &self._tab_btn_vehicle,
            &self._tab_btn_weapon,
            &self._tab_btn_defense,
            &self._tab_btn_npc,
        ]
    }

    pub fn set_active_tab(&mut self, tab: ToolboxTab) {
        self._active_tab = tab;
        self._last_opened_tab = tab;

        // Reset all buttons to inactive colour
        for btn in self.all_tab_buttons() {
            ptr_as_mut(btn.as_ref()).get_ui_component_mut().set_color(TAB_INACTIVE_COLOR);
        }

        // Close all panes
        self._skill_tab.close();
        self._architecture_tab.close();
        self._cooking_tab.close();
        self._item_craft_tab.close();
        self._vehicle_tab.close();
        self._weapon_tab.close();
        self._defense_tab.close();
        self._npc_tab.close();

        // Activate selected tab
        let (active_btn, open_fn): (&Rc<WidgetDefault<'a>>, Box<dyn FnOnce(&mut ToolboxWidget<'a>)>) = match tab {
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
            ToolboxTab::Npc => (
                &self._tab_btn_npc,
                Box::new(|w: &mut ToolboxWidget<'a>| w._npc_tab.open()),
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
            ToolboxTab::Architecture => &mut self._architecture_tab,
            ToolboxTab::Cooking => &mut self._cooking_tab,
            ToolboxTab::ItemCraft => &mut self._item_craft_tab,
            ToolboxTab::Vehicle => &mut self._vehicle_tab,
            ToolboxTab::Weapon => &mut self._weapon_tab,
            ToolboxTab::Defense => &mut self._defense_tab,
            ToolboxTab::Npc => &mut self._npc_tab,
        }
    }

    pub fn update_item_selection(&mut self) {
        let selected_idx = self._selected_item_index;
        let active_tab = self.get_active_tab_mut();
        let item_count = active_tab._items.len();
        if item_count == 0 {
            return;
        }

        for (idx, item) in active_tab._items.iter_mut().enumerate() {
            let is_selected = idx == selected_idx;
            let layout_ui = ptr_as_mut(item._layout.as_ref()).get_ui_component_mut();
            if is_selected {
                layout_ui.set_border_color(get_color32(180, 180, 180, 255));
                layout_ui.set_color(get_color32(65, 65, 65, 230));
            } else {
                layout_ui.set_border_color(get_color32(80, 80, 80, 255));
                layout_ui.set_color(get_color32(45, 45, 45, 200));
            }
        }
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
            ptr_as_mut(self._tab_btn_npc.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

            // Restore last opened tab
            let last_tab = self._last_opened_tab;
            self.set_active_tab(last_tab);
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

        // Refresh material counts for active tab
        for item in self.get_active_tab_mut()._items.iter_mut() {
            item.update_ui();
        }

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
                ToolboxTab::Skill => ToolboxTab::Architecture,
                ToolboxTab::Architecture => ToolboxTab::Cooking,
                ToolboxTab::Cooking => ToolboxTab::ItemCraft,
                ToolboxTab::ItemCraft => ToolboxTab::Vehicle,
                ToolboxTab::Vehicle => ToolboxTab::Weapon,
                ToolboxTab::Weapon => ToolboxTab::Defense,
                ToolboxTab::Defense => ToolboxTab::Npc,
                ToolboxTab::Npc => ToolboxTab::Skill,
            };
            self.set_active_tab(next_tab);
        } else if switch_tab_prev {
            let prev_tab = match self._active_tab {
                ToolboxTab::Skill => ToolboxTab::Npc,
                ToolboxTab::Architecture => ToolboxTab::Skill,
                ToolboxTab::Cooking => ToolboxTab::Architecture,
                ToolboxTab::ItemCraft => ToolboxTab::Cooking,
                ToolboxTab::Vehicle => ToolboxTab::ItemCraft,
                ToolboxTab::Weapon => ToolboxTab::Vehicle,
                ToolboxTab::Defense => ToolboxTab::Weapon,
                ToolboxTab::Npc => ToolboxTab::Defense,
            };
            self.set_active_tab(prev_tab);
        }

        // Item navigation (Keyboard W/S, Up/Down, Joystick D-Pad Up/Down, Left Stick Up/Down)
        let move_up = keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || joystick_input_data._btn_up == ButtonState::Pressed
            || (joystick_input_data._stick_left_direction.y > 10000 && self._last_lstick_y <= 10000);

        let move_down = keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || joystick_input_data._btn_down == ButtonState::Pressed
            || (joystick_input_data._stick_left_direction.y < -10000 && self._last_lstick_y >= -10000);

        self._last_lstick_y = joystick_input_data._stick_left_direction.y;

        let item_count = self.get_active_tab_mut()._items.len();
        if item_count > 0 {
            if move_up {
                if self._selected_item_index == 0 {
                    self._selected_item_index = item_count - 1;
                } else {
                    self._selected_item_index -= 1;
                }
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                self.update_item_selection();
            } else if move_down {
                if self._selected_item_index + 1 >= item_count {
                    self._selected_item_index = 0;
                } else {
                    self._selected_item_index += 1;
                }
                get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                self.update_item_selection();
            }
        }

        // Action / Unlock confirm (Keyboard Enter/Space, Joystick A/X)
        let action_pressed = keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed
            || joystick_input_data._btn_x == ButtonState::Pressed;

        if action_pressed {
            let selected_idx = self._selected_item_index;
            let active_tab = self.get_active_tab_mut();
            if selected_idx < active_tab._items.len() {
                active_tab._items[selected_idx].toggle_state();
            }
        }

        let close =
            keyboard_input_data.get_key_pressed(KeyCode::Escape) || joystick_input_data._btn_b == ButtonState::Pressed;

        if close {
            self.close_toolbox();
        }
    }

    pub fn get_unlocked_items(&self) -> HashSet<String> {
        let mut unlocked = HashSet::new();
        let tabs = [
            &self._skill_tab,
            &self._architecture_tab,
            &self._cooking_tab,
            &self._item_craft_tab,
            &self._vehicle_tab,
            &self._weapon_tab,
            &self._defense_tab,
            &self._npc_tab,
        ];
        for tab in tabs {
            for item in &tab._items {
                if item._state == ToolboxItemState::Unlocked {
                    unlocked.insert(item._data.id.clone());
                }
            }
        }
        unlocked
    }

    pub fn load_unlocked_items(&mut self, unlocked_set: &HashSet<String>) {
        let tabs = [
            &mut self._skill_tab,
            &mut self._architecture_tab,
            &mut self._cooking_tab,
            &mut self._item_craft_tab,
            &mut self._vehicle_tab,
            &mut self._weapon_tab,
            &mut self._defense_tab,
            &mut self._npc_tab,
        ];
        for tab in tabs {
            for item in &mut tab._items {
                if unlocked_set.contains(&item._data.id) {
                    item._state = ToolboxItemState::Unlocked;
                    item.update_ui();
                }
            }
        }
    }

    pub fn get_last_opened_tab(&self) -> String {
        self._last_opened_tab.as_str().to_string()
    }

    pub fn set_last_opened_tab(&mut self, tab_name: &str) {
        self._last_opened_tab = ToolboxTab::from_str(tab_name);
    }
}
