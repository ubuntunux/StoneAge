use crate::game_module::game_constants::{EFFECT_RAIN, AMBIENT_SOUND_THUNDER};
use crate::game_module::game_service_locator::get_character_manager;
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::{get_scene_manager, get_scene_manager_mut, get_audio_manager_mut};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::utilities::system::State;
use std::cmp::PartialEq;
use strum::IntoEnumIterator;

#[derive(PartialEq, Copy, Clone)]
pub enum WeatherType {
    None,
    Rain,
}

pub struct Weather {
    pub _next_weather_type: WeatherType,
    pub _weather_type: WeatherType,
    pub _rain_effect: Option<uuid::Uuid>,
    pub _sun_light_color: Vector3<f32>,
    pub _rainy_light_color: Vector3<f32>,
    pub _thunder_light_color: Vector3<f32>,
    pub _thunder_timer: f32,
    pub _thunder_fade_timer: f32,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            _next_weather_type: WeatherType::None,
            _weather_type: WeatherType::None,
            _rain_effect: None,
            _sun_light_color: Vector3::new(5.0, 5.0, 5.0),
            _rainy_light_color: Vector3::new(0.2, 0.2, 0.2),
            _thunder_light_color: Vector3::new(20.0, 20.0, 20.0),
            _thunder_timer: 0.0,
            _thunder_fade_timer: 0.0,
        }
    }
}

impl Weather {
    pub fn get_weather_type(&self) -> WeatherType {
        self._weather_type
    }

    pub fn set_next_weather(&mut self, weather_type: WeatherType) {
        self._next_weather_type = weather_type;
    }

    pub fn clear_weather(&mut self) {
        if let Some(rain_effect_id) = self._rain_effect.take() {
            if let Some(effect) = get_scene_manager().get_effect(rain_effect_id) {
                effect.borrow_mut().set_dead();
            }
            get_scene_manager().get_main_light().borrow_mut()._light_data._light_color = self._sun_light_color.clone();
            self._weather_type = WeatherType::None;
        }
    }

    pub fn update_weather(&mut self, _delta_time: f64) {
        let prev_weather_type = self._weather_type;
        let next_weather_type = self._next_weather_type;
        for state in State::iter() {
            if prev_weather_type == next_weather_type && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_weather_type: WeatherType = match state {
                State::End => prev_weather_type,
                State::Begin => {
                    self._weather_type = next_weather_type;
                    next_weather_type
                }
                State::Update => next_weather_type,
            };
            match update_weather_type {
                WeatherType::None => match state {
                    State::Begin => {
                        self.clear_weather();
                    }
                    _ => {}
                },
                WeatherType::Rain => match state {
                    State::Begin => {
                        let effect_create_info = EffectCreateInfo {
                            _effect_data_name: String::from(EFFECT_RAIN),
                            ..Default::default()
                        };
                        self._rain_effect = Some(get_scene_manager_mut().add_effect(EFFECT_RAIN, &effect_create_info));
                        get_scene_manager().get_main_light().borrow_mut()._light_data._light_color = self._rainy_light_color.clone();
                        self._thunder_timer = rand::random_range(5.0..=10.0);
                        self._thunder_fade_timer = 0.0;
                    }
                    State::Update => {
                        if let Some(player) = get_character_manager().get_maybe_player() {
                            if let Some(rain_effect_id) = &self._rain_effect {
                                if let Some(effect) = get_scene_manager().get_effect(*rain_effect_id) {
                                    effect.borrow_mut()._effect_transform.set_position(&player.borrow().get_center());
                                }
                            }
                        }

                        const THUNDER_FADE_TIME: f32 = 1.5;
                        self._thunder_timer -= _delta_time as f32;
                        if self._thunder_timer <= 0.0 {
                            self._thunder_timer = rand::random_range(5.0..=60.0);
                            self._thunder_fade_timer = THUNDER_FADE_TIME;
                            get_scene_manager().get_main_light().borrow_mut()._light_data._light_color =
                                self._thunder_light_color.clone();
                            get_audio_manager_mut().play_audio_bank(
                                AMBIENT_SOUND_THUNDER,
                                AudioLoop::ONCE,
                                None,
                            );
                        }

                        if self._thunder_fade_timer > 0.0 {
                            self._thunder_fade_timer -= _delta_time as f32;
                            let t = (1.0 - self._thunder_fade_timer / THUNDER_FADE_TIME).clamp(0.0, 1.0);
                            let current_color = self._thunder_light_color.lerp(&self._rainy_light_color, t);
                            get_scene_manager().get_main_light().borrow_mut()._light_data._light_color = current_color;
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
