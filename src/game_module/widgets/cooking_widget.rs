use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut,
};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::item_detail_layout::*;
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
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

// ────────────────────────────────────────────────────────────────
// Cooking Specific UI Layout & Dimension Constants
// ────────────────────────────────────────────────────────────────
pub const COOKING_PANEL_WIDTH: f32 = 780.0;
pub const COOKING_PANEL_HEIGHT: f32 = 520.0;

pub const COLOR_PANEL_BG: u32 = get_color32(30, 32, 36, 245);
pub const COLOR_PANEL_BORDER: u32 = get_color32(90, 95, 105, 255);

pub const COLOR_BTN_COOK_BG: u32 = get_color32(75, 130, 85, 255);
pub const COLOR_BTN_COOK_BORDER: u32 = get_color32(115, 190, 130, 255);

pub struct IngredientReq {
    pub item_type: ItemDataType,
    pub count: usize,
}

impl IngredientReq {
    pub fn item_code(&self) -> &'static str {
        self.item_type.item_code()
    }
}

pub struct CookingRecipeData {
    pub id: &'static str,
    pub item_type: ItemDataType,
    pub ingredients: &'static [IngredientReq],
}

impl CookingRecipeData {
    pub fn item_code(&self) -> &'static str {
        self.item_type.item_code()
    }
}

pub const COOKING_RECIPES: [CookingRecipeData; 5] = [
    CookingRecipeData {
        id: "roast_meat",
        item_type: ItemDataType::RoastMeat,
        ingredients: &[IngredientReq {
            item_type: ItemDataType::Meat,
            count: 1,
        }],
    },
    CookingRecipeData {
        id: "fish_soup",
        item_type: ItemDataType::FishSoup,
        ingredients: &[IngredientReq {
            item_type: ItemDataType::Coconut,
            count: 1,
        }],
    },
    CookingRecipeData {
        id: "steamed_veg",
        item_type: ItemDataType::SteamedVegetables,
        ingredients: &[IngredientReq {
            item_type: ItemDataType::Coconut,
            count: 1,
        }],
    },
    CookingRecipeData {
        id: "energy_stew",
        item_type: ItemDataType::EnergyStew,
        ingredients: &[
            IngredientReq {
                item_type: ItemDataType::Meat,
                count: 1,
            },
            IngredientReq {
                item_type: ItemDataType::Coconut,
                count: 1,
            },
        ],
    },
    CookingRecipeData {
        id: "golden_feast",
        item_type: ItemDataType::GoldenFeast,
        ingredients: &[
            IngredientReq {
                item_type: ItemDataType::Meat,
                count: 2,
            },
            IngredientReq {
                item_type: ItemDataType::Coconut,
                count: 2,
            },
        ],
    },
];

pub struct CookingWidgetItem<'a> {
    pub _layout: Rc<WidgetDefault<'a>>,
    pub _icon: Rc<WidgetDefault<'a>>,
    pub _name_lbl: Rc<WidgetDefault<'a>>,
    pub _action_btn: Rc<WidgetDefault<'a>>,
    pub _recipe_index: usize,
    pub _widget_ptr: *const c_void,
}

pub struct CookingWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _list_container: Rc<WidgetDefault<'a>>,
    pub _detail_container: Rc<WidgetDefault<'a>>,
    pub _close_btn: Rc<WidgetDefault<'a>>,
    pub _items: Vec<Box<CookingWidgetItem<'a>>>,
    pub _detail_icon: Rc<WidgetDefault<'a>>,
    pub _detail_name_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_desc_lbl: Rc<WidgetDefault<'a>>,
    pub _detail_ing_widgets: Vec<IngredientWidgetItem<'a>>,
    pub _is_opened: bool,
    pub _selected_index: usize,
    pub _last_stick_y: i8,
    pub _nav_repeat_controller: WidgetNavRepeatController,
}

impl<'a> CookingWidget<'a> {
    pub fn get_item_name_from_resource(item_code: &str) -> String {
        get_item_name_from_resource(item_code)
    }

    pub fn get_item_description_from_resource(item_code: &str) -> String {
        get_item_description_from_resource(item_code)
    }

    pub fn setup_item_icon(icon_widget: &Rc<WidgetDefault<'a>>, item_code: &str) {
        setup_item_icon(icon_widget, item_code, true);
    }

