use rust_engine_3d::scene::ui::WidgetDefault;

pub struct ItemAcquireEntry {
    pub _item_data_name: String,
    pub _item_name: String,
    pub _remaining_time: f32,
}

pub struct ItemAcquireSlot<'a> {
    pub _root_widget: *const WidgetDefault<'a>,
    pub _icon_widget: *const WidgetDefault<'a>,
    pub _name_widget: *const WidgetDefault<'a>,
}

pub struct ItemAcquireNotificationWidget<'a> {
    pub _container_widget: *const WidgetDefault<'a>,
    pub _slots: Vec<ItemAcquireSlot<'a>>,
    pub _entries: Vec<ItemAcquireEntry>,
    pub _slot_height: f32,
}

pub const MAX_NOTIFICATION_SLOTS: usize = 5;
pub const NOTIFICATION_LAYOUT_WIDTH: f32 = 200.0;
pub const NOTIFICATION_ICON_SIZE: f32 = 48.0;
pub const NOTIFICATION_ICON_TEXT_MARGIN: f32 = 8.0;
pub const NOTIFICATION_MARGIN_LEFT: f32 = 20.0;
pub const NOTIFICATION_MARGIN_TOP: f32 = 100.0;
pub const NOTIFICATION_ROW_MARGIN: f32 = 6.0;
pub const NOTIFICATION_FONT_SIZE: f32 = 25.0;
pub const NOTIFICATION_DISPLAY_DURATION: f32 = 3.0;
pub const NOTIFICATION_FADE_DURATION: f32 = 0.5;
pub const NOTIFICATION_BG_COLOR: u32 = 0x80_00_00_00;
