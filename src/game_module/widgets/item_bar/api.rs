use crate::game_module::actors::items::{ItemDataType, ItemManager};
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::widgets::key_binding_widget::KeyBindingWidgetMap;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::WidgetDefault;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

pub type InventoryItemCreateInfoList = HashMap<usize, Vec<InventoryItemCreateInfo>>;

pub const ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM: f32 = 50.0;
pub const MAX_ITEM_COUNT: usize = 10;
pub const ITEM_UI_SIZE: f32 = 64.0;
pub const ITEM_WIDGET_UI_MARGIN: f32 = 5.0;
pub const INVALID_ITEM_INDEX: usize = usize::MAX;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct InventoryItemCreateInfo {
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_index: usize,
    pub _item_count: usize,
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
    pub _window_size: Vector2<i32>,
}
