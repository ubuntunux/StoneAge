use rust_engine_3d::scene::ui::{UILayoutType, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

const WIDGET_UI_WIDTH: f32 = 300.0;
const WIDGET_UI_HEIGHT: f32 = 40.0;
const WIDGET_UI_MARGIN: f32 = 2.0;
const WIDGET_UI_PADDING: f32 = 2.0;
const STATUS_BAR_DECAY_SPEED: f32 = 0.25;

pub struct StatusBarWidget<'a> {
    pub _status_layer: *const WidgetDefault<'a>,
    pub _max_status_bar: *const WidgetDefault<'a>,
    pub _status_bar: *const WidgetDefault<'a>,
}

// Implementation
fn create_status_layer_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
) -> *const WidgetDefault<'a> {
    let status_layer = UIManager::create_widget("status_layer", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_layer.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::FloatLayout);
    ui_component.set_size(WIDGET_UI_WIDTH, WIDGET_UI_HEIGHT);
    ui_component.set_color(get_color32(0, 0, 0, 128));
    ui_component.set_border_color(get_color32(0, 0, 0, 255));
    ui_component.set_round(10.0);
    ui_component.set_border(4.0);
    ui_component.set_margin(WIDGET_UI_MARGIN);
    ui_component.set_padding(WIDGET_UI_PADDING);
    parent_widget.add_widget(&status_layer);
    status_layer.as_ref()
}

fn create_status_bar_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    color: u32,
) -> (*const WidgetDefault<'a>, *const WidgetDefault<'a>) {
    let max_status_bar = UIManager::create_widget("max_status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(max_status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_enable_renderable_area(true);
    ui_component.set_color(get_color32(50, 50, 50, 255));
    parent_widget.add_widget(&max_status_bar);

    let status_bar = UIManager::create_widget("status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_enable_renderable_area(true);
    ui_component.set_color(color);
    parent_widget.add_widget(&status_bar);
    (status_bar.as_ref(), max_status_bar.as_ref())
}

impl<'a> StatusBarWidget<'a> {
    pub fn create_status_widget(
        parent_widget: &mut WidgetDefault<'a>,
        color: u32,
    ) -> StatusBarWidget<'a> {
        let status_layer = create_status_layer_widget(parent_widget);
        let (status_bar, max_status_bar) = create_status_bar_widget(ptr_as_mut(status_layer), color);
        StatusBarWidget {
            _status_layer: status_layer,
            _max_status_bar: max_status_bar,
            _status_bar: status_bar,
        }
    }

    pub fn update_status_widget(&self, status: f32, max_status: f32, max_status_data: f32, delta_time: f64) {
        let status_ratio = 0f32.max(1.0f32.min(status / max_status_data));
        let status_bar = ptr_as_mut(self._status_bar).get_ui_component_mut();
        let mut status = status_bar.get_size_hint_x().unwrap_or(1.0);
        if status < status_ratio {
            status = status_ratio.min(status + delta_time as f32 * STATUS_BAR_DECAY_SPEED);
            status_bar.set_size_hint_x(Some(status));
        } else {
            status_bar.set_size_hint_x(Some(status_ratio));
        }

        let max_status_ratio = 1.0f32.min(max_status / max_status_data);
        let max_status_bar = ptr_as_mut(self._max_status_bar).get_ui_component_mut();
        let mut size_hint_x = max_status_bar.get_size_hint_x().unwrap_or(1.0);
        if max_status_ratio < size_hint_x {
            size_hint_x = max_status_ratio.max(size_hint_x - delta_time as f32 * STATUS_BAR_DECAY_SPEED);
            max_status_bar.set_size_hint_x(Some(size_hint_x));
        } else {
            max_status_bar.set_size_hint_x(Some(max_status_ratio));
        }
    }
}
