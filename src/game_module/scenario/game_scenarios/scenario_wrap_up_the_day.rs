use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{GameSceneManager};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    Performance,
    Sleep,
    End,
}

pub struct ScenarioWrapUpTheDay<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
}

impl<'a> ScenarioWrapUpTheDay<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWrapUpTheDay<'a>> {
        newRcRefCell(ScenarioWrapUpTheDay {
            _is_load_completed: false,
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            }
        })
    }
}

impl<'a> ScenarioBase<'a> for ScenarioWrapUpTheDay<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_load_completed(&self) -> bool {
        self._is_load_completed
    }

    fn is_play_scenario_mode(&self) -> bool {
        true
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self._is_load_completed = false;
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        log::info!("TEST - on_open_game_scene: {:?}, name: {:?}", self.get_scenario_type(), _game_scene_data_name);

        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if let Some(actor) = game_scene_manager.get_actor("monkey_aru") {
            self._actor_aru = Some(actor.clone());
        } else if let Some(actor) = game_scene_manager.get_actor("aru") {
            self._actor_aru = Some(actor.clone());
        }

        if let Some(actor) = game_scene_manager.get_actor("monkey_ewa") {
            self._actor_ewa = Some(actor.clone());
        } else if let Some(actor) = game_scene_manager.get_actor("ewa") {
            self._actor_ewa = Some(actor.clone());
        }

        self._prop_table = Some(game_scene_manager.get_prop_manager().get_prop_by_name("table").unwrap().clone());
        self._prop_bed_for_aru = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").unwrap().clone());
        self._prop_bed_for_ewa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").unwrap().clone());
        self._prop_bed_for_koa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").unwrap().clone());

        self._is_load_completed = true;
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
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::Performance => {
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_laying_down();
            }
            ScenarioPhase::Sleep => {
                game_scene_manager.get_game_client_mut().set_next_game_phase(GamePhase::Sleep);
            }
            ScenarioPhase::End => {
                ptr_as_mut(self._game_scene_manager).get_game_client_mut().set_next_game_phase(GamePhase::Sleep);
            }
            _ => {}
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => ()
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        let _phase_time = self._scenario_track.get_phase_time();
        let _phase_ratio = self._scenario_track.get_phase_ratio();
        let current_scenario_phase = self._scenario_track._scenario_phase;
        match current_scenario_phase {
            ScenarioPhase::Begin => {
                self.set_scenario_phase(ScenarioPhase::Performance.to_string().as_str(), None);
            },
            ScenarioPhase::Performance => {
                if self._actor_aru.as_ref().unwrap().borrow().is_action(ActionAnimationState::Sleep) {
                    self.set_scenario_phase(ScenarioPhase::Sleep.to_string().as_str(), None);
                }
            },
            ScenarioPhase::Sleep => {
                self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
            },
            _ => {}
        }

        self._scenario_track.update_scenario_track(current_scenario_phase, delta_time as f32);
    }
}