use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, AUDIO_SELECT_ITEM};
use crate::game_module::game_controller::WidgetNavRepeatController;
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut,
};
use crate::game_module::game_ui_manager::move_mouse_to_ui_component;
use crate::game_module::widgets::item_detail_layout::*;
use crate::game_module::widgets::toolbox_widget::item_tab_widget::ToolboxIconType;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign,
    WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
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

    pub fn toolbox_icon_type(&self) -> ToolboxIconType {
        match self.item_type {
            ItemDataType::WoodenClub => ToolboxIconType::WoodenClub,
            ItemDataType::StoneAxe => ToolboxIconType::StoneAxe,
            ItemDataType::Spear => ToolboxIconType::FlintSpear,
            ItemDataType::Bow => ToolboxIconType::HuntingBow,
            ItemDataType::LeatherArmor => ToolboxIconType::LeatherArmor,
            ItemDataType::BoneShield => ToolboxIconType::BoneShield,
            ItemDataType::Campfire => ToolboxIconType::Campfire,
            ItemDataType::Worktable => ToolboxIconType::Worktable,
            _ => ToolboxIconType::WoodenClub,
        }
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
        get_item_name_from_resource(item_code)
    }

    pub fn get_item_description_from_resource(item_code: &str) -> String {
        get_item_description_from_resource(item_code)
    }

    pub fn setup_item_icon(icon_widget: &Rc<WidgetDefault<'a>>, item_code: &str) {
        setup_item_icon(icon_widget, item_code, true);
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

        let master_detail = create_master_detail_components("craft", layer_mut, 70.0, MAX_INGREDIENT_ENTRIES);

        // Build Left List Items
        let list_container_mut = ptr_as_mut(master_detail._list_container.as_ref());
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
            _list_container: master_detail._list_container,
            _detail_container: master_detail._detail_container,
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

    pub fn is_recipe_unlocked(index: usize) -> bool {
        if index < CRAFT_RECIPES.len() {
            let unlocked = get_game_ui_manager().get_unlocked_toolbox_items();
            unlocked.contains(&CRAFT_RECIPES[index].toolbox_icon_type())
        } else {
            false
        }
    }

    pub fn get_first_unlocked_index(&self) -> Option<usize> {
        let unlocked = get_game_ui_manager().get_unlocked_toolbox_items();
        (0..CRAFT_RECIPES.len()).find(|&i| unlocked.contains(&CRAFT_RECIPES[i].toolbox_icon_type()))
    }

    pub fn get_prev_unlocked_index(&self, current: usize) -> Option<usize> {
        let unlocked = get_game_ui_manager().get_unlocked_toolbox_items();
        (0..current).rfind(|&i| unlocked.contains(&CRAFT_RECIPES[i].toolbox_icon_type()))
    }

    pub fn get_next_unlocked_index(&self, current: usize) -> Option<usize> {
        let unlocked = get_game_ui_manager().get_unlocked_toolbox_items();
        ((current + 1)..CRAFT_RECIPES.len()).find(|&i| unlocked.contains(&CRAFT_RECIPES[i].toolbox_icon_type()))
    }

    pub fn select_recipe(&mut self, index: usize) {
        if index >= self._items.len() || !Self::is_recipe_unlocked(index) {
            return;
        }
        self._selected_index = index;
        self.update_selection_highlight();
        self.update_detail_panel();
    }

    pub fn try_craft_recipe(recipe_index: usize) -> bool {
        if recipe_index >= CRAFT_RECIPES.len() || !Self::is_recipe_unlocked(recipe_index) {
            return false;
        }
        let recipe = &CRAFT_RECIPES[recipe_index];

        // Check ingredient counts
        for req in recipe.materials {
            let have_count = get_game_ui_manager().get_total_item_count(req.item_code());
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

            let self_ptr = self as *const CraftWidget<'a> as *const c_void;
            for item in self._items.iter_mut() {
                item._widget_ptr = self_ptr;
                let item_ptr = item.as_ref() as *const CraftWidgetItem<'a> as *const c_void;
                ptr_as_mut(item._action_btn.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
                ptr_as_mut(item._layout.as_ref()).get_ui_component_mut().set_user_data(item_ptr);
            }

            self.refresh_recipe_labels();

            if let Some(first_unlocked_idx) = self.get_first_unlocked_index() {
                self.select_recipe(first_unlocked_idx);
            } else {
                self.update_detail_panel();
            }
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
        let unlocked_set = get_game_ui_manager().get_unlocked_toolbox_items();

        if self._selected_index >= CRAFT_RECIPES.len()
            || !unlocked_set.contains(&CRAFT_RECIPES[self._selected_index].toolbox_icon_type())
        {
            if let Some(first_unlocked) = self.get_first_unlocked_index() {
                self._selected_index = first_unlocked;
            } else {
                ptr_as_mut(self._detail_name_lbl.as_ref()).get_ui_component_mut().set_text("");
                ptr_as_mut(self._detail_desc_lbl.as_ref()).get_ui_component_mut().set_text("");
                for ing_widget in self._detail_ing_widgets.iter_mut() {
                    let layout_ui = ptr_as_mut(ing_widget._layout.as_ref()).get_ui_component_mut();
                    layout_ui.set_enable(false);
                }
                return;
            }
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
        let unlocked_set = ui_mgr.get_unlocked_toolbox_items();

        for item in self._items.iter_mut() {
            if item._recipe_index < CRAFT_RECIPES.len() {
                let recipe = &CRAFT_RECIPES[item._recipe_index];
                let is_unlocked = unlocked_set.contains(&recipe.toolbox_icon_type());

                let layout_ui = ptr_as_mut(item._layout.as_ref()).get_ui_component_mut();
                layout_ui.set_enable(is_unlocked);

                if !is_unlocked {
                    continue;
                }

                let recipe_item_name = Self::get_item_name_from_resource(recipe.item_code());
                let name_ui = ptr_as_mut(item._name_lbl.as_ref()).get_ui_component_mut();
                name_ui.set_text(&recipe_item_name);

                let mut can_craft = true;
                for req in recipe.materials.iter() {
                    let have = ui_mgr.get_total_item_count(req.item_code());
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
        let unlocked_set = get_game_ui_manager().get_unlocked_toolbox_items();
        let container_ui = ptr_as_mut(self._list_container.as_ref()).get_ui_component_mut();
        for (idx, item) in self._items.iter_mut().enumerate() {
            let is_unlocked = unlocked_set.contains(&CRAFT_RECIPES[item._recipe_index].toolbox_icon_type());
            if !is_unlocked {
                continue;
            }
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
            let selected_idx = if dir_y < 0 {
                self.get_prev_unlocked_index(self._selected_index)
            } else if dir_y > 0 {
                self.get_next_unlocked_index(self._selected_index)
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
