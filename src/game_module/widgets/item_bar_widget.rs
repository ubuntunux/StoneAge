use nalgebra::Vector2;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::ui::{HorizontalAlign, Orientation, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

const ITEM_UI_SIZE: f32 = 128.0;

const WIDGET_UI_MARGIN: f32 = 5.0;
const WIDGET_UI_PADDING: f32 = 10.0;

pub struct ItemWidget<'a> {
    pub _widget: *const WidgetDefault<'a>,
}

pub struct ItemBarWidget<'a> {
    pub _layer: *const WidgetDefault<'a>,
    pub _item_widgets: Vec<*const WidgetDefault<'a>>,
}

impl<'a> ItemWidget<'a> {
    pub fn create_item_widget(parent_widget: &mut WidgetDefault<'a>, material_instance: &RcRefCell<MaterialInstanceData<'a>>) -> *const WidgetDefault<'a> {
        let item_widget = UIManager::create_widget("item_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(item_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(ITEM_UI_SIZE, ITEM_UI_SIZE);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_round(2.0);
        ui_component.set_margin(WIDGET_UI_MARGIN);
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_text("item");
        ui_component.set_font_size(30.0);
        ui_component.set_material_instance(&material_instance);
        parent_widget.add_widget(&item_widget);
        item_widget.as_ref()
    }
}

impl<'a> ItemBarWidget<'a> {
    pub fn create_item_bar_widget(parent_widget: &mut WidgetDefault<'a>, item_bar_materials: &Vec<&RcRefCell<MaterialInstanceData<'a>>>) -> ItemBarWidget<'a> {
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
        let layer_ref = layer.as_ref();

        let mut item_bar_widget = ItemBarWidget {
            _layer: layer_ref,
            _item_widgets: Vec::new(),
        };

        for material in item_bar_materials {
            item_bar_widget.add_item_widget(*material);
        }

        item_bar_widget
    }

    pub fn add_item_widget(&mut self, material: &RcRefCell<MaterialInstanceData<'a>>) {
        let item_widget = ItemWidget::create_item_widget(ptr_as_mut(self._layer), material);
        self._item_widgets.push(item_widget);

        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_size_x(ui_component.get_size_x() + ITEM_UI_SIZE);
        ui_component.set_size_y(ITEM_UI_SIZE);
    }

    pub fn changed_window_size(&mut self, window_size: &Vector2<i32>) {
        let ui_component = ptr_as_mut(self._layer).get_ui_component_mut();
        ui_component.set_pos_x(window_size.x as f32 * 0.5 - ui_component.get_size_x() * 0.5);
        ui_component.set_pos_y(window_size.y as f32 - ui_component.get_size_y() - 50.0);
    }
}
