use std::collections::HashMap;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type PropMap<'a> = HashMap<u64, RcRefCell<Prop<'a>>>;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PropDataType {
    None
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PropCreateInfo {
    pub _prop_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PropData {
    pub _prop_type: PropDataType,
    pub _model_data_name: String,
    pub _name: String,
    pub _max_hp: i32,
    pub _item_data_name: String,
    pub _item_drop_count_max: i32,
    pub _item_drop_count_min: i32,
    pub _item_regenerate_count: i32,
    pub _item_regenerate_time: f32
}

pub struct PropStats {
    pub _is_alive: bool,
    pub _prop_hp: i32,
    pub _item_regenerate_count: i32,
    pub _item_regenerate_time: f32,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Prop<'a> {
    pub _prop_name: String,
    pub _prop_id: u64,
    pub _prop_radius: f32,
    pub _prop_manager: *const PropManager<'a>,
    pub _render_object: RcRefCell<RenderObjectData<'a>>,
    pub _prop_data: RcRefCell<PropData>,
    pub _prop_stats: Box<PropStats>
}

pub struct PropManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _id_generator: u64,
    pub _props: PropMap<'a>
}