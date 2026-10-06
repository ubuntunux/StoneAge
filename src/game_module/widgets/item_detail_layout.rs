use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_service_locator::get_game_resources;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::rc::Rc;

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
pub const DESCRIPTION_LABEL_HEIGHT: f32 = 28.0;
pub const REQUIREMENT_HEADER_HEIGHT: f32 = 26.0;
pub const INGREDIENT_SET_HEIGHT: f32 = 36.0;
pub const INGREDIENT_ICON_SIZE: f32 = 30.0;
pub const INGREDIENT_LABEL_HEIGHT: f32 = 30.0;

pub const FONT_SIZE_TITLE: f32 = 22.0;
pub const FONT_SIZE_NORMAL: f32 = 22.0;
pub const FONT_SIZE_BUTTON: f32 = 22.0;
pub const FONT_SIZE_DESC: f32 = 22.0;

// ────────────────────────────────────────────────────────────────
// Color Constants
// ────────────────────────────────────────────────────────────────
pub const COLOR_TRANSPARENT: u32 = get_color32(0, 0, 0, 0);
pub const COLOR_WHITE: u32 = get_color32(255, 255, 255, 255);

pub const COLOR_LIST_BG: u32 = get_color32(245, 232, 205, 220);
pub const COLOR_LIST_BORDER: u32 = get_color32(188, 148, 105, 200);

pub const COLOR_DETAIL_BG: u32 = get_color32(252, 245, 226, 230);
pub const COLOR_DETAIL_BORDER: u32 = get_color32(188, 148, 105, 200);
pub const COLOR_BOX_BG: u32 = get_color32(240, 225, 195, 200);
pub const COLOR_BOX_BORDER: u32 = get_color32(195, 160, 120, 255);

pub const COLOR_ITEM_NORMAL_BG: u32 = get_color32(242, 228, 198, 220);
pub const COLOR_ITEM_NORMAL_BORDER: u32 = get_color32(195, 160, 120, 255);
pub const COLOR_ITEM_SELECTED_BG: u32 = get_color32(235, 205, 155, 240);
pub const COLOR_ITEM_SELECTED_BORDER: u32 = get_color32(255, 195, 40, 255);

pub const COLOR_TEXT_TITLE: u32 = get_color32(75, 45, 20, 255);
pub const COLOR_TEXT_NORMAL: u32 = get_color32(75, 45, 20, 255);
pub const COLOR_TEXT_MUTED: u32 = get_color32(120, 85, 55, 255);
pub const COLOR_TEXT_DISABLED: u32 = get_color32(150, 125, 95, 255);
pub const COLOR_TEXT_SUCCESS: u32 = get_color32(65, 135, 45, 255);
pub const COLOR_TEXT_ERROR: u32 = get_color32(195, 60, 50, 255);

pub const COLOR_BTN_CRAFT_BG: u32 = get_color32(118, 168, 68, 240);
pub const COLOR_BTN_CRAFT_BORDER: u32 = get_color32(72, 115, 38, 255);
pub const COLOR_BTN_DISABLED_BG: u32 = get_color32(210, 185, 150, 255);
pub const COLOR_BTN_DISABLED_BORDER: u32 = get_color32(175, 145, 110, 255);
pub const COLOR_BTN_DEFAULT_BG: u32 = get_color32(195, 118, 58, 240);
pub const COLOR_BTN_DEFAULT_BORDER: u32 = get_color32(115, 60, 28, 220);

pub const MAX_INGREDIENT_ENTRIES: usize = 4;

// ────────────────────────────────────────────────────────────────
// Helper Functions
// ────────────────────────────────────────────────────────────────
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

pub fn setup_item_icon<'a>(icon_widget: &Rc<WidgetDefault<'a>>, item_code: &str, enable: bool) {
    let mut has_item_data = false;
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

// ────────────────────────────────────────────────────────────────
// Common Structs
// ────────────────────────────────────────────────────────────────
pub struct IngredientWidgetItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _label: Rc<WidgetDefault<'a>>,
    pub _item_type: ItemDataType,
    pub _count: usize,
}

impl<'a> IngredientWidgetItem<'a> {
    pub fn new(layout: Rc<WidgetDefault<'a>>, icon: Rc<WidgetDefault<'a>>, label: Rc<WidgetDefault<'a>>) -> Self {
        Self {
            _layout: layout,
            _icon: icon,
            _label: label,
            _item_type: ItemDataType::None,
            _count: 0,
        }
    }

    pub fn item_code(&self) -> &'static str {
        self._item_type.item_code()
    }
}

pub struct MasterDetailComponents<'a> {
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _detail_container: Rc<WidgetDefault<'a>>,
    pub _detail_icon: Rc<WidgetDefault<'a>>,
    pub _detail_name_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_desc_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_desc_box: Rc<WidgetDefault<'a>>,
    pub _detail_req_box: Rc<WidgetDefault<'a>>,
    pub _detail_ing_widgets: Vec<IngredientWidgetItem<'a>>,
}

pub fn create_master_detail_components<'a>(
    prefix: &str,
    parent_layout: &mut WidgetDefault<'a>,
    desc_box_height: f32,
    max_ingredients: usize,
) -> MasterDetailComponents<'a> {
    // 1. Left List Container
    let list_container = UIManager::create_widget(&format!("{}_list_container", prefix), UIWidgetTypes::Default);
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
    parent_layout.add_widget(&list_container);

    // 2. Right Detail Container
    let detail_container = UIManager::create_widget(&format!("{}_detail_container", prefix), UIWidgetTypes::Default);
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
    parent_layout.add_widget(&detail_container);

    // Detail Header: Icon + Name
    let detail_hdr = UIManager::create_widget(&format!("{}_detail_hdr", prefix), UIWidgetTypes::Default);
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

    let detail_icon = UIManager::create_widget(&format!("{}_detail_icon", prefix), UIWidgetTypes::Default);
    let ui = ptr_as_mut(detail_icon.as_ref()).get_ui_component_mut();
    ui.set_size(DETAIL_ICON_SIZE, DETAIL_ICON_SIZE);
    ui.set_valign(VerticalAlign::CENTER);
    ui.set_margin_right(12.0);
    ui.set_color(COLOR_TEXT_TITLE);
    detail_hdr_mut.add_widget(&detail_icon);

    let detail_name_lbl = UIManager::create_widget(&format!("{}_detail_name", prefix), UIWidgetTypes::Default);
    let ui = ptr_as_mut(detail_name_lbl.as_ref()).get_ui_component_mut();
    ui.set_size_hint_x(Some(1.0));
    ui.set_size_y(DETAIL_NAME_LABEL_HEIGHT);
    ui.set_valign(VerticalAlign::CENTER);
    ui.set_font_size(FONT_SIZE_TITLE);
    ui.set_font_color(COLOR_TEXT_TITLE);
    ui.set_color(COLOR_TRANSPARENT);
    detail_hdr_mut.add_widget(&detail_name_lbl);

    // Description Box
    let detail_desc_box = UIManager::create_widget(&format!("{}_detail_desc_box", prefix), UIWidgetTypes::Default);
    let detail_desc_box_mut = ptr_as_mut(detail_desc_box.as_ref());
    let ui = detail_desc_box_mut.get_ui_component_mut();
    ui.set_layout_type(UILayoutType::BoxLayout);
    ui.set_layout_orientation(Orientation::VERTICAL);
    ui.set_size_hint_x(Some(1.0));
    if desc_box_height > 0.0 {
        ui.set_size_y(desc_box_height);
    } else {
        ui.set_expandable_y(true);
        ui.set_size_y(0.0);
    }
    ui.set_padding(8.0);
    ui.set_color(COLOR_BOX_BG);
    ui.set_border_color(COLOR_BOX_BORDER);
    ui.set_border(1.0);
    ui.set_round(6.0);
    ui.set_margin_bottom(12.0);
    detail_container_mut.add_widget(&detail_desc_box);

    let detail_desc_lbl = UIManager::create_widget(&format!("{}_detail_desc", prefix), UIWidgetTypes::Default);
    let ui = ptr_as_mut(detail_desc_lbl.as_ref()).get_ui_component_mut();
    ui.set_size_hint_x(Some(1.0));
    ui.set_size_hint_y(Some(1.0));
    ui.set_valign(VerticalAlign::TOP);
    ui.set_font_size(FONT_SIZE_DESC);
    ui.set_font_color(COLOR_TEXT_MUTED);
    ui.set_color(COLOR_TRANSPARENT);
    detail_desc_box_mut.add_widget(&detail_desc_lbl);

    // Requirements Header
    let req_text = UIManager::create_widget(&format!("{}_req_text", prefix), UIWidgetTypes::Default);
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
    let detail_req_box = UIManager::create_widget(&format!("{}_req_box", prefix), UIWidgetTypes::Default);
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
    for i in 0..max_ingredients {
        let ing_set = UIManager::create_widget(&format!("{}_detail_ing_set_{}", prefix, i), UIWidgetTypes::Default);
        let ing_set_mut = ptr_as_mut(ing_set.as_ref());
        let ui = ing_set_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(INGREDIENT_SET_HEIGHT);
        ui.set_color(COLOR_TRANSPARENT);
        detail_req_box_mut.add_widget(&ing_set);

        let ing_icon = UIManager::create_widget(&format!("{}_detail_ing_icon_{}", prefix, i), UIWidgetTypes::Default);
        let ui = ptr_as_mut(ing_icon.as_ref()).get_ui_component_mut();
        ui.set_size(INGREDIENT_ICON_SIZE, INGREDIENT_ICON_SIZE);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_margin_right(8.0);
        ui.set_color(COLOR_TEXT_TITLE);
        ing_set_mut.add_widget(&ing_icon);

        let ing_lbl = UIManager::create_widget(&format!("{}_detail_ing_lbl_{}", prefix, i), UIWidgetTypes::Default);
        let ui = ptr_as_mut(ing_lbl.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(INGREDIENT_LABEL_HEIGHT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_font_size(FONT_SIZE_NORMAL);
        ui.set_font_color(COLOR_TEXT_NORMAL);
        ui.set_color(COLOR_TRANSPARENT);
        ing_set_mut.add_widget(&ing_lbl);

        detail_ing_widgets.push(IngredientWidgetItem::new(ing_set, ing_icon, ing_lbl));
    }

    MasterDetailComponents {
        _list_container: list_container,
        _detail_container: detail_container,
        _detail_icon: detail_icon,
        _detail_name_lbl: detail_name_lbl,
        _detail_desc_lbl: detail_desc_lbl,
        _detail_desc_box: detail_desc_box,
        _detail_req_box: detail_req_box,
        _detail_ing_widgets: detail_ing_widgets,
    }
}
