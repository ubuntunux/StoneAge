use std::str::FromStr;
use nalgebra::Vector3;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::game_scene_manager::{GameSceneManager, Stages};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    AppearUfo,
    UfoLongShot,
    End,
}

pub struct ScenarioUfo<'a> {
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
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
            _actor_ufo: None,
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

impl<'a> ScenarioBase<'a> for ScenarioUfo<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Begin |
            ScenarioPhase::AppearUfo |
            ScenarioPhase::UfoLongShot => true,
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

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if game_scene_data_name == Stages::Home.get_stage_data_name() {
            self._actor_ufo = Some(game_scene_manager.get_actor("ufo").unwrap().clone());
            self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
            self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
            self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
        }
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
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);

        match self._scenario_track._scenario_phase {
            ScenarioPhase::UfoLongShot => {
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
                main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));
            }
            ScenarioPhase::End => {
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_wake_up();
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_wake_up();
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_wake_up();
            }
            _ => {}
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => ()
        }
    }

    fn update_game_scenario(&mut self, any_key_hold: bool, _any_key_pressed: bool, mut delta_time: f64) {
        if any_key_hold {
            delta_time *= 5.0;
        }

        let _game_scene_manager = ptr_as_mut(self._game_scene_manager);

        let _phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Begin => {
                self.set_scenario_phase(ScenarioPhase::AppearUfo.to_string().as_str(), Some(6.0));
            },
            ScenarioPhase::AppearUfo => {
                let actor = self._actor_ufo.as_ref().unwrap();
                let target = self._actor_aru.as_ref().unwrap();
                let direction = (target.borrow().get_position() - actor.borrow().get_position()).normalize();
                actor.borrow_mut().set_move(&direction);
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::UfoLongShot.to_string().as_str(), Some(5.0));
                }
            },
            ScenarioPhase::UfoLongShot => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
                }
            }
            _ => ()
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
