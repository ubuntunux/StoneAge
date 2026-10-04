use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_resources, get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

// ────────────────────────────────────────────────────────────────
// UI Layout & Dimension Constants
// ────────────────────────────────────────────────────────────────
pub const LEFT_LIST_PANEL_WIDTH: f32 = 290.0;
pub const LIST_ITEM_ROW_HEIGHT: f32 = 54.0;
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
pub const FONT_SIZE_TITLE: f32 = 22.0;
pub const FONT_SIZE_NORMAL: f32 = 18.0;
pub const FONT_SIZE_BUTTON: f32 = 18.0;
pub const FONT_SIZE_DESC: f32 = 16.0;

// ────────────────────────────────────────────────────────────────
// Color Constants
// ────────────────────────────────────────────────────────────────
pub const COLOR_TRANSPARENT: u32 = get_color32(0, 0, 0, 0);
pub const COLOR_WHITE: u32 = get_color32(255, 255, 255, 255);

pub const COLOR_LIST_BG: u32 = get_color32(25, 27, 30, 220);
pub const COLOR_LIST_BORDER: u32 = get_color32(50, 55, 60, 255);

pub const COLOR_DETAIL_BG: u32 = get_color32(35, 38, 43, 230);
pub const COLOR_DETAIL_BORDER: u32 = get_color32(65, 70, 78, 255);
pub const COLOR_BOX_BG: u32 = get_color32(28, 30, 34, 200);
pub const COLOR_BOX_BORDER: u32 = get_color32(55, 60, 68, 255);

pub const COLOR_ITEM_NORMAL_BG: u32 = get_color32(40, 43, 48, 220);
pub const COLOR_ITEM_NORMAL_BORDER: u32 = get_color32(65, 70, 78, 255);
pub const COLOR_ITEM_SELECTED_BG: u32 = get_color32(60, 70, 85, 230);
pub const COLOR_ITEM_SELECTED_BORDER: u32 = get_color32(110, 160, 220, 255);

pub const COLOR_TEXT_TITLE: u32 = get_color32(240, 240, 240, 255);
pub const COLOR_TEXT_NORMAL: u32 = get_color32(220, 225, 230, 255);
pub const COLOR_TEXT_MUTED: u32 = get_color32(190, 195, 205, 255);
pub const COLOR_TEXT_DISABLED: u32 = get_color32(150, 150, 150, 255);
pub const COLOR_TEXT_SUCCESS: u32 = get_color32(130, 220, 160, 255);
pub const COLOR_TEXT_ERROR: u32 = get_color32(235, 100, 100, 255);

pub const COLOR_BTN_CRAFT_BG: u32 = get_color32(75, 130, 85, 255);
pub const COLOR_BTN_CRAFT_BORDER: u32 = get_color32(115, 190, 130, 255);
pub const COLOR_BTN_DISABLED_BG: u32 = get_color32(45, 48, 52, 255);
pub const COLOR_BTN_DISABLED_BORDER: u32 = get_color32(65, 70, 75, 255);
pub const COLOR_BTN_DEFAULT_BG: u32 = get_color32(65, 65, 65, 255);
pub const COLOR_BTN_DEFAULT_BORDER: u32 = get_color32(100, 100, 100, 255);

pub const MAX_INGREDIENT_ENTRIES: usize = 4;

pub struct IngredientReq {
    pub item_type: ItemDataType,
    pub count: usize,
}

impl IngredientReq {
    pub fn item_code(&self) -> &'static str {
        self.item_type.item_code()
    }
}

pub struct CraftRecipeData {
    pub id: &'static str,
    pub item_type: ItemDataType,
    pub materials: &'static [IngredientReq],
}

impl CraftRecipeData {
    pub fn item_code(&self) -> &'static str {
        self.item_type.item_code()
    }
}

