use crate::game_module::game_constants::EFFECT_RAIN;
use crate::game_module::game_service_locator::get_character_manager;
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::{get_scene_manager, get_scene_manager_mut};
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
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            _next_weather_type: WeatherType::None,
            _weather_type: WeatherType::None,
            _rain_effect: None,
            _sun_light_color: Vector3::new(5.0, 5.0, 5.0),
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
                        get_scene_manager().get_main_light().borrow_mut()._light_data._light_color =
                            Vector3::new(0.1, 0.1, 0.1);
                    }
                    State::Update => {
                        if let Some(player) = get_character_manager().get_maybe_player() {
                            if let Some(rain_effect_id) = &self._rain_effect {
                                if let Some(effect) = get_scene_manager().get_effect(*rain_effect_id) {
                                    effect.borrow_mut()._effect_transform.set_position(&player.borrow().get_center());
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
