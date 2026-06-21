use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, PosHintX, PosHintY, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::actors::items::ItemManager;
use crate::game_module::game_constants::ITEM_NONE;
use crate::game_module::game_controller::KeyBindingType;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::widgets::key_binding_widget::{KeyBindingWidget, KeyBindingWidgetManager, KeyBindingWidgetMap, KEY_BINDING_FONT_SIZE, KEY_BINDING_ICON_MARGIN, KEY_BINDING_TEXT_MARGIN, KEY_BINDING_UI_SIZE};

pub const ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM: f32 = 50.0;
pub const MAX_ITEM_COUNT: usize = 10;
pub const ITEM_UI_SIZE: f32 = 64.0;
pub const ITEM_WIDGET_UI_MARGIN: f32 = 5.0;
const INVALID_ITEM_INDEX: usize = usize::MAX;

fn create_inventory_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_text: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>
) -> KeyBindingWidget<'a> {
    let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
    let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_expandable_x(true);
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_round(10.0);
    ui_component.set_color(get_color32(0, 0, 0, 128));
    parent_widget.add_widget(&layout_widget);

    // icons
    let mut binding_icon_widgets: Vec<*const WidgetDefault> = Vec::new();
    let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
    ui_component.set_halign(HorizontalAlign::RIGHT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_margin_right(KEY_BINDING_ICON_MARGIN);
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    layout_widget_mut.add_widget(&binding_icon_widget);
    binding_icon_widgets.push(binding_icon_widget.as_ref());

    // text widget
    let binding_name_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_name_widget.as_ref()).get_ui_component_mut();
    ui_component.set_expandable_x(true);
    ui_component.set_size_x(0.0);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_margin_left(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_margin_right(KEY_BINDING_TEXT_MARGIN);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_font_size(KEY_BINDING_FONT_SIZE);
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_color(get_color32(255, 255, 255, 0));
    ui_component.set_text(key_binding_text);
    layout_widget_mut.add_widget(&binding_name_widget);

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: binding_name_widget.as_ref(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons
    }
}

fn create_quick_slot_key_binding_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    key_binding_type: KeyBindingType,
    widget_name: &str,
    key_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>,
    joystick_binding_icons: Vec<RcRefCell<MaterialInstanceData<'a>>>
) -> KeyBindingWidget<'a> {
    let layout_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let layout_widget_mut = ptr_as_mut(layout_widget.as_ref());
    let ui_component = ptr_as_mut(layout_widget.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::HORIZONTAL);
    ui_component.set_pos_hint_x(PosHintX::Center(0.5));
    ui_component.set_pos_hint_y(PosHintY::Top(1.0));
    ui_component.set_size_x(KEY_BINDING_UI_SIZE);
    ui_component.set_size_y(KEY_BINDING_UI_SIZE);
    ui_component.set_round(10.0);
    ui_component.set_color(get_color32(0, 0, 0, 0));
    parent_widget.add_widget(&layout_widget);

    // icons
    let mut binding_icon_widgets: Vec<*const WidgetDefault<'a>> = Vec::new();
    let binding_icon_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(binding_icon_widget.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    layout_widget_mut.add_widget(&binding_icon_widget);
    binding_icon_widgets.push(binding_icon_widget.as_ref());

    KeyBindingWidget {
        _key_binding_type: key_binding_type,
        _layout_widget: layout_widget.as_ref(),
        _binding_name_widget: std::ptr::null(),
        _binding_icon_widgets: binding_icon_widgets,
        _key_binding_icons: key_binding_icons,
        _joystick_binding_icons: joystick_binding_icons
    }
}

pub struct ItemWidget<'a> {
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_index: usize,
    pub _item_count: usize,
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemSelectionWidget<'a> {
    pub _item_index: usize,
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemBarWidget<'a> {
    pub _game_resources: *const GameResources<'a>,
    pub _engine_resources: *const EngineResources<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _item_manager: *const ItemManager<'a>,
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: *const WidgetDefault<'a>,
    pub _item_widgets: Vec<ItemWidget<'a>>,
    pub _selected_item_widget: ItemSelectionWidget<'a>,
    pub _selected_item_index: usize,
    pub _item_count: usize,
    pub _max_item_count: usize,
    pub _inventory_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _quick_slot_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _window_size: Vector2<i32>
}

