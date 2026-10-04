use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_QUEST_COMPLETE, DEFAULT_GATE_NAME};
use crate::game_module::game_service_locator::{
    get_character_manager, get_character_manager_mut, get_game_resources, get_game_scene_manager,
    get_game_scene_manager_mut, get_game_ui_manager, get_game_ui_manager_mut,
};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::rc::Rc;

// ────────────────────────────────────────────────────────────────
// UI Layout & Dimension Constants
// ────────────────────────────────────────────────────────────────
pub const LIST_ITEM_ROW_HEIGHT: f32 = 54.0;
pub const LEFT_LIST_PANEL_WIDTH: f32 = 290.0;
pub const LEFT_LIST_PANEL_HEIGHT: f32 = 480.0;
pub const ACTION_BUTTON_WIDTH: f32 = 150.0;
pub const ACTION_BUTTON_HEIGHT: f32 = 40.0;
pub const ACTION_BUTTON_BOX_HEIGHT: f32 = ACTION_BUTTON_HEIGHT + 10.0;

pub const LIST_ITEM_ICON_SIZE: f32 = 40.0;
pub const LIST_ITEM_NAME_HEIGHT: f32 = 28.0;
pub const LIST_ITEM_BTN_WIDTH: f32 = 80.0;
pub const LIST_ITEM_BTN_HEIGHT: f32 = 32.0;

pub const DETAIL_HEADER_HEIGHT: f32 = 54.0;
pub const DETAIL_ICON_SIZE: f32 = 48.0;
pub const DETAIL_NAME_LABEL_HEIGHT: f32 = 48.0;
pub const DESCRIPTION_LABEL_HEIGHT: f32 = 22.0;
pub const REQUIREMENT_HEADER_HEIGHT: f32 = 20.0;
pub const INGREDIENT_SET_HEIGHT: f32 = 36.0;
pub const INGREDIENT_ICON_SIZE: f32 = 30.0;
pub const INGREDIENT_LABEL_HEIGHT: f32 = 30.0;

// ────────────────────────────────────────────────────────────────
// Font Size Constants
// ────────────────────────────────────────────────────────────────
pub const FONT_SIZE_TITLE: f32 = 24.0;
pub const FONT_SIZE_NORMAL: f32 = 20.0;
pub const FONT_SIZE_BUTTON: f32 = 18.0;
pub const DESCRIPTION_LABEL_FONT_SIZE: f32 = 20.0;

// ────────────────────────────────────────────────────────────────
// Margin, Padding & Border Constants
// ────────────────────────────────────────────────────────────────
pub const TAB_PADDING: f32 = 8.0;
pub const LIST_CONTAINER_PADDING: f32 = 4.0;
pub const DETAIL_CONTAINER_PADDING: f32 = 14.0;
pub const BOX_PADDING: f32 = 8.0;
pub const LIST_ITEM_PADDING: f32 = 6.0;

pub const BORDER_WIDTH_NORMAL: f32 = 2.0;
pub const BORDER_WIDTH_THICK: f32 = 2.0;
pub const CORNER_ROUND_NORMAL: f32 = 6.0;

// ────────────────────────────────────────────────────────────────
// Color Constants
// ────────────────────────────────────────────────────────────────
pub const COLOR_TRANSPARENT: u32 = get_color32(0, 0, 0, 0);
pub const COLOR_WHITE: u32 = get_color32(255, 255, 255, 255);

// Background & Border Colors
pub const COLOR_TAB_BG: u32 = get_color32(30, 30, 30, 220);
pub const COLOR_PANEL_BG: u32 = get_color32(25, 27, 30, 220);
pub const COLOR_PANEL_BORDER: u32 = get_color32(50, 55, 60, 255);
pub const COLOR_DETAIL_BG: u32 = get_color32(35, 38, 43, 230);
pub const COLOR_DETAIL_BORDER: u32 = get_color32(65, 70, 78, 255);
pub const COLOR_BOX_BG: u32 = get_color32(28, 30, 34, 200);
pub const COLOR_BOX_BORDER: u32 = get_color32(55, 60, 68, 255);

// List Item Colors
pub const COLOR_ITEM_NORMAL_BG: u32 = get_color32(40, 43, 48, 220);
pub const COLOR_ITEM_NORMAL_BORDER: u32 = get_color32(65, 70, 78, 255);
pub const COLOR_ITEM_SELECTED_BG: u32 = get_color32(60, 70, 85, 230);
pub const COLOR_ITEM_SELECTED_BORDER: u32 = get_color32(110, 160, 220, 255);

// Text Colors
pub const COLOR_TEXT_TITLE: u32 = get_color32(240, 240, 240, 255);
pub const COLOR_TEXT_NORMAL: u32 = get_color32(220, 225, 230, 255);
pub const COLOR_TEXT_MUTED: u32 = get_color32(190, 195, 205, 255);
pub const COLOR_TEXT_DISABLED: u32 = get_color32(150, 150, 150, 255);
pub const COLOR_TEXT_DISABLED_ALT: u32 = get_color32(120, 120, 120, 255);
pub const COLOR_TEXT_SUCCESS: u32 = get_color32(130, 220, 160, 255);
pub const COLOR_TEXT_WARNING: u32 = get_color32(240, 180, 120, 255);
pub const COLOR_TEXT_ERROR: u32 = get_color32(235, 100, 100, 255);
pub const COLOR_STATUS_LOCKED: u32 = get_color32(210, 165, 160, 255);
pub const COLOR_STATUS_UNLOCKED: u32 = get_color32(100, 210, 120, 255);

// Button Colors
pub const COLOR_BTN_UNLOCK_BG: u32 = get_color32(75, 130, 85, 255);
pub const COLOR_BTN_UNLOCK_BORDER: u32 = get_color32(115, 190, 130, 255);
pub const COLOR_BTN_TELEPORT_BG: u32 = get_color32(50, 110, 180, 255);
pub const COLOR_BTN_TELEPORT_BORDER: u32 = get_color32(90, 160, 240, 255);
pub const COLOR_BTN_DISABLED_BG: u32 = get_color32(45, 48, 52, 255);
pub const COLOR_BTN_DISABLED_BORDER: u32 = get_color32(65, 70, 75, 255);
pub const COLOR_BTN_UNLOCKED_BG: u32 = get_color32(40, 45, 50, 255);
pub const COLOR_BTN_UNLOCKED_BORDER: u32 = get_color32(70, 75, 80, 255);
pub const COLOR_BTN_DEFAULT_BG: u32 = get_color32(65, 65, 65, 255);
pub const COLOR_BTN_DEFAULT_BORDER: u32 = get_color32(100, 100, 100, 255);

