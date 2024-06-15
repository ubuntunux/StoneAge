use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAnimationState {
    None,
    Attack,
    Dead,
    Hit,
    PowerAttack,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveDirections {
    NONE,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    Roamer
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SpawnPointData {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _attack_animation_mesh: String,
    pub _dead_animation_mesh: String,
    pub _idle_animation_mesh: String,
    pub _hit_animation_mesh: String,
    pub _jump_animation_mesh: String,
    pub _power_attack_animation_mesh: String,
    pub _roll_animation_mesh: String,
    pub _run_animation_mesh: String,
    pub _running_jump_animation_mesh: String,
    pub _upper_animation_layer: String,
    pub _walk_animation_mesh: String,
    pub _max_hp: i32,
}

impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::Roamer,
            _model_data_name: String::default(),
            _attack_animation_mesh: String::default(),
            _dead_animation_mesh: String::default(),
            _hit_animation_mesh: String::default(),
            _idle_animation_mesh: String::default(),
            _jump_animation_mesh: String::default(),
            _power_attack_animation_mesh: String::default(),
            _roll_animation_mesh: String::default(),
            _run_animation_mesh: String::default(),
            _running_jump_animation_mesh: String::default(),
            _upper_animation_layer: String::default(),
            _walk_animation_mesh: String::default(),
            _max_hp: 100,
        }
    }
}