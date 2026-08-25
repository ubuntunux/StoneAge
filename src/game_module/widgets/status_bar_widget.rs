use rust_engine_3d::scene::ui::{UILayoutType, UIManager, UIWidgetTypes, WidgetDefault};
use rust_engine_3d::utilities::system::ptr_as_mut;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::cell::Cell;

const WIDGET_UI_WIDTH: f32 = 300.0;
const WIDGET_UI_HEIGHT: f32 = 40.0;
const WIDGET_UI_MARGIN: f32 = 2.0;
const WIDGET_UI_PADDING: f32 = 2.0;
const STATUS_BAR_DECAY_SPEED: f32 = 0.25;

pub struct StatusBarWidget<'a> {
    pub _status_layer: *const WidgetDefault<'a>,
    pub _max_status_bar: *const WidgetDefault<'a>,
    pub _status_bar: *const WidgetDefault<'a>,
    pub _default_color: Cell<u32>,
    pub _warning_timer: Cell<f32>,
    pub _accum_flash_time: Cell<f32>,
}

// Implementation
fn create_status_layer_widget<'a>(parent_widget: &mut WidgetDefault<'a>) -> *const WidgetDefault<'a> {
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
    ui_component.set_enable_renderable_area(true);
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
    ui_component.set_color(get_color32(50, 50, 50, 255));
    parent_widget.add_widget(&max_status_bar);

    let status_bar = UIManager::create_widget("status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_color(color);
    parent_widget.add_widget(&status_bar);
    (status_bar.as_ref(), max_status_bar.as_ref())
}

impl<'a> StatusBarWidget<'a> {
    pub fn create_status_widget(parent_widget: &mut WidgetDefault<'a>, color: u32) -> StatusBarWidget<'a> {
        let status_layer = create_status_layer_widget(parent_widget);
        let (status_bar, max_status_bar) = create_status_bar_widget(ptr_as_mut(status_layer), color);
        StatusBarWidget {
            _status_layer: status_layer,
            _max_status_bar: max_status_bar,
            _status_bar: status_bar,
            _default_color: Cell::new(color),
            _warning_timer: Cell::new(0.0),
            _accum_flash_time: Cell::new(0.0),
        }
    }

    pub fn create_vertical_status_widget(
        parent_widget: &mut WidgetDefault<'a>,
        color: u32,
        width: f32,
        height: f32,
    ) -> StatusBarWidget<'a> {
        let status_layer = create_vertical_status_layer_widget(parent_widget, width, height);
        let (status_bar, max_status_bar) = create_vertical_status_bar_widget(ptr_as_mut(status_layer), color);
        StatusBarWidget {
            _status_layer: status_layer,
            _max_status_bar: max_status_bar,
            _status_bar: status_bar,
            _default_color: Cell::new(color),
            _warning_timer: Cell::new(0.0),
            _accum_flash_time: Cell::new(0.0),
        }
    }

    pub fn trigger_warning(&self) {
        self._warning_timer.set(1.0);
    }

    pub fn set_bar_color(&self, color: u32) {
        ptr_as_mut(self._status_bar).get_ui_component_mut().set_color(color);
    }

    pub fn set_bg_color(&self, color: u32) {
        ptr_as_mut(self._max_status_bar).get_ui_component_mut().set_color(color);
    }