fn spawn_npc_near_monolith(character_data_name: &str, offset: Vector3<f32>) {
    let monolith_pos = if let Some(monolith) = get_game_scene_manager().get_prop_manager().get_prop_by_name("monolith")
    {
        *monolith.borrow().get_position()
    } else if get_character_manager().is_valid_player() {
        *get_character_manager().get_player().borrow().get_position()
    } else {
        Vector3::zeros()
    };

    let spawn_pos = monolith_pos + offset;

    let character_create_info = CharacterCreateInfo {
        _character_id: Default::default(),
        _character_data_name: character_data_name.to_string(),
        _position: spawn_pos,
        _rotation: Vector3::zeros(),
        _scale: Vector3::new(1.0, 1.0, 1.0),
    };

    let character_name = format!("npc_{}", character_data_name.replace('/', "_"));
    get_character_manager_mut().create_character(&character_name, &character_create_info, false);
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum ToolboxIconType {
    #[default]
    HandSkill,
    QuickGather,
    StoneShelter,
    Watchtower,
    RoastMeat,
    FishSoup,
    StoneAxe,
    Worktable,
    Campfire,
    WoodenCart,
    RidingMammoth,
    FlintSpear,
    HuntingBow,
    LeatherArmor,
    BoneShield,
    NpcGatherer,
    NpcCrafter,
    NpcGuard,
    NpcHunter,
    WoodenClub,
    MapHome,
    MapForest,
    MapCave,
    MapUfo,
}

impl ToolboxIconType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolboxIconType::HandSkill => "Hand Skill",
            ToolboxIconType::QuickGather => "Quick Gather",
            ToolboxIconType::StoneShelter => "Stone Shelter",
            ToolboxIconType::Watchtower => "Watchtower",
            ToolboxIconType::RoastMeat => "Roast Meat",
            ToolboxIconType::FishSoup => "Fish Soup",
            ToolboxIconType::StoneAxe => "Stone Axe",
            ToolboxIconType::Worktable => "Worktable",
            ToolboxIconType::Campfire => "Campfire",
            ToolboxIconType::WoodenCart => "Wooden Cart",
            ToolboxIconType::RidingMammoth => "Riding Mammoth",
            ToolboxIconType::FlintSpear => "Flint Spear",
            ToolboxIconType::HuntingBow => "Hunting Bow",
            ToolboxIconType::LeatherArmor => "Leather Armor",
            ToolboxIconType::BoneShield => "Bone Shield",
            ToolboxIconType::NpcGatherer => "Gatherer NPC",
            ToolboxIconType::NpcCrafter => "Crafter NPC",
            ToolboxIconType::NpcGuard => "Guard NPC",
            ToolboxIconType::NpcHunter => "Hunter NPC",
            ToolboxIconType::WoodenClub => "Wooden Club",
            ToolboxIconType::MapHome => "Home",
            ToolboxIconType::MapForest => "Forest",
            ToolboxIconType::MapCave => "Cave",
            ToolboxIconType::MapUfo => "UFO",
        }
    }

    pub fn item_code(&self) -> &'static str {
        match self {
            ToolboxIconType::HandSkill => "items/hand",
            ToolboxIconType::QuickGather => "items/hand",
            ToolboxIconType::StoneShelter => "items/rock",
            ToolboxIconType::Watchtower => "items/wood",
            ToolboxIconType::RoastMeat => "items/foods/roast_meat",
            ToolboxIconType::FishSoup => "items/foods/fish_soup",
            ToolboxIconType::StoneAxe => "items/equipment/stone_axe",
            ToolboxIconType::Worktable => "items/equipment/worktable",
            ToolboxIconType::Campfire => "items/equipment/campfire",
            ToolboxIconType::WoodenCart => "items/wood",
            ToolboxIconType::RidingMammoth => "items/meat",
            ToolboxIconType::FlintSpear => "items/equipment/flint_spear",
            ToolboxIconType::HuntingBow => "items/equipment/hunting_bow",
            ToolboxIconType::LeatherArmor => "items/equipment/leather_armor",
            ToolboxIconType::BoneShield => "items/equipment/bone_shield",
            ToolboxIconType::NpcGatherer => "items/hand",
            ToolboxIconType::NpcCrafter => "items/equipment/worktable",
            ToolboxIconType::NpcGuard => "items/equipment/flint_spear",
            ToolboxIconType::NpcHunter => "items/equipment/hunting_bow",
            ToolboxIconType::WoodenClub => "items/wooden_club",
            ToolboxIconType::MapHome => "items/wood",
            ToolboxIconType::MapForest => "items/wood",
            ToolboxIconType::MapCave => "items/wood",
            ToolboxIconType::MapUfo => "items/wood",
        }
    }

    pub fn icon_str(&self) -> &'static str {
        match self {
            ToolboxIconType::HandSkill => "[HAND]",
            ToolboxIconType::QuickGather => "[GATHER]",
            ToolboxIconType::StoneShelter => "[SHELTER]",
            ToolboxIconType::Watchtower => "[TOWER]",
            ToolboxIconType::RoastMeat => "[MEAT]",
            ToolboxIconType::FishSoup => "[SOUP]",
            ToolboxIconType::StoneAxe => "[AXE]",
            ToolboxIconType::Worktable => "[TABLE]",
            ToolboxIconType::Campfire => "[FIRE]",
            ToolboxIconType::WoodenCart => "[CART]",
            ToolboxIconType::RidingMammoth => "[MAMMOTH]",
            ToolboxIconType::FlintSpear => "[SPEAR]",
            ToolboxIconType::HuntingBow => "[BOW]",
            ToolboxIconType::LeatherArmor => "[ARMOR]",
            ToolboxIconType::BoneShield => "[SHIELD]",
            ToolboxIconType::NpcGatherer => "[GATHERER]",
            ToolboxIconType::NpcCrafter => "[CRAFTER]",
            ToolboxIconType::NpcGuard => "[GUARD]",
            ToolboxIconType::NpcHunter => "[HUNTER]",
            ToolboxIconType::WoodenClub => "[CLUB]",
            ToolboxIconType::MapHome => "[HOME]",
            ToolboxIconType::MapForest => "[FOREST]",
            ToolboxIconType::MapCave => "[CAVE]",
            ToolboxIconType::MapUfo => "[UFO]",
        }
    }

    pub fn stage_data_name(&self) -> Option<&'static str> {
        match self {
            ToolboxIconType::MapHome => Some("game_scenes/intro_stage"),
            ToolboxIconType::MapForest => Some("game_scenes/stage_01"),
            ToolboxIconType::MapCave => Some("game_scenes/stage_cave"),
            ToolboxIconType::MapUfo => Some("game_scenes/stage_ufo"),
            _ => None,
        }
    }

    pub fn world_discovered_info(&self) -> Option<(Vec<String>, Vec<String>)> {
        let stage_name = self.stage_data_name()?;
        let game_scene_manager = get_game_scene_manager();

        if let Some(data) = game_scene_manager.get_discovered_world_data(stage_name) {
            return Some((data._items.clone(), data._characters.clone()));
        }

        None
    }

    pub fn npc_character_info(&self) -> Option<(&'static str, Vector3<f32>)> {
        match self {
            ToolboxIconType::NpcGatherer => Some(("characters/villager_00", Vector3::new(3.0, 0.0, 3.0))),
            ToolboxIconType::NpcCrafter => Some(("characters/jack", Vector3::new(-3.0, 0.0, 3.0))),
            ToolboxIconType::NpcGuard => Some(("characters/neanderthal", Vector3::new(3.0, 0.0, -3.0))),
            ToolboxIconType::NpcHunter => Some(("characters/family/aru", Vector3::new(-3.0, 0.0, -3.0))),
            _ => None,
        }
    }

    pub fn toolbox_data_name(&self) -> &'static str {
        match self {
            ToolboxIconType::HandSkill => "toolbox_items/skill/hand_skill",
            ToolboxIconType::QuickGather => "toolbox_items/skill/quick_gather",
            ToolboxIconType::StoneShelter => "toolbox_items/craft/stone_shelter",
            ToolboxIconType::Watchtower => "toolbox_items/craft/watchtower",
            ToolboxIconType::RoastMeat => "toolbox_items/craft/roast_meat",
            ToolboxIconType::FishSoup => "toolbox_items/craft/fish_soup",
            ToolboxIconType::StoneAxe => "toolbox_items/craft/stone_axe",
            ToolboxIconType::Worktable => "toolbox_items/craft/worktable",
            ToolboxIconType::Campfire => "toolbox_items/craft/campfire",
            ToolboxIconType::WoodenCart => "toolbox_items/craft/wooden_cart",
            ToolboxIconType::RidingMammoth => "toolbox_items/craft/riding_mammoth",
            ToolboxIconType::FlintSpear => "toolbox_items/craft/flint_spear",
            ToolboxIconType::HuntingBow => "toolbox_items/craft/hunting_bow",
            ToolboxIconType::LeatherArmor => "toolbox_items/craft/leather_armor",
            ToolboxIconType::BoneShield => "toolbox_items/craft/bone_shield",
            ToolboxIconType::NpcGatherer => "toolbox_items/npc/npc_gatherer",
            ToolboxIconType::NpcCrafter => "toolbox_items/npc/npc_crafter",
            ToolboxIconType::NpcGuard => "toolbox_items/npc/npc_guard",
            ToolboxIconType::NpcHunter => "toolbox_items/npc/npc_hunter",
            ToolboxIconType::WoodenClub => "toolbox_items/craft/wooden_club",
            ToolboxIconType::MapHome => "toolbox_items/teleport/map_home",
            ToolboxIconType::MapForest => "toolbox_items/teleport/map_forest",
            ToolboxIconType::MapCave => "toolbox_items/teleport/map_cave",
            ToolboxIconType::MapUfo => "toolbox_items/teleport/map_ufo",
        }
    }

    pub fn get_display_name(&self) -> String {
        if self.stage_data_name().is_some() {
            self.as_str().to_string()
        } else {
            ToolboxItemWidget::resolve_display_name(self.item_code())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolboxItemState {
    Locked,
    Unlocked,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ToolboxItemData {
    pub _icon_type: ToolboxIconType,
    pub _description: String,
    pub _item_data_type: ItemDataType,
    pub _item_data_count: usize,
}

impl ToolboxItemData {
    pub fn cost_label(&self) -> String {
        if self._item_data_count == 0 || self._item_data_type == ItemDataType::None {
            "Free".to_string()
        } else {
            let item_code = self._item_data_type.item_code();
            let mat_name = ToolboxItemWidget::get_item_name_from_resource(item_code);
            if self._item_data_count == 1 {
                format!("1 {}", mat_name)
            } else {
                format!("{} {}", self._item_data_count, mat_name)
            }
        }
    }
}

pub struct IngredientWidgetItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _label: Rc<WidgetDefault<'a>>,
    pub _item_type: ItemDataType,
    pub _count: usize,
}

impl<'a> IngredientWidgetItem<'a> {
    pub fn item_code(&self) -> &'static str {
        self._item_type.item_code()
    }
}

pub const INFO_LABEL_INDEX_DESC: usize = 0;
pub const INFO_LABEL_INDEX_ITEMS_HDR: usize = 1;
pub const INFO_LABEL_INDEX_CHARS_HDR: usize = 2;
pub const INFO_LABEL_INDEX_UNEXP: usize = 3;
pub const MAX_DETAIL_INFO_ENTRIES: usize = 8;

pub struct DetailInfoEntryWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _label: Rc<WidgetDefault<'a>>,
}

