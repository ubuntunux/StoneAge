use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::TIME_OF_NOON;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const SLEEP_PHASE_TIME: f32 = 10.0;

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioIntroPhase {
    None,
    Intro,
    Tutorial,
    GameStart
}

pub struct ScenarioIntro<'a> {
    pub _scenario_name: String,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,
    pub _around_start_position: Vector3<f32>,
    pub _around_end_position: Vector3<f32>,
    pub _around_start_rotation: Vector3<f32>,
    pub _around_end_rotation: Vector3<f32>,
    pub _scenario_track: ScenarioTrack<ScenarioIntroPhase>
}

impl<'a> ScenarioIntro<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        scenario_name: &str,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> ScenarioIntro<'a> {
        ScenarioIntro {
            _scenario_name: String::from(scenario_name),
            _game_scene_manager: game_scene_manager.clone(),
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _around_start_position: Vector3::new(65.0, 33.25, -26.0),
            _around_end_position: Vector3::new(13.086, 13.4679, 10.657),
            _around_start_rotation: Vector3::new(0.37, -1.0, 0.0),
            _around_end_rotation: Vector3::new(0.7205, -0.002, 0.0),
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioIntroPhase::None,
                _phase_time: 0.0,
                _phase_duration: 0.0,
            },
        }
    }
}

impl<'a> ScenarioBase for ScenarioIntro<'a> {
    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioIntroPhase::GameStart
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: f32) {
        let next_scenario_phase = ScenarioIntroPhase::from_str(next_scenario_phase).unwrap();
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self._scenario_track._phase_time = 0.0;
            self._scenario_track._phase_duration = phase_duration;
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::Intro => {
                let game_scene_manager = ptr_as_mut(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);
                game_scene_manager.set_time_of_day(TIME_OF_NOON, 0.0);
                self._actor_aru = if let Some(actor) = game_scene_manager.get_actor("aru") {
                    Some(actor.clone())
                } else {
                    None
                };
                self._actor_ewa = if let Some(actor) = game_scene_manager.get_actor("ewa") {
                    Some(actor.clone())
                } else {
                    None
                };
                self._actor_koa = if let Some(actor) = game_scene_manager.get_actor("koa") {
                    Some(actor.clone())
                } else {
                    None
                };
            }
            ScenarioIntroPhase::Tutorial => {
                self._actor_aru.as_ref().unwrap().borrow_mut()._character_stats.set_hunger(0.8);
            }
            _ => (),
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, any_key_hold: bool, delta_time: f64) {
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::None => {
                self.set_scenario_phase(ScenarioIntroPhase::Intro.to_string().as_str(), SLEEP_PHASE_TIME);
            }
            ScenarioIntroPhase::Intro => {
                let game_scene_manager = ptr_as_mut(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                let progress = 1.0 - (phase_ratio * -5.0).exp2();
                let position = self._around_start_position.lerp(&self._around_end_position, progress);
                let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                main_camera._transform_object.set_position(&position);
                main_camera._transform_object.set_rotation(&rotation);

                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioIntroPhase::Tutorial.to_string().as_str(), 0.0);
                }
            }
            ScenarioIntroPhase::Tutorial => {
                self.set_scenario_phase(ScenarioIntroPhase::GameStart.to_string().as_str(), 0.0);
            }
            _ => (),
        }

        self._scenario_track.update_scenario_track(delta_time as f32 * if any_key_hold { 5.0 } else { 1.0 });
    }
}