pub const CRAFT_RECIPES: [CraftRecipeData; 8] = [
    CraftRecipeData {
        id: "wooden_club",
        item_type: ItemDataType::WoodenClub,
        materials: &[IngredientReq {
            item_type: ItemDataType::Wood,
            count: 1,
        }],
    },
    CraftRecipeData {
        id: "stone_axe",
        item_type: ItemDataType::StoneAxe,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 2,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 2,
            },
        ],
    },
    CraftRecipeData {
        id: "flint_spear",
        item_type: ItemDataType::Spear,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 3,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 2,
            },
        ],
    },
    CraftRecipeData {
        id: "hunting_bow",
        item_type: ItemDataType::Bow,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 4,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 2,
            },
        ],
    },
    CraftRecipeData {
        id: "leather_armor",
        item_type: ItemDataType::LeatherArmor,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Meat,
                count: 3,
            },
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 2,
            },
        ],
    },
    CraftRecipeData {
        id: "bone_shield",
        item_type: ItemDataType::BoneShield,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 4,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 2,
            },
        ],
    },
    CraftRecipeData {
        id: "campfire",
        item_type: ItemDataType::Campfire,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 5,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 5,
            },
        ],
    },
    CraftRecipeData {
        id: "worktable",
        item_type: ItemDataType::Worktable,
        materials: &[
            IngredientReq {
                item_type: ItemDataType::Wood,
                count: 8,
            },
            IngredientReq {
                item_type: ItemDataType::Rock,
                count: 4,
            },
        ],
    },
];

pub struct IngredientWidgetItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _label: Rc<WidgetDefault<'a>>,
}

pub struct CraftWidgetItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _name_lbl: Rc<WidgetDefault<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _recipe_index: usize,
    pub _widget_ptr: *const c_void,
}

