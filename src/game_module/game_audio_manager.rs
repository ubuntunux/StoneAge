use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut};
use rust_engine_3d::utilities::system::RcRefCell;
use crate::game_module::game_constants::{
    AMBIENT_SOUND, AMBIENT_SOUND_NIGHT, AMBIENT_SOUND_RAIN, DEFAULT_BGM_VOLUME, GAME_MUSIC,
    TIME_OF_DAWN, TIME_OF_NIGHT,
};
use crate::game_module::game_service_locator::get_game_scene_manager;
use crate::game_module::game_weather::WeatherType;

pub struct GameAudioManager {
    pub _ambient_sound_name: String,
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>,
    pub _was_play_scenario_mode: bool,
    pub _bgm_volume: f32,
}

impl Default for GameAudioManager {
    fn default() -> Self {
        GameAudioManager {
            _ambient_sound_name: "".to_string(),
            _ambient_sound: None,
            _was_play_scenario_mode: false,
            _bgm_volume: DEFAULT_BGM_VOLUME.unwrap(),
        }
    }
}

impl GameAudioManager {
    pub fn play_ambient_sound(&mut self, audio_name: &str, volume: Option<f32>) {
        self.stop_ambient_sound();
        self._ambient_sound_name = audio_name.to_string();
        self._ambient_sound = get_audio_manager_mut().play_audio_bank(audio_name, AudioLoop::LOOP, volume);
    }

    pub fn stop_ambient_sound(&mut self) {
        if let Some(audio_instance_refcell) = self._ambient_sound.as_ref() {
            get_audio_manager_mut().stop_audio_instance(audio_instance_refcell);
        }
        self._ambient_sound_name.clear();
        self._ambient_sound = None;
    }

    pub fn update_game_sound(&mut self) {
        let game_scene_manager = get_game_scene_manager();
        let weather_type = game_scene_manager._weather.get_weather_type();
        let time_of_day = game_scene_manager.get_time_of_day();

        // control ambient sound
        let target_ambient_sound = if weather_type == WeatherType::Rain {
            AMBIENT_SOUND_RAIN
        } else if time_of_day >= TIME_OF_NIGHT || time_of_day < TIME_OF_DAWN {
            AMBIENT_SOUND_NIGHT
        } else {
            AMBIENT_SOUND
        };

        let is_playing = if let Some(ambient_sound) = self._ambient_sound.as_ref() {
            get_audio_manager().is_playing_audio_instance(ambient_sound)
        } else {
            false
        };

        if self._ambient_sound_name != target_ambient_sound || !is_playing {
            self.play_ambient_sound(target_ambient_sound, None);
        }

        // control bgm
        let audio_manager = get_audio_manager_mut();
        if target_ambient_sound == AMBIENT_SOUND_RAIN || target_ambient_sound == AMBIENT_SOUND_NIGHT {
            if audio_manager.is_playing_bgm() {
                audio_manager.stop_bgm();
            }
        } else if target_ambient_sound == AMBIENT_SOUND {
            if !audio_manager.is_playing_bgm() {
                audio_manager.play_bgm(GAME_MUSIC, Some(self._bgm_volume));
            }
        }

        // let is_play_scenario_mode = game_scene_manager.is_play_scenario_mode();
        // if is_play_scenario_mode != self._was_play_scenario_mode {
        //     self._bgm_volume = if is_play_scenario_mode {
        //         0.0
        //     } else {
        //         DEFAULT_BGM_VOLUME.unwrap()
        //     };
        //     audio_manager.set_bgm_volume(self._bgm_volume);
        //     self._was_play_scenario_mode = is_play_scenario_mode;
        // }
    }
}
