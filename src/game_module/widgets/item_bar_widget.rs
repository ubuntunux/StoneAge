use std::collections::HashMap;
use nalgebra::Vector2;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::actors::items::ItemDataType;

const ITEM_UI_SIZE: f32 = 128.0;
const WIDGET_UI_MARGIN: f32 = 5.0;
const _WIDGET_UI_PADDING: f32 = 10.0;

pub struct ItemWidget<'a> {
    pub _item_name: String,
    pub _item_count: i32,
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemBarWidget<'a> {
    pub _engine_resources: *const EngineResources<'a>,
    pub _layer: *const WidgetDefault<'a>,
    pub _item_widgets: HashMap<ItemDataType, ItemWidget<'a>>,
}

impl<'a> ItemWidget<'a> {
    pub fn create_item_widget(
        parent_widget: &mut WidgetDefault<'a>,
        item_data_type: &ItemDataType,
        material_instance: &RcRefCell<MaterialInstanceData<'a>>,
        item_count: i32
    ) -> ItemWidget<'a> {
        let item_widget = UIManager::create_widget("item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_round(2.0);
        ui_component.set_margin(WIDGET_UI_MARGIN);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_size(30.0);
        ui_component.set_material_instance(&material_instance);
        parent_widget.add_widget(&item_widget);

        let mut item_widget = ItemWidget {
            _item_name: item_data_type.to_string(),
            _item_count: 0,
            _widget: item_widget.as_ref()
        };

        item_widget.set_item_count(item_count);
        item_widget
    }

    pub fn set_item_count(&mut self, item_count: i32) {
        self._item_count = 0.max(item_count);
        let ui_component = ptr_as_mut(self._widget).get_ui_component_mut();
        ui_component.set_text(format!("{}: {}", self._item_name, self._item_count).as_str());
    }

    pub fn add_item_count(&mut self, item_count: i32) {
        self.set_item_count(self._item_count + item_count);
    }
}

impl<'a> ItemBarWidget<'a> {
    pub fn create_item_bar_widget(engine_resources: &EngineResources<'a>, parent_widget: &mut WidgetDefault<'a>) -> ItemBarWidget<'a> {
        let layer = UIManager::create_widget("item_bar_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(layer.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::HORIZONTAL);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(50, 50, 50, 128));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        parent_widget.add_widget(&layer);

        ItemBarWidget {
            _engine_resources: engine_resources,
            _layer: layer.as_ref(),
            _item_widgets: HashMap::new(),
        }
    }

    pub fn update_layer(&mut self) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_size_x(ui_component.get_size_x() + ITEM_UI_SIZE);
        ui_component.set_size_y(ITEM_UI_SIZE);
    }

    pub fn add_item(&mut self, item_data_type: &ItemDataType, item_count: i32) {
        if let Some(item_widget) = self._item_widgets.get_mut(item_data_type) {
            item_widget.add_item_count(item_count)
        } else {
            let material = ptr_as_ref(self._engine_resources).get_material_instance_data(
                ItemDataType::get_item_material_instance_name(&item_data_type)
            );
            let new_item_widget = ItemWidget::create_item_widget(ptr_as_mut(self._layer), item_data_type, material, item_count);
            self._item_widgets.insert(*item_data_type, new_item_widget);
        }
        self.update_layer();
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_pos_x(window_size.x as f32 * 0.5 - ui_component.get_size_x() * 0.5);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - 50.0);
    }
}