impl<'a> DetailInfoEntryWidget<'a> {
    pub fn create(tab_id: &str, category: &str, index: usize, margin_bottom: f32) -> DetailInfoEntryWidget<'a> {
        let entry_layout = UIManager::create_widget(
            &format!("{}_detail_{}_entry_{}", tab_id, category, index),
            UIWidgetTypes::Default,
        );
        let entry_layout_mut = ptr_as_mut(entry_layout.as_ref());
        let ui = entry_layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(DESCRIPTION_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_left(12.0);
        if margin_bottom > 0.0 {
            ui.set_margin_bottom(margin_bottom);
        }
        ui.set_color(COLOR_TRANSPARENT);

        let entry_icon = UIManager::create_widget(
            &format!("{}_detail_{}_icon_{}", tab_id, category, index),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(entry_icon.as_ref()).get_ui_component_mut();
        ui.set_size(DESCRIPTION_LABEL_HEIGHT, DESCRIPTION_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(6.0);
        ui.set_color(COLOR_WHITE);
        entry_layout_mut.add_widget(&entry_icon);

        let entry_label = UIManager::create_widget(
            &format!("{}_detail_{}_lbl_{}", tab_id, category, index),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(entry_label.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(DESCRIPTION_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(DESCRIPTION_LABEL_FONT_SIZE);
        ui.set_font_color(COLOR_TEXT_NORMAL);
        ui.set_color(COLOR_TRANSPARENT);
        entry_layout_mut.add_widget(&entry_label);

        DetailInfoEntryWidget {
            _layout: entry_layout,
            _icon: entry_icon,
            _label: entry_label,
        }
    }
}

// ────────────────────────────────────────────────────────────────
// ────────────────────────────────────────────────────────────────
// ToolboxItemWidget - Left side list item entry
// Layout: [Icon] [Icon name] [Action button]
// ────────────────────────────────────────────────────────────────
pub struct ToolboxItemWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _name_lbl: Rc<WidgetDefault<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _state: ToolboxItemState,
    pub _data: ToolboxItemData,
    pub _tab_ptr: *mut c_void,
    pub _item_index: usize,
}

impl<'a> ToolboxItemWidget<'a> {
    pub fn get_item_name_from_resource(item_code: &str) -> String {
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            resources.get_item_data(item_code).borrow()._name.clone()
        } else {
            item_code.to_string()
        }
    }

    pub fn resolve_display_name(code: &str) -> String {
        let name = Self::get_item_name_from_resource(code);
        if name == code { code.to_string() } else { name }
    }

    pub fn get_item_description_from_resource(item_code: &str) -> String {
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            let desc = resources.get_item_data(item_code).borrow()._description.clone();
            if !desc.is_empty() {
                return desc;
            }
        }
        item_code.to_string()
    }

    pub fn setup_item_icon(icon_widget: &Rc<WidgetDefault<'a>>, item_code: &str, enable: bool) {
        let mut has_item_data: bool = false;
        let ui = ptr_as_mut(icon_widget.as_ref()).get_ui_component_mut();
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            let item_data = resources.get_item_data(item_code).borrow();
            let mat_name = &item_data._ui_material_instance;
            if !mat_name.is_empty() {
                let engine_res = get_engine_resources();
                if engine_res.has_material_instance_data(mat_name.as_str()) {
                    let material = engine_res.get_material_instance_data(mat_name.as_str());
                    ui.set_material_instance(Some(material.clone()));
                    has_item_data = true;
                }
            }
        }
        ui.set_enable(enable && has_item_data);
    }

    pub fn callback_item_touch_over(
        _ui_component: &UIComponentInstance<'a>,
        _touched_pos: &nalgebra::Vector2<f32>,
        _touched_pos_delta: &nalgebra::Vector2<f32>,
    ) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_item_select(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &nalgebra::Vector2<f32>,
        _touched_pos_delta: &nalgebra::Vector2<f32>,
    ) -> bool {
        let item_ptr = ui_component.get_user_data() as *mut ToolboxItemWidget<'a>;
        if item_ptr.is_null() {
            return false;
        }
        let item = ptr_as_mut(item_ptr);
        if !item._tab_ptr.is_null() {
            let tab = ptr_as_mut(item._tab_ptr as *mut ToolboxTabWidget<'a>);
            tab.select_item(item._item_index);
        }
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_item_action_btn(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &nalgebra::Vector2<f32>,
        _touched_pos_delta: &nalgebra::Vector2<f32>,
    ) -> bool {
        let item_ptr = ui_component.get_user_data() as *mut ToolboxItemWidget<'a>;
        if item_ptr.is_null() {
            return false;
        }
        let item = ptr_as_mut(item_ptr);
        if !item._tab_ptr.is_null() {
            let tab = ptr_as_mut(item._tab_ptr as *mut ToolboxTabWidget<'a>);
            tab.execute_item_action(item._item_index);
        }
        true
    }

    pub fn update_list_item_ui(&mut self, is_selected: bool) {
        let layout_ui = ptr_as_mut(self._layout.as_ref()).get_ui_component_mut();
        if is_selected {
            layout_ui.set_color(COLOR_ITEM_SELECTED_BG);
            layout_ui.set_border_color(COLOR_ITEM_SELECTED_BORDER);
        } else {
            layout_ui.set_color(COLOR_ITEM_NORMAL_BG);
            layout_ui.set_border_color(COLOR_ITEM_NORMAL_BORDER);
        }

        let is_map_item = self._data._icon_type.stage_data_name().is_some();
        let ui_mgr = get_game_ui_manager();
        let btn_ui = ptr_as_mut(self._action_btn.as_ref()).get_ui_component_mut();

        match self._state {
            ToolboxItemState::Locked => {
                btn_ui.set_text("Unlock");
                let current_count = if self._data._item_data_type != ItemDataType::None {
                    ui_mgr.get_item_count(self._data._item_data_type.item_code())
                } else {
                    0
                };
                if self._data._item_data_count == 0
                    || self._data._item_data_type == ItemDataType::None
                    || current_count >= self._data._item_data_count
                {
                    btn_ui.set_color(COLOR_BTN_UNLOCK_BG);
                    btn_ui.set_border_color(COLOR_BTN_UNLOCK_BORDER);
                    btn_ui.set_font_color(COLOR_WHITE);
                    btn_ui.set_touchable(true);
                } else {
                    btn_ui.set_color(COLOR_BTN_DISABLED_BG);
                    btn_ui.set_border_color(COLOR_BTN_DISABLED_BORDER);
                    btn_ui.set_font_color(COLOR_TEXT_DISABLED);
                    btn_ui.set_touchable(false);
                }
                btn_ui.set_enable(true);
            }
            ToolboxItemState::Unlocked => {
                if is_map_item {
                    btn_ui.set_text("Teleport");
                    btn_ui.set_color(COLOR_BTN_TELEPORT_BG);
                    btn_ui.set_border_color(COLOR_BTN_TELEPORT_BORDER);
                    btn_ui.set_font_color(COLOR_WHITE);
                    btn_ui.set_touchable(true);
                    btn_ui.set_enable(true);
                } else {
                    btn_ui.set_text("Unlocked");
                    btn_ui.set_color(COLOR_BTN_UNLOCKED_BG);
                    btn_ui.set_border_color(COLOR_BTN_UNLOCKED_BORDER);
                    btn_ui.set_font_color(COLOR_TEXT_DISABLED_ALT);
                    btn_ui.set_touchable(false);
                    btn_ui.set_enable(false);
                }
            }
        }
    }

    pub fn create(
        parent_widget: &mut WidgetDefault<'a>,
        data: ToolboxItemData,
        item_index: usize,
    ) -> Box<ToolboxItemWidget<'a>> {
        // Single Row Entry: [Icon] [Icon name] [Action button]
        let layout = UIManager::create_widget(
            &format!("item_list_row_{:?}_{}", data._icon_type, item_index),
            UIWidgetTypes::Default,
        );
        let layout_mut = ptr_as_mut(layout.as_ref());
        let ui = layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(LIST_ITEM_ROW_HEIGHT);
        ui.set_padding(LIST_ITEM_PADDING);
        ui.set_color(COLOR_ITEM_NORMAL_BG);
        ui.set_border_color(COLOR_ITEM_NORMAL_BORDER);
        ui.set_border(BORDER_WIDTH_THICK);
        ui.set_round(CORNER_ROUND_NORMAL);
        ui.set_margin(2.0);
        ui.set_touchable(true);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_select)));
        parent_widget.add_widget(&layout);

        // Icon (40x40)
        let icon = UIManager::create_widget(
            &format!("item_list_icon_{:?}_{}", data._icon_type, item_index),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
        ui.set_size(LIST_ITEM_ICON_SIZE, LIST_ITEM_ICON_SIZE);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_margin_right(8.0);
        ui.set_color(COLOR_WHITE);
        layout_mut.add_widget(&icon);
        Self::setup_item_icon(&icon, data._icon_type.item_code(), true);

        // Name Label
        let display_name = data._icon_type.get_display_name();
        let name_lbl = UIManager::create_widget(
            &format!("item_list_name_{:?}_{}", data._icon_type, item_index),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(LIST_ITEM_NAME_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text(&display_name);
        ui.set_font_size(FONT_SIZE_TITLE);
        ui.set_font_color(COLOR_TEXT_TITLE);
        ui.set_color(COLOR_TRANSPARENT);
        layout_mut.add_widget(&name_lbl);

        // Action Button (Unlock / Teleport / Unlocked)
        let action_btn = UIManager::create_widget(
            &format!("item_list_btn_{:?}_{}", data._icon_type, item_index),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(action_btn.as_ref()).get_ui_component_mut();
        ui.set_size(LIST_ITEM_BTN_WIDTH, LIST_ITEM_BTN_HEIGHT);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(COLOR_BTN_DEFAULT_BG);
        ui.set_border_color(COLOR_BTN_DEFAULT_BORDER);
        ui.set_border(BORDER_WIDTH_NORMAL);
        ui.set_round(CORNER_ROUND_NORMAL);
        ui.set_font_size(FONT_SIZE_BUTTON);
        ui.set_font_color(COLOR_TEXT_NORMAL);
        ui.set_margin_right(4.0);
        ui.set_touchable(false);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_action_btn)));
        layout_mut.add_widget(&action_btn);

        let mut item = Box::new(ToolboxItemWidget {
            _layout: layout,
            _icon: icon,
            _name_lbl: name_lbl,
            _action_btn: action_btn,
            _state: ToolboxItemState::Locked,
            _data: data,
            _tab_ptr: std::ptr::null_mut(),
            _item_index: item_index,
        });

        item.update_list_item_ui(false);
        item
    }
}

// Helper functions for updating discovered detail info list entries
fn update_detail_info_entry_list<'a>(entries: &[DetailInfoEntryWidget<'a>], codes: &[String]) {
    for (i, entry) in entries.iter().enumerate() {
        if i < codes.len() {
            let code = &codes[i];
            let name_text = ToolboxItemWidget::resolve_display_name(code);
            ptr_as_mut(entry._label.as_ref()).get_ui_component_mut().set_text(&name_text);
            ToolboxItemWidget::setup_item_icon(&entry._icon, code, true);
            ptr_as_mut(entry._layout.as_ref()).get_ui_component_mut().set_enable(true);
        } else {
            ptr_as_mut(entry._layout.as_ref()).get_ui_component_mut().set_enable(false);
        }
    }
}