    pub fn update_status_widget(
        &self,
        status: f32,
        max_status: f32,
        max_status_data: f32,
        delta_time: f64,
        smooth_update: bool,
        low_status_warning_threshold: Option<f32>,
    ) {
        let default_bg_color = get_color32(50, 50, 50, 255);
        let mut warning_timer = self._warning_timer.get();

        let current_ratio = if max_status > 0.0 {
            0.0f32.max(1.0f32.min(status / max_status))
        } else {
            0.0
        };

        if warning_timer > 0.0 {
            warning_timer = (warning_timer - delta_time as f32).max(0.0);
            self._warning_timer.set(warning_timer);
            let elapsed = 1.0 - warning_timer;
            let flash_phase = (elapsed * 5.0 * std::f32::consts::TAU).sin();
            if flash_phase > 0.0 {
                self.set_bar_color(get_color32(255, 30, 30, 230));
                self.set_bg_color(get_color32(120, 30, 30, 255));
            } else {
                self.set_bar_color(self._default_color.get());
                self.set_bg_color(default_bg_color);
            }
        } else if 0.0 < current_ratio
            && let Some(threshold) = low_status_warning_threshold
        {
            if current_ratio < threshold {
                let proximity = (threshold - current_ratio).max(0.0) / threshold;
                let speed_mult = 1.0 + proximity; // 1.0x to 2.0x faster
                let base_freq = 3.0;
                let freq = base_freq * speed_mult;

                let current_accum = self._accum_flash_time.get() + delta_time as f32 * freq;
                self._accum_flash_time.set(current_accum);
                let flash_phase = (current_accum * std::f32::consts::TAU).sin();

                if flash_phase > 0.0 {
                    self.set_bar_color(get_color32(255, 30, 30, 230));
                    self.set_bg_color(get_color32(120, 30, 30, 255));
                } else {
                    self.set_bar_color(self._default_color.get());
                    self.set_bg_color(default_bg_color);
                }
            } else {
                self._accum_flash_time.set(0.0);
                self.set_bar_color(self._default_color.get());
                self.set_bg_color(default_bg_color);
            }
        } else {
            self._accum_flash_time.set(0.0);
            self.set_bar_color(self._default_color.get());
            self.set_bg_color(default_bg_color);
        }

        let status_ratio = 0f32.max(1.0f32.min(status / max_status_data));
        let status_bar = ptr_as_mut(self._status_bar).get_ui_component_mut();
        let mut status = status_bar.get_size_hint_x().unwrap_or(1.0);
        if smooth_update && status < status_ratio {
            status = status_ratio.min(status + delta_time as f32 * STATUS_BAR_DECAY_SPEED);
            status_bar.set_size_hint_x(Some(status));
        } else {
            status_bar.set_size_hint_x(Some(status_ratio));
        }

        let max_status_ratio = 1.0f32.min(max_status / max_status_data);
        let max_status_bar = ptr_as_mut(self._max_status_bar).get_ui_component_mut();
        let mut size_hint_x = max_status_bar.get_size_hint_x().unwrap_or(1.0);
        if smooth_update && max_status_ratio < size_hint_x {
            size_hint_x = max_status_ratio.max(size_hint_x - delta_time as f32 * STATUS_BAR_DECAY_SPEED);
            max_status_bar.set_size_hint_x(Some(size_hint_x));
        } else {
            max_status_bar.set_size_hint_x(Some(max_status_ratio));
        }
    }

    pub fn update_vertical_status_widget(&self, status: f32, _delta_time: f64, _smooth_update: bool) {
        let status_bar = ptr_as_mut(self._status_bar).get_ui_component_mut();
        status_bar.set_size_hint_y(Some(status));
    }
}

fn create_vertical_status_layer_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    width: f32,
    height: f32,
) -> *const WidgetDefault<'a> {
    let status_layer = UIManager::create_widget("vertical_status_layer", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_layer.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::FloatLayout);
    ui_component.set_size(width, height);
    ui_component.set_color(get_color32(0, 0, 0, 180));
    ui_component.set_border_color(get_color32(0, 0, 0, 255));
    ui_component.set_round(10.0);
    ui_component.set_border(3.0);
    ui_component.set_margin(WIDGET_UI_MARGIN);
    ui_component.set_padding(WIDGET_UI_PADDING);
    ui_component.set_enable_renderable_area(true);
    parent_widget.add_widget(&status_layer);
    status_layer.as_ref()
}

fn create_vertical_status_bar_widget<'a>(
    parent_widget: &mut WidgetDefault<'a>,
    color: u32,
) -> (*const WidgetDefault<'a>, *const WidgetDefault<'a>) {
    let max_status_bar = UIManager::create_widget("max_status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(max_status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_color(get_color32(50, 50, 50, 255));
    parent_widget.add_widget(&max_status_bar);

    let status_bar = UIManager::create_widget("status_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(status_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_pivot_preset(rust_engine_3d::scene::ui::PIVOT_BOTTOM_LEFT);
    ui_component.set_pos_hint(Some(0.0), Some(1.0));
    ui_component.set_valign(rust_engine_3d::scene::ui::VerticalAlign::BOTTOM);
    ui_component.set_color(color);
    parent_widget.add_widget(&status_bar);
    (status_bar.as_ref(), max_status_bar.as_ref())
}
