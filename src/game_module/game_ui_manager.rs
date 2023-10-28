use std::rc::Rc;

use rust_engine_3d::scene::ui::{UIManager, Widget};
use crate::game_module::game_client::GameClient;
use crate::game_module::widgets::hud::{CrossHair, PlayerHud, SelectionArea, TargetHud};

pub struct GameUIManager {
    pub _ui_manager: *const UIManager,
    pub _game_client: *const GameClient,
    pub _root_widget: *const dyn Widget,
    pub _game_ui_layout: *const dyn Widget,
    pub _ui_switch: Option<UISwitch>,
    pub _ui_world_axis: Option<UIWorldAxis>,
    pub _cross_hair: Option<CrossHair>,
    pub _target_hud: Option<TargetHud>,
    pub _player_hud: Option<PlayerHud>,
    pub _selection_area: Option<Box<SelectionArea>>,
}

pub struct UISwitch {
    pub _ui_switch_widget: Rc<dyn Widget>,
}

pub struct UIWorldAxis {
    pub _widget_axis_x: Rc<dyn Widget>,
    pub _widget_axis_y: Rc<dyn Widget>,
    pub _widget_axis_z: Rc<dyn Widget>,
}
