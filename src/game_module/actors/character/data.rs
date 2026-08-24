use crate::game_module::actors::weapons::WeaponCreateInfo;
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::{get_engine_resources, get_engine_resources_mut};
use rust_engine_3d::resource::resource::ResourceData;
use rust_engine_3d::scene::animation::AnimationLayerData;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionAnimationState {
    #[default]
    None,
    Attack,
    Dance,
    Dead,
    Eating,
    Hit,
    Hungry,
    Kick,
    LayingDown,
    Pickup,
    PowerAttack,
    Sleep,
    SleepNoSnoring,
    WakeUp,
    FishingBegin,
    FishingLoop,
    FishingEnd,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionEvent {
    #[default]
    None,
    Attack,
    Dance,
    Dead,
    Eating,
    Hit,
    Hungry,
    Kick,
    LayingDown,
    Pickup,
    PowerAttack,
    Sleep,
    SleepNoSnoring,
    WakeUp,
    Fishing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterFishingState {
    pub _fishing_gauge: f32,
    pub _fishing_gauge_dir: f32,
    pub _is_fishing_button_held: bool,
    pub _fishing_cast_distance: f32,
    pub _fish_gauge: f32,
    pub _fish_angle: f32,
    pub _fish_target_angle: f32,
    pub _fish_change_timer: f32,
    pub _player_angle: f32,
    pub _direction_dot: f32,
    pub _is_pulling: bool,
    pub _is_direction_matched: bool,
    pub _is_minigame_active: bool,
    pub _wait_timer: f32,
    pub _minigame_success: Option<bool>,
    pub _difficulty_angle_range: f32,
}

impl Default for CharacterFishingState {
    fn default() -> CharacterFishingState {
        CharacterFishingState {
            _fishing_gauge: 0.0,
            _fishing_gauge_dir: 1.0,
            _is_fishing_button_held: false,
            _fishing_cast_distance: 2.0,
            _fish_gauge: 0.5,
            _fish_angle: 0.0,
            _fish_target_angle: 0.0,
            _fish_change_timer: 0.0,
            _player_angle: 0.0,
            _direction_dot: 1.0,
            _is_pulling: false,
            _is_direction_matched: false,
            _is_minigame_active: false,
            _wait_timer: 0.0,
            _minigame_success: None,
            _difficulty_angle_range: 70.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MoveAnimationState {
    #[default]
    None,
    Idle,
    Jump,
    Roll,
    Run,
    RunningJump,
    SitDownLoop,
    Walk,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub enum CharacterDataType {
    #[default]
    None,
    Player,
    Chef,
    Crafter,
    Civilian,
    Roamer,
    Guardian,
    Stalker,
    Invader,
    Ufo,
}

impl CharacterDataType {
    pub fn get_request_name(&self) -> Option<&'static str> {
        match self {
            CharacterDataType::Chef => Some("Cooking"),
            CharacterDataType::Crafter => Some("Craft"),
            _ => None,
        }
    }
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
    pub _name: String,
    pub _character_animation_data: CharacterAnimationDataCreateInfo,
    pub _character_audio_data: CharacterAudioDataCreateInfo,
    pub _character_stat_data: CharacterStatData,
    pub _weapon_create_info: WeaponCreateInfo,
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
    pub _dance_animation: String,
    pub _dead_animation: String,
    pub _dead_animation_speed: f32,
    pub _eating_animation: String,
    pub _hungry_animation: String,
    pub _idle_animation: String,
    pub _idle_animation_speed: f32,
    pub _hit_animation: String,
    pub _hit_animation_speed: f32,
    pub _jump_animation: String,
    pub _jump_animation_speed: f32,
    pub _kick_animation: String,
    pub _kick_animation_speed: f32,
    pub _laying_down_animation: String,
    pub _pickup_animation: String,
    pub _power_attack_animation: String,
    pub _power_attack_animation_speed: f32,
    pub _roll_animation: String,
    pub _roll_animation_speed: f32,
    pub _run_animation: String,
    pub _run_animation_speed: f32,
    pub _running_jump_animation: String,
    pub _running_jump_animation_speed: f32,
    pub _sit_down_animation: String,
    pub _sit_down_loop_animation: String,
    pub _sleep_animation: String,
    pub _stand_up_animation: String,
    pub _upper_animation_layer: String,
    pub _wake_up_animation: String,
    pub _walk_animation: String,
    pub _walk_animation_speed: f32,
    pub _fishing_begin_animation: String,
    pub _fishing_loop_animation: String,
    pub _fishing_end_animation: String,
}

impl Default for CharacterAnimationDataCreateInfo {
    fn default() -> CharacterAnimationDataCreateInfo {
        CharacterAnimationDataCreateInfo {
            _attack_animation: String::default(),
            _attack_animation_speed: 1.0,
            _dance_animation: String::default(),
            _dead_animation: String::default(),
            _dead_animation_speed: 1.0,
            _eating_animation: String::default(),
            _hit_animation: String::default(),
            _hit_animation_speed: 1.0,
            _hungry_animation: String::default(),
            _idle_animation: String::default(),
            _idle_animation_speed: 1.0,
            _jump_animation: String::default(),
            _jump_animation_speed: 1.0,
            _kick_animation: String::default(),
            _kick_animation_speed: 1.0,
            _laying_down_animation: String::default(),
            _pickup_animation: String::default(),
            _power_attack_animation: String::default(),
            _power_attack_animation_speed: 1.0,
            _roll_animation: String::default(),
            _roll_animation_speed: 1.0,
            _run_animation: String::default(),
            _run_animation_speed: 1.0,
            _running_jump_animation: String::default(),
            _running_jump_animation_speed: 1.0,
            _sit_down_animation: String::default(),
            _sit_down_loop_animation: String::default(),
            _sleep_animation: String::default(),
            _stand_up_animation: String::default(),
            _upper_animation_layer: String::default(),
            _wake_up_animation: String::default(),
            _walk_animation: String::default(),
            _walk_animation_speed: 1.0,
            _fishing_begin_animation: String::default(),
            _fishing_loop_animation: String::default(),
            _fishing_end_animation: String::default(),
        }
    }
}

pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _name: String,
    pub _audio_data: CharacterAudioData,
    pub _animation_data: CharacterAnimationData,
    pub _stat_data: CharacterStatData,
    pub _weapon_create_info: WeaponCreateInfo,
}

impl CharacterData {
    pub fn create_character_data(character_data_create_info: &CharacterDataCreateInfo) -> CharacterData {
        CharacterData {
            _character_type: character_data_create_info._character_type,
            _model_data_name: character_data_create_info._model_data_name.clone(),
            _name: character_data_create_info._name.clone(),
            _animation_data: CharacterAnimationData::create_character_animation_data(
                &character_data_create_info._character_animation_data,
            ),
            _audio_data: CharacterAudioData::create_character_audio_data(
                &character_data_create_info._character_audio_data,
            ),
            _stat_data: character_data_create_info._character_stat_data.clone(),
            _weapon_create_info: character_data_create_info._weapon_create_info.clone(),
        }
    }

    pub fn can_fly(&self) -> bool {
        self._character_type == CharacterDataType::Ufo
    }
}

pub struct CharacterAudioData {
    pub _audio_dead: ResourceData,
    pub _audio_growl: ResourceData,
    pub _audio_pain: ResourceData,
}

impl CharacterAudioData {
    pub fn create_character_audio_data(audio_data_create_info: &CharacterAudioDataCreateInfo) -> CharacterAudioData {
        let engine_resources = get_engine_resources_mut();
        CharacterAudioData {
            _audio_dead: engine_resources.get_audio_bank_data(&audio_data_create_info._audio_dead).clone(),
            _audio_growl: engine_resources.get_audio_bank_data(&audio_data_create_info._audio_growl).clone(),
            _audio_pain: engine_resources.get_audio_bank_data(&audio_data_create_info._audio_pain).clone(),
        }
    }
}

pub struct CharacterAnimationData {
    pub _attack_animation: RcRefCell<MeshData>,
    pub _attack_animation_speed: f32,
    pub _dance_animation: RcRefCell<MeshData>,
    pub _dead_animation: RcRefCell<MeshData>,
    pub _dead_animation_speed: f32,
    pub _eating_animation: RcRefCell<MeshData>,
    pub _hit_animation: RcRefCell<MeshData>,
    pub _hit_animation_speed: f32,
    pub _hungry_animation: RcRefCell<MeshData>,
    pub _idle_animation: RcRefCell<MeshData>,
    pub _idle_animation_speed: f32,
    pub _jump_animation: RcRefCell<MeshData>,
    pub _jump_animation_speed: f32,
    pub _kick_animation: RcRefCell<MeshData>,
    pub _kick_animation_speed: f32,
    pub _laying_down_animation: RcRefCell<MeshData>,
    pub _pickup_animation: RcRefCell<MeshData>,
    pub _power_attack_animation: RcRefCell<MeshData>,
    pub _power_attack_animation_speed: f32,
    pub _roll_animation: RcRefCell<MeshData>,
    pub _roll_animation_speed: f32,
    pub _run_animation: RcRefCell<MeshData>,
    pub _run_animation_speed: f32,
    pub _running_jump_animation: RcRefCell<MeshData>,
    pub _running_jump_animation_speed: f32,
    pub _sit_down_loop_animation: RcRefCell<MeshData>,
    pub _sleep_animation: RcRefCell<MeshData>,
    pub _wake_up_animation: RcRefCell<MeshData>,
    pub _walk_animation: RcRefCell<MeshData>,
    pub _walk_animation_speed: f32,
    pub _fishing_begin_animation: RcRefCell<MeshData>,
    pub _fishing_loop_animation: RcRefCell<MeshData>,
    pub _fishing_end_animation: RcRefCell<MeshData>,
    pub _upper_animation_layer: RcRefCell<AnimationLayerData>,
}

impl CharacterAnimationData {
    pub fn create_character_animation_data(
        animation_data_create_info: &CharacterAnimationDataCreateInfo,
    ) -> CharacterAnimationData {
        let engine_resources = get_engine_resources();
        CharacterAnimationData {
            _attack_animation: engine_resources.get_mesh_data(&animation_data_create_info._attack_animation).clone(),
            _attack_animation_speed: animation_data_create_info._attack_animation_speed,
            _dance_animation: engine_resources.get_mesh_data(&animation_data_create_info._dance_animation).clone(),
            _dead_animation: engine_resources.get_mesh_data(&animation_data_create_info._dead_animation).clone(),
            _dead_animation_speed: animation_data_create_info._dead_animation_speed,
            _eating_animation: engine_resources.get_mesh_data(&animation_data_create_info._eating_animation).clone(),
            _hit_animation: engine_resources.get_mesh_data(&animation_data_create_info._hit_animation).clone(),
            _hit_animation_speed: animation_data_create_info._hit_animation_speed,
            _hungry_animation: engine_resources.get_mesh_data(&animation_data_create_info._hungry_animation).clone(),
            _idle_animation: engine_resources.get_mesh_data(&animation_data_create_info._idle_animation).clone(),
            _idle_animation_speed: animation_data_create_info._idle_animation_speed,
            _jump_animation: engine_resources.get_mesh_data(&animation_data_create_info._jump_animation).clone(),
            _jump_animation_speed: animation_data_create_info._jump_animation_speed,
            _kick_animation: engine_resources.get_mesh_data(&animation_data_create_info._kick_animation).clone(),
            _kick_animation_speed: animation_data_create_info._kick_animation_speed,
            _laying_down_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._laying_down_animation)
                .clone(),
            _pickup_animation: engine_resources.get_mesh_data(&animation_data_create_info._pickup_animation).clone(),
            _power_attack_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._power_attack_animation)
                .clone(),
            _power_attack_animation_speed: animation_data_create_info._power_attack_animation_speed,
            _roll_animation: engine_resources.get_mesh_data(&animation_data_create_info._roll_animation).clone(),
            _roll_animation_speed: animation_data_create_info._roll_animation_speed,
            _run_animation: engine_resources.get_mesh_data(&animation_data_create_info._run_animation).clone(),
            _run_animation_speed: animation_data_create_info._run_animation_speed,
            _running_jump_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._running_jump_animation)
                .clone(),
            _running_jump_animation_speed: animation_data_create_info._running_jump_animation_speed,
            _sit_down_loop_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._sit_down_loop_animation)
                .clone(),
            _sleep_animation: engine_resources.get_mesh_data(&animation_data_create_info._sleep_animation).clone(),
            _wake_up_animation: engine_resources.get_mesh_data(&animation_data_create_info._wake_up_animation).clone(),
            _walk_animation: engine_resources.get_mesh_data(&animation_data_create_info._walk_animation).clone(),
            _walk_animation_speed: animation_data_create_info._walk_animation_speed,
            _fishing_begin_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._fishing_begin_animation)
                .clone(),
            _fishing_loop_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._fishing_loop_animation)
                .clone(),
            _fishing_end_animation: engine_resources
                .get_mesh_data(&animation_data_create_info._fishing_end_animation)
                .clone(),
            _upper_animation_layer: engine_resources
                .get_animation_layer_data(&animation_data_create_info._upper_animation_layer)
                .clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterStatData {
    pub _max_hp: i32,
    pub _attack_damage: i32,
    pub _attack_event_time: f32,
    pub _attack_range: f32,
    pub _power_attack_damage: i32,
    pub _power_attack_event_time: f32,
    pub _power_attack_range: f32,
    pub _kick_damage: i32,
    pub _kick_event_time: f32,
    pub _kick_range: f32,
    pub _jump_speed: f32,
    pub _roll_speed: f32,
    pub _run_speed: f32,
    pub _walk_speed: f32,
}

impl Default for CharacterStatData {
    fn default() -> CharacterStatData {
        CharacterStatData {
            _max_hp: 100,
            _attack_damage: 50,
            _attack_event_time: 0.5,
            _attack_range: 0.5,
            _power_attack_damage: 100,
            _power_attack_event_time: 1.0,
            _power_attack_range: 1.0,
            _kick_damage: 70,
            _kick_event_time: 0.6,
            _kick_range: 0.6,
            _jump_speed: 13.0,
            _roll_speed: 4.5,
            _run_speed: 5.4,
            _walk_speed: 3.0,
        }
    }
}
