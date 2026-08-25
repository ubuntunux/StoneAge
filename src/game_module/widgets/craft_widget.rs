use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_resources, get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_resources};
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

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

pub const CRAFT_RECIPES: [CraftRecipeData; 7] = [
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
    pub _desc_lbl: Rc<WidgetDefault<'a>>,
    pub _ing_widgets: Vec<IngredientWidgetItem<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _recipe_index: usize,
}

pub struct CraftWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _close_btn: Rc<WidgetDefault<'a>>,
    pub _items: Vec<Box<CraftWidgetItem<'a>>>,
    pub _is_opened: bool,
    pub _selected_index: usize,
    pub _last_stick_y: i8,
}

impl<'a> CraftWidget<'a> {
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
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_size(840.0, 520.0);
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
        ui.set_color(get_color32(30, 32, 36, 245));
        ui.set_border_color(get_color32(90, 95, 105, 255));
        ui.set_border(3.0);
        ui.set_round(10.0);
        ui.set_padding(12.0);
        ui.set_renderable(true);
        ui.set_enable(true);

        // Header container (Horizontal)
        let header = UIManager::create_widget("craft_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header.as_ref());
        let ui = header_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(40.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&header);

        // Header Title Label
        let title_label = UIManager::create_widget("craft_title", UIWidgetTypes::Default);
        let ui = ptr_as_mut(title_label.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(32.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text("Crafter's Workshop");
        ui.set_font_size(22.0);
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        header_mut.add_widget(&title_label);

        // Close Button [X]
        let close_btn = UIManager::create_widget("craft_close_btn", UIWidgetTypes::Default);
        let ui = ptr_as_mut(close_btn.as_ref()).get_ui_component_mut();
        ui.set_size(32.0, 32.0);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(70, 75, 85, 255));
        ui.set_border_color(get_color32(110, 115, 125, 255));
        ui.set_border(2.0);
        ui.set_round(6.0);
        ui.set_text("X");
        ui.set_font_size(18.0);
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(Self::callback_close_btn)));
        header_mut.add_widget(&close_btn);