fn setup_discovered_category_header<'a>(
    hdr_label: &Rc<WidgetDefault<'a>>,
    category_name: &str,
    entries: &[DetailInfoEntryWidget<'a>],
    codes: &[String],
) {
    let hdr_ui = ptr_as_mut(hdr_label.as_ref()).get_ui_component_mut();
    hdr_ui.set_enable(true);
    if codes.is_empty() {
        hdr_ui.set_text(&format!("Discovered {}: None", category_name));
        for entry in entries {
            ptr_as_mut(entry._layout.as_ref()).get_ui_component_mut().set_enable(false);
        }
    } else {
        hdr_ui.set_text(&format!("Discovered {}:", category_name));
        update_detail_info_entry_list(entries, codes);
    }
}

fn create_info_label<'a>(
    name: &str,
    height: f32,
    font_size: f32,
    font_color: u32,
    margin_top: f32,
) -> Rc<WidgetDefault<'a>> {
    let lbl = UIManager::create_widget(name, UIWidgetTypes::Default);
    let ui = ptr_as_mut(lbl.as_ref()).get_ui_component_mut();
    ui.set_size_hint_x(Some(1.0));
    ui.set_size_y(height);
    ui.set_font_size(font_size);
    ui.set_font_color(font_color);
    ui.set_color(COLOR_TRANSPARENT);
    if margin_top > 0.0 {
        ui.set_margin_top(margin_top);
    }
    lbl
}

