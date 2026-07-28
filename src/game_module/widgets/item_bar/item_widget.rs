use crate::game_module::actors::items::ItemDataType;
use crate::game_module::game_constants::ITEM_NONE;
use crate::game_module::widgets::item_bar::{ITEM_UI_SIZE, ITEM_WIDGET_UI_MARGIN, ItemWidget};
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

impl<'a> ItemWidget<'a> {
    pub fn create_item_widget(parent_widget: &mut WidgetDefault<'a>, item_index: usize) -> ItemWidget<'a> {
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
