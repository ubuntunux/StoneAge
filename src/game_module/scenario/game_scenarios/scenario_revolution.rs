use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_scene_manager::{GameSceneManager};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioPhase {
    Begin,
    Loop,
    End,
}

pub struct ScenarioRevolution<'a> {
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
}

impl<'a> ScenarioRevolution<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioRevolution<'a>> {
        newRcRefCell(ScenarioRevolution {
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            }
        })
    }
}

impl<'a> ScenarioBase<'a> for ScenarioRevolution<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Begin |
            ScenarioPhase::Loop => true,
            _ => false
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
        self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
        self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>) {
        let next_scenario_phase = ScenarioPhase::from_str(next_scenario_phase).expect("scenario error");
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        let _game_scene_manager = ptr_as_mut(self._game_scene_manager);

        match self._scenario_track._scenario_phase {
            _ => (),
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
            ScenarioPhase::Begin => {
                if self._actor_aru.is_some() {
                    self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep();
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep();
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep();
                    self.set_scenario_phase(ScenarioPhase::Loop.to_string().as_ref(), None);
                }
            }
            ScenarioPhase::Loop =>{
            }
            ScenarioPhase::End => {
            }
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