// ────────────────────────────────────────────────────────────────
// ToolboxTabWidget - Master-Detail Content pane for a category tab
// Left: List Container (Items)
// Right: Detail Container (Selected Item Info: Icon, Name, Desc, Requirements, Unlock Button)
// ────────────────────────────────────────────────────────────────
pub struct ToolboxTabWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _detail_container: Rc<WidgetDefault<'a>>,
    pub _items: Vec<Box<ToolboxItemWidget<'a>>>,
    pub _selected_index: usize,

    // Right Detail Panel widgets
    pub _detail_icon: Rc<WidgetDefault<'a>>,
    pub _detail_name_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_desc_box: Rc<WidgetDefault<'a>>,
    pub _detail_info_labels: Vec<Rc<WidgetDefault<'a>>>,
    pub _detail_info_item_entries: Vec<DetailInfoEntryWidget<'a>>,
    pub _detail_info_char_entries: Vec<DetailInfoEntryWidget<'a>>,
    pub _detail_req_box: Rc<WidgetDefault<'a>>,
    pub _detail_ing_widgets: Vec<IngredientWidgetItem<'a>>,

    pub _is_visible: bool,
}

impl<'a> ToolboxTabWidget<'a> {
    fn unlock_item(&mut self, index: usize) {
        if index >= self._items.len() {
            return;
        }
        let item = &mut self._items[index];
        item._state = ToolboxItemState::Unlocked;
        get_game_ui_manager_mut().notify_item_crafted();
        let reward_item_code = item._data._icon_type.item_code();
        get_game_ui_manager_mut().notify_item_acquired(reward_item_code, 1, true);
        get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
        if let Some((char_data_name, offset)) = item._data._icon_type.npc_character_info() {
            spawn_npc_near_monolith(char_data_name, offset);
        }
        self.update_detail_panel();
    }

