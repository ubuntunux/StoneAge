use crate::game_module::game_scene_manager::GameSceneManager;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::scene::ui::WidgetDefault;
use std::collections::HashMap;
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
    pub _bridge_widget: Rc<WidgetDefault<'a>>,
}

pub struct WorldMapStage<'a> {
    pub _world_map_widget: *const WorldMapWidget<'a>,
    pub _stage_data_name: String,
    pub _selected: bool,
    pub _world_map_stage: Rc<WidgetDefault<'a>>,
    pub _linked_stages: [Option<Rc<WorldMapStage<'a>>>; WorldMapDirection::COUNT as usize + 1],
    pub _linked_bridges: [Option<Rc<WidgetDefault<'a>>>; WorldMapDirection::COUNT as usize + 1],
}

pub struct WorldMapPlayer<'a> {
    pub _world_map_widget: *const WorldMapWidget<'a>,
    pub _player_icon: Rc<WidgetDefault<'a>>,
}

pub struct WorldMapWidget<'a> {
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _root_widget: *const WidgetDefault<'a>,
    pub _background_layout: Rc<WidgetDefault<'a>>,
    pub _world_map_widget: Rc<WidgetDefault<'a>>,
    pub _bridge_layer_widget: Rc<WidgetDefault<'a>>,
    pub _stage_layer_widget: Rc<WidgetDefault<'a>>,
    pub _player_layer_widget: Rc<WidgetDefault<'a>>,
    pub _image_aspect: f32,
    pub _selected_stage_name: String,
    pub _world_map_player: Option<Box<WorldMapPlayer<'a>>>,
    pub _world_map_stages: HashMap<String, Rc<WorldMapStage<'a>>>,
    pub _is_opened_world_map: bool,
    pub _request_close_world_map: bool,
}
