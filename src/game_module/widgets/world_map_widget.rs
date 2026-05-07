use std::collections::HashMap;
use nalgebra::Vector2;
use rust_engine_3d::scene::ui::{WidgetDefault};
use std::rc::Rc;
use strum_macros::{Display, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, EnumString, Copy)]
pub enum WorldMapDirection {
    LEFT = 0,
    RIGHT = 1,
    UP = 2,
    DOWN = 3,
    COUNT = 4,
}

pub struct WorldMapBridge<'a> {
    pub _bridge_widget: Rc<WidgetDefault<'a>>
}

pub struct WorldMapStage<'a> {
    pub _selected: bool,
    pub _stage_name: String,
    pub _world_map_stage: Rc<WidgetDefault<'a>>,
    pub _linked_stages: [Option<Rc<WorldMapStage<'a>>>; WorldMapDirection::COUNT as usize],
    pub _linked_bridges: [Option<Rc<WidgetDefault<'a>>>; WorldMapDirection::COUNT as usize],
}

pub struct WorldMapPlayer<'a> {
    pub _player_icon: Rc<WidgetDefault<'a>>,
    pub _player_icon_pos: Vector2<f32>
}

pub struct WorldMapWidget<'a> {
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _world_map_widget: Rc<WidgetDefault<'a>>,
    pub _bridge_layer_widget: Rc<WidgetDefault<'a>>,
    pub _stage_layer_widget: Rc<WidgetDefault<'a>>,
    pub _player_layer_widget: Rc<WidgetDefault<'a>>,
    pub _image_aspect: f32,
    pub _world_map_player: Box<WorldMapPlayer<'a>>,
    pub _world_map_stages: HashMap<String, Rc<WorldMapStage<'a>>>
}