impl<'a> ItemWidget<'a> {
    pub fn create_item_widget(
        parent_widget: &mut WidgetDefault<'a>,
        item_index: usize,
    ) -> ItemWidget<'a> {
        let item_widget = UIManager::create_widget("item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_pos_x(ITEM_UI_SIZE * item_index as f32);
        ui_component.set_round(2.0);
        ui_component.set_margin(ITEM_WIDGET_UI_MARGIN);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(30.0);
        ui_component.set_visible(false);
        parent_widget.add_widget(&item_widget);

        ItemWidget {
            _item_data_name: String::new(),
            _item_name: String::new(),
            _item_data_type: ItemDataType::None,
            _item_index: item_index,
            _item_count: 0,
            _widget: item_widget.as_ref(),
        }
    }

    pub fn set_item_data(
        &mut self,
        item_name: &str,
        item_data_name: &str,
        item_data_type: ItemDataType,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        item_count: usize,
    ) {
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_material_instance(material_instance);
        self._item_data_name = String::from(item_data_name);
        self._item_name = String::from(item_name);
        self._item_data_type = item_data_type;
        self.set_item_count(item_count);
    }

    pub fn get_item_count(&self) -> usize {
        self._item_count
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        self._item_count = item_count;
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_text(format!("{}", self._item_count).as_str());
        if 0 < self._item_count {
            ui_component.set_visible(true);
        } else {
            self._item_data_name = String::from(ITEM_NONE);
            self._item_name = String::from(ITEM_NONE);
            ui_component.set_visible(false);
        }
    }

    pub fn add_item_count(&mut self, item_count: usize) -> usize {
        self.set_item_count(self._item_count + item_count);
        self._item_count
    }

    pub fn remove_item_count(&mut self, item_count: usize) -> usize {
        if item_count <= self._item_count {
            self.set_item_count(self._item_count - item_count);
        }
        self._item_count
    }
}

impl<'a> ItemSelectionWidget<'a> {
    pub fn get_item_index(&self) -> usize {
        self._item_index
    }

    pub fn update_selected_item_widget(&mut self, item_index: usize, item_widget: Option<&ItemWidget<'a>>) {
        self._item_index = item_index;
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        if let Some(item_widget) = item_widget {
            let item_ui_component = ptr_as_ref(item_widget._widget).get_ui_component();
            let item_ui_area = item_ui_component.get_ui_area();
            ui_component.set_center((item_ui_area.x + item_ui_area.z) * 0.5, (item_ui_area.y + item_ui_area.w) * 0.5);
            ui_component.set_visible(true);
        } else {
            ui_component.set_visible(false);
        }
    }
}

