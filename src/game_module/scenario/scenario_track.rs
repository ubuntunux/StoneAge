use serde::de::StdError;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ScenarioTrackCreateInfo {
    pub _scenario_phase: String,
    pub _next_scenario_phase: String,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
}

pub struct ScenarioTrack<T: Copy + PartialEq + Hash + FromStr + ToString> {
    pub _scenario_phase: T,
    pub _next_scenario_phase: T,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
}

impl<T: Copy + PartialEq + Hash + FromStr + ToString> ScenarioTrack<T> {
    pub fn load_scenario_track_data(&mut self, scenario_track_data: &ScenarioTrackCreateInfo)
    where
        <T as FromStr>::Err: StdError,
    {
        self._scenario_phase = scenario_track_data._scenario_phase.parse::<T>().unwrap();
        self._next_scenario_phase = scenario_track_data._next_scenario_phase.parse::<T>().unwrap();
        self._phase_duration = scenario_track_data._phase_duration;
        self._next_phase_duration = scenario_track_data._next_phase_duration;
        self._phase_time = scenario_track_data._phase_time;
    }
    pub fn save_scenario_track_data(&self) -> ScenarioTrackCreateInfo {
        ScenarioTrackCreateInfo {
            _scenario_phase: self._scenario_phase.to_string(),
            _next_scenario_phase: self._next_scenario_phase.to_string(),
            _phase_duration: None,
            _next_phase_duration: None,
            _phase_time: 0.0,
        }
    }
    pub fn set_next_scenario_phase(&mut self, next_scenario_phase: T, next_phase_duration: Option<f32>) {
        self._next_scenario_phase = next_scenario_phase;
        self._next_phase_duration = next_phase_duration;
    }

    pub fn set_scenario_phase(&mut self, scenario_phase: T, phase_duration: Option<f32>) {
        self._scenario_phase = scenario_phase;
        self._phase_duration = phase_duration;
        self._phase_time = 0.0;
    }

    pub fn get_phase_ratio(&self) -> f32 {
        if let Some(phase_duration) = self._phase_duration.as_ref()
            && 0.0f32 < *phase_duration
        {
            return 0f32.max(1f32.min(self._phase_time / phase_duration));
        }
        0.0
    }

    pub fn get_phase_time(&self) -> f32 {
        self._phase_time
    }

    pub fn update_scenario_phase_time(&mut self, delta_time: f32) {
        self._phase_time += delta_time;
    }
}
