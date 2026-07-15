use std::hash::Hash;

pub struct ScenarioTrack<T: Copy + PartialEq + Hash> {
    pub _scenario_phase: T,
    pub _next_scenario_phase: T,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
}

impl<T: Copy + PartialEq + Hash> ScenarioTrack<T> {
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
