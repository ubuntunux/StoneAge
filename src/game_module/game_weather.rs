use crate::game_module::game_constants::{
    AMBIENT_SOUND, AMBIENT_SOUND_RAIN, DEFAULT_BGM_VOLUME, EFFECT_RAIN, GAME_MUSIC,
};
use crate::game_module::game_service_locator::get_game_scene_manager_mut;
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager, get_scene_manager_mut};
use rust_engine_3d::effect::effect_data::EffectCreateInfo;

pub struct Weather {
    pub _is_rain: bool,
    pub _rain_effect: Option<uuid::Uuid>,
    pub _sun_light_color: Vector3<f32>,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            _is_rain: false,
            _rain_effect: None,
            _sun_light_color: Vector3::new(5.0, 5.0, 5.0),
        }
    }
}

impl Weather {
    pub fn is_weather_rainy(&self) -> bool {
        self._is_rain
    }

    pub fn set_weather_rainy(&mut self, rainy: bool) {
        self._is_rain = rainy;
    }

    pub fn clear_weather_rain(&mut self) {
        if let Some(rain_effect_id) = self._rain_effect.take() {
            if let Some(effect) = get_scene_manager().get_effect(rain_effect_id) {
                effect.borrow_mut().set_dead();
            }
            get_audio_manager_mut().play_bgm(GAME_MUSIC, DEFAULT_BGM_VOLUME);
            get_game_scene_manager_mut().play_ambient_sound(AMBIENT_SOUND, None);
            get_scene_manager().get_main_light().borrow_mut()._light_data._light_color = self._sun_light_color.clone();
            self._is_rain = false;
        }
    }

    pub fn update_weather(&mut self, _delta_time: f64) {
        if self._is_rain {
            if self._rain_effect.is_none() {
                let effect_create_info = EffectCreateInfo {
                    _effect_data_name: String::from(EFFECT_RAIN),
                    ..Default::default()
                };
                let effect_id = get_scene_manager_mut().add_effect(EFFECT_RAIN, &effect_create_info);
                self._rain_effect = Some(effect_id);

                get_audio_manager_mut().stop_bgm();
                get_game_scene_manager_mut().play_ambient_sound(AMBIENT_SOUND_RAIN, None);
                self._sun_light_color =
                    get_scene_manager().get_main_light().borrow_mut()._light_data._light_color.clone();
                get_scene_manager().get_main_light().borrow_mut()._light_data._light_color =
                    Vector3::new(0.1, 0.1, 0.1);
            }

            if let Some(rain_effect_id) = &self._rain_effect {
                let camera_pos = get_scene_manager().get_main_camera()._transform_object.get_position().clone();
                if let Some(effect) = get_scene_manager().get_effect(*rain_effect_id) {
                    effect.borrow_mut()._effect_transform.set_position(&camera_pos);
                }
            }
        } else {
            self.clear_weather_rain();
        }
    }
}
