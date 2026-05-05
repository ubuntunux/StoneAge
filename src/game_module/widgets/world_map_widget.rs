use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{WidgetDefault};
use std::rc::Rc;
pub struct WorldMapBridge<'a> {
    pub _bridge_widget: Rc<WidgetDefault<'a>>,
    pub _stage_widget_a: Rc<WidgetDefault<'a>>,
    pub _stage_widget_b: Rc<WidgetDefault<'a>>
}

pub struct WorldMapStage<'a> {
    pub _world_map_stage: Rc<WidgetDefault<'a>>,
}

pub struct WorldMapPlayer<'a> {
    pub _player_icon: Rc<WidgetDefault<'a>>,
    pub _player_icon_pos: Vector2<f32>
}

pub struct WorldMapWidget<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _world_map_widget: Rc<WidgetDefault<'a>>,
    pub _image_aspect: f32,
    pub _world_map_player: Box<WorldMapPlayer<'a>>,
    pub _world_map_stages: Vec<Box<WorldMapStage<'a>>>,
    pub _world_map_bridges: Vec<Box<WorldMapBridge<'a>>>,
}