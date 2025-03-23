use nalgebra::Vector3;
use rust_engine_3d::resource::resource::ResourceData;
use rust_engine_3d::scene::animation::AnimationLayerData;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use crate::game_module::actors::weapons::WeaponCreateInfo;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAnimationState {
    None,
    Attack,
    Dead,
    Hit,
    PowerAttack,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveAnimationState {
    None,
    Idle,
    Jump,
    Roll,
    Run,
    RunningJump,
    Walk,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    None,
    Roamer
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SpawnPointData {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterDataCreateInfo {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _character_animation_data: CharacterAnimationDataCreateInfo,
    pub _character_audio_data: CharacterAudioDataCreateInfo,
    pub _character_stat_data: CharacterStatData,
    pub _weapon_create_info: WeaponCreateInfo
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterAudioDataCreateInfo {
    pub _audio_dead: String,
    pub _audio_growl: String,
    pub _audio_pain: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterAnimationDataCreateInfo {
    pub _attack_animation: String,
    pub _attack_animation_speed: f32,
    pub _attack_event_time: f32,
    pub _dead_animation: String,
    pub _dead_animation_speed: f32,
    pub _idle_animation: String,
    pub _idle_animation_speed: f32,
    pub _hit_animation: String,
    pub _hit_animation_speed: f32,
    pub _jump_animation: String,
    pub _jump_animation_speed: f32,
    pub _power_attack_animation: String,
    pub _power_attack_animation_speed: f32,
    pub _power_attack_event_time: f32,
    pub _roll_animation: String,
    pub _roll_animation_speed: f32,
    pub _run_animation: String,
    pub _run_animation_speed: f32,
    pub _running_jump_animation: String,
    pub _running_jump_animation_speed: f32,
    pub _upper_animation_layer: String,
    pub _walk_animation: String,
    pub _walk_animation_speed: f32,
}

pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _audio_data: CharacterAudioData,
    pub _animation_data: CharacterAnimationData,
    pub _stat_data: CharacterStatData,
    pub _weapon_create_info: WeaponCreateInfo,
}

pub struct CharacterAudioData {
    pub _audio_dead: ResourceData,
    pub _audio_growl: ResourceData,
    pub _audio_pain: ResourceData,
}

pub struct CharacterAnimationData {
    pub _attack_animation: RcRefCell<MeshData>,
    pub _attack_animation_speed: f32,
    pub _attack_event_time: f32,
    pub _dead_animation: RcRefCell<MeshData>,
    pub _dead_animation_speed: f32,
    pub _hit_animation: RcRefCell<MeshData>,
    pub _hit_animation_speed: f32,
    pub _idle_animation: RcRefCell<MeshData>,
    pub _idle_animation_speed: f32,
    pub _jump_animation: RcRefCell<MeshData>,
    pub _jump_animation_speed: f32,
    pub _power_attack_animation: RcRefCell<MeshData>,
    pub _power_attack_animation_speed: f32,
    pub _power_attack_event_time: f32,
    pub _roll_animation: RcRefCell<MeshData>,
    pub _roll_animation_speed: f32,
    pub _run_animation: RcRefCell<MeshData>,
    pub _run_animation_speed: f32,
    pub _running_jump_animation: RcRefCell<MeshData>,
    pub _running_jump_animation_speed: f32,
    pub _walk_animation: RcRefCell<MeshData>,
    pub _walk_animation_speed: f32,
    pub _upper_animation_layer: RcRefCell<AnimationLayerData>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterStatData {
    pub _max_hp: i32,
    pub _attack_damage: i32,
    pub _attack_range: f32,
    pub _power_attack_damage: i32,
    pub _power_attack_range: f32,
    pub _jump_speed: f32,
    pub _roll_speed: f32,
    pub _run_speed: f32,
    pub _walk_speed: f32
}