    pub fn create_cooking_widget(parent_widget: &mut WidgetDefault<'a>) -> CookingWidget<'a> {
        let layer = UIManager::create_widget("cooking_widget_layer", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui = layer_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::VERTICAL);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_pivot_preset(PIVOT_CENTER);
        ui.set_pos_hint(Some(0.5), Some(0.5));
        ui.set_size(COOKING_PANEL_WIDTH, COOKING_PANEL_HEIGHT);
        ui.set_expandable(false);
        ui.set_enable_renderable_area(true);
        ui.set_color(COLOR_PANEL_BG);
        ui.set_border_color(COLOR_PANEL_BORDER);
        ui.set_border(3.0);
        ui.set_round(10.0);
        ui.set_padding(12.0);
        ui.set_renderable(true);
        ui.set_enable(true);

        // Header container (Horizontal)
        let header = UIManager::create_widget("cooking_header", UIWidgetTypes::Default);
        let header_mut = ptr_as_mut(header.as_ref());
        let ui = header_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(40.0);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(COLOR_TRANSPARENT);
        layer_mut.add_widget(&header);

        // Header Title Label
        let title_label = UIManager::create_widget("cooking_title", UIWidgetTypes::Default);
        let ui = ptr_as_mut(title_label.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(32.0);
        ui.set_halign(HorizontalAlign::LEFT);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_text("Chef's Cooking Station");
        ui.set_font_size(FONT_SIZE_TITLE);
        ui.set_font_color(COLOR_WHITE);
        ui.set_color(COLOR_TRANSPARENT);
        header_mut.add_widget(&title_label);

        // Close Button [X]
        let close_btn = UIManager::create_widget("cooking_close_btn", UIWidgetTypes::Default);
        let ui = ptr_as_mut(close_btn.as_ref()).get_ui_component_mut();
        ui.set_size(32.0, 32.0);
        ui.set_halign(HorizontalAlign::CENTER);
        ui.set_valign(VerticalAlign::CENTER);
        ui.set_color(get_color32(70, 75, 85, 255));
        ui.set_border_color(get_color32(110, 115, 125, 255));
        ui.set_border(2.0);
        ui.set_round(6.0);
        ui.set_text("X");
        ui.set_font_size(FONT_SIZE_NORMAL);
        ui.set_font_color(COLOR_WHITE);
        ui.set_touchable(true);
        ui.set_callback_touch_down(Some(Box::new(Self::callback_close_btn)));
        header_mut.add_widget(&close_btn);

        // Separator
        let separator = UIManager::create_widget("cooking_sep", UIWidgetTypes::Default);
        let ui = ptr_as_mut(separator.as_ref()).get_ui_component_mut();
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_y(2.0);
        ui.set_color(get_color32(75, 80, 90, 200));
        ui.set_margin(4.0);
        layer_mut.add_widget(&separator);

        // Main Master-Detail Split Pane (Horizontal)
        let main_pane = UIManager::create_widget("cooking_main_pane", UIWidgetTypes::Default);
        let main_pane_mut = ptr_as_mut(main_pane.as_ref());
        let ui = main_pane_mut.get_ui_component_mut();
        ui.set_layout_type(UILayoutType::BoxLayout);
        ui.set_layout_orientation(Orientation::HORIZONTAL);
        ui.set_size_hint_x(Some(1.0));
        ui.set_size_hint_y(Some(1.0));
        ui.set_padding(4.0);
        ui.set_color(COLOR_TRANSPARENT);
        layer_mut.add_widget(&main_pane);

        let master_detail = create_master_detail_components("cooking", main_pane_mut, 80.0, MAX_INGREDIENT_ENTRIES);

        // Build Left List Items
        let list_container_mut = ptr_as_mut(master_detail._list_container.as_ref());
        let mut items = Vec::new();
        for (idx, recipe) in COOKING_RECIPES.iter().enumerate() {
            let row = UIManager::create_widget(&format!("cooking_row_{}", recipe.id), UIWidgetTypes::Default);
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
            let icon = UIManager::create_widget(&format!("cooking_icon_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(icon.as_ref()).get_ui_component_mut();
            ui.set_size(LIST_ITEM_ICON_SIZE, LIST_ITEM_ICON_SIZE);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_margin_right(8.0);
            ui.set_color(COLOR_WHITE);
            row_mut.add_widget(&icon);
            Self::setup_item_icon(&icon, recipe.item_code());

            // Name Label
            let item_name = Self::get_item_name_from_resource(recipe.item_code());
            let name_lbl = UIManager::create_widget(&format!("cooking_name_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(name_lbl.as_ref()).get_ui_component_mut();
            ui.set_size_hint_x(Some(1.0));
            ui.set_size_y(LIST_ITEM_NAME_HEIGHT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_text(&item_name);
            ui.set_font_size(FONT_SIZE_TITLE);
            ui.set_font_color(COLOR_TEXT_TITLE);
            ui.set_color(COLOR_TRANSPARENT);
            row_mut.add_widget(&name_lbl);

            // Action Button ("Cook")
            let cook_btn = UIManager::create_widget(&format!("cooking_btn_{}", recipe.id), UIWidgetTypes::Default);
            let ui = ptr_as_mut(cook_btn.as_ref()).get_ui_component_mut();
            ui.set_size(LIST_ITEM_BTN_WIDTH, LIST_ITEM_BTN_HEIGHT);
            ui.set_valign(VerticalAlign::CENTER);
            ui.set_halign(HorizontalAlign::CENTER);
            ui.set_color(COLOR_BTN_DEFAULT_BG);
            ui.set_border_color(COLOR_BTN_DEFAULT_BORDER);
            ui.set_border(2.0);
            ui.set_round(6.0);
            ui.set_text("Cook");
            ui.set_font_size(FONT_SIZE_BUTTON);
            ui.set_font_color(COLOR_TEXT_NORMAL);
            ui.set_margin_right(4.0);
            ui.set_touchable(true);
            ui.set_callback_touch_over(Some(Box::new(Self::callback_item_touch_over)));
            ui.set_callback_touch_down(Some(Box::new(Self::callback_cook_action)));
            row_mut.add_widget(&cook_btn);

            items.push(Box::new(CookingWidgetItem {
                _layout: row,
                _icon: icon,
                _name_lbl: name_lbl,
                _action_btn: cook_btn,
                _recipe_index: idx,
                _widget_ptr: std::ptr::null(),
            }));
        }

        CookingWidget {
            _parent_widget: parent_widget as *const WidgetDefault<'a>,
            _layer: layer,
            _list_container: master_detail._list_container,
            _detail_container: master_detail._detail_container,
            _close_btn: close_btn,
            _items: items,
            _detail_icon: master_detail._detail_icon,
            _detail_name_lbl: master_detail._detail_name_lbl,
            _detail_desc_lbl: master_detail._detail_desc_lbl,
            _detail_ing_widgets: master_detail._detail_ing_widgets,
            _is_opened: false,
            _selected_index: 0,
            _last_stick_y: 0,
            _nav_repeat_controller: WidgetNavRepeatController::new(),
        }
    }

    fn callback_close_btn(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let self_ptr = ui.get_user_data() as *const CookingWidget<'a>;
        if !self_ptr.is_null() {
            ptr_as_mut(self_ptr).close_cooking();
        }
        true
    }

    fn callback_item_touch_over(_ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_item_select(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let item_ptr = ui.get_user_data() as *const CookingWidgetItem<'a>;
        if !item_ptr.is_null() {
            let widget_ptr = unsafe { (*item_ptr)._widget_ptr as *mut CookingWidget<'a> };
            if !widget_ptr.is_null() {
                let recipe_index = unsafe { (*item_ptr)._recipe_index };
                ptr_as_mut(widget_ptr).select_recipe(recipe_index);
            }
        }
        get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
        true
    }

    fn callback_cook_action(ui: &UIComponentInstance<'a>, _pos: &Vector2<f32>, _delta: &Vector2<f32>) -> bool {
        let item_ptr = ui.get_user_data() as *const CookingWidgetItem<'a>;
        if !item_ptr.is_null() {
            let widget_ptr = unsafe { (*item_ptr)._widget_ptr as *mut CookingWidget<'a> };
            let recipe_index = unsafe { (*item_ptr)._recipe_index };
            if !widget_ptr.is_null() {
                ptr_as_mut(widget_ptr).select_recipe(recipe_index);
            }
            Self::try_cook_recipe(recipe_index);
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

    pub fn try_cook_recipe(recipe_index: usize) -> bool {
        if recipe_index >= COOKING_RECIPES.len() {
            return false;
        }
        let recipe = &COOKING_RECIPES[recipe_index];

        // Check ingredient counts
        for req in recipe.ingredients {
            let have_count = get_game_ui_manager().get_total_item_count(req.item_code());
            if have_count < req.count {
                let recipe_name = Self::get_item_name_from_resource(recipe.item_code());
                let ing_name = Self::get_item_name_from_resource(req.item_code());
                log::warn!(
                    "[CookingWidget] Cannot cook {}: missing {} (have {}, need {})",
                    recipe_name,
                    ing_name,
                    have_count,
                    req.count
                );
                return false;
            }
        }

        // Deduct ingredients
        let item_mgr = get_item_manager_mut();
        for req in recipe.ingredients {
            item_mgr.remove_inventory_item(req.item_code(), req.count);
        }

        // Grant cooked item
        item_mgr.pick_item(recipe.item_code(), 1);
        get_game_ui_manager_mut().notify_item_acquired(recipe.item_code(), 1, true);
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        let recipe_name = Self::get_item_name_from_resource(recipe.item_code());
        log::info!("[CookingWidget] Cooked {}", recipe_name);
        true
    }

    pub fn is_opened_cooking(&self) -> bool {
        self._is_opened
    }

    pub fn open_cooking(&mut self) {
        if !self._is_opened {
            ptr_as_mut(self._parent_widget).add_widget(&self._layer);
            self._is_opened = true;
            self._selected_index = 0;

            let self_ptr = self as *const CookingWidget<'a> as *const c_void;
            ptr_as_mut(self._close_btn.as_ref()).get_ui_component_mut().set_user_data(self_ptr);

            for item in self._items.iter_mut() {
                item._widget_ptr = self_ptr;
                let item_ptr = item.as_ref() as *const CookingWidgetItem<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
            }

            self.select_recipe(0);
            self.refresh_recipe_labels();
        }
    }

    pub fn close_cooking(&mut self) {
        if self._is_opened {
            ptr_as_mut(self._parent_widget).remove_widget(self._layer.as_ref());
            self._is_opened = false;
            get_character_manager_mut().reset_all_npc_interacting();
        }
    }

    pub fn update_detail_panel(&mut self) {
        if self._selected_index >= COOKING_RECIPES.len() {
            return;
        }

        let recipe = &COOKING_RECIPES[self._selected_index];
        let item_code = recipe.item_code();

        Self::setup_item_icon(&self._detail_icon, item_code);
        let name_text = Self::get_item_name_from_resource(item_code);
        ptr_as_mut(self._detail_name_lbl.as_ref()).get_ui_component_mut().set_text(&name_text);

        let desc_text = Self::get_item_description_from_resource(item_code);
        ptr_as_mut(self._detail_desc_lbl.as_ref()).get_ui_component_mut().set_text(&desc_text);

        let ui_mgr = get_game_ui_manager();
        for (i, ing_widget) in self._detail_ing_widgets.iter_mut().enumerate() {
            let layout_ui = ptr_as_mut(ing_widget._layout.as_ref()).get_ui_component_mut();
            if i < recipe.ingredients.len() {
                let req = &recipe.ingredients[i];
                let req_code = req.item_code();
                Self::setup_item_icon(&ing_widget._icon, req_code);

                let have = ui_mgr.get_total_item_count(req_code);
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
            if item._recipe_index < COOKING_RECIPES.len() {
                let recipe = &COOKING_RECIPES[item._recipe_index];

                let recipe_item_name = Self::get_item_name_from_resource(recipe.item_code());
                let name_ui = ptr_as_mut(item._name_lbl.as_ref()).get_ui_component_mut();
                name_ui.set_text(&recipe_item_name);

                let mut can_cook = true;
                for req in recipe.ingredients.iter() {
                    let have = ui_mgr.get_total_item_count(req.item_code());
                    if have < req.count {
                        can_cook = false;
                        break;
                    }
                }

                let btn_ui = ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut();
                if can_cook {
                    btn_ui.set_color(COLOR_BTN_COOK_BG);
                    btn_ui.set_border_color(COLOR_BTN_COOK_BORDER);
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

    pub fn update_cooking_widget(
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
            self.close_cooking();
            return;
        }

        // Navigation (Up/Down with hold repeat)
        let delta_time: f32 = time_data._delta_time_with_scale as f32;
        let (should_move, dir_opt) =
            self._nav_repeat_controller.update(keyboard_input_data, joystick_input_data, delta_time);

        if should_move {
            let (_dir_x, dir_y) = dir_opt.unwrap();
            let selected_idx = if dir_y < 0 && self._selected_index > 0 {
                Some(self._selected_index - 1)
            } else if dir_y > 0 && self._selected_index + 1 < self._items.len() {
                Some(self._selected_index + 1)
            } else {
                None
            };
            if let Some(idx) = selected_idx {
                self.select_recipe(idx);
                get_audio_manager_mut().play_audio_bank(AUDIO_SELECT_ITEM, AudioLoop::ONCE, None);
                if let Some(item) = self._items.get(idx) {
                    move_mouse_to_ui_component(item._layout.get_ui_component());
                }
            }
        }

        // Enter or Space or Gamepad A/X to Cook selected item
        if keyboard_input_data.get_key_pressed(KeyCode::Enter)
            || keyboard_input_data.get_key_pressed(KeyCode::Space)
            || joystick_input_data._btn_a == ButtonState::Pressed
            || joystick_input_data._btn_x == ButtonState::Pressed
        {
            if Self::try_cook_recipe(self._selected_index) {
                self.refresh_recipe_labels();
            }
        }
    }
}
