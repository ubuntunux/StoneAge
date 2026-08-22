use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::{AUDIO_PICKUP_ITEM, ITEM_NONE};
use crate::game_module::game_controller::{HoldRepeatController, NAV_INITIAL_DELAY, NAV_REPEAT_INTERVAL};
use crate::game_module::game_service_locator::{get_game_ui_manager, get_game_ui_manager_mut, get_item_manager_mut};
use crate::game_module::widgets::item_bar::{
    INVALID_ITEM_INDEX, ITEM_UI_SIZE, ITEM_WIDGET_UI_MARGIN, MAX_INVENTORY_ROWS, SLOTS_PER_ROW, TOTAL_INVENTORY_SLOTS,
};
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::core::input::{ButtonState, JoystickInputData, KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UIComponentInstance, UILayoutType, UIManager, UIWidgetTypes,
    VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;
use winit::keyboard::KeyCode;

pub struct InventorySlotWidget<'a> {
    pub _inventory_widget: *const InventoryWidget<'a>,
    pub _slot_index: usize,
    pub _widget: Rc<WidgetDefault<'a>>,
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_count: usize,
}

impl<'a> InventorySlotWidget<'a> {
    pub fn create(
        inventory_widget: &InventoryWidget<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        slot_index: usize,
    ) -> Box<InventorySlotWidget<'a>> {
        let slot_widget = UIManager::create_widget(&format!("inv_slot_{}", slot_index), UIWidgetTypes::Default);
        let slot_widget_mut = ptr_as_mut(slot_widget.as_ref());
        let ui_component = slot_widget_mut.get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_margin(ITEM_WIDGET_UI_MARGIN);
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_border_color(get_color32(100, 100, 120, 255));
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_touchable(true);

        parent_widget.add_widget(&slot_widget);

        let inv_slot = Box::new(InventorySlotWidget {
            _inventory_widget: inventory_widget,
            _slot_index: slot_index,
            _widget: slot_widget,
            _item_data_name: String::new(),
            _item_name: String::new(),
            _item_data_type: ItemDataType::None,
            _item_count: 0,
        });

        let ui_component = ptr_as_mut(inv_slot._widget.as_ref()).get_ui_component_mut();
        ui_component.set_callback_touch_down(Some(Box::new(InventoryWidget::callback_slot_click)));
        ui_component.set_user_data(inv_slot.as_ref() as *const InventorySlotWidget<'a> as *const c_void);

        inv_slot
    }

    pub fn set_data(
        &mut self,
        item_name: &str,
        item_data_name: &str,
        item_data_type: ItemDataType,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        item_count: usize,
        is_active_quick_row: bool,
        is_selected_item: bool,
    ) {
        self._item_name = item_name.to_string();
        self._item_data_name = item_data_name.to_string();
        self._item_data_type = item_data_type;
        self._item_count = item_count;

        let ui_component = ptr_as_mut(self._widget.as_ref()).get_ui_component_mut();
        if material_instance.is_some() {
            ui_component.set_color(get_color32(255, 255, 255, 255));
        } else {
            ui_component.set_color(get_color32(255, 255, 255, 0));
        }

        if is_selected_item {
            ui_component.set_border_color(get_color32(255, 255, 0, 255));
        } else if is_active_quick_row {
            ui_component.set_border_color(get_color32(50, 220, 100, 255));
        } else {
            ui_component.set_border_color(get_color32(80, 80, 100, 255));
        }

        if item_count > 0 && item_data_name != ITEM_NONE {
            ui_component.set_text(&format!("{}", item_count));
        } else {
            ui_component.set_text("");
        }

        ui_component.set_material_instance(material_instance);
    }
}

pub struct InventoryWidget<'a> {
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: Rc<WidgetDefault<'a>>,
    pub _inventory_bg: Rc<WidgetDefault<'a>>,
    pub _drag_widget: Rc<WidgetDefault<'a>>,
    pub _slot_widgets: Vec<Box<InventorySlotWidget<'a>>>,
    pub _focused_slot_index: usize,
    pub _drag_source_slot_index: usize,
    pub _is_opened_inventory: bool,
    pub _nav_repeat_controller: HoldRepeatController<(i32, i32)>,
}

impl<'a> InventoryWidget<'a> {
    pub fn create_inventory_widget(parent_widget: &mut WidgetDefault<'a>) -> Box<InventoryWidget<'a>> {
        let layer = UIManager::create_widget("inventory_widget_root", UIWidgetTypes::Default);
        let layer_mut = ptr_as_mut(layer.as_ref());
        let ui_component = layer_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::FloatLayout);
        ui_component.set_expandable(true);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);

