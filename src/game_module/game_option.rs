use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::scene::ui::set_global_dpi_scale;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

pub const DEFAULT_MASTER_VOLUME: f32 = 0.75;
pub const DEFAULT_BGM_VOLUME: f32 = 1.0;
pub const DEFAULT_SFX_VOLUME: f32 = 1.0;
pub const DEFAULT_IS_MUTE: bool = false;
pub const DEFAULT_MOUSE_SENSITIVITY: f32 = 1.0;
pub const DEFAULT_INVERT_MOUSE_Y: bool = false;
pub const DEFAULT_R_STICK_SENSITIVITY: f32 = 1.0;
pub const DEFAULT_L_STICK_SENSITIVITY: f32 = 1.0;
pub const DEFAULT_FONT_SCALE: f32 = 1.0;

pub const MIN_VOLUME: f32 = 0.0;
pub const MAX_VOLUME: f32 = 1.0;
pub const VOLUME_STEP: f32 = 0.1;

pub const MIN_MOUSE_SENSITIVITY: f32 = 0.2;
pub const MAX_MOUSE_SENSITIVITY: f32 = 2.0;
pub const MOUSE_SENSITIVITY_STEP: f32 = 0.1;

pub const MIN_STICK_SENSITIVITY: f32 = 0.2;
pub const MAX_STICK_SENSITIVITY: f32 = 3.0;
pub const STICK_SENSITIVITY_STEP: f32 = 0.1;

pub const MIN_FONT_SCALE: f32 = 0.7;
pub const MAX_FONT_SCALE: f32 = 1.5;
pub const FONT_SCALE_STEP: f32 = 0.1;

pub const MAX_AUDIO_MIXER_VOLUME: f32 = 128.0;

static GLOBAL_R_STICK_SENSITIVITY: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32
static GLOBAL_L_STICK_SENSITIVITY: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32
static GLOBAL_FONT_SCALE: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32

pub fn get_global_r_stick_sensitivity() -> f32 {
    f32::from_bits(GLOBAL_R_STICK_SENSITIVITY.load(Ordering::Relaxed))
}

pub fn get_global_l_stick_sensitivity() -> f32 {
    f32::from_bits(GLOBAL_L_STICK_SENSITIVITY.load(Ordering::Relaxed))
}

pub fn get_global_font_scale() -> f32 {
    f32::from_bits(GLOBAL_FONT_SCALE.load(Ordering::Relaxed))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct GameOptionData {
    pub _master_volume: f32,
    pub _bgm_volume: f32,
    pub _sfx_volume: f32,
    pub _is_mute: bool,
    pub _mouse_sensitivity: f32,
    pub _invert_mouse_y: bool,
    pub _r_stick_sensitivity: f32,
    pub _l_stick_sensitivity: f32,
    pub _font_scale: f32,
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
            _r_stick_sensitivity: DEFAULT_R_STICK_SENSITIVITY,
            _l_stick_sensitivity: DEFAULT_L_STICK_SENSITIVITY,
            _font_scale: DEFAULT_FONT_SCALE,
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

        GLOBAL_R_STICK_SENSITIVITY.store(self._r_stick_sensitivity.to_bits(), Ordering::Relaxed);
        GLOBAL_L_STICK_SENSITIVITY.store(self._l_stick_sensitivity.to_bits(), Ordering::Relaxed);
        GLOBAL_FONT_SCALE.store(self._font_scale.to_bits(), Ordering::Relaxed);

        set_global_dpi_scale(self._font_scale);
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

    pub fn set_r_stick_sensitivity(&mut self, sensitivity: f32) {
        self._r_stick_sensitivity = sensitivity.clamp(MIN_STICK_SENSITIVITY, MAX_STICK_SENSITIVITY);
    }

    pub fn set_l_stick_sensitivity(&mut self, sensitivity: f32) {
        self._l_stick_sensitivity = sensitivity.clamp(MIN_STICK_SENSITIVITY, MAX_STICK_SENSITIVITY);
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self._font_scale = scale.clamp(MIN_FONT_SCALE, MAX_FONT_SCALE);
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
        assert_eq!(default_option._r_stick_sensitivity, DEFAULT_R_STICK_SENSITIVITY);
        assert_eq!(default_option._l_stick_sensitivity, DEFAULT_L_STICK_SENSITIVITY);
        assert_eq!(default_option._font_scale, DEFAULT_FONT_SCALE);

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

        option.set_r_stick_sensitivity(0.01);
        assert_eq!(option._r_stick_sensitivity, MIN_STICK_SENSITIVITY);

        option.set_r_stick_sensitivity(10.0);
        assert_eq!(option._r_stick_sensitivity, MAX_STICK_SENSITIVITY);

        option.set_l_stick_sensitivity(0.01);
        assert_eq!(option._l_stick_sensitivity, MIN_STICK_SENSITIVITY);

        option.set_l_stick_sensitivity(10.0);
        assert_eq!(option._l_stick_sensitivity, MAX_STICK_SENSITIVITY);

        option.set_font_scale(0.1);
        assert_eq!(option._font_scale, MIN_FONT_SCALE);

        option.set_font_scale(5.0);
        assert_eq!(option._font_scale, MAX_FONT_SCALE);
    }
}
