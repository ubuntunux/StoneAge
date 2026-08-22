use crate::game_module::game_constants::ITEM_ENERGY_BALL;
use crate::game_module::game_service_locator::{get_game_ui_manager, get_game_ui_manager_mut};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

const ITEM_ROW_HEIGHT: f32 = 90.0;
const ITEM_ICON_SIZE: f32 = 70.0;
const ACTION_BUTTON_WIDTH: f32 = 130.0;
const ACTION_BUTTON_HEIGHT: f32 = 40.0;

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
    WoodenCart,
    RidingMammoth,
    FlintSpear,
    HuntingBow,
    LeatherArmor,
    BoneShield,
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
            ToolboxIconType::WoodenCart => "Wooden Cart",
            ToolboxIconType::RidingMammoth => "Riding Mammoth",
            ToolboxIconType::FlintSpear => "Flint Spear",
            ToolboxIconType::HuntingBow => "Hunting Bow",
            ToolboxIconType::LeatherArmor => "Leather Armor",
            ToolboxIconType::BoneShield => "Bone Shield",
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
            ToolboxIconType::WoodenCart => "[CART]",
            ToolboxIconType::RidingMammoth => "[MAMMOTH]",
            ToolboxIconType::FlintSpear => "[SPEAR]",
            ToolboxIconType::HuntingBow => "[BOW]",
            ToolboxIconType::LeatherArmor => "[ARMOR]",
            ToolboxIconType::BoneShield => "[SHIELD]",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolboxItemState {
    Unpurchased,
    Purchased,
    Active,
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
            ToolboxItemState::Unpurchased => {
                let cost = self._data.energy_cost;
                if cost > 0 {
                    let current_energy_balls = get_game_ui_manager().get_item_count(ITEM_ENERGY_BALL);
                    if current_energy_balls >= cost {
                        if get_game_ui_manager_mut().remove_item(ITEM_ENERGY_BALL, cost) {
                            self._state = ToolboxItemState::Purchased;
                            log::info!(
                                "[Toolbox] Purchased item: {} (Consumed {} EnergyBall(s))",
                                self._data.icon_type.as_str(),
                                cost
                            );
                            self.update_ui();
                        } else {
                            log::warn!("[Toolbox] Failed to remove EnergyBall from inventory");
                        }
                    } else {
                        log::warn!(
                            "[Toolbox] Cannot purchase {}: Needs {} EnergyBall(s), but only have {}",
                            self._data.icon_type.as_str(),
                            cost,
                            current_energy_balls
                        );
                        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
                        status_ui.set_text(&format!(
                            "Need {} EnergyBall (Have {})",
                            cost, current_energy_balls
                        ));
                        status_ui.set_font_color(get_color32(230, 80, 80, 255));
                    }
                } else {
                    self._state = ToolboxItemState::Purchased;
                    log::info!(
                        "[Toolbox] Purchased free item: {}",
                        self._data.icon_type.as_str()
                    );
                    self.update_ui();
                }
            }
            ToolboxItemState::Purchased => {
                self._state = ToolboxItemState::Active;
                log::info!(
                    "[Toolbox] Activated/Used item: {}",
                    self._data.icon_type.as_str()
                );
                self.update_ui();
            }
            ToolboxItemState::Active => {
                self._state = ToolboxItemState::Purchased;
                log::info!(
                    "[Toolbox] Deactivated item: {}",
                    self._data.icon_type.as_str()
                );
                self.update_ui();
            }
        }
    }

    pub fn update_ui(&mut self) {
        let status_ui = ptr_as_mut(self._status_label.as_ref()).get_ui_component_mut();
        let btn_ui = ptr_as_mut(self._action_btn.as_ref()).get_ui_component_mut();

        match self._state {
            ToolboxItemState::Unpurchased => {
                status_ui.set_text("Status: Not Owned");
                status_ui.set_font_color(get_color32(150, 150, 150, 255));

                btn_ui.set_text(&format!("Buy ({})", self._data.cost_label()));
                btn_ui.set_color(get_color32(65, 65, 65, 255)); // Dark Gray
                btn_ui.set_border_color(get_color32(100, 100, 100, 255));
            }
            ToolboxItemState::Purchased => {
                status_ui.set_text("Status: Owned");
                status_ui.set_font_color(get_color32(190, 190, 190, 255));

                btn_ui.set_text("Use");
                btn_ui.set_color(get_color32(100, 100, 100, 255)); // Medium Gray
                btn_ui.set_border_color(get_color32(140, 140, 140, 255));
            }
            ToolboxItemState::Active => {
                status_ui.set_text("Status: Active");
                status_ui.set_font_color(get_color32(230, 230, 230, 255));

                btn_ui.set_text("In Use");
                btn_ui.set_color(get_color32(145, 145, 145, 255)); // Light Gray Highlight
                btn_ui.set_border_color(get_color32(185, 185, 185, 255));
            }
        }
    }

    pub fn create(
        parent_widget: &mut WidgetDefault<'a>,
        data: ToolboxItemData,
    ) -> Box<ToolboxItemWidget<'a>> {
        // Main row container (Neutral dark gray)
        let layout = UIManager::create_widget(
            &format!("item_row_{}", data.id),
            UIWidgetTypes::Default,
        );
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
        parent_widget.add_widget(&layout);

        // Icon display (Gray tone)
        let icon = UIManager::create_widget(
            &format!("item_icon_{}", data.id),
            UIWidgetTypes::Default,
        );
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
        let info = UIManager::create_widget(
            &format!("item_info_{}", data.id),
            UIWidgetTypes::Default,
        );
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
        let name_label = UIManager::create_widget(
            &format!("item_name_{}", data.id),
            UIWidgetTypes::Default,
        );
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
        let desc_label = UIManager::create_widget(
            &format!("item_desc_{}", data.id),
            UIWidgetTypes::Default,
        );
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
        let status_label = UIManager::create_widget(
            &format!("item_status_{}", data.id),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(status_label.as_ref()).get_ui_component_mut();
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(20.0);
        ui.set_text("Status: Not Owned");
        ui.set_font_size(14.0);
        ui.set_font_color(get_color32(150, 150, 150, 255));
        ui.set_renderable(false);
        info_mut.add_widget(&status_label);

        // Action button (Buy / Use)
        let action_btn = UIManager::create_widget(
            &format!("item_action_{}", data.id),
            UIWidgetTypes::Default,
        );
        let ui = ptr_as_mut(action_btn.as_ref()).get_ui_component_mut();
        ui.set_size(ACTION_BUTTON_WIDTH, ACTION_BUTTON_HEIGHT);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(65, 65, 65, 255));
        ui.set_border_color(get_color32(100, 100, 100, 255));
        ui.set_border(2.0);
        ui.set_round(6.0);
        ui.set_margin(10.0);
        ui.set_text(&format!("Buy ({})", data.cost_label()));
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(230, 230, 230, 255));
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(Self::callback_item_action)));
        layout_mut.add_widget(&action_btn);

        let item = Box::new(ToolboxItemWidget {
            _layout: layout,
            _status_label: status_label,
            _action_btn: action_btn,
            _state: ToolboxItemState::Unpurchased,
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
        let layout = UIManager::create_widget(
            &format!("{}_tab_layout", tab_id),
            UIWidgetTypes::Default,
        );
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
        let section_label = UIManager::create_widget(
            &format!("{}_section_label", tab_id),
            UIWidgetTypes::Default,
        );
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
        let separator = UIManager::create_widget(
            &format!("{}_separator", tab_id),
            UIWidgetTypes::Default,
        );
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
            let item_widget = ToolboxItemWidget::create(
                ptr_as_mut(tab_widget._layout.as_ref()),
                item_data,
            );
            tab_widget._items.push(item_widget);
        }

        tab_widget
    }

    pub fn open(&mut self) {
        if !self._is_visible {
            for item in self._items.iter_mut() {
                let item_ptr = item.as_ref() as *const ToolboxItemWidget<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref())
                    .get_ui_component_mut()
                    .set_user_data(item_ptr);
            }
            ptr_as_mut(self._layout.as_ref())
                .get_ui_component_mut()
                .set_enable(true);
            self._is_visible = true;
        }
    }

    pub fn close(&mut self) {
        if self._is_visible {
            ptr_as_mut(self._layout.as_ref())
                .get_ui_component_mut()
                .set_enable(false);
            self._is_visible = false;
        }
    }
}