    pub fn execute_item_action(&mut self, index: usize) {
        if index >= self._items.len() {
            return;
        }
        self.select_item(index);
        let item = &mut self._items[index];
        match item._state {
            ToolboxItemState::Locked => {
                let cost = item._data._item_data_count;
                let item_type = item._data._item_data_type;
                let has_enough_cost = if cost > 0 && item_type != ItemDataType::None {
                    let item_code = item_type.item_code();
                    let current_count = get_game_ui_manager().get_item_count(item_code);
                    if current_count >= cost {
                        get_game_ui_manager_mut().remove_item(item_code, cost)
                    } else {
                        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                        false
                    }
                } else {
                    true
                };

                if has_enough_cost {
                    self.unlock_item(index);
                }
            }
            ToolboxItemState::Unlocked => {
                if let Some(stage_name) = item._data._icon_type.stage_data_name() {
                    get_game_scene_manager_mut().set_teleport_stage(stage_name, DEFAULT_GATE_NAME);
                    get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                    get_game_ui_manager_mut().close_toolbox();
                }
            }
        }
    }

    pub fn execute_selected_action(&mut self) {
        self.execute_item_action(self._selected_index);
    }

    pub fn select_item(&mut self, index: usize) {
        if index >= self._items.len() {
            return;
        }
        self._selected_index = index;
        self.update_detail_panel();
    }

    pub fn update_detail_panel(&mut self) {
        let ui_mgr = get_game_ui_manager();

        for (idx, item) in self._items.iter_mut().enumerate() {
            let is_sel = idx == self._selected_index;
            item.update_list_item_ui(is_sel);
        }

        if self._selected_index >= self._items.len() {
            return;
        }

        let item = &self._items[self._selected_index];
        let is_map_item = item._data._icon_type.stage_data_name().is_some();

        // 1. Setup Detail Icon & Name
        ToolboxItemWidget::setup_item_icon(&self._detail_icon, item._data._icon_type.item_code(), true);
        let display_name = item._data._icon_type.get_display_name();
        ptr_as_mut(self._detail_name_lbl.as_ref()).get_ui_component_mut().set_text(&display_name);

        // 2. Setup Description & Info Labels/Lists
        if self._detail_info_labels.len() > INFO_LABEL_INDEX_DESC {
            let desc_ui = ptr_as_mut(self._detail_info_labels[INFO_LABEL_INDEX_DESC].as_ref()).get_ui_component_mut();
            if is_map_item {
                desc_ui.set_text(&item._data._description);
            } else {
                let item_code = item._data._icon_type.item_code();
                let desc_text = ToolboxItemWidget::get_item_description_from_resource(item_code);
                if !desc_text.is_empty() && desc_text != item_code {
                    desc_ui.set_text(&desc_text);
                } else if !item._data._description.is_empty() {
                    desc_ui.set_text(&item._data._description);
                }
            }
        }

        // 3. Setup Requirements / Map Discovered Info
        for ing_widget in self._detail_ing_widgets.iter_mut() {
            ing_widget._item_type = item._data._item_data_type;
            ing_widget._count = item._data._item_data_count;
            let unlocked = item._state == ToolboxItemState::Unlocked;
            let show_req =
                !unlocked && item._data._item_data_count > 0 && item._data._item_data_type != ItemDataType::None;
            let item_code = ing_widget.item_code();
            let have_count = ui_mgr.get_item_count(item_code);
            let mat_name = ToolboxItemWidget::get_item_name_from_resource(item_code);
            let text = if unlocked {
                "Unlocked".to_string()
            } else if show_req {
                format!("{} ({}/{})", mat_name, have_count, ing_widget._count)
            } else {
                "Unlock for Free".to_string()
            };
            let lbl_ui = ptr_as_mut(ing_widget._label.as_ref()).get_ui_component_mut();
            lbl_ui.set_text(&text);
            if have_count >= ing_widget._count || unlocked {
                lbl_ui.set_font_color(COLOR_TEXT_NORMAL);
            } else {
                lbl_ui.set_font_color(COLOR_TEXT_ERROR);
            }

            ToolboxItemWidget::setup_item_icon(&ing_widget._icon, item_code, show_req);
        }

        // Helper function to disable all map info labels and entry lists
        let disable_all_info = |labels: &Vec<Rc<WidgetDefault<'a>>>,
                                item_entries: &Vec<DetailInfoEntryWidget<'a>>,
                                char_entries: &Vec<DetailInfoEntryWidget<'a>>| {
            if labels.len() > INFO_LABEL_INDEX_ITEMS_HDR {
                ptr_as_mut(labels[INFO_LABEL_INDEX_ITEMS_HDR].as_ref()).get_ui_component_mut().set_enable(false);
            }
            if labels.len() > INFO_LABEL_INDEX_CHARS_HDR {
                ptr_as_mut(labels[INFO_LABEL_INDEX_CHARS_HDR].as_ref()).get_ui_component_mut().set_enable(false);
            }
            if labels.len() > INFO_LABEL_INDEX_UNEXP {
                ptr_as_mut(labels[INFO_LABEL_INDEX_UNEXP].as_ref()).get_ui_component_mut().set_enable(false);
            }
            for entry in item_entries {
                ptr_as_mut(entry._layout.as_ref()).get_ui_component_mut().set_enable(false);
            }
            for entry in char_entries {
                ptr_as_mut(entry._layout.as_ref()).get_ui_component_mut().set_enable(false);
            }
        };

