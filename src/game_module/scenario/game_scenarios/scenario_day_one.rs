use std::str::FromStr;
use nalgebra::Vector3;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::actors::character::{Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_scene_manager::{GameSceneManager};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    ReleaseFamily,
    UfoGone,
    Awake,
    End,
}

pub struct ScenarioDayOne<'a> {
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_ufo: Option<RcRefCell<Character<'a>>>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>
}

impl<'a> ScenarioDayOne<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioDayOne<'a>> {
        newRcRefCell(ScenarioDayOne {
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_ufo: None,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
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

    pub fn update_ufo_movement(&self) {
        self._actor_ufo.as_ref().unwrap().borrow_mut().set_move(&Vector3::new(0.0, 0.0, -1.0));
    }

    pub fn update_release_actor(&mut self, actor: RcRefCell<Character>, target: RcRefCell<Prop>, delta_time: f64) -> bool {
        let to_target = target.borrow().get_position() - actor.borrow().get_position();
        let mut pos = actor.borrow().get_position().clone();
        let move_dist = 3.0f32 * delta_time as f32;
        let range = 0.1f32;
        if range < to_target.magnitude_squared() {
            pos += to_target.normalize() * move_dist;
            actor.borrow_mut().set_position(&pos);
            return false;
        }

        actor.borrow_mut().set_position(&pos);
        true
    }

    pub fn update_release_family(&mut self, delta_time: f64) -> bool {
        if self._actor_aru.is_none() {
            return false;
        }

        let arrived_aru = self.update_release_actor(self._actor_aru.as_ref().unwrap().clone(), self._prop_bed_for_aru.as_ref().unwrap().clone(), delta_time);
        let arrived_ewa = self.update_release_actor(self._actor_ewa.as_ref().unwrap().clone(), self._prop_bed_for_ewa.as_ref().unwrap().clone(), delta_time);
        let arrived_koa = self.update_release_actor(self._actor_koa.as_ref().unwrap().clone(), self._prop_bed_for_koa  .as_ref().unwrap().clone(), delta_time);
        arrived_aru && arrived_ewa && arrived_koa
    }
}

impl<'a> ScenarioBase<'a> for ScenarioDayOne<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_play_scenario_mode(&self) -> bool {
        self._scenario_track._scenario_phase != ScenarioPhase::End
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
        self._actor_ufo = Some(game_scene_manager.get_actor("ufo").unwrap().clone());
        self._actor_aru = Some(game_scene_manager.get_actor("aru").unwrap().clone());
        self._actor_ewa = Some(game_scene_manager.get_actor("ewa").unwrap().clone());
        self._actor_koa = Some(game_scene_manager.get_actor("koa").unwrap().clone());
        self._prop_bed_for_aru = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").unwrap().clone());
        self._prop_bed_for_ewa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").unwrap().clone());
        self._prop_bed_for_koa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").unwrap().clone());

        self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
        self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep();
        self._actor_aru.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep();
        self._actor_ewa.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(true);
        self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
        self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep();
        self._actor_koa.as_ref().unwrap().borrow_mut().set_position(self._actor_ufo.as_ref().unwrap().borrow().get_position());

        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
        main_camera._transform_object.set_position(&Vector3::new(13.48, 26.56, -5.02));
        main_camera._transform_object.set_rotation(&Vector3::new(0.76, 0.33, 0.0));
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
            ScenarioPhase::Awake => {
                game_scene_manager.get_character_manager_mut().remove_character(self._actor_ufo.as_ref().unwrap());
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_wake_up();
                self._actor_aru.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_wake_up();
                self._actor_ewa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_wake_up();
                self._actor_koa.as_ref().unwrap().borrow_mut()._controller.set_flying_mode(false);
            }
            ScenarioPhase::End => {
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

        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let _game_ui_manager = game_scene_manager.get_game_ui_manager_mut();

        let _phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Begin => {
                self.set_scenario_phase(ScenarioPhase::ReleaseFamily.to_string().as_str(), Some(6.0));
            },
            ScenarioPhase::ReleaseFamily => {
                let complete = self.update_release_family(delta_time);
                if complete {
                    self.set_scenario_phase(ScenarioPhase::UfoGone.to_string().as_str(), Some(3.0));
                }
            },
            ScenarioPhase::UfoGone => {
                self.update_ufo_movement();
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::Awake.to_string().as_str(), Some(3.0));
                }
            },
            ScenarioPhase::Awake => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
                }
            }
            ScenarioPhase::End => {}
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