pub struct CraftWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _detail_container: Rc<WidgetDefault<'a>>,
    pub _items: Vec<Box<CraftWidgetItem<'a>>>,
    pub _detail_icon: Rc<WidgetDefault<'a>>,
    pub _detail_name_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_desc_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_ing_widgets: Vec<IngredientWidgetItem<'a>>,
    pub _is_opened: bool,
    pub _selected_index: usize,
    pub _last_stick_y: i8,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> CraftWidget<'a> {
    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {}

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

    pub fn create_craft_widget(parent_widget: &mut WidgetDefault<'a>) -> CraftWidget<'a> {
        let layer = UIManager::create_widget("craft_widget_layer", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui = layer_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
        ui.set_color(COLOR_TRANSPARENT);
        ui.set_renderable(true);
        ui.set_enable(true);

        // 1. Left List Container (Width ~ 290)
        let list_container = UIManager::create_widget("craft_list_container", UIWidgetTypes::Default);
        let list_container_mut = ptr_as_mut(list_container.as_ref());
        let ui = list_container_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_scroll_y(true);
        ui.set_enable_renderable_area(true);
        ui.set_padding(4.0);
        ui.set_margin_right(8.0);
        ui.set_color(COLOR_LIST_BG);
        ui.set_border_color(COLOR_LIST_BORDER);
        ui.set_border(1.0);
        ui.set_round(6.0);
        layer_mut.add_widget(&list_container);

        // 2. Right Detail Container
        let detail_container = UIManager::create_widget("craft_detail_container", UIWidgetTypes::Default);
        let detail_container_mut = ptr_as_mut(detail_container.as_ref());
        let ui = detail_container_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::TOP);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_padding(14.0);
        ui.set_color(COLOR_DETAIL_BG);
        ui.set_border_color(COLOR_DETAIL_BORDER);
        ui.set_border(1.0);
        ui.set_round(6.0);
        layer_mut.add_widget(&detail_container);

        // Detail Header: Icon + Name
        let detail_hdr = UIManager::create_widget("craft_detail_hdr", UIWidgetTypes::Default);
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

        let detail_icon = UIManager::create_widget("craft_detail_icon", UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_icon.as_ref()).get_ui_component_mut();
        ui.set_size(DETAIL_ICON_SIZE, DETAIL_ICON_SIZE);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(12.0);
        ui.set_color(COLOR_WHITE);
        detail_hdr_mut.add_widget(&detail_icon);

        let detail_name_lbl = UIManager::create_widget("craft_detail_name", UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_name_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(DETAIL_NAME_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(FONT_SIZE_TITLE);
        ui.set_font_color(COLOR_WHITE);
        ui.set_color(COLOR_TRANSPARENT);
        detail_hdr_mut.add_widget(&detail_name_lbl);

        // Description Box
        let detail_desc_box = UIManager::create_widget("craft_detail_desc_box", UIWidgetTypes::Default);
        let detail_desc_box_mut = ptr_as_mut(detail_desc_box.as_ref());
        let ui = detail_desc_box_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(70.0);
        ui.set_padding(8.0);
        ui.set_color(COLOR_BOX_BG);
        ui.set_border_color(COLOR_BOX_BORDER);
        ui.set_border(1.0);
        ui.set_round(6.0);
        ui.set_margin_bottom(12.0);
        detail_container_mut.add_widget(&detail_desc_box);

        let detail_desc_lbl = UIManager::create_widget("craft_detail_desc", UIWidgetTypes::Default);
        let ui = ptr_as_mut(detail_desc_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_valign(VerticalAlign::TOP);
        ui.set_font_size(FONT_SIZE_DESC);
        ui.set_font_color(COLOR_TEXT_MUTED);
        ui.set_color(COLOR_TRANSPARENT);
        detail_desc_box_mut.add_widget(&detail_desc_lbl);

        // Requirements Header
        let req_text = UIManager::create_widget("craft_req_text", UIWidgetTypes::Default);
        let ui = ptr_as_mut(req_text.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(REQUIREMENT_HEADER_HEIGHT);
        ui.set_margin_bottom(4.0);
        ui.set_color(COLOR_TRANSPARENT);
        ui.set_font_size(FONT_SIZE_NORMAL);
        ui.set_text("Requirements:");
        ui.set_font_color(COLOR_TEXT_SUCCESS);
        detail_container_mut.add_widget(&req_text);

        // Requirements Box
        let detail_req_box = UIManager::create_widget("craft_req_box", UIWidgetTypes::Default);
        let detail_req_box_mut = ptr_as_mut(detail_req_box.as_ref());
        let ui = detail_req_box_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
        ui.set_padding(8.0);
        ui.set_color(COLOR_BOX_BG);
        ui.set_border_color(COLOR_BOX_BORDER);
        ui.set_border(1.0);
        ui.set_round(6.0);
        detail_container_mut.add_widget(&detail_req_box);

        let mut detail_ing_widgets = Vec::new();
        for i in 0..MAX_INGREDIENT_ENTRIES {
            let ing_set = UIManager::create_widget(&format!("craft_detail_ing_set_{}", i), UIWidgetTypes::Default);
            let ing_set_mut = ptr_as_mut(ing_set.as_ref());
            let ui = ing_set_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(INGREDIENT_SET_HEIGHT);
            ui.set_color(COLOR_TRANSPARENT);
            detail_req_box_mut.add_widget(&ing_set);

            let ing_icon = UIManager::create_widget(&format!("craft_detail_ing_icon_{}", i), UIWidgetTypes::Default);
            let ui = ptr_as_mut(ing_icon.as_ref()).get_ui_component_mut();
            ui.set_size(INGREDIENT_ICON_SIZE, INGREDIENT_ICON_SIZE);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(8.0);
            ui.set_color(COLOR_WHITE);
            ing_set_mut.add_widget(&ing_icon);

            let ing_lbl = UIManager::create_widget(&format!("craft_detail_ing_lbl_{}", i), UIWidgetTypes::Default);
            let ui = ptr_as_mut(ing_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(INGREDIENT_LABEL_HEIGHT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_font_size(FONT_SIZE_NORMAL);
            ui.set_font_color(COLOR_TEXT_NORMAL);
            ui.set_color(COLOR_TRANSPARENT);
            ing_set_mut.add_widget(&ing_lbl);

            detail_ing_widgets.push(IngredientWidgetItem {
                _layout: ing_set,
                _icon: ing_icon,
                _label: ing_lbl,
            });
        }

        // Build Left List Items
        let mut items = Vec::new();
        for (idx, recipe) in CRAFT_RECIPES.iter().enumerate() {
            let row = UIManager::create_widget(&format!("craft_row_{}", recipe.id), UIWidgetTypes::Default);
            let row_mut = ptr_as_mut(row.as_ref());
            let ui = row_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(LIST_ITEM_ROW_HEIGHT);
            ui.set_padding(6.0);
            ui.set_color(COLOR_ITEM_NORMAL_BG);
            ui.set_border_color(COLOR_ITEM_NORMAL_BORDER);
            ui.set_border(2.0);
            ui.set_round(6.0);
            ui.set_margin(2.0);
            ui.set_touchable(true);
            ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
            ui.set_callback_touch_down(Some(Box::new(Self::callback_item_select)));
            list_container_mut.add_widget(&row);

            // Icon
            let icon = UIManager::create_widget(&format!("craft_icon_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
            ui.set_size(LIST_ITEM_ICON_SIZE, LIST_ITEM_ICON_SIZE);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(8.0);
            ui.set_color(COLOR_WHITE);
            row_mut.add_widget(&icon);
            Self::setup_item_icon(&icon, recipe.item_code());

            // Name Label
            let item_name = Self::get_item_name_from_resource(recipe.item_code());
            let name_lbl = UIManager::create_widget(&format!("craft_name_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(LIST_ITEM_NAME_HEIGHT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_text(&item_name);
            ui.set_font_size(FONT_SIZE_TITLE);
            ui.set_font_color(COLOR_TEXT_TITLE);
            ui.set_color(COLOR_TRANSPARENT);
            row_mut.add_widget(&name_lbl);

            // Action Button ("Craft")
            let craft_btn = UIManager::create_widget(&format!("craft_btn_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(craft_btn.as_ref()).get_ui_component_mut();
            ui.set_size(LIST_ITEM_BTN_WIDTH, LIST_ITEM_BTN_HEIGHT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_color(COLOR_BTN_DEFAULT_BG);
            ui.set_border_color(COLOR_BTN_DEFAULT_BORDER);
            ui.set_border(2.0);
            ui.set_round(6.0);
            ui.set_text("Craft");
            ui.set_font_size(FONT_SIZE_BUTTON);
            ui.set_font_color(COLOR_TEXT_NORMAL);
            ui.set_margin_right(4.0);
            ui.set_touchable(true);
            ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
            ui.set_callback_touch_down(Some(Box::new(Self::callback_craft_action)));
            row_mut.add_widget(&craft_btn);

            items.push(Box::new(CraftWidgetItem {
                _layout: row,
                _icon: icon,
                _name_lbl: name_lbl,
                _action_btn: craft_btn,
                _recipe_index: idx,
                _widget_ptr: std::ptr::null(),
            }));
        }

        CraftWidget {
            _parent_widget: parent_widget as *const WidgetDefault<'a>,
            _layer: layer,
            _list_container: list_container,
            _detail_container: detail_container,
            _items: items,
            _detail_icon: detail_icon,
            _detail_name_lbl: detail_name_lbl,
            _detail_desc_lbl: detail_desc_lbl,
            _detail_ing_widgets: detail_ing_widgets,
            _is_opened: false,
            _selected_index: 0,
            _last_stick_y: 0,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        }
    }

    fn callback_item_touch_over(_ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_item_select(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let item_ptr = ui.get_user_data() as *const CraftWidgetItem<'a>;
        if !item_ptr.is_null() {
            let widget_ptr = unsafe { (*item_ptr)._widget_ptr as *mut CraftWidget<'a> };
            if !widget_ptr.is_null() {
                let recipe_index = unsafe { (*item_ptr)._recipe_index };
                ptr_as_mut(widget_ptr).select_recipe(recipe_index);
            }
        }
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_craft_action(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let item_ptr = ui.get_user_data() as *const CraftWidgetItem<'a>;
        if !item_ptr.is_null() {
            let widget_ptr = unsafe { (*item_ptr)._widget_ptr as *mut CraftWidget<'a> };
            let recipe_index = unsafe { (*item_ptr)._recipe_index };
            if !widget_ptr.is_null() {
                ptr_as_mut(widget_ptr).select_recipe(recipe_index);
            }
            Self::try_craft_recipe(recipe_index);
            if !widget_ptr.is_null() {
                ptr_as_mut(widget_ptr).refresh_recipe_labels();
            }
        }
        true
    }

    pub fn select_recipe(&mut self, index: usize) {
        if index >= self._items.len() {
            return;
        }
        self._selected_index = index;
        self.update_selection_highlight();
        self.update_detail_panel();
    }

    pub fn try_craft_recipe(recipe_index: usize) -> bool {
        if recipe_index >= CRAFT_RECIPES.len() {
            return false;
        }
        let recipe = &CRAFT_RECIPES[recipe_index];

        // Check ingredient counts
        for req in recipe.materials {
            let have_count = get_game_ui_manager().get_item_count(req.item_code());
            if have_count < req.count {
                let recipe_name = Self::get_item_name_from_resource(recipe.item_code());
                let ing_name = Self::get_item_name_from_resource(req.item_code());
                log::warn!(
                    "[CraftWidget] Cannot craft {}: missing {} (have {}, need {})",
                    recipe_name,
                    ing_name,
                    have_count,
                    req.count
                );
                return false;
            }
        }

        // Deduct materials
        let item_mgr = get_item_manager_mut();
        for req in recipe.materials {
            item_mgr.remove_inventory_item(req.item_code(), req.count);
        }

        // Grant crafted item
        item_mgr.pick_item(recipe.item_code(), 1);
        get_game_ui_manager_mut().notify_item_crafted();
        get_game_ui_manager_mut().notify_item_acquired(recipe.item_code(), 1, true);
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        let recipe_name = Self::get_item_name_from_resource(recipe.item_code());
        log::info!("[CraftWidget] Crafted {}", recipe_name);
        true
    }

    pub fn is_opened_craft(&self) -> bool {
        self._is_opened
    }

    pub fn open_craft(&mut self) {
        if !self._is_opened {
            ptr_as_mut(self._parent_widget).add_widget(&self._layer);
            self._is_opened = true;
            self._selected_index = 0;

            let self_ptr = self as *const CraftWidget<'a> as *const c_void;
            for item in self._items.iter_mut() {
                item._widget_ptr = self_ptr;
                let item_ptr = item.as_ref() as *const CraftWidgetItem<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
            }

            self.select_recipe(0);
            self.refresh_recipe_labels();
        }
    }

    pub fn close_craft(&mut self) {
        if self._is_opened {
            ptr_as_mut(self._parent_widget).remove_widget(self._layer.as_ref());
            self._is_opened = false;
            get_character_manager_mut().reset_all_npc_interacting();
        }
    }

    pub fn update_detail_panel(&mut self) {
        if self._selected_index >= CRAFT_RECIPES.len() {
            return;
        }

        let recipe = &CRAFT_RECIPES[self._selected_index];
        let item_code = recipe.item_code();

        Self::setup_item_icon(&self._detail_icon, item_code);
        let name_text = Self::get_item_name_from_resource(item_code);
        ptr_as_mut(self._detail_name_lbl.as_ref()).get_ui_component_mut().set_text(&name_text);

        let desc_text = Self::get_item_description_from_resource(item_code);
        ptr_as_mut(self._detail_desc_lbl.as_ref()).get_ui_component_mut().set_text(&desc_text);

        let ui_mgr = get_game_ui_manager();
        for (i, ing_widget) in self._detail_ing_widgets.iter_mut().enumerate() {
            let layout_ui = ptr_as_mut(ing_widget._layout.as_ref()).get_ui_component_mut();
            if i < recipe.materials.len() {
                let req = &recipe.materials[i];
                let req_code = req.item_code();
                Self::setup_item_icon(&ing_widget._icon, req_code);

                let have = ui_mgr.get_item_count(req_code);
                let ing_name = Self::get_item_name_from_resource(req_code);
                let text = format!("{} ({}/{})", ing_name, have, req.count);
                let lbl_ui = ptr_as_mut(ing_widget._label.as_ref()).get_ui_component_mut();
                lbl_ui.set_text(&text);
                if have >= req.count {
                    lbl_ui.set_font_color(COLOR_TEXT_NORMAL);
                } else {
                    lbl_ui.set_font_color(COLOR_TEXT_ERROR);
                }
                layout_ui.set_enable(true);
            } else {
                layout_ui.set_enable(false);
            }
        }
    }

    pub fn refresh_recipe_labels(&mut self) {
        let ui_mgr = get_game_ui_manager();
        for item in self._items.iter_mut() {
            if item._recipe_index < CRAFT_RECIPES.len() {
                let recipe = &CRAFT_RECIPES[item._recipe_index];

                let recipe_item_name = Self::get_item_name_from_resource(recipe.item_code());
                let name_ui = ptr_as_mut(item._name_lbl.as_ref()).get_ui_component_mut();
                name_ui.set_text(&recipe_item_name);

                let mut can_craft = true;
                for req in recipe.materials.iter() {
                    let have = ui_mgr.get_item_count(req.item_code());
                    if have < req.count {
                        can_craft = false;
                        break;
                    }
                }

                let btn_ui = ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut();
                if can_craft {
                    btn_ui.set_color(COLOR_BTN_CRAFT_BG);
                    btn_ui.set_border_color(COLOR_BTN_CRAFT_BORDER);
                    btn_ui.set_font_color(COLOR_WHITE);
                    btn_ui.set_touchable(true);
                } else {
                    btn_ui.set_color(COLOR_BTN_DISABLED_BG);
                    btn_ui.set_border_color(COLOR_BTN_DISABLED_BORDER);
                    btn_ui.set_font_color(COLOR_TEXT_DISABLED);
                    btn_ui.set_touchable(false);
                }
            }
        }

        self.update_detail_panel();
    }

    fn update_selection_highlight(&mut self) {
        let container_ui = ptr_as_mut(self._list_container.as_ref()).get_ui_component_mut();
        for (idx, item) in self._items.iter_mut().enumerate() {
            let is_selected = idx == self._selected_index;
            let layout_ui = ptr_as_mut(item._layout.as_ref()).get_ui_component_mut();
            if is_selected {
                layout_ui.set_border_color(COLOR_ITEM_SELECTED_BORDER);
                layout_ui.set_color(COLOR_ITEM_SELECTED_BG);
                container_ui.scroll_into_view(layout_ui);
            } else {
                layout_ui.set_border_color(COLOR_ITEM_NORMAL_BORDER);
                layout_ui.set_color(COLOR_ITEM_NORMAL_BG);
            }
        }
    }

    pub fn update_craft_widget(
        &mut self,
        time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character>,
    ) {
        if !self._is_opened {
            return;
        }

        // Refresh ingredient counts on update
        self.refresh_recipe_labels();

        // ESC or KeyB or Gamepad Start/Back to close
        if keyboard_input_data.get_key_pressed(KeyCode::Escape)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyB)
            || joystick_input_data._btn_b == ButtonState::Pressed
            || joystick_input_data._btn_start == ButtonState::Pressed
        {
            self.close_craft();
            return;
        }

        // Navigation (Up/Down with hold repeat)
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if should_move {
            let (_dir_x, dir_y) = dir_opt.unwrap();
            if dir_y < 0 && self._selected_index > 0 {
                self.select_recipe(self._selected_index - 1);
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            } else if dir_y > 0 && self._selected_index + 1 < self._items.len() {
                self.select_recipe(self._selected_index + 1);
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
            }
        }

        // Enter or Space or Gamepad A/X to Craft selected item
        if keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed
            || joystick_input_data._btn_x == ButtonState::Pressed
        {
            if Self::try_craft_recipe(self._selected_index) {
                self.refresh_recipe_labels();
            }
        }
    }
}
