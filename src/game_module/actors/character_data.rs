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
    pub _attack_animation_speed: f32,
    pub _dead_animation_mesh: String,
    pub _dead_animation_speed: f32,
    pub _idle_animation_mesh: String,
    pub _idle_animation_speed: f32,
    pub _hit_animation_mesh: String,
    pub _hit_animation_speed: f32,
    pub _jump_animation_mesh: String,
    pub _jump_animation_speed: f32,
    pub _power_attack_animation_mesh: String,
    pub _power_attack_animation_speed: f32,
    pub _roll_animation_mesh: String,
    pub _roll_animation_speed: f32,
    pub _run_animation_mesh: String,
    pub _run_animation_speed: f32,
    pub _running_jump_animation_mesh: String,
    pub _running_jump_animation_speed: f32,
    pub _upper_animation_layer: String,
    pub _walk_animation_mesh: String,
    pub _walk_animation_speed: f32,
    pub _max_hp: i32,
    pub _attack_damage: i32,
    pub _attack_event_time: f32,
    pub _attack_range: f32,
    pub _attack_thickness: f32,
    pub _power_attack_damage: i32,
    pub _power_attack_event_time: f32,
    pub _power_attack_range: f32,
    pub _jump_speed: f32,
    pub _roll_speed: f32,
    pub _run_speed: f32,
    pub _walk_speed: f32,
    pub _audio_dead: String,
    pub _audio_growl: String,
    pub _audio_pain: String,

}

impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::Roamer,
            _model_data_name: String::default(),
            _attack_animation_mesh: String::default(),
            _attack_animation_speed: 1.0,
            _dead_animation_mesh: String::default(),
            _dead_animation_speed: 1.0,
            _hit_animation_mesh: String::default(),
            _hit_animation_speed: 1.0,
            _idle_animation_mesh: String::default(),
            _idle_animation_speed: 1.0,
            _jump_animation_mesh: String::default(),
            _jump_animation_speed: 1.0,
            _power_attack_animation_mesh: String::default(),
            _power_attack_animation_speed: 1.0,
            _roll_animation_mesh: String::default(),
            _roll_animation_speed: 1.0,
            _run_animation_mesh: String::default(),
            _run_animation_speed: 1.0,
            _running_jump_animation_mesh: String::default(),
            _running_jump_animation_speed: 1.0,
            _upper_animation_layer: String::default(),
            _walk_animation_mesh: String::default(),
            _walk_animation_speed: 1.0,
            _max_hp: 100,
            _attack_damage: 50,
            _attack_event_time: 0.15,
            _attack_range: 0.5,
            _attack_thickness: 0.5,
            _power_attack_damage: 100,
            _power_attack_event_time: 1.0,
            _power_attack_range: 1.0,
            _jump_speed: 13.0,
            _roll_speed: 4.5,
            _run_speed: 5.4,
            _walk_speed: 3.0,
            _audio_dead: String::default(),
            _audio_growl: String::default(),
            _audio_pain: String::default(),
        }
    }
}