        // Separator
        let separator = UIManager::create_widget("craft_sep", UIWidgetTypes::Default);
        let ui = ptr_as_mut(separator.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(2.0);
        ui.set_color(get_color32(75, 80, 90, 200));
        ui.set_margin(4.0);
        layer_mut.add_widget(&separator);

        // Subtitle / Prompt
        let subtitle = UIManager::create_widget("craft_subtitle", UIWidgetTypes::Default);
        let ui = ptr_as_mut(subtitle.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(20.0);
        ui.set_text("Select an item blueprint and click Craft to consume materials & forge equipment!");
        ui.set_font_size(14.0);
        ui.set_font_color(get_color32(255, 255, 255, 255));
        ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&subtitle);

        // Recipe items list container
        let content_pane = UIManager::create_widget("craft_content_pane", UIWidgetTypes::Default);
        let content_pane_mut = ptr_as_mut(content_pane.as_ref());
        let ui = content_pane_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_expandable(false);
        ui.set_scroll_y(true);
        ui.set_enable_renderable_area(true);
        ui.set_padding(4.0);
        ui.set_color(get_color32(0, 0, 0, 0));
        layer_mut.add_widget(&content_pane);

        let mut items = Vec::new();
        for (idx, recipe) in CRAFT_RECIPES.iter().enumerate() {
            let row = UIManager::create_widget(&format!("craft_row_{}", recipe.id), UIWidgetTypes::Default);
            let row_mut = ptr_as_mut(row.as_ref());
            let ui = row_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(74.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_color(get_color32(40, 43, 48, 220));
            ui.set_border_color(get_color32(65, 70, 78, 255));
            ui.set_border(2.0);
            ui.set_round(6.0);
            ui.set_margin(2.0);
            ui.set_padding(8.0);
            ui.set_touchable(true);
            ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
            ui.set_callback_touch_down(Some(Box::new(Self::callback_item_select)));
            content_pane_mut.add_widget(&row);

            // 1. Left Product Section (Vertical: Icon + Name on top, Description below)
            let product_set = UIManager::create_widget(&format!("craft_prod_set_{}", recipe.id), UIWidgetTypes::Default);
            let product_set_mut = ptr_as_mut(product_set.as_ref());
            let ui = product_set_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::VERTICAL);
            ui.set_size(250.0, 56.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(12.0);
            ui.set_color(get_color32(0, 0, 0, 0));
            row_mut.add_widget(&product_set);

            // Top Header: Icon + Name
            let product_hdr = UIManager::create_widget(&format!("craft_prod_hdr_{}", recipe.id), UIWidgetTypes::Default);
            let product_hdr_mut = ptr_as_mut(product_hdr.as_ref());
            let ui = product_hdr_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(32.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_color(get_color32(0, 0, 0, 0));
            product_set_mut.add_widget(&product_hdr);

            // Product Icon (32x32)
            let product_icon = UIManager::create_widget(&format!("craft_icon_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(product_icon.as_ref()).get_ui_component_mut();
            ui.set_size(32.0, 32.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_halign(HorizontalAlign::LEFT);
            ui.set_margin_right(6.0);
            ui.set_color(get_color32(255, 255, 255, 255));
            product_hdr_mut.add_widget(&product_icon);
            Self::setup_item_icon(&product_icon, recipe.item_code());

            // Product Name Label
            let item_name = Self::get_item_name_from_resource(recipe.item_code());
            let name_lbl = UIManager::create_widget(&format!("craft_name_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(32.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_text(&item_name);
            ui.set_font_size(20.0);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            product_hdr_mut.add_widget(&name_lbl);

            // Description Label
            let desc_lbl = UIManager::create_widget(&format!("craft_desc_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(desc_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(22.0);
            let display_desc = Self::get_item_description_from_resource(recipe.item_code());
            ui.set_text(&display_desc);
            ui.set_font_size(15.0);
            ui.set_font_color(get_color32(180, 185, 195, 255));
            ui.set_color(get_color32(0, 0, 0, 0));
            product_set_mut.add_widget(&desc_lbl);

            // 2. Middle Materials Box (Horizontal layout placed to the right of product section)
            let ing_box = UIManager::create_widget(&format!("craft_ing_box_{}", recipe.id), UIWidgetTypes::Default);
            let ing_box_mut = ptr_as_mut(ing_box.as_ref());
            let ui = ing_box_mut.get_ui_component_mut();
            ui.set_layout_type(UILayoutType::BoxLayout);
            ui.set_layout_orientation(Orientation::HORIZONTAL);
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(56.0);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_color(get_color32(0, 0, 0, 0));
            row_mut.add_widget(&ing_box);

            let mut ing_widgets = Vec::new();
            for (ing_idx, req) in recipe.materials.iter().enumerate() {
                let ing_set = UIManager::create_widget(&format!("craft_ing_set_{}_{}", recipe.id, ing_idx), UIWidgetTypes::Default);
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
                let ing_icon = UIManager::create_widget(&format!("craft_ing_icon_{}_{}", recipe.id, ing_idx), UIWidgetTypes::Default);
                let ui = ptr_as_mut(ing_icon.as_ref()).get_ui_component_mut();
                ui.set_size(28.0, 28.0);
                ui.set_valign(VerticalAlign::CENTER);
                ui.set_margin_right(4.0);
                ui.set_color(get_color32(255, 255, 255, 255));
                ing_set_mut.add_widget(&ing_icon);
                Self::setup_item_icon(&ing_icon, req.item_code());

                // Material Label (Name (have/need))
                let ing_lbl = UIManager::create_widget(&format!("craft_ing_lbl_{}_{}", recipe.id, ing_idx), UIWidgetTypes::Default);
                let ui = ptr_as_mut(ing_lbl.as_ref()).get_ui_component_mut();
                ui.set_size_y(28.0);
                ui.set_valign(VerticalAlign::CENTER);
                ui.set_font_size(17.0);
                ui.set_font_color(get_color32(230, 235, 240, 255));
                ui.set_color(get_color32(0, 0, 0, 0));
                ing_set_mut.add_widget(&ing_lbl);

                ing_widgets.push(IngredientWidgetItem {
                    _layout: ing_set,
                    _icon: ing_icon,
                    _label: ing_lbl,
                });
            }

            // 3. Right Action Button
            let craft_btn = UIManager::create_widget(&format!("craft_btn_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(craft_btn.as_ref()).get_ui_component_mut();
            ui.set_size(95.0, 38.0);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_color(get_color32(75, 80, 90, 255));
            ui.set_border_color(get_color32(115, 120, 130, 255));
            ui.set_border(2.0);
            ui.set_round(6.0);
            ui.set_text("Craft");
            ui.set_font_size(15.0);
            ui.set_font_color(get_color32(255, 255, 255, 255));
            ui.set_touchable(true);
            ui.set_callback_touch_down(Some(Box::new(Self::callback_craft_action)));
            row_mut.add_widget(&craft_btn);

            items.push(Box::new(CraftWidgetItem {
                _layout: row,
                _icon: product_icon,
                _name_lbl: name_lbl,
                _desc_lbl: desc_lbl,
                _ing_widgets: ing_widgets,
                _action_btn: craft_btn,
                _recipe_index: idx,
            }));
        }

        let widget = CraftWidget {
            _parent_widget: parent_widget as *const WidgetDefault<'a>,
            _layer: layer,
            _close_btn: close_btn,
            _items: items,
            _is_opened: false,
            _selected_index: 0,
            _last_stick_y: 0,
        };

        widget
    }

    fn callback_close_btn(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let self_ptr = ui.get_user_data() as *const CraftWidget<'a>;
        if !self_ptr.is_null() {
            ptr_as_mut(self_ptr).close_craft();
        }
        true
    }

    fn callback_item_touch_over(_ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_item_select(_ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_craft_action(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let item_ptr = ui.get_user_data() as *const CraftWidgetItem<'a>;
        if !item_ptr.is_null() {
            let recipe_index = unsafe { (*item_ptr)._recipe_index };
            Self::try_craft_recipe(recipe_index);
        }
        true
    }

    pub fn try_craft_recipe(recipe_index: usize) -> bool {
        if recipe_index >= CRAFT_RECIPES.len() {
            return false;
        }
        let recipe = &CRAFT_RECIPES[recipe_index];

        // Check material counts
        for req in recipe.materials {
            let have_count = get_game_ui_manager().get_item_count(req.item_code());
            if have_count < req.count {
                let recipe_name = Self::get_item_name_from_resource(recipe.item_code());
                let mat_name = Self::get_item_name_from_resource(req.item_code());
                log::warn!(
                    "[CraftWidget] Cannot craft {}: missing {} (have {}, need {})",
                    recipe_name,
                    mat_name,
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
            ptr_as_mut(self._close_btn.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

            for item in self._items.iter_mut() {
                let item_ptr = item.as_ref() as *const CraftWidgetItem<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
            }

            self.update_selection_highlight();
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

    pub fn refresh_recipe_labels(&mut self) {
        let ui_mgr = get_game_ui_manager();
        for item in self._items.iter_mut() {
            if item._recipe_index < CRAFT_RECIPES.len() {
                let recipe = &CRAFT_RECIPES[item._recipe_index];

                // Refresh main item name & description from ItemData
                let recipe_item_name = Self::get_item_name_from_resource(recipe.item_code());
                let name_ui = ptr_as_mut(item._name_lbl.as_ref()).get_ui_component_mut();
                name_ui.set_text(&recipe_item_name);

                let recipe_item_desc = Self::get_item_description_from_resource(recipe.item_code());
                if !recipe_item_desc.is_empty() && recipe_item_desc != recipe.item_code() {
                    let desc_ui = ptr_as_mut(item._desc_lbl.as_ref()).get_ui_component_mut();
                    desc_ui.set_text(&recipe_item_desc);
                }

                // Refresh materials with ItemData _name & icon
                let mut can_craft = true;
                for (ing_idx, req) in recipe.materials.iter().enumerate() {
                    if ing_idx < item._ing_widgets.len() {
                        let have = ui_mgr.get_item_count(req.item_code());
                        if have < req.count {
                            can_craft = false;
                        }
                        let mat_name = Self::get_item_name_from_resource(req.item_code());
                        let text = format!("{} ({}/{})", mat_name, have, req.count);
                        let lbl_ui = ptr_as_mut(item._ing_widgets[ing_idx]._label.as_ref()).get_ui_component_mut();
                        lbl_ui.set_text(&text);

                        if have >= req.count {
                            lbl_ui.set_font_color(get_color32(230, 235, 240, 255));
                        } else {
                            lbl_ui.set_font_color(get_color32(235, 100, 100, 255));
                        }
                    }
                }

                let btn_ui = ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut();
                if can_craft {
                    btn_ui.set_color(get_color32(75, 80, 90, 255));
                    btn_ui.set_border_color(get_color32(115, 120, 130, 255));
                    btn_ui.set_font_color(get_color32(255, 255, 255, 255));
                } else {
                    btn_ui.set_color(get_color32(45, 48, 52, 255));
                    btn_ui.set_border_color(get_color32(65, 70, 75, 255));
                    btn_ui.set_font_color(get_color32(150, 150, 150, 255));
                }
            }
        }
    }

    fn update_selection_highlight(&mut self) {
        for (idx, item) in self._items.iter_mut().enumerate() {
            let is_selected = idx == self._selected_index;
            let layout_ui = ptr_as_mut(item._layout.as_ref()).get_ui_component_mut();
            if is_selected {
                layout_ui.set_border_color(get_color32(180, 185, 195, 255));
                layout_ui.set_color(get_color32(65, 70, 80, 245));
            } else {
                layout_ui.set_border_color(get_color32(65, 70, 78, 255));
                layout_ui.set_color(get_color32(40, 43, 48, 220));
            }
        }
    }

    pub fn update_craft_widget(
        &mut self,
        _time_data: &TimeData,
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

        // Refresh material counts on update
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

        // Navigation (Up/Down)
        let is_up = keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || joystick_input_data._btn_up == ButtonState::Pressed;
        let is_down = keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || joystick_input_data._btn_down == ButtonState::Pressed;

        let stick_y = if joystick_input_data._stick_left_direction.y > 0 {
            1
        } else if joystick_input_data._stick_left_direction.y < 0 {
            -1
        } else {
            0
        };

        let stick_up = stick_y > 0 && self._last_stick_y <= 0;
        let stick_down = stick_y < 0 && self._last_stick_y >= 0;
        self._last_stick_y = stick_y;

        if (is_up || stick_up) && self._selected_index > 0 {
            self._selected_index -= 1;
            self.update_selection_highlight();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        } else if (is_down || stick_down) && self._selected_index + 1 < self._items.len() {
            self._selected_index += 1;
            self.update_selection_highlight();
            get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
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
