use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use serde::{Deserialize, Serialize};

pub const DEFAULT_MASTER_VOLUME: f32 = 1.0;
pub const DEFAULT_BGM_VOLUME: f32 = 1.0;
pub const DEFAULT_SFX_VOLUME: f32 = 1.0;
pub const DEFAULT_IS_MUTE: bool = false;
pub const DEFAULT_MOUSE_SENSITIVITY: f32 = 1.0;
pub const DEFAULT_INVERT_MOUSE_Y: bool = false;

pub const MIN_VOLUME: f32 = 0.0;
pub const MAX_VOLUME: f32 = 1.0;
pub const VOLUME_STEP: f32 = 0.1;

pub const MIN_MOUSE_SENSITIVITY: f32 = 0.2;
pub const MAX_MOUSE_SENSITIVITY: f32 = 2.0;
pub const MOUSE_SENSITIVITY_STEP: f32 = 0.1;

pub const MAX_AUDIO_MIXER_VOLUME: f32 = 128.0;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct GameOptionData {
    pub _master_volume: f32,
    pub _bgm_volume: f32,
    pub _sfx_volume: f32,
    pub _is_mute: bool,
    pub _mouse_sensitivity: f32,
    pub _invert_mouse_y: bool,
}

impl Default for GameOptionData {
    fn default() -> Self {
        GameOptionData {
            _master_volume: DEFAULT_MASTER_VOLUME,
            _bgm_volume: DEFAULT_BGM_VOLUME,
            _sfx_volume: DEFAULT_SFX_VOLUME,
            _is_mute: DEFAULT_IS_MUTE,
            _mouse_sensitivity: DEFAULT_MOUSE_SENSITIVITY,
            _invert_mouse_y: DEFAULT_INVERT_MOUSE_Y,
        }
    }
}

impl GameOptionData {
    pub fn apply(&self) {
        let effective_bgm_volume = if self._is_mute {
            0.0
        } else {
            (self._bgm_volume * self._master_volume).clamp(MIN_VOLUME, MAX_VOLUME)
        };

        let effective_sfx_volume = if self._is_mute {
            0.0
        } else {
            (self._sfx_volume * self._master_volume).clamp(MIN_VOLUME, MAX_VOLUME)
        };

        let audio_manager = get_audio_manager_mut();
        audio_manager.set_bgm_volume(effective_bgm_volume);
        audio_manager._volume = (effective_sfx_volume * MAX_AUDIO_MIXER_VOLUME) as i32;
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self._master_volume = volume.clamp(MIN_VOLUME, MAX_VOLUME);
    }

    pub fn set_bgm_volume(&mut self, volume: f32) {
        self._bgm_volume = volume.clamp(MIN_VOLUME, MAX_VOLUME);
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self._sfx_volume = volume.clamp(MIN_VOLUME, MAX_VOLUME);
    }

    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self._mouse_sensitivity = sensitivity.clamp(MIN_MOUSE_SENSITIVITY, MAX_MOUSE_SENSITIVITY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_option_default_and_serialization() {
        let default_option = GameOptionData::default();
        assert_eq!(default_option._master_volume, DEFAULT_MASTER_VOLUME);
        assert_eq!(default_option._bgm_volume, DEFAULT_BGM_VOLUME);
        assert_eq!(default_option._sfx_volume, DEFAULT_SFX_VOLUME);
        assert_eq!(default_option._is_mute, DEFAULT_IS_MUTE);
        assert_eq!(default_option._mouse_sensitivity, DEFAULT_MOUSE_SENSITIVITY);
        assert_eq!(default_option._invert_mouse_y, DEFAULT_INVERT_MOUSE_Y);

        let json_str = serde_json::to_string_pretty(&default_option).expect("Serialization failed");
        let deserialized: GameOptionData = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(default_option, deserialized);
    }

    #[test]
    fn test_game_option_clamping() {
        let mut option = GameOptionData::default();
        option.set_master_volume(-0.5);
        assert_eq!(option._master_volume, MIN_VOLUME);

        option.set_master_volume(2.5);
        assert_eq!(option._master_volume, MAX_VOLUME);

        option.set_mouse_sensitivity(0.01);
        assert_eq!(option._mouse_sensitivity, MIN_MOUSE_SENSITIVITY);

        option.set_mouse_sensitivity(5.0);
        assert_eq!(option._mouse_sensitivity, MAX_MOUSE_SENSITIVITY);
    }
}
