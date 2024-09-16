use rust_engine_3d::scene::ui::{
    HorizontalAlign, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

const WIDGET_UI_WIDTH: f32 = 200.0;
const WIDGET_UI_HEIGHT: f32 = 24.0;
const WIDGET_UI_MARGIN: f32 = 2.0;
const WIDGET_UI_PADDING: f32 = 2.0;

pub struct StatusBarWidget<'a> {
    pub _status_layer: *const WidgetDefault<'a>,
    pub _status_bar: *const WidgetDefault<'a>,
}

// Implementation
fn create_status_layer_widget<'a>(parent_widget: &mut WidgetDefault<'a>) -> *const WidgetDefault<'a> {
    let status_layer = UIManager::create_widget("status_layer", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_layer.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_size(WIDGET_UI_WIDTH, WIDGET_UI_HEIGHT);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_color(get_color32(50, 50, 50, 255));
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_border_color(get_color32(0, 0, 0, 255));
    ui_component.set_round(5.0);
    ui_component.set_border(2.0);
    ui_component.set_margin(WIDGET_UI_MARGIN);
    ui_component.set_padding(WIDGET_UI_PADDING);
    parent_widget.add_widget(&status_layer);
    status_layer.as_ref()
}

fn create_status_bar_widget<'a>(parent_widget: &mut WidgetDefault<'a>, color: u32) -> *const WidgetDefault<'a> {
    let status_bar = UIManager::create_widget("status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_color(color);
    ui_component.set_round(1.0);
    parent_widget.add_widget(&status_bar);
    status_bar.as_ref()
}

impl<'a> StatusBarWidget<'a> {
    pub fn create_status_widget(parent_widget: &mut WidgetDefault<'a>, color: u32) -> StatusBarWidget<'a> {
        let status_layer = create_status_layer_widget(parent_widget);
        let status_bar = create_status_bar_widget(ptr_as_mut(status_layer), color);
        StatusBarWidget {
            _status_layer: status_layer,
            _status_bar: status_bar,
        }
    }

    pub fn update_status_widget(&self, status: f32, max_status: f32) {
        let status_ratio = 1.0f32.min(status / max_status);
        let status_bar = ptr_as_mut(self._status_bar).get_ui_component_mut();
        status_bar.set_size_hint_x(Some(status_ratio));
    }
}
