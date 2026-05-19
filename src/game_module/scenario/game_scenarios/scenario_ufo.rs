use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioUfoPhase {
    Begin,
    End,
}

pub struct ScenarioUfo<'a> {
    pub _scenario_type: ScenarioType,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,
    pub _scenario_track: ScenarioTrack<ScenarioUfoPhase>,
    pub _scenario_phase: usize,
}

impl<'a> ScenarioUfo<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioUfo<'a>> {
        newRcRefCell(ScenarioUfo {
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioUfoPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            },
            _scenario_phase: 0,
        })
    }
}

impl<'a> ScenarioUfo<'a> {
    pub fn get_scenario_phase(&self) -> usize {
        self._scenario_phase
    }
    pub fn clear_scenario_phase(&mut self) {
        self._scenario_phase = 0;
    }
    pub fn next_scenario_phase(&mut self) {
        self._scenario_phase += 1;
    }
}

impl<'a> ScenarioBase<'a> for ScenarioUfo<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioUfoPhase::End => {
                false
            }
            _ => true
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioUfoPhase::End
    }

    fn destroy_game_scenario(&mut self) {
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>) {
        let next_scenario_phase = ScenarioUfoPhase::from_str(next_scenario_phase).expect("scenario error");
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        let _game_scene_manager = ptr_as_mut(self._game_scene_manager);

        match self._scenario_track._scenario_phase {
            ScenarioUfoPhase::Begin => {}
            ScenarioUfoPhase::End => {}
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, any_key_hold: bool, _any_key_pressed: bool, mut delta_time: f64) {
        if any_key_hold {
            delta_time *= 5.0;
        }

        let _game_scene_manager = ptr_as_mut(self._game_scene_manager);

        let _phase_time = self._scenario_track.get_phase_time();
        let _phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioUfoPhase::Begin => {
            }
            ScenarioUfoPhase::End => {
            }
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