        let inventory_bg = UIManager::create_widget("inventory_bg", UIWidgetTypes::Default);
        let inventory_bg_mut = ptr_as_mut(inventory_bg.as_ref());
        let ui_component = inventory_bg_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_pivot_preset(PIVOT_CENTER);
        ui_component.set_pos_hint(Some(0.5), Some(0.5));
        ui_component.set_expandable(true);
        ui_component.set_padding(10.0);
        ui_component.set_color(get_color32(220, 200, 160, 200));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        layer_mut.add_widget(&inventory_bg);

        let drag_widget = UIManager::create_widget("inv_drag_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(drag_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_font_size(24.0);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_draggable(false);
        ui_component.set_touchable(false);
        ui_component.set_visible(false);
        layer_mut.add_widget(&drag_widget);

        let mut inventory_widget = Box::new(InventoryWidget {
            _parent_widget: parent_widget,
            _layer: layer,
            _inventory_bg: inventory_bg,
            _drag_widget: drag_widget,
            _slot_widgets: Vec::new(),
            _focused_slot_index: 0,
            _drag_source_slot_index: INVALID_ITEM_INDEX,
            _is_opened_inventory: false,
            _nav_repeat_controller: HoldRepeatController::new(NAV_INITIAL_DELAY, NAV_REPEAT_INTERVAL),
        });

        // Grid Layout: 2 Rows x 10 Slots
        for row in 0..MAX_INVENTORY_ROWS {
            let row_layout =
                InventoryWidget::create_inventory_row(ptr_as_mut(inventory_widget._inventory_bg.as_ref()), row);
            for column in 0..SLOTS_PER_ROW {
                let slot_idx = row * SLOTS_PER_ROW + column;
                let slot_item =
                    InventorySlotWidget::create(inventory_widget.as_ref(), ptr_as_mut(row_layout.as_ref()), slot_idx);
                inventory_widget._slot_widgets.push(slot_item);
            }
        }

        inventory_widget
    }

    pub fn create_inventory_row(inventory_widget: &mut WidgetDefault<'a>, row: usize) -> Rc<WidgetDefault<'a>> {
        let row_layout = UIManager::create_widget(&format!("inventory_row_{}", row), UIWidgetTypes::Default);
        let row_layout_mut = ptr_as_mut(row_layout.as_ref());
        let ui_component = row_layout_mut.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(160, 140, 100, 200));
        ui_component.set_round(5.0);
        ui_component.set_margin(10.0);
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_y(0.0);
        ui_component.set_expandable(true);
        inventory_widget.add_widget(&row_layout);
        row_layout
    }

    pub fn callback_close_click(
        _ui_component: &UIComponentInstance<'a>,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        get_game_ui_manager_mut().close_inventory();
        true
    }

