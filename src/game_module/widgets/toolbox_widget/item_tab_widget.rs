use crate::game_module::actors::character::CharacterCreateInfo;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_QUEST_COMPLETE, ITEM_ENERGY_BALL};
use crate::game_module::game_service_locator::{
    get_character_manager, get_character_manager_mut, get_game_scene_manager, get_game_ui_manager,
    get_game_ui_manager_mut,
};
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

const ITEM_ROW_HEIGHT: f32 = 90.0;
const ITEM_ICON_SIZE: f32 = 70.0;
const ACTION_BUTTON_WIDTH: f32 = 130.0;
const ACTION_BUTTON_HEIGHT: f32 = 40.0;

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

pub struct ToolboxItemWidget<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _status_label: Rc<WidgetDefault<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _state: ToolboxItemState,
    pub _data: ToolboxItemData,
}

impl<'a> ToolboxItemWidget<'a> {
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
                        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
                        status_ui.set_text(&format!("Need {} EnergyBall (Have {})", cost, current_energy_balls));
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
        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        let btn_ui = ptr_as_mut(self._action_btn.as_ref()).get_ui_component_mut();

        match self._state {
            ToolboxItemState::Locked => {
                status_ui.set_text("Status: Locked");
                status_ui.set_font_color(get_color32(150, 150, 150, 255));

                btn_ui.set_text(&format!("Unlock ({})", self._data.cost_label()));
                btn_ui.set_color(get_color32(65, 65, 65, 255)); // Dark Gray
                btn_ui.set_border_color(get_color32(100, 100, 100, 255));
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
        ui.set_padding(10.0);
        ui.set_color(get_color32(45, 45, 45, 200));
        ui.set_border_color(get_color32(80, 80, 80, 255));
        ui.set_border(1.0);
        ui.set_round(8.0);
        ui.set_margin(5.0);
        ui.set_touchable(true);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_select)));
        parent_widget.add_widget(&layout);

        // Icon display (Gray tone)
        let icon = UIManager::create_widget(&format!("item_icon_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
        ui.set_size(ITEM_ICON_SIZE, ITEM_ICON_SIZE);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(65, 65, 65, 255));
        ui.set_border_color(get_color32(110, 110, 110, 255));
        ui.set_border(2.0);
        ui.set_round(8.0);
        ui.set_margin(5.0);
        ui.set_text(data.icon_type.icon_str());
        ui.set_font_size(15.0);
        ui.set_font_color(get_color32(230, 230, 230, 255));
        layout_mut.add_widget(&icon);

        // Info container (Vertical layout: Name, Description, Status Label)
        let info = UIManager::create_widget(&format!("item_info_{}", data.id), UIWidgetTypes::Default);
        let info_mut = ptr_as_mut(info.as_ref());
        let ui = info_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(ITEM_ROW_HEIGHT);
        ui.set_padding(5.0);
        ui.set_renderable(false);
        layout_mut.add_widget(&info);

        // Name label
        let name_label = UIManager::create_widget(&format!("item_name_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(name_label.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(26.0);
        ui.set_text(data.icon_type.as_str());
        ui.set_font_size(20.0);
        ui.set_font_color(get_color32(220, 220, 220, 255));
        ui.set_renderable(false);
        info_mut.add_widget(&name_label);

        // Description label
        let desc_label = UIManager::create_widget(&format!("item_desc_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(desc_label.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(22.0);
        ui.set_text(&data.description);
        ui.set_font_size(15.0);
        ui.set_font_color(get_color32(160, 160, 160, 220));
        ui.set_renderable(false);
        info_mut.add_widget(&desc_label);

        // Status label
        let status_label = UIManager::create_widget(&format!("item_status_{}", data.id), UIWidgetTypes::Default);
        let ui = ptr_as_mut(status_label.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(20.0);
        ui.set_text("Status: Locked");
        ui.set_font_size(14.0);
        ui.set_font_color(get_color32(150, 150, 150, 255));
        ui.set_renderable(false);
        info_mut.add_widget(&status_label);

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
        ui.set_margin(10.0);
        ui.set_text(&format!("Unlock ({})", data.cost_label()));
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(230, 230, 230, 255));
        ui.set_touchable(true);
        ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_action)));
        layout_mut.add_widget(&action_btn);

        let item = Box::new(ToolboxItemWidget {
            _layout: layout,
            _status_label: status_label,
            _action_btn: action_btn,
            _state: ToolboxItemState::Locked,
            _data: data,
        });

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
