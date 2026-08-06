use crate::game_module::widgets::status_bar_widget::StatusBarWidget;
use rust_engine_3d::scene::ui::{WidgetDefault};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

const GAUGE_WIDTH: f32 = 300.0;
const GAUGE_HEIGHT: f32 = 30.0;

pub struct FishingGaugeWidget<'a> {
    pub _status_widget: StatusBarWidget<'a>,
}

impl<'a> FishingGaugeWidget<'a> {
    pub fn create_fishing_gauge_widget(parent_widget: &mut WidgetDefault<'a>) -> FishingGaugeWidget<'a> {
        let status_widget = StatusBarWidget::create_status_widget(parent_widget, get_color32(0, 200, 255, 230));
        let layer_ui = ptr_as_mut(status_widget._status_layer).get_ui_component_mut();
        layer_ui.set_pivot_preset(rust_engine_3d::scene::ui::PIVOT_CENTER);
        layer_ui.set_pos_hint(Some(0.5), Some(0.8));
        layer_ui.set_size(GAUGE_WIDTH, GAUGE_HEIGHT);
        layer_ui.set_visible(false);

        FishingGaugeWidget {
            _status_widget: status_widget,
        }
    }

    pub fn set_visible_fishing_gauge(&self, visible: bool) {
        ptr_as_mut(self._status_widget._status_layer).get_ui_component_mut().set_visible(visible);
    }

    pub fn update_fishing_gauge_widget(&self, gauge_value: f32, max_gauge_value: f32, delta_time: f64) {
        self._status_widget.update_status_widget(gauge_value, max_gauge_value, max_gauge_value, delta_time, false);
    }
}
