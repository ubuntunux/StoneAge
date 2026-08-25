use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_QUEST_COMPLETE, ITEM_ENERGY_BALL};
use crate::game_module::game_service_locator::{
    get_character_manager, get_character_manager_mut, get_game_resources, get_game_scene_manager, get_game_ui_manager,
    get_game_ui_manager_mut,
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
use std::ffi::c_void;
use std::rc::Rc;

const ITEM_ROW_HEIGHT: f32 = 80.0;
const ACTION_BUTTON_WIDTH: f32 = 120.0;
const ACTION_BUTTON_HEIGHT: f32 = 34.0;

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
    log::info!(
        "[Toolbox] Spawned NPC {} near Monolith at {:?}",
        character_name,
        spawn_pos
    );
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolboxIconType {
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
        }
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolboxItemState {
    Locked,
    Unlocked,
}

#[derive(Clone, Debug)]
pub struct ToolboxItemData {
    pub id: String,
    pub icon_type: ToolboxIconType,
    pub description: String,
    pub energy_cost: usize,
}

impl ToolboxItemData {
    pub fn cost_label(&self) -> String {
        if self.energy_cost == 0 {
            "Free".to_string()
        } else if self.energy_cost == 1 {
            "1 Energy".to_string()
        } else {
            format!("{} Energy", self.energy_cost)
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

pub struct ToolboxItemWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _product_icon: Rc<WidgetDefault<'a>>,
    pub _name_lbl: Rc<WidgetDefault<'a>>,
    pub _desc_lbl: Rc<WidgetDefault<'a>>,
    pub _status_label: Rc<WidgetDefault<'a>>,
    pub _ing_widgets: Vec<IngredientWidgetItem<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _state: ToolboxItemState,
    pub _data: ToolboxItemData,
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

    pub fn setup_item_icon(icon_widget: &Rc<WidgetDefault<'a>>, item_code: &str) {
        let resources = get_game_resources();
        if resources.has_item_data(item_code) {
            let item_data = resources.get_item_data(item_code).borrow();
            let mat_name = &item_data._ui_material_instance;
            if !mat_name.is_empty() {
                let engine_res = get_engine_resources();
                if engine_res.has_material_instance_data(mat_name.as_str()) {
                    let material = engine_res.get_material_instance_data(mat_name.as_str());
                    let ui = ptr_as_mut(icon_widget.as_ref()).get_ui_component_mut();
                    ui.set_material_instance(Some(material.clone()));
                }
            }
        }
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
        _ui_component: &UIComponentInstance<'a>,
        _touched_pos: &nalgebra::Vector2<f32>,
        _touched_pos_delta: &nalgebra::Vector2<f32>,
    ) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        true
    }

    pub fn callback_item_action(
        ui_component: &UIComponentInstance<'a>,
        _touched_pos: &nalgebra::Vector2<f32>,
        _touched_pos_delta: &nalgebra::Vector2<f32>,
    ) -> bool {
        let item_ptr = ui_component.get_user_data() as *mut ToolboxItemWidget<'a>;
        if item_ptr.is_null() {
            return false;
        }
        let item = ptr_as_mut(item_ptr);
        item.toggle_state();
        true
    }

    pub fn toggle_state(&mut self) {
        match self._state {
            ToolboxItemState::Locked => {
                let cost = self._data.energy_cost;
                if cost > 0 {
                    let current_energy_balls = get_game_ui_manager().get_item_count(ITEM_ENERGY_BALL);
                    if current_energy_balls >= cost {
                        if get_game_ui_manager_mut().remove_item(ITEM_ENERGY_BALL, cost) {
                            self._state = ToolboxItemState::Unlocked;
                            get_game_ui_manager_mut().notify_item_crafted();
                            get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                            if let Some((char_data_name, offset)) = self._data.icon_type.npc_character_info() {
                                spawn_npc_near_monolith(char_data_name, offset);
                            }
                            log::info!(
                                "[Toolbox] Unlocked item: {} (Consumed {} EnergyBall(s))",
                                self._data.icon_type.as_str(),
                                cost
                            );
                            self.update_ui();
                        } else {
                            log::warn!("[Toolbox] Failed to remove EnergyBall from inventory");
                        }
                    } else {
                        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
                        log::warn!(
                            "[Toolbox] Cannot unlock {}: Needs {} EnergyBall(s), but only have {}",
                            self._data.icon_type.as_str(),
                            cost,
                            current_energy_balls
                        );
                        let mat_name = Self::get_item_name_from_resource(ITEM_ENERGY_BALL);
                        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
                        status_ui.set_text(&format!("Need {} {} (Have {})", cost, mat_name, current_energy_balls));
                        status_ui.set_font_color(get_color32(230, 80, 80, 255));
                    }
                } else {
                    self._state = ToolboxItemState::Unlocked;
                    get_game_ui_manager_mut().notify_item_crafted();
                    get_audio_manager_mut().play_audio_bank(AUDIO_QUEST_COMPLETE, AudioLoop::ONCE, None);
                    if let Some((char_data_name, offset)) = self._data.icon_type.npc_character_info() {
                        spawn_npc_near_monolith(char_data_name, offset);
                    }
                    log::info!("[Toolbox] Unlocked free item: {}", self._data.icon_type.as_str());
                    self.update_ui();
                }
            }
            ToolboxItemState::Unlocked => {
                // Item is already unlocked
            }
        }
    }

    pub fn update_ui(&mut self) {
        let ui_mgr = get_game_ui_manager();

        // Refresh description if set in ItemData
        let item_code = self._data.icon_type.item_code();
        let desc_text = Self::get_item_description_from_resource(item_code);
        if !desc_text.is_empty() && desc_text != item_code {
            let desc_ui = ptr_as_mut(self._desc_lbl.as_ref()).get_ui_component_mut();
            desc_ui.set_text(&desc_text);
        }

        // Refresh material widgets
        for ing_widget in self._ing_widgets.iter_mut() {
            let item_code = ing_widget.item_code();
            let have_count = ui_mgr.get_item_count(item_code);
            let mat_name = Self::get_item_name_from_resource(item_code);
            let text = format!("{} ({}/{})", mat_name, have_count, ing_widget._count);
            let lbl_ui = ptr_as_mut(ing_widget._label.as_ref()).get_ui_component_mut();
            lbl_ui.set_text(&text);
            if have_count >= ing_widget._count {
                lbl_ui.set_font_color(get_color32(230, 235, 240, 255));
            } else {
                lbl_ui.set_font_color(get_color32(235, 100, 100, 255));
            }
        }

        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        let btn_ui = ptr_as_mut(self._action_btn.as_ref()).get_ui_component_mut();

        match self._state {
            ToolboxItemState::Locked => {
                status_ui.set_text("Status: Locked");
                status_ui.set_font_color(get_color32(150, 150, 150, 255));

                btn_ui.set_text(&format!("Unlock ({})", self._data.cost_label()));
                let current_energy_balls = ui_mgr.get_item_count(ITEM_ENERGY_BALL);
                if self._data.energy_cost == 0 || current_energy_balls >= self._data.energy_cost {
                    btn_ui.set_color(get_color32(75, 80, 90, 255));
                    btn_ui.set_border_color(get_color32(115, 120, 130, 255));
                    btn_ui.set_font_color(get_color32(255, 255, 255, 255));
                } else {
                    btn_ui.set_color(get_color32(45, 48, 52, 255));
                    btn_ui.set_border_color(get_color32(65, 70, 75, 255));
                    btn_ui.set_font_color(get_color32(150, 150, 150, 255));
                }
                btn_ui.set_renderable(true);
                btn_ui.set_touchable(true);
                btn_ui.set_enable(true);
            }
            ToolboxItemState::Unlocked => {
                status_ui.set_text("Status: Unlocked");
                status_ui.set_font_color(get_color32(100, 210, 120, 255));

                btn_ui.set_renderable(false);
                btn_ui.set_touchable(false);
                btn_ui.set_enable(false);
            }
        }
    }

    pub fn create(parent_widget: &mut WidgetDefault<'a>, data: ToolboxItemData) -> Box<ToolboxItemWidget<'a>> {
        // Main row container (Neutral dark gray)
        let layout = UIManager::create_widget(&format!("item_row_{}", data.id), UIWidgetTypes::Default);
        let layout_mut = ptr_as_mut(layout.as_ref());
        let ui = layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(ITEM_ROW_HEIGHT);
        ui.set_padding(8.0);
        ui.set_color(get_color32(40, 43, 48, 220));
        ui.set_border_color(get_color32(65, 70, 78, 255));
        ui.set_border(2.0);
        ui.set_round(6.0);
        ui.set_margin(3.0);
        ui.set_touchable(true);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_select)));
        parent_widget.add_widget(&layout);

        // 1. Left Product Section (Vertical: Icon + Name on top, Description below)
        let product_set = UIManager::create_widget(&format!("item_prod_set_{}", data.id), UIWidgetTypes::Default);
        let product_set_mut = ptr_as_mut(product_set.as_ref());
        let ui = product_set_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_size(240.0, 68.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(10.0);
        ui.set_color(get_color32(0, 0, 0, 0));
        layout_mut.add_widget(&product_set);

        // Top Header: Icon + Name
        let product_hdr = UIManager::create_widget(&format!("item_prod_hdr_{}", data.id), UIWidgetTypes::Default);
        let product_hdr_mut = ptr_as_mut(product_hdr.as_ref());
        let ui = product_hdr_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(36.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(0, 0, 0, 0));
        product_set_mut.add_widget(&product_hdr);

        // Product Icon (32x32)
        let product_icon = UIManager::create_widget(&format!("item_prod_icon_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(product_icon.as_ref()).get_ui_component_mut();
        ui.set_size(32.0, 32.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_margin_right(6.0);
        ui.set_color(get_color32(255, 255, 255, 255));
        product_hdr_mut.add_widget(&product_icon);
        Self::setup_item_icon(&product_icon, data.icon_type.item_code());

        // Product Name Label
        let item_code = data.icon_type.item_code();
        let item_name = Self::get_item_name_from_resource(item_code);
        let display_name = if item_name == item_code {
            data.icon_type.as_str().to_string()
        } else {
            item_name
        };
        let name_lbl = UIManager::create_widget(&format!("item_name_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(32.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text(&display_name);
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        product_hdr_mut.add_widget(&name_lbl);

        // Description Label
        let desc_text = Self::get_item_description_from_resource(item_code);
        let display_desc = if desc_text.is_empty() || desc_text == item_code {
            data.description.clone()
        } else {
            desc_text
        };
        let desc_lbl = UIManager::create_widget(&format!("item_desc_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(desc_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(26.0);
        ui.set_text(&display_desc);
        ui.set_font_size(14.0);
        ui.set_font_color(get_color32(180, 185, 195, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        product_set_mut.add_widget(&desc_lbl);

        // 2. Middle Materials Box (Horizontal layout for required ingredients)
        let ing_box = UIManager::create_widget(&format!("item_ing_box_{}", data.id), UIWidgetTypes::Default);
        let ing_box_mut = ptr_as_mut(ing_box.as_ref());
        let ui = ing_box_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(56.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(0, 0, 0, 0));
        layout_mut.add_widget(&ing_box);

        let mut ing_widgets = Vec::new();
        if data.energy_cost > 0 {
            let ing_set = UIManager::create_widget(&format!("item_ing_set_{}", data.id), UIWidgetTypes::Default);
            let ing_set_mut = ptr_as_mut(ing_set.as_ref());
            let ui = ing_set_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_y(32.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(14.0);
            ui.set_color(get_color32(0, 0, 0, 0));
            ing_box_mut.add_widget(&ing_set);

            // Material Icon (28x28)
            let ing_icon = UIManager::create_widget(&format!("item_ing_icon_{}", data.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(ing_icon.as_ref()).get_ui_component_mut();
            ui.set_size(28.0, 28.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(4.0);
            ui.set_color(get_color32(255, 255, 255, 255));
            ing_set_mut.add_widget(&ing_icon);
            Self::setup_item_icon(&ing_icon, ITEM_ENERGY_BALL);

            // Material Label (Name (have/cost))
            let ing_lbl = UIManager::create_widget(&format!("item_ing_lbl_{}", data.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(ing_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_y(28.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(16.0);
            ui.set_font_color(get_color32(230, 235, 240, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ing_set_mut.add_widget(&ing_lbl);

            ing_widgets.push(IngredientWidgetItem {
                _layout: ing_set,
                _icon: ing_icon,
                _label: ing_lbl,
                _item_type: ItemDataType::EnergyBall,
                _count: data.energy_cost,
            });
        } else {
            let free_lbl = UIManager::create_widget(&format!("item_free_lbl_{}", data.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(free_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_y(28.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_text("Free");
            ui.set_font_size(16.0);
            ui.set_font_color(get_color32(100, 210, 120, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            ing_box_mut.add_widget(&free_lbl);
        }

        // 3. Right Section: Status label & Action Button
        let right_set = UIManager::create_widget(&format!("item_right_set_{}", data.id), UIWidgetTypes::Default);
        let right_set_mut = ptr_as_mut(right_set.as_ref());
        let ui = right_set_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_size(ACTION_BUTTON_WIDTH + 10.0, 56.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_color(get_color32(0, 0, 0, 0));
        layout_mut.add_widget(&right_set);

        // Status label
        let status_label = UIManager::create_widget(&format!("item_status_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(status_label.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(18.0);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text("Status: Locked");
        ui.set_font_size(13.0);
        ui.set_font_color(get_color32(150, 150, 150, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        right_set_mut.add_widget(&status_label);

        // Action button (Unlock)
        let action_btn = UIManager::create_widget(&format!("item_action_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(action_btn.as_ref()).get_ui_component_mut();
        ui.set_size(ACTION_BUTTON_WIDTH, ACTION_BUTTON_HEIGHT);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(65, 65, 65, 255));
        ui.set_border_color(get_color32(100, 100, 100, 255));
        ui.set_border(2.0);
        ui.set_round(6.0);
        ui.set_text(&format!("Unlock ({})", data.cost_label()));
        ui.set_font_size(15.0);
        ui.set_font_color(get_color32(230, 230, 230, 255));
        ui.set_touchable(true);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_action)));
        right_set_mut.add_widget(&action_btn);

        let mut item = Box::new(ToolboxItemWidget {
            _layout: layout,
            _product_icon: product_icon,
            _name_lbl: name_lbl,
            _desc_lbl: desc_lbl,
            _status_label: status_label,
            _ing_widgets: ing_widgets,
            _action_btn: action_btn,
            _state: ToolboxItemState::Locked,
            _data: data,
        });

        item.update_ui();
        item
    }
}

// ────────────────────────────────────────────────────────────────
// ToolboxTabWidget - Content pane for a category tab
// ────────────────────────────────────────────────────────────────
pub struct ToolboxTabWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _items: Vec<Box<ToolboxItemWidget<'a>>>,
    pub _is_visible: bool,
}

impl<'a> ToolboxTabWidget<'a> {
    pub fn create(
        tab_id: &str,
        category_title: &str,
        parent_widget: &mut WidgetDefault<'a>,
        item_list: Vec<ToolboxItemData>,
    ) -> Box<ToolboxTabWidget<'a>> {
        // Pane layout container (Neutral dark gray)
        let layout = UIManager::create_widget(&format!("{}_tab_layout", tab_id), UIWidgetTypes::Default);
        let layout_mut = ptr_as_mut(layout.as_ref());
        let ui = layout_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable(false);
        ui.set_scroll_y(true);
        ui.set_enable_renderable_area(true);
        ui.set_padding(10.0);
        ui.set_color(get_color32(30, 30, 30, 220));
        ui.set_renderable(true);
        ui.set_enable(false);
        parent_widget.add_widget(&layout);

        // Section header label
        let section_label = UIManager::create_widget(&format!("{}_section_label", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(section_label.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(36.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text(category_title);
        ui.set_font_size(20.0);
        ui.set_font_color(get_color32(200, 200, 200, 255));
        ui.set_margin(5.0);
        ui.set_renderable(false);
        layout_mut.add_widget(&section_label);

        // Separator line
        let separator = UIManager::create_widget(&format!("{}_separator", tab_id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(separator.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(2.0);
        ui.set_color(get_color32(75, 75, 75, 200));
        ui.set_margin(3.0);
        layout_mut.add_widget(&separator);

        let mut tab_widget = Box::new(ToolboxTabWidget {
            _layout: layout,
            _items: Vec::new(),
            _is_visible: false,
        });

        // Add items to the tab pane
        for item_data in item_list {
            let item_widget = ToolboxItemWidget::create(ptr_as_mut(tab_widget._layout.as_ref()), item_data);
            tab_widget._items.push(item_widget);
        }

        tab_widget
    }

    pub fn open(&mut self) {
        if !self._is_visible {
            for item in self._items.iter_mut() {
                let item_ptr = item.as_ref() as *const ToolboxItemWidget<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                item.update_ui();
            }
            ptr_as_mut(self._layout.as_ref()).get_ui_component_mut().set_enable(true);
            self._is_visible = true;
        }
    }

    pub fn close(&mut self) {
        if self._is_visible {
            ptr_as_mut(self._layout.as_ref()).get_ui_component_mut().set_enable(false);
            self._is_visible = false;
        }
    }
}
