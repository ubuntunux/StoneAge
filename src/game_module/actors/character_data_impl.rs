use crate::game_module::actors::character_data::*;
use crate::game_module::game_resource::GameResources;
use rust_engine_3d::resource::resource::EngineResources;

impl Default for ActionAnimationState {
    fn default() -> Self {
        ActionAnimationState::None
    }
}

impl Default for MoveAnimationState {
    fn default() -> Self {
        MoveAnimationState::None
    }
}

impl Default for CharacterDataType {
    fn default() -> Self {
        CharacterDataType::None
    }
}

impl Default for CharacterAnimationDataCreateInfo {
    fn default() -> CharacterAnimationDataCreateInfo {
        CharacterAnimationDataCreateInfo {
            _attack_animation: String::default(),
            _attack_animation_speed: 1.0,
            _dead_animation: String::default(),
            _dead_animation_speed: 1.0,
            _hit_animation: String::default(),
            _hit_animation_speed: 1.0,
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
            _sleep_animation: String::default(),
            _stand_up_animation: String::default(),
            _upper_animation_layer: String::default(),
            _walk_animation: String::default(),
            _walk_animation_speed: 1.0,
        }
    }
}

impl CharacterData {
    pub fn create_character_data(
        character_data_create_info: &CharacterDataCreateInfo,
        game_resources: &GameResources,
    ) -> CharacterData {
        CharacterData {
            _character_type: character_data_create_info._character_type,
            _model_data_name: character_data_create_info._model_data_name.clone(),
            _name: character_data_create_info._name.clone(),
            _animation_data: CharacterAnimationData::create_character_animation_data(
                &character_data_create_info._character_animation_data,
                game_resources.get_engine_resources(),
            ),
            _audio_data: CharacterAudioData::create_character_audio_data(
                &character_data_create_info._character_audio_data,
                game_resources.get_engine_resources_mut(),
            ),
            _stat_data: character_data_create_info._character_stat_data.clone(),
            _weapon_create_info: character_data_create_info._weapon_create_info.clone(),
        }
    }
}

impl CharacterAudioData {
    pub fn create_character_audio_data(
        audio_data_create_info: &CharacterAudioDataCreateInfo,
        engine_resources: &mut EngineResources,
    ) -> CharacterAudioData {
        CharacterAudioData {
            _audio_dead: engine_resources
                .get_audio_bank_data(&audio_data_create_info._audio_dead)
                .clone(),
            _audio_growl: engine_resources
                .get_audio_bank_data(&audio_data_create_info._audio_growl)
                .clone(),
            _audio_pain: engine_resources
                .get_audio_bank_data(&audio_data_create_info._audio_pain)
                .clone(),
        }
    }
}

impl CharacterAnimationData {
    pub fn create_character_animation_data(
        animation_data_create_info: &CharacterAnimationDataCreateInfo,
        engine_resources: &EngineResources,
    ) -> CharacterAnimationData {
        CharacterAnimationData {
            _attack_animation: engine_resources.get_mesh_data(&animation_data_create_info._attack_animation).clone(),
            _attack_animation_speed: animation_data_create_info._attack_animation_speed,
            _dead_animation: engine_resources.get_mesh_data(&animation_data_create_info._dead_animation).clone(),
            _dead_animation_speed: animation_data_create_info._dead_animation_speed,
            _hit_animation: engine_resources.get_mesh_data(&animation_data_create_info._hit_animation).clone(),
            _hit_animation_speed: animation_data_create_info._hit_animation_speed,
            _idle_animation: engine_resources.get_mesh_data(&animation_data_create_info._idle_animation).clone(),
            _idle_animation_speed: animation_data_create_info._idle_animation_speed,
            _jump_animation: engine_resources.get_mesh_data(&animation_data_create_info._jump_animation).clone(),
            _jump_animation_speed: animation_data_create_info._jump_animation_speed,
            _kick_animation: engine_resources.get_mesh_data(&animation_data_create_info._kick_animation).clone(),
            _kick_animation_speed: animation_data_create_info._kick_animation_speed,
            _laying_down_animation: engine_resources.get_mesh_data(&animation_data_create_info._laying_down_animation).clone(),
            _pickup_animation: engine_resources.get_mesh_data(&animation_data_create_info._pickup_animation).clone(),
            _power_attack_animation: engine_resources.get_mesh_data(&animation_data_create_info._power_attack_animation).clone(),
            _power_attack_animation_speed: animation_data_create_info._power_attack_animation_speed,
            _roll_animation: engine_resources.get_mesh_data(&animation_data_create_info._roll_animation).clone(),
            _roll_animation_speed: animation_data_create_info._roll_animation_speed,
            _run_animation: engine_resources.get_mesh_data(&animation_data_create_info._run_animation).clone(),
            _run_animation_speed: animation_data_create_info._run_animation_speed,
            _running_jump_animation: engine_resources.get_mesh_data(&animation_data_create_info._running_jump_animation).clone(),
            _running_jump_animation_speed: animation_data_create_info._running_jump_animation_speed,
            _sleep_animation: engine_resources.get_mesh_data(&animation_data_create_info._sleep_animation).clone(),
            _stand_up_animation: engine_resources.get_mesh_data(&animation_data_create_info._stand_up_animation).clone(),
            _walk_animation: engine_resources.get_mesh_data(&animation_data_create_info._walk_animation).clone(),
            _walk_animation_speed: animation_data_create_info._walk_animation_speed,
            _upper_animation_layer: engine_resources.get_animation_layer_data(&animation_data_create_info._upper_animation_layer).clone(),
        }
    }
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
