use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{UIManager, WidgetDefault};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::controller_help::ControllerHelpWidget;
use crate::game_module::widgets::cross_hair_widget::CrossHairWidget;
use crate::game_module::widgets::image_widget::ImageLayout;
use crate::game_module::widgets::item_bar_widget::ItemBarWidget;
use crate::game_module::widgets::player_hud::PlayerHud;
use crate::game_module::widgets::target_status_bar::TargetStatusWidget;
use crate::game_module::widgets::time_of_day::TimeOfDayWidget;

pub struct EditorUIManager<'a> {
    pub _ui_manager: *const UIManager<'a>,
    pub _game_client: *const GameClient<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _root_widget: *const WidgetDefault<'a>,
    pub _editor_ui_layout: *const WidgetDefault<'a>,
    pub _actor_positions: Vec<*const WidgetDefault<'a>>,
    pub _window_size: Vector2<i32>,
}

pub struct GameUIManager<'a> {
    pub _ui_manager: *const UIManager<'a>,
    pub _game_client: *const GameClient<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _root_widget: *const WidgetDefault<'a>,
    pub _game_ui_layout: *const WidgetDefault<'a>,
    pub _game_image: Option<Box<ImageLayout<'a>>>,
    pub _cross_hair: Option<Box<CrossHairWidget<'a>>>,
    pub _player_hud: Option<Box<PlayerHud<'a>>>,
    pub _controller_help_widget: Option<Box<ControllerHelpWidget<'a>>>,
    pub _target_status_bar: Option<Box<TargetStatusWidget<'a>>>,
    pub _time_of_day: Option<Box<TimeOfDayWidget<'a>>>,
    pub _item_bar_widget: Option<Box<ItemBarWidget<'a>>>,
    pub _window_size: Vector2<i32>,
}