        if is_map_item {
            if let Some((items, chars)) = item._data._icon_type.world_discovered_info() {
                if self._detail_info_labels.len() > INFO_LABEL_INDEX_UNEXP {
                    ptr_as_mut(self._detail_info_labels[INFO_LABEL_INDEX_UNEXP].as_ref())
                        .get_ui_component_mut()
                        .set_enable(false);
                }

                // Setup Items list
                if self._detail_info_labels.len() > INFO_LABEL_INDEX_ITEMS_HDR {
                    setup_discovered_category_header(
                        &self._detail_info_labels[INFO_LABEL_INDEX_ITEMS_HDR],
                        "items",
                        &self._detail_info_item_entries,
                        &items,
                    );
                }

                // Setup Characters list
                if self._detail_info_labels.len() > INFO_LABEL_INDEX_CHARS_HDR {
                    setup_discovered_category_header(
                        &self._detail_info_labels[INFO_LABEL_INDEX_CHARS_HDR],
                        "characters",
                        &self._detail_info_char_entries,
                        &chars,
                    );
                }
            } else {
                disable_all_info(
                    &self._detail_info_labels,
                    &self._detail_info_item_entries,
                    &self._detail_info_char_entries,
                );
                if self._detail_info_labels.len() > INFO_LABEL_INDEX_UNEXP {
                    let ui =
                        ptr_as_mut(self._detail_info_labels[INFO_LABEL_INDEX_UNEXP].as_ref()).get_ui_component_mut();
                    ui.set_text("Unexplored Region");
                    ui.set_enable(true);
                }
            }
        } else {
            disable_all_info(
                &self._detail_info_labels,
                &self._detail_info_item_entries,
                &self._detail_info_char_entries,
            );
        }
    }

    pub fn create_toolbox_tab_widget(
        tab_id: &str,
        parent_widget: &mut WidgetDefault<'a>,
        item_list: Vec<ToolboxItemData>,
    ) -> Box<ToolboxTabWidget<'a>> {
        // Main Tab Layout (Horizontal Split View)
        let layout = UIManager::create_widget(&format!("{}_tab_layout", tab_id), UIWidgetTypes::Default);
        let layout_mut = ptr_as_mut(layout.as_ref());
        let ui = layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_padding(TAB_PADDING);
        ui.set_color(COLOR_TAB_BG);
        ui.set_enable(false);
        parent_widget.add_widget(&layout);

        // 1. Left List Container (Width ~ 290)
        let list_container = UIManager::create_widget(&format!("{}_list_container", tab_id), UIWidgetTypes::Default);
        let list_container_mut = ptr_as_mut(list_container.as_ref());
        let ui = list_container_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_scroll_y(true);
        ui.set_enable_renderable_area(true);
        ui.set_padding(LIST_CONTAINER_PADDING);
        ui.set_margin_right(TAB_PADDING);
        ui.set_color(COLOR_PANEL_BG);
        ui.set_border_color(COLOR_PANEL_BORDER);
        ui.set_border(BORDER_WIDTH_NORMAL);
        ui.set_round(CORNER_ROUND_NORMAL);
        layout_mut.add_widget(&list_container);

        // 2. Right Detail Container (Width ~ 430)
        let detail_container_layout =
            UIManager::create_widget(&format!("{}_detail_container_layout", tab_id), UIWidgetTypes::Default);
        let detail_container_layout_mut = ptr_as_mut(detail_container_layout.as_ref());
        let ui = detail_container_layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_scroll_y(true);
        ui.set_color(COLOR_TRANSPARENT);
        ui.set_border_color(COLOR_TRANSPARENT);
        layout_mut.add_widget(&detail_container_layout);

        let detail_container =
            UIManager::create_widget(&format!("{}_detail_container", tab_id), UIWidgetTypes::Default);
        let detail_container_mut = ptr_as_mut(detail_container.as_ref());
        let ui = detail_container_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
        ui.set_padding(DETAIL_CONTAINER_PADDING);
        ui.set_color(COLOR_DETAIL_BG);
        ui.set_border_color(COLOR_DETAIL_BORDER);
        ui.set_border(BORDER_WIDTH_NORMAL);
        ui.set_round(CORNER_ROUND_NORMAL);
        detail_container_layout_mut.add_widget(&detail_container);

        // Detail Header: Icon + Name
        let detail_hdr = UIManager::create_widget(&format!("{}_detail_hdr", tab_id), UIWidgetTypes::Default);
        let detail_hdr_mut = ptr_as_mut(detail_hdr.as_ref());
        let ui = detail_hdr_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(DETAIL_HEADER_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_bottom(10.0);
        ui.set_color(COLOR_TRANSPARENT);
        detail_container_mut.add_widget(&detail_hdr);

        // Detail Large Icon (48x48)
        let detail_icon = UIManager::create_widget(&format!("{}_detail_icon", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_icon.as_ref()).get_ui_component_mut();
        ui.set_size(DETAIL_ICON_SIZE, DETAIL_ICON_SIZE);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(12.0);
        ui.set_color(COLOR_WHITE);
        detail_hdr_mut.add_widget(&detail_icon);

        // Detail Name Label
        let detail_name_lbl = UIManager::create_widget(&format!("{}_detail_name", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_name_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(DETAIL_NAME_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(FONT_SIZE_TITLE);
        ui.set_font_color(COLOR_WHITE);
        ui.set_color(COLOR_TRANSPARENT);
        detail_hdr_mut.add_widget(&detail_name_lbl);

        // Detail Description Box
        let detail_desc_box = UIManager::create_widget(&format!("{}_detail_desc_box", tab_id), UIWidgetTypes::Default);
        let detail_desc_box_mut = ptr_as_mut(detail_desc_box.as_ref());
        let ui = detail_desc_box_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
        ui.set_padding(BOX_PADDING);
        ui.set_color(COLOR_BOX_BG);
        ui.set_border_color(COLOR_BOX_BORDER);
        ui.set_border(BORDER_WIDTH_NORMAL);
        ui.set_round(CORNER_ROUND_NORMAL);
        ui.set_margin_bottom(16.0);
        detail_container_mut.add_widget(&detail_desc_box);

        let mut detail_info_labels = Vec::new();

        // Index 0: Description Label
        let detail_desc_lbl = UIManager::create_widget(&format!("{}_detail_desc", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_desc_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_font_size(DESCRIPTION_LABEL_FONT_SIZE);
        ui.set_font_color(COLOR_TEXT_MUTED);
        ui.set_color(COLOR_TRANSPARENT);
        ui.set_margin_bottom(12.0);
        detail_desc_box_mut.add_widget(&detail_desc_lbl);
        detail_info_labels.push(detail_desc_lbl);

        // Index 1: Items Header Label
        let items_hdr_lbl = create_info_label(
            &format!("{}_detail_info_items", tab_id),
            DESCRIPTION_LABEL_HEIGHT,
            DESCRIPTION_LABEL_FONT_SIZE,
            COLOR_TEXT_SUCCESS,
            0.0,
        );
        detail_desc_box_mut.add_widget(&items_hdr_lbl);
        detail_info_labels.push(items_hdr_lbl);

        // Item Entries
        let mut detail_info_item_entries = Vec::with_capacity(MAX_DETAIL_INFO_ENTRIES);
        for i in 0..MAX_DETAIL_INFO_ENTRIES {
            let entry = DetailInfoEntryWidget::create(tab_id, "item", i, 0.0);
            detail_desc_box_mut.add_widget(&entry._layout);
            detail_info_item_entries.push(entry);
        }

        // Index 2: Characters Header Label
        let chars_hdr_lbl = create_info_label(
            &format!("{}_detail_info_chars", tab_id),
            DESCRIPTION_LABEL_HEIGHT,
            DESCRIPTION_LABEL_FONT_SIZE,
            COLOR_TEXT_WARNING,
            6.0,
        );
        detail_desc_box_mut.add_widget(&chars_hdr_lbl);
        detail_info_labels.push(chars_hdr_lbl);

        // Character Entries
        let mut detail_info_char_entries = Vec::with_capacity(MAX_DETAIL_INFO_ENTRIES);
        for i in 0..MAX_DETAIL_INFO_ENTRIES {
            let entry = DetailInfoEntryWidget::create(tab_id, "char", i, 2.0);
            detail_desc_box_mut.add_widget(&entry._layout);
            detail_info_char_entries.push(entry);
        }

        // Index 3: Unexplored Label
        let unexp_lbl = create_info_label(
            &format!("{}_detail_info_unexp", tab_id),
            DESCRIPTION_LABEL_HEIGHT,
            DESCRIPTION_LABEL_FONT_SIZE,
            COLOR_TEXT_DISABLED,
            0.0,
        );
        detail_desc_box_mut.add_widget(&unexp_lbl);
        detail_info_labels.push(unexp_lbl);

        // Detail Requirements / Map Info Box
        let detail_req_text = UIManager::create_widget(&format!("{}_detail_req_text", tab_id), UIWidgetTypes::Default);
        let detail_req_text_mut = ptr_as_mut(detail_req_text.as_ref());
        let ui = detail_req_text_mut.get_ui_component_mut();
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(REQUIREMENT_HEADER_HEIGHT);
        ui.set_margin_bottom(4.0);
        ui.set_color(COLOR_TRANSPARENT);
        ui.set_font_size(FONT_SIZE_NORMAL);
        ui.set_text("Requirements:");
        ui.set_font_color(COLOR_TEXT_SUCCESS);
        detail_container_mut.add_widget(&detail_req_text);

        let detail_req_box = UIManager::create_widget(&format!("{}_detail_req_box", tab_id), UIWidgetTypes::Default);
        let detail_req_box_mut = ptr_as_mut(detail_req_box.as_ref());
        let ui = detail_req_box_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
        ui.set_padding(BOX_PADDING);
        ui.set_color(COLOR_BOX_BG);
        ui.set_border_color(COLOR_BOX_BORDER);
        ui.set_border(BORDER_WIDTH_NORMAL);
        ui.set_round(CORNER_ROUND_NORMAL);
        ui.set_margin_bottom(16.0);
        detail_container_mut.add_widget(&detail_req_box);

        // Material Requirements Item Widget
        let ing_set = UIManager::create_widget(&format!("{}_detail_ing_set", tab_id), UIWidgetTypes::Default);
        let ing_set_mut = ptr_as_mut(ing_set.as_ref());
        let ui = ing_set_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(INGREDIENT_SET_HEIGHT);
        ui.set_color(COLOR_TRANSPARENT);
        detail_req_box_mut.add_widget(&ing_set);

        let ing_icon = UIManager::create_widget(&format!("{}_detail_ing_icon", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(ing_icon.as_ref()).get_ui_component_mut();
        ui.set_size(INGREDIENT_ICON_SIZE, INGREDIENT_ICON_SIZE);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(8.0);
        ui.set_color(COLOR_WHITE);
        ing_set_mut.add_widget(&ing_icon);

        let ing_lbl = UIManager::create_widget(&format!("{}_detail_ing_lbl", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(ing_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(INGREDIENT_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(FONT_SIZE_NORMAL);
        ui.set_font_color(COLOR_TEXT_NORMAL);
        ui.set_color(COLOR_TRANSPARENT);
        ing_set_mut.add_widget(&ing_lbl);

        // Populate items into left list container
        let first_item_data = item_list.first().cloned();
        let mut items = Vec::new();
        for (idx, item_data) in item_list.into_iter().enumerate() {
            let item_widget = ToolboxItemWidget::create(list_container_mut, item_data, idx);
            items.push(item_widget);
        }

        let first_req_type = first_item_data.as_ref().map(|d| d._item_data_type).unwrap_or(ItemDataType::None);
        let first_req_count = first_item_data.as_ref().map(|d| d._item_data_count).unwrap_or(0);

        let ing_widget = IngredientWidgetItem {
            _layout: ing_set,
            _icon: ing_icon,
            _label: ing_lbl,
            _item_type: first_req_type,
            _count: first_req_count,
        };

        Box::new(ToolboxTabWidget {
            _layout: layout,
            _list_container: list_container,
            _detail_container: detail_container,
            _items: items,
            _selected_index: 0,
            _detail_icon: detail_icon,
            _detail_name_lbl: detail_name_lbl,
            _detail_desc_box: detail_desc_box,
            _detail_info_labels: detail_info_labels,
            _detail_info_item_entries: detail_info_item_entries,
            _detail_info_char_entries: detail_info_char_entries,
            _detail_req_box: detail_req_box,
            _detail_ing_widgets: vec![ing_widget],
            _is_visible: false,
        })
    }

    pub fn open(&mut self) {
        if !self._is_visible {
            let tab_ptr = self as *mut ToolboxTabWidget<'a> as *mut c_void;
            for item in self._items.iter_mut() {
                item._tab_ptr = tab_ptr;
                let item_ptr = item.as_ref() as *const ToolboxItemWidget<'a> as *const c_void;
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
            }

            ptr_as_mut(self._layout.as_ref()).get_ui_component_mut().set_enable(true);
            self._is_visible = true;
            self.select_item(self._selected_index);
        }
    }

    pub fn close(&mut self) {
        if self._is_visible {
            ptr_as_mut(self._layout.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_visible = false;
        }
    }
}
