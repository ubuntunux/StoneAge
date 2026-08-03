use rust_engine_3d::audio::audio_manager::{AudioInstance, AudioLoop};
use rust_engine_3d::core::engine_service_locator::{get_audio_manager, get_audio_manager_mut};
use rust_engine_3d::utilities::system::RcRefCell;
use crate::game_module::game_constants::{AMBIENT_SOUND, DEFAULT_BGM_VOLUME, GAME_MUSIC};

#[derive(Default)]
pub struct GameAudioManager {
    pub _ambient_sound: Option<RcRefCell<AudioInstance>>,
}

impl GameAudioManager {
    pub fn play_ambient_sound(&mut self, audio_name: &str, volume: Option<f32>) {
        self.stop_ambient_sound();
        self._ambient_sound = get_audio_manager_mut().play_audio_bank(audio_name, AudioLoop::LOOP, volume);
    }

    pub fn stop_ambient_sound(&mut self) {
        if let Some(audio_instance_refcell) = self._ambient_sound.as_ref() {
            get_audio_manager_mut().stop_audio_instance(audio_instance_refcell);
        }
        self._ambient_sound = None;
    }

    pub fn update_game_sound(&mut self) {
        let audio_manager = get_audio_manager_mut();
        if !audio_manager.is_playing_bgm() {
            audio_manager.play_bgm(GAME_MUSIC, DEFAULT_BGM_VOLUME);
        }

        if let Some(ambient_sound) = self._ambient_sound.as_ref() {
            if !get_audio_manager().is_playing_audio_instance(ambient_sound) {
                self.play_ambient_sound(AMBIENT_SOUND, None);
            }
        }

        //get_game_scene_manager_mut().play_ambient_sound(AMBIENT_SOUND_RAIN, Some(2.0));
    }
}
