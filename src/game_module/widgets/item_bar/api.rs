use crate::game_module::actors::items::ItemDataType;
use crate::game_module::widgets::key_binding_widget::KeyBindingWidgetMap;
use nalgebra::Vector2;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::WidgetDefault;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

pub type InventoryItemCreateInfoList = HashMap<usize, Vec<InventoryItemCreateInfo>>;

pub const ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM: f32 = 50.0;
pub const DEFAULT_INVENTORY_ROWS: usize = 2;
pub const SLOTS_PER_ROW: usize = 10;
pub const NUM_EQUIPMENT_SLOTS: usize = 3;
pub const EQUIPMENT_SLOT_START_INDEX: usize = 100;
pub const MAX_ITEM_COUNT: usize = SLOTS_PER_ROW;
pub const ITEM_UI_SIZE: f32 = 64.0;
pub const ITEM_WIDGET_UI_MARGIN: f32 = 5.0;
pub const INVALID_ITEM_INDEX: usize = usize::MAX;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EquipmentSlotType {
    Hat = 0,   // Hat
    Armor = 1, // Armor
    Shoes = 2, // Shoes
}

impl EquipmentSlotType {
    pub fn from_slot_index(slot_index: usize) -> Option<Self> {
        if slot_index >= EQUIPMENT_SLOT_START_INDEX && slot_index < EQUIPMENT_SLOT_START_INDEX + NUM_EQUIPMENT_SLOTS {
            match slot_index - EQUIPMENT_SLOT_START_INDEX {
                0 => Some(EquipmentSlotType::Hat),
                1 => Some(EquipmentSlotType::Armor),
                2 => Some(EquipmentSlotType::Shoes),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn to_slot_index(&self) -> usize {
        EQUIPMENT_SLOT_START_INDEX + (*self as usize)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            EquipmentSlotType::Hat => "Hat",
            EquipmentSlotType::Armor => "Armor",
            EquipmentSlotType::Shoes => "Shoes",
        }
    }
}

#[derive(Clone, Debug)]
pub struct InventorySlotData<'a> {
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
    pub _item_count: usize,
}

impl<'a> Default for InventorySlotData<'a> {
    fn default() -> Self {
        Self {
            _item_data_name: String::new(),
            _item_name: String::new(),
            _item_data_type: ItemDataType::None,
            _material_instance: None,
            _item_count: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct InventoryItemCreateInfo {
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_index: usize,
    pub _row: usize,
    pub _column: usize,
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
    pub _parent_widget: *const WidgetDefault<'a>,
    pub _layer: *const WidgetDefault<'a>,
    pub _item_widgets: Vec<ItemWidget<'a>>,
    pub _inventory_slots: Vec<InventorySlotData<'a>>,
    pub _inventory_rows: usize,
    pub _active_row_index: usize,
    pub _selected_item_widget: ItemSelectionWidget<'a>,
    pub _selected_inventory_slot_index: usize,
    pub _item_count: usize,
    pub _max_item_count: usize,
    pub _inventory_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _quick_slot_key_binding_widget_map: Rc<KeyBindingWidgetMap<'a>>,
    pub _window_size: Vector2<i32>,
}