    pub fn callback_slot_click(
        ui_component: &UIComponentInstance<'a>,
        touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        let slot_item = ptr_as_ref(ui_component.get_user_data() as *const InventorySlotWidget<'a>);
        let inventory_widget = ptr_as_mut(slot_item._inventory_widget);
        let clicked_slot = slot_item._slot_index;

        if inventory_widget._drag_source_slot_index == INVALID_ITEM_INDEX {
            // First click: Pick up / detach item for dragging if slot is not empty
            if slot_item._item_count > 0 && slot_item._item_data_name != ITEM_NONE {
                inventory_widget._drag_source_slot_index = clicked_slot;
                inventory_widget._focused_slot_index = clicked_slot;

                let item_bar = get_game_ui_manager().get_item_bar_widget();
                let slot_data = item_bar.get_inventory_slot_data(clicked_slot);

                let drag_ui = ptr_as_mut(inventory_widget._drag_widget.as_ref()).get_ui_component_mut();
                drag_ui.set_material_instance(slot_data._material_instance.clone());
                drag_ui.set_text(&format!("{}", slot_data._item_count));
                drag_ui.set_draggable(true);
                drag_ui.set_visible(true);

                let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
                let parent_area = ptr_as_ref(inventory_widget._layer.as_ref()).get_ui_component().get_ui_area();
                drag_ui.set_pos(
                    (touched_pos.x - parent_area.x) / dpi_scale - ITEM_UI_SIZE * 0.5,
                    (touched_pos.y - parent_area.y) / dpi_scale - ITEM_UI_SIZE * 0.5,
                );

                inventory_widget._focused_slot_index = clicked_slot;
            } else {
                inventory_widget._focused_slot_index = clicked_slot;
            }
        } else {
            // Second click: Attach / swap dragged item to target slot
            let src_slot = inventory_widget._drag_source_slot_index;
            if src_slot != clicked_slot {
                get_game_ui_manager_mut().swap_inventory_slots(src_slot, clicked_slot);
            }
            inventory_widget._drag_source_slot_index = INVALID_ITEM_INDEX;
            inventory_widget._focused_slot_index = clicked_slot;

            let drag_ui = ptr_as_mut(inventory_widget._drag_widget.as_ref()).get_ui_component_mut();
            drag_ui.set_draggable(false);
            drag_ui.set_visible(false);
        }

        inventory_widget.refresh_inventory_widget();
        true
    }

    pub fn open_inventory(&mut self) {
        if !self._is_opened_inventory {
            self._is_opened_inventory = true;
            let selected_slot = get_game_ui_manager().get_selected_inventory_item_index();
            if selected_slot != INVALID_ITEM_INDEX {
                self._focused_slot_index = selected_slot;
            } else {
                self._focused_slot_index = 0;
            }
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.add_widget(&self._layer);
            self.refresh_inventory_widget();
        }
    }

    pub fn close_inventory(&mut self) {
        if self._is_opened_inventory {
            self._is_opened_inventory = false;
            self._nav_repeat_controller.reset();
            if self._drag_source_slot_index != INVALID_ITEM_INDEX {
                self._drag_source_slot_index = INVALID_ITEM_INDEX;
                let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
                drag_ui.set_draggable(false);
                drag_ui.set_visible(false);
            }
            let parent_mut = ptr_as_mut(self._parent_widget);
            parent_mut.remove_widget(self._layer.as_ref());
        }
    }

    pub fn is_opened_inventory(&self) -> bool {
        self._is_opened_inventory
    }

    pub fn refresh_inventory_widget(&mut self) {
        let item_bar = get_game_ui_manager().get_item_bar_widget();
        let active_row = item_bar.get_active_row_index();

        for (slot_idx, slot_widget) in self._slot_widgets.iter_mut().enumerate() {
            let slot_row = slot_idx / SLOTS_PER_ROW;
            let slot_data = item_bar.get_inventory_slot_data(slot_idx);
            let is_active_quick_row = slot_row == active_row;
            let is_selected_item = slot_idx == self._focused_slot_index;

            if slot_idx == self._drag_source_slot_index {
                slot_widget.set_data(
                    "",
                    ITEM_NONE,
                    ItemDataType::None,
                    None,
                    0,
                    is_active_quick_row,
                    is_selected_item,
                );
            } else {
                slot_widget.set_data(
                    &slot_data._item_name,
                    &slot_data._item_data_name,
                    slot_data._item_data_type,
                    slot_data._material_instance.clone(),
                    slot_data._item_count,
                    is_active_quick_row,
                    is_selected_item,
                );
            }
        }
    }

    pub fn update_inventory_widget(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _player: &RcRefCell<Character>,
    ) {
        if self._drag_source_slot_index != INVALID_ITEM_INDEX {
            let engine_core = rust_engine_3d::core::engine_service_locator::get_engine_core();
            let mouse_pos = &engine_core._mouse_move_data._mouse_pos;
            let dpi_scale = rust_engine_3d::scene::ui::get_global_dpi_scale();
            let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
            let parent_area = ptr_as_ref(self._layer.as_ref()).get_ui_component().get_ui_area();
            drag_ui.set_pos(
                (mouse_pos.x as f32 - parent_area.x) / dpi_scale - ITEM_UI_SIZE * 0.5,
                (mouse_pos.y as f32 - parent_area.y) / dpi_scale - ITEM_UI_SIZE * 0.5,
            );
        }

        let delta_time: f32 = _time_data._delta_time_with_scale as f32;

        // WASD & Arrow Key Navigation (Supports Hold Repeat)
        let is_left = keyboard_input_data.get_key_pressed(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowLeft)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyA)
            || keyboard_input_data.get_key_hold(KeyCode::KeyA)
            || joystick_input_data._btn_left == ButtonState::Pressed
            || joystick_input_data._btn_left == ButtonState::Hold;
        let is_right = keyboard_input_data.get_key_pressed(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowRight)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyD)
            || keyboard_input_data.get_key_hold(KeyCode::KeyD)
            || joystick_input_data._btn_right == ButtonState::Pressed
            || joystick_input_data._btn_right == ButtonState::Hold;
        let is_up = keyboard_input_data.get_key_pressed(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowUp)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyW)
            || keyboard_input_data.get_key_hold(KeyCode::KeyW)
            || joystick_input_data._btn_up == ButtonState::Pressed
            || joystick_input_data._btn_up == ButtonState::Hold;
        let is_down = keyboard_input_data.get_key_pressed(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_hold(KeyCode::ArrowDown)
            || keyboard_input_data.get_key_pressed(KeyCode::KeyS)
            || keyboard_input_data.get_key_hold(KeyCode::KeyS)
            || joystick_input_data._btn_down == ButtonState::Pressed
            || joystick_input_data._btn_down == ButtonState::Hold;

        let current_dir: Option<(i32, i32)> = if is_left {
            Some((-1, 0))
        } else if is_right {
            Some((1, 0))
        } else if is_up {
            Some((0, -1))
        } else if is_down {
            Some((0, 1))
        } else {
            None
        };

        let (should_move_slot, dir_opt) = self._nav_repeat_controller.update(current_dir, delta_time);

        if should_move_slot {
            let (dir_x, dir_y) = dir_opt.unwrap();
            let mut r = self._focused_slot_index / SLOTS_PER_ROW;
            let mut c = self._focused_slot_index % SLOTS_PER_ROW;

            if dir_x < 0 {
                c = (c + SLOTS_PER_ROW - 1) % SLOTS_PER_ROW;
            } else if dir_x > 0 {
                c = (c + 1) % SLOTS_PER_ROW;
            }

            if dir_y < 0 {
                r = (r + MAX_INVENTORY_ROWS - 1) % MAX_INVENTORY_ROWS;
            } else if dir_y > 0 {
                r = (r + 1) % MAX_INVENTORY_ROWS;
            }

            let new_focused_slot = r * SLOTS_PER_ROW + c;
            if self._drag_source_slot_index != INVALID_ITEM_INDEX {
                // If keyboard navigating while dragging an item, attach/swap to the new slot
                let src_slot = self._drag_source_slot_index;
                if src_slot != new_focused_slot {
                    get_game_ui_manager_mut().swap_inventory_slots(src_slot, new_focused_slot);
                }
                self._drag_source_slot_index = INVALID_ITEM_INDEX;
                let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
                drag_ui.set_draggable(false);
                drag_ui.set_visible(false);
            }

            self._focused_slot_index = new_focused_slot;
        }

        // Mouse Right Click OR Keyboard/Joystick Drop Input
        let is_mouse_drop = _mouse_input_data._btn_r_pressed;
        let is_controller_drop = joystick_input_data._btn_b == ButtonState::Pressed
            || keyboard_input_data.get_key_pressed(KeyCode::KeyF)
            || keyboard_input_data.get_key_pressed(KeyCode::Delete);

        if is_mouse_drop {
            let engine_core = rust_engine_3d::core::engine_service_locator::get_engine_core();
            let mouse_pos = &engine_core._mouse_move_data._mouse_pos;
            let mouse_pos_f32 = Vector2::new(mouse_pos.x as f32, mouse_pos.y as f32);

            let mut hovered_slot = INVALID_ITEM_INDEX;
            for slot_widget in self._slot_widgets.iter() {
                let ui_comp = slot_widget._widget.get_ui_component();
                if ui_comp.check_collide(&mouse_pos_f32) {
                    hovered_slot = slot_widget._slot_index;
                    break;
                }
            }

            // Only proceed if mouse pointer is directly over a valid slot
            if hovered_slot != INVALID_ITEM_INDEX && hovered_slot < TOTAL_INVENTORY_SLOTS {
                // Focus the slot under mouse pointer
                self._focused_slot_index = hovered_slot;

                // Drop item if slot is not empty and is droppable
                let game_ui_manager = get_game_ui_manager();
                let item_bar = game_ui_manager.get_item_bar_widget();
                let slot_data = item_bar.get_inventory_slot_data(hovered_slot);
                if slot_data._item_count > 0
                    && slot_data._item_data_name != ITEM_NONE
                    && slot_data._item_data_type.is_droppable()
                {
                    let item_data_name = slot_data._item_data_name.clone();
                    get_item_manager_mut().drop_inventory_item(&item_data_name, 1);

                    if self._drag_source_slot_index == hovered_slot {
                        self._drag_source_slot_index = INVALID_ITEM_INDEX;
                        let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
                        drag_ui.set_draggable(false);
                        drag_ui.set_visible(false);
                    }
                }
            }
        } else if is_controller_drop {
            let target_slot = self._focused_slot_index;
            if target_slot != INVALID_ITEM_INDEX && target_slot < TOTAL_INVENTORY_SLOTS {
                let game_ui_manager = get_game_ui_manager();
                let item_bar = game_ui_manager.get_item_bar_widget();
                let slot_data = item_bar.get_inventory_slot_data(target_slot);
                if slot_data._item_count > 0
                    && slot_data._item_data_name != ITEM_NONE
                    && slot_data._item_data_type.is_droppable()
                {
                    let item_data_name = slot_data._item_data_name.clone();
                    get_item_manager_mut().drop_inventory_item(&item_data_name, 1);

                    if self._drag_source_slot_index == target_slot {
                        self._drag_source_slot_index = INVALID_ITEM_INDEX;
                        let drag_ui = ptr_as_mut(self._drag_widget.as_ref()).get_ui_component_mut();
                        drag_ui.set_draggable(false);
                        drag_ui.set_visible(false);
                    }
                }
            }
        }

        self.refresh_inventory_widget();
    }

    pub fn changed_window_size(&mut self, _window_size: &Vector2<i32>) {
        self.refresh_inventory_widget();
    }
}
