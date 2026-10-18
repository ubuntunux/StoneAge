use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::ITEM_NONE;
use crate::game_module::widgets::item_bar::{ITEM_UI_SIZE, ITEM_WIDGET_UI_MARGIN};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::ffi::c_void;
use std::rc::Rc;

pub const DEFAULT_SLOT_FONT_SIZE: f32 = 24.0;
pub const DEFAULT_SLOT_BORDER_WIDTH: f32 = 2.0;
pub const DEFAULT_SLOT_ROUND: f32 = 5.0;

pub struct ItemSlotWidgetBase<'a> {
    pub _slot_index: usize,
    pub _widget: Rc<WidgetDefault<'a>>,
    pub _item_data_name: String,
    pub _item_name: String,
    pub _item_data_type: ItemDataType,
    pub _item_count: usize,
}

impl<'a> ItemSlotWidgetBase<'a> {
    pub fn create(
        widget_name: &str,
        parent_widget: &mut WidgetDefault<'a>,
        slot_index: usize,
        border_color: u32,
        callback_down: Option<Box<CallbackTouchEvent<'a>>>,
        callback_over: Option<Box<CallbackTouchEvent<'a>>>,
        callback_out: Option<Box<CallbackTouchEvent<'a>>>,
        user_data: *const c_void,
    ) -> Self {
        let slot_widget = UIManager::create_widget(widget_name, UIWidgetTypes::Default);
        let slot_widget_mut = ptr_as_mut(slot_widget.as_ref());
        let ui_component = slot_widget_mut.get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_margin(ITEM_WIDGET_UI_MARGIN);
        ui_component.set_round(DEFAULT_SLOT_ROUND);
        ui_component.set_border(DEFAULT_SLOT_BORDER_WIDTH);
        ui_component.set_border_color(border_color);
        ui_component.set_font_size(DEFAULT_SLOT_FONT_SIZE);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_touchable(true);

        if callback_down.is_some() {
            ui_component.set_callback_touch_down(callback_down);
        }
        if callback_over.is_some() {
            ui_component.set_callback_touch_over(callback_over);
        }
        if callback_out.is_some() {
            ui_component.set_callback_touch_out(callback_out);
        }
        if !user_data.is_null() {
            ui_component.set_user_data(user_data);
        }

        parent_widget.add_widget(&slot_widget);

        ItemSlotWidgetBase {
            _slot_index: slot_index,
            _widget: slot_widget,
            _item_data_name: String::new(),
            _item_name: String::new(),
            _item_data_type: ItemDataType::None,
            _item_count: 0,
        }
    }

    pub fn set_data(
        &mut self,
        item_name: &str,
        item_data_name: &str,
        item_data_type: ItemDataType,
        material_instance: Option<RcRefCell<MaterialInstanceData<'a>>>,
        item_count: usize,
        bg_color: u32,
        border_color: u32,
    ) {
        self._item_name = item_name.to_string();
        self._item_data_name = item_data_name.to_string();
        self._item_data_type = item_data_type;
        self._item_count = item_count;

        let ui_component = ptr_as_mut(self._widget.as_ref()).get_ui_component_mut();
        ui_component.set_material_instance(material_instance);
        ui_component.set_color(bg_color);
        ui_component.set_border_color(border_color);

        if item_count > 1 && item_data_name != ITEM_NONE {
            ui_component.set_font_size(DEFAULT_SLOT_FONT_SIZE);
            ui_component.set_font_color(get_color32(255, 255, 255, 255));
            ui_component.set_text(&format!("{}", item_count));
        } else {
            ui_component.set_text("");
        }
    }
}
