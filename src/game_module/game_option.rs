use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::WindowMode;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_engine_core_mut, is_engine_core_valid};
use rust_engine_3d::scene::ui::set_global_font_scale;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

pub const DEFAULT_MASTER_VOLUME: f32 = 0.5;
pub const DEFAULT_BGM_VOLUME: f32 = 1.0;
pub const DEFAULT_SFX_VOLUME: f32 = 1.0;
pub const DEFAULT_IS_MUTE: bool = false;
pub const DEFAULT_CAMERA_ROTATION_SPEED: f32 = 1.0;
pub const DEFAULT_INVERT_MOUSE_Y: bool = false;
pub const DEFAULT_FONT_SCALE: f32 = 1.0;
pub const DEFAULT_WINDOW_MODE: WindowMode = WindowMode::FullScreenBorderlessMode;
pub const DEFAULT_MONITOR_INDEX: Option<usize> = None;
pub const DEFAULT_RESOLUTION_WIDTH: u32 = 1920;
pub const DEFAULT_RESOLUTION_HEIGHT: u32 = 1080;
pub const DEFAULT_RESOLUTION: Vector2<u32> = Vector2::new(DEFAULT_RESOLUTION_WIDTH, DEFAULT_RESOLUTION_HEIGHT);

pub const MIN_VOLUME: f32 = 0.0;
pub const MAX_VOLUME: f32 = 1.0;
pub const VOLUME_STEP: f32 = 0.1;

pub const MIN_CAMERA_ROTATION_SPEED: f32 = 0.2;
pub const MAX_CAMERA_ROTATION_SPEED: f32 = 3.0;
pub const CAMERA_ROTATION_SPEED_STEP: f32 = 0.1;

pub const MIN_FONT_SCALE: f32 = 0.7;
pub const MAX_FONT_SCALE: f32 = 1.5;
pub const FONT_SCALE_STEP: f32 = 0.1;

pub const MAX_AUDIO_MIXER_VOLUME: f32 = 128.0;

static GLOBAL_CAMERA_ROTATION_SPEED: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32
static GLOBAL_FONT_SCALE: AtomicU32 = AtomicU32::new(0x3f800000); // 1.0f32

pub fn get_global_camera_rotation_speed() -> f32 {
    f32::from_bits(GLOBAL_CAMERA_ROTATION_SPEED.load(Ordering::Relaxed))
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
    pub _camera_rotation_speed: f32,
    pub _invert_mouse_y: bool,
    pub _font_scale: f32,
    pub _window_mode: WindowMode,
    pub _monitor_index: Option<usize>,
    pub _resolution: Vector2<u32>,
}

impl Default for GameOptionData {
    fn default() -> Self {
        GameOptionData {
            _master_volume: DEFAULT_MASTER_VOLUME,
            _bgm_volume: DEFAULT_BGM_VOLUME,
            _sfx_volume: DEFAULT_SFX_VOLUME,
            _is_mute: DEFAULT_IS_MUTE,
            _camera_rotation_speed: DEFAULT_CAMERA_ROTATION_SPEED,
            _invert_mouse_y: DEFAULT_INVERT_MOUSE_Y,
            _font_scale: DEFAULT_FONT_SCALE,
            _window_mode: DEFAULT_WINDOW_MODE,
            _monitor_index: DEFAULT_MONITOR_INDEX,
            _resolution: DEFAULT_RESOLUTION,
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

        GLOBAL_CAMERA_ROTATION_SPEED.store(self._camera_rotation_speed.to_bits(), Ordering::Relaxed);
        GLOBAL_FONT_SCALE.store(self._font_scale.to_bits(), Ordering::Relaxed);

        set_global_font_scale(self._font_scale);

        if is_engine_core_valid() {
            let audio_manager = get_audio_manager_mut();
            audio_manager.set_bgm_volume(effective_bgm_volume);
            audio_manager._volume = (effective_sfx_volume * MAX_AUDIO_MIXER_VOLUME) as i32;

            let engine_core = get_engine_core_mut();
            engine_core.set_display_options(self._window_mode, self._monitor_index, self._resolution);
        }
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

    pub fn set_camera_rotation_speed(&mut self, speed: f32) {
        self._camera_rotation_speed = speed.clamp(MIN_CAMERA_ROTATION_SPEED, MAX_CAMERA_ROTATION_SPEED);
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self._font_scale = scale.clamp(MIN_FONT_SCALE, MAX_FONT_SCALE);
    }

    pub fn set_window_mode(&mut self, window_mode: WindowMode) {
        self._window_mode = window_mode;
    }

    pub fn set_monitor_index(&mut self, monitor_index: Option<usize>) {
        self._monitor_index = monitor_index;
    }

    pub fn set_resolution(&mut self, resolution: Vector2<u32>) {
        self._resolution = resolution;
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
        assert_eq!(default_option._camera_rotation_speed, DEFAULT_CAMERA_ROTATION_SPEED);
        assert_eq!(default_option._invert_mouse_y, DEFAULT_INVERT_MOUSE_Y);
        assert_eq!(default_option._font_scale, DEFAULT_FONT_SCALE);
        assert_eq!(default_option._window_mode, DEFAULT_WINDOW_MODE);
        assert_eq!(default_option._monitor_index, None);
        assert_eq!(default_option._resolution, DEFAULT_RESOLUTION);

        let json_str = serde_json::to_string_pretty(&default_option).expect("Serialization failed");
        let deserialized: GameOptionData = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(default_option, deserialized);
        assert_eq!(deserialized._monitor_index, None);

        let mut custom_option = default_option.clone();
        custom_option.set_monitor_index(Some(2));
        let custom_json = serde_json::to_string(&custom_option).expect("Serialization failed");
        let custom_deserialized: GameOptionData = serde_json::from_str(&custom_json).expect("Deserialization failed");
        assert_eq!(custom_deserialized._monitor_index, Some(2));
    }

    #[test]
    fn test_game_option_clamping() {
        let mut option = GameOptionData::default();
        option.set_master_volume(-0.5);
        assert_eq!(option._master_volume, MIN_VOLUME);

        option.set_master_volume(2.5);
        assert_eq!(option._master_volume, MAX_VOLUME);

        option.set_camera_rotation_speed(0.01);
        assert_eq!(option._camera_rotation_speed, MIN_CAMERA_ROTATION_SPEED);

        option.set_camera_rotation_speed(5.0);
        assert_eq!(option._camera_rotation_speed, MAX_CAMERA_ROTATION_SPEED);

        option.set_font_scale(0.1);
        assert_eq!(option._font_scale, MIN_FONT_SCALE);

        option.set_font_scale(5.0);
        assert_eq!(option._font_scale, MAX_FONT_SCALE);
    }
}
