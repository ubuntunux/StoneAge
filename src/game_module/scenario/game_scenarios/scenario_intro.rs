use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use crate::game_module::actors::character::Character;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::TIME_OF_MORNING;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioIntroPhase {
    None,
    Sleep,
}

pub struct ScenarioIntro<'a> {
    pub _scenario_name: String,
    pub _scenario_phase: ScenarioIntroPhase,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,

}

impl<'a> ScenarioIntro<'a> {
    pub fn create_game_scenario(game_scene_manager: *const GameSceneManager<'a>, scenario_name: &str, _scenario_create_info: &ScenarioDataCreateInfo) -> ScenarioIntro<'a> {
        ScenarioIntro {
            _scenario_name: String::from(scenario_name),
            _scenario_phase: ScenarioIntroPhase::None,
            _game_scene_manager: game_scene_manager.clone(),
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
        }
    }
}

impl<'a> ScenarioBase for ScenarioIntro<'a> {
    fn set_scenario_data(&mut self, next_scenario_phase: &str) {
        let next_scenario_phase = ScenarioIntroPhase::from_str(next_scenario_phase).unwrap();
        if next_scenario_phase != self._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_phase = next_scenario_phase;
            self.update_game_scenario_start();
        }
    }

    fn update_game_scenario_start(&mut self) {
        match self._scenario_phase {
            ScenarioIntroPhase::Sleep => {
                let game_scene_manager = ptr_as_mut(self._game_scene_manager);
                game_scene_manager.set_time_of_day(TIME_OF_MORNING, 0.0);
                self._actor_aru = if let Some(actor) = game_scene_manager.get_actor("aru") { Some(actor.clone()) } else { None };
                self._actor_ewa = if let Some(actor) = game_scene_manager.get_actor("ewa") { Some(actor.clone()) } else { None };
                self._actor_koa = if let Some(actor) = game_scene_manager.get_actor("koa") { Some(actor.clone()) } else { None };
                self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
            },
            _ => ()
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_phase {
            _ => ()
        }
    }

    fn update_game_scenario(&mut self, _delta_time: f64) {
        match self._scenario_phase {
            ScenarioIntroPhase::None => {
                self.set_scenario_data(ScenarioIntroPhase::Sleep.to_string().as_str())
            },
            ScenarioIntroPhase::Sleep => {
            }
        }
    }
}