impl<'a> ItemBarWidget<'a> {
    pub fn create_item_bar_widget(
        game_resources: *const GameResources<'a>,
        engine_resources: *const EngineResources<'a>,
        game_scene_manager: *const GameSceneManager<'a>,
        item_manager: *const ItemManager<'a>,
        key_binding_widget_manager: *const KeyBindingWidgetManager<'a>,
        parent_widget: &mut WidgetDefault<'a>,
        window_size: &Vector2<i32>,
    ) -> ItemBarWidget<'a> {
        let layer = UIManager::create_widget("item_bar_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_expandable(true);
        ui_component.set_pos_hint_x(PosHintX::Center(0.5));
        ui_component.set_pos_hint_y(PosHintY::Bottom(1.0));
        ui_component.set_margin_bottom(ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM);
        parent_widget.add_widget(&layer);

        let selected_item_widget = UIManager::create_widget("selected_item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(selected_item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_color(get_color32(255, 255, 255, 0));
        ui_component.set_border_color(get_color32(255, 255, 255, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_visible(false);
        parent_widget.add_widget(&selected_item_widget);

        let mut item_bar_widget = ItemBarWidget {
            _game_resources: game_resources,
            _engine_resources: engine_resources,
            _game_scene_manager: game_scene_manager,
            _item_manager: item_manager,
            _parent_widget: parent_widget,
            _layer: layer.as_ref(),
            _item_widgets: Vec::new(),
            _selected_item_widget: ItemSelectionWidget {
                _item_index: INVALID_ITEM_INDEX,
                _widget: selected_item_widget.as_ref(),
            },
            _selected_item_index: usize::MAX,
            _item_count: 0,
            _max_item_count: MAX_ITEM_COUNT,
            _inventory_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _quick_slot_key_binding_widget_map: Rc::new(KeyBindingWidgetMap::default()),
            _window_size: window_size.clone()
        };

        for item_index in 0..MAX_ITEM_COUNT {
            let item_widget = ItemWidget::create_item_widget(ptr_as_mut(layer.as_ref()), item_index);
            item_bar_widget._item_widgets.push(item_widget);
        }

        item_bar_widget.register_item_bar_key_binding_widgets(key_binding_widget_manager);
        item_bar_widget.update_item_bar_widget_layout();
        item_bar_widget
    }

    pub fn get_inventory_key_binding_widget_map_mut(&mut self) -> &mut KeyBindingWidgetMap<'a> {
        ptr_as_mut(self._inventory_key_binding_widget_map.as_ref())
    }

    pub fn get_quick_slot_key_binding_widget_map_mut(&mut self) -> &mut KeyBindingWidgetMap<'a> {
        ptr_as_mut(self._quick_slot_key_binding_widget_map.as_ref())
    }

    pub fn register_item_bar_key_binding_widgets(&mut self, key_binding_widget_manager: *const KeyBindingWidgetManager<'a>) {
        let engine_resources = ptr_as_ref(self._engine_resources);

        let key_binding_widget_manager = ptr_as_mut(key_binding_widget_manager);
        key_binding_widget_manager.register_key_binding_widget_map(&self._inventory_key_binding_widget_map);
        key_binding_widget_manager.register_key_binding_widget_map(&self._quick_slot_key_binding_widget_map);

        // inventory
        let inventory_key_binding_widget_map = ptr_as_mut(self._inventory_key_binding_widget_map.as_ref());
        inventory_key_binding_widget_map.register_key_binding_widget(create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::SelectPrevItem,
            "select_prev_item_key_binding",
            "Previous Item",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_q").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_left").clone()],
        ));
        inventory_key_binding_widget_map.register_key_binding_widget(create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::SelectNextItem,
            "select_next_item_key_binding",
            "Next Item",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_e").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_right").clone()],
        ));
        inventory_key_binding_widget_map.register_key_binding_widget(create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::DropItem,
            "drop_item_key_binding",
            "Drop Item",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_f").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_x").clone()],
        ));
        inventory_key_binding_widget_map.register_key_binding_widget(create_inventory_key_binding_widget(
            ptr_as_mut(self._parent_widget),
            KeyBindingType::UseItem,
            "use_item_key_binding",
            "Use Item",
            vec![engine_resources.get_material_instance_data("ui/controller/keycode_c").clone()],
            vec![engine_resources.get_material_instance_data("ui/controller/joystick_y").clone()],
        ));

        // quick slot
        let quick_slot_key_binding_widget_map = ptr_as_mut(self._quick_slot_key_binding_widget_map.as_ref());
        let key_binding_types = [
            KeyBindingType::SelectItem01,
            KeyBindingType::SelectItem02,
            KeyBindingType::SelectItem03,
            KeyBindingType::SelectItem04,
            KeyBindingType::SelectItem05,
            KeyBindingType::SelectItem06,
            KeyBindingType::SelectItem07,
            KeyBindingType::SelectItem08,
            KeyBindingType::SelectItem09,
            KeyBindingType::SelectItem10,
        ];
        let key_codes = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        for i in 0..MAX_ITEM_COUNT {
            quick_slot_key_binding_widget_map.register_key_binding_widget(create_quick_slot_key_binding_widget(
                ptr_as_mut(self.get_item_widget(i)._widget),
                key_binding_types[i],
                &format!("select_item{:02}_key_binding", i + 1),
                vec![engine_resources.get_material_instance_data(&format!("ui/controller/keycode_{}", key_codes[i])).clone()],
                vec![],
            ));
        }
    }

    pub fn get_item_bar_width() -> f32 {
        (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 2.0) * MAX_ITEM_COUNT as f32
    }

    pub fn get_item_bar_pos_top() -> f32 {
        ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM + ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 2.0
    }

    pub fn get_item_bar_center_y() -> f32 {
        ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM + (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN) * 0.5
    }

    pub fn get_selected_item_pos_left(item_index: usize) -> f32 {
        (item_index as f32 - MAX_ITEM_COUNT as f32 / 2.0) * (ITEM_UI_SIZE + ITEM_WIDGET_UI_MARGIN * 2.0)
    }

    pub fn get_selected_item_widget(&self) -> &ItemSelectionWidget<'a> {
        &self._selected_item_widget
    }

    pub fn update_item_bar_widget_layout(&mut self) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_size_y(ITEM_UI_SIZE);
    }

    pub fn get_item_widget(&self, index: usize) -> &ItemWidget<'a> {
        &self._item_widgets[index]
    }

    pub fn get_item_widget_mut(&mut self, index: usize) -> &mut ItemWidget<'a> {
        &mut self._item_widgets[index]
    }

    pub fn find_item_widget(&self, item_data_name: &str) -> Option<&ItemWidget<'a>> {
        self._item_widgets.iter().find(|item_widget| item_widget._item_data_name == item_data_name)
    }

    pub fn find_item_widget_mut(
        &mut self,
        item_data_name: &str,
    ) -> Option<&mut ItemWidget<'a>> {
        self._item_widgets.iter_mut().find(|item_widget| item_widget._item_data_name.as_str() == item_data_name)
    }

    pub fn get_item_count(&self, item_data_name: &str) -> usize {
        if let Some(item_widget) = self.find_item_widget(item_data_name) {
            return item_widget.get_item_count();
        }
        0
    }

    pub fn get_selected_item_data_name(&self) -> &str {
        if self.get_selected_item_index() != INVALID_ITEM_INDEX {
            return self._item_widgets[self.get_selected_item_index()]._item_data_name.as_str();
        }
        ITEM_NONE
    }

    pub fn get_selected_item_name(&self) -> &str {
        if self.get_selected_item_index() != INVALID_ITEM_INDEX {
            return self._item_widgets[self.get_selected_item_index()]._item_name.as_str();
        }
        ITEM_NONE
    }

    pub fn get_selected_item_data_type(&self) -> ItemDataType {
        if self.get_selected_item_index() != INVALID_ITEM_INDEX {
            return self._item_widgets[self.get_selected_item_index()]._item_data_type
        }
        ItemDataType::None
    }

    pub fn get_selected_item_index(&self) -> usize {
        self._selected_item_widget.get_item_index()
    }

    pub fn add_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        if item_data_name != ITEM_NONE && self._item_count < self._max_item_count {
            let was_empty_item = self.get_selected_item_data_name() == ITEM_NONE;
            let mut item_index = 0;

            if let Some(item_widget) = self.find_item_widget_mut(item_data_name) {
                item_widget.add_item_count(item_count);
            } else {
                for item_widget in self._item_widgets.iter_mut() {
                    if item_widget._item_data_name == ITEM_NONE {
                        let item_data = ptr_as_ref(self._game_resources).get_item_data(item_data_name).borrow();
                        let material = ptr_as_ref(self._engine_resources).get_material_instance_data(item_data._ui_material_instance.as_str());
                        item_widget.set_item_data(
                            item_data._name.as_ref(),
                            item_data_name,
                            item_data._item_type,
                            Some(material.clone()),
                            item_count,
                        );
                        item_index = item_widget._item_index;
                        self._item_count += 1;
                        break;
                    }
                }
            }

            if was_empty_item {
                self.select_item(item_index);
            }

            self.update_item_bar_widget_layout();
            return true;
        }
        false
    }

    pub fn remove_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        if item_data_name == ITEM_NONE {
            return false;
        }

        if let Some(item_widget) = self.find_item_widget_mut(item_data_name) {
            let item_count = item_widget.remove_item_count(item_count);
            if item_count == 0 {
                item_widget.set_item_data(ITEM_NONE, ITEM_NONE, ItemDataType::None, None, 0);
                self._item_count -= 1;

                let player = ptr_as_mut(ptr_as_ref(self._game_scene_manager).get_character_manager().get_player().as_ptr());
                ptr_as_mut(self._item_manager).detach_item(player);
                //self.select_previous_item();
            } else {
                let item_index = self.get_selected_item_index();
                self.select_item(item_index);
            }
            return true;
        }
        false
    }

    pub fn select_item(&mut self, item_index: usize) {
        let player = ptr_as_mut(ptr_as_ref(self._game_scene_manager).get_character_manager().get_player().as_ptr());
        if item_index < self._item_widgets.len() && self._item_widgets[item_index]._item_data_name != ITEM_NONE {
            let item_widget = &self._item_widgets[item_index];
            self._selected_item_widget.update_selected_item_widget(item_index, Some(item_widget));
            ptr_as_mut(self._item_manager).attach_item(player, self.get_selected_item_data_name());
        } else {
            self._selected_item_widget.update_selected_item_widget(INVALID_ITEM_INDEX, None);
            ptr_as_mut(self._item_manager).detach_item(player);
        }
    }

    pub fn select_next_item(&mut self) {
        let mut item_index = INVALID_ITEM_INDEX;

        if 0 < self._item_count {
            item_index = if self._selected_item_widget.get_item_index() == INVALID_ITEM_INDEX || self._selected_item_widget.get_item_index() == (self._item_widgets.len() - 1) {
                0
            } else {
                self._selected_item_widget.get_item_index() + 1
            };

            for _ in 0..self._item_widgets.len() {
                if self._item_widgets[item_index]._item_data_name != ITEM_NONE {
                    break;
                }

                item_index += 1;
                if self._item_widgets.len() <= item_index {
                    item_index -= self._item_widgets.len();
                }
            }
        }

        self.select_item(item_index);
    }

    pub fn select_previous_item(&mut self) {
        let mut item_index = INVALID_ITEM_INDEX;

        if 0 < self._item_count {
            item_index = if self._selected_item_widget.get_item_index() == INVALID_ITEM_INDEX || 0 == self._selected_item_widget.get_item_index() {
                self._item_widgets.len() - 1
            } else {
                self._selected_item_widget.get_item_index() - 1
            };

            for _ in 0..self._item_widgets.len() {
                if self._item_widgets[item_index]._item_data_name != ITEM_NONE {
                    break;
                }

                if item_index == 0 {
                    item_index = self._item_widgets.len() - 1;
                } else {
                    item_index -= 1;
                }
            }
        }

        self.select_item(item_index);
    }

    pub fn update_selected_item_helper_widget(&mut self, force_update: bool) {
        let inventory_key_binding_widget_map = ptr_as_mut(self._inventory_key_binding_widget_map.as_ref());
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        let selected_item_index = game_scene_manager.get_game_ui_manager().get_selected_inventory_item_index();

        if self._selected_item_index != selected_item_index || force_update {
            let _item_name = game_scene_manager.get_game_ui_manager().get_selected_inventory_item_name();
            let pos_x = self._window_size.x as f32 * 0.5 + ItemBarWidget::get_selected_item_pos_left(selected_item_index);

            let key_binding_widget = inventory_key_binding_widget_map.get_key_binding_widget(KeyBindingType::UseItem);
            let ui_component = ptr_as_mut(key_binding_widget._layout_widget).get_ui_component_mut();
            ui_component.set_pos_x(pos_x);
            ui_component.set_pos_y(self._window_size.y as f32 - (ItemBarWidget::get_item_bar_pos_top() + ui_component.get_ui_size().y));

            let key_binding_widget = inventory_key_binding_widget_map.get_key_binding_widget(KeyBindingType::DropItem);
            let ui_component = ptr_as_mut(key_binding_widget._layout_widget).get_ui_component_mut();
            ui_component.set_pos_x(pos_x);
            ui_component.set_pos_y(self._window_size.y as f32 - (ItemBarWidget::get_item_bar_pos_top() + ui_component.get_ui_size().y * 2.0));

            self._selected_item_index = selected_item_index;
        }

        let key_binding_widget = inventory_key_binding_widget_map.get_key_binding_widget(KeyBindingType::SelectPrevItem);
        let ui_component = ptr_as_mut(key_binding_widget._layout_widget).get_ui_component_mut();
        ui_component.set_pos_x((self._window_size.x as f32 - ItemBarWidget::get_item_bar_width()) * 0.5 - ui_component.get_ui_size().x - KEY_BINDING_TEXT_MARGIN);
        ui_component.set_pos_y(self._window_size.y as f32 - (ItemBarWidget::get_item_bar_center_y() + ui_component.get_ui_size().y * 0.5));

        let key_binding_widget = inventory_key_binding_widget_map.get_key_binding_widget(KeyBindingType::SelectNextItem);
        let ui_component = ptr_as_mut(key_binding_widget._layout_widget).get_ui_component_mut();
        ui_component.set_pos_x((self._window_size.x as f32 + ItemBarWidget::get_item_bar_width()) * 0.5 + KEY_BINDING_TEXT_MARGIN);
        ui_component.set_pos_y(self._window_size.y as f32 - (ItemBarWidget::get_item_bar_center_y() + ui_component.get_ui_size().y * 0.5));
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        self._window_size = window_size.clone();
        self.update_selected_item_helper_widget(true);
    }

    pub fn update_item_bar_widget(&mut self) {
        self.update_selected_item_helper_widget(true);
    }
}