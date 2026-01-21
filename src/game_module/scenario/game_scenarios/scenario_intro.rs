use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::{STORY_BOARD_FADE_TIME, STORY_IMAGE_NONE, TIME_OF_NOON};
use crate::game_module::game_scene_manager::GameSceneManager;
use  crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{ptr_as_mut, RcRefCell};
use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use crate::game_module::behavior::behavior_base::BehaviorState;

const SLEEP_PHASE_TIME: f32 = 3.0;

pub const STORY_BOARDS: [&str; 2] = [
    "ui/story_board/story_board_intro_00",
    "ui/story_board/story_board_intro_01"
];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioIntroPhase {
    None,
    Start,
    StoryBoard,
    Zoomin,
    End
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
    pub _scenario_track: ScenarioTrack<ScenarioIntroPhase>,
    pub _story_board_phase: usize,
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
            _around_start_position: Vector3::new(0.91, 14.61, -35.0),
            _around_end_position: Vector3::new(0.91, 14.61, -20.0),
            _around_start_rotation: Vector3::new(0.37, 0.0, 0.0),
            _around_end_rotation: Vector3::new(0.35, 0.0, 0.0),
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioIntroPhase::None,
                _phase_time: 0.0,
                _phase_duration: 0.0,
            },
            _story_board_phase: 0,
        }
    }
}

impl<'a> ScenarioIntro<'a> {
    pub fn get_story_board_phase(&self) -> usize {
        self._story_board_phase
    }
    pub fn clear_story_board_phase(&mut self) {
        self._story_board_phase = 0;
    }
    pub fn next_story_board_phase(&mut self) {
        self._story_board_phase += 1;
    }
}

impl<'a> ScenarioBase for ScenarioIntro<'a> {
    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioIntroPhase::End
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: f32) {
        let next_scenario_phase = ScenarioIntroPhase::from_str(next_scenario_phase).unwrap();
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::Start => {
                let game_scene_manager = ptr_as_mut(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);
                game_scene_manager.set_time_of_day(TIME_OF_NOON, 0.0);
                self._actor_aru = if let Some(actor) = game_scene_manager.get_actor("aru") { Some(actor.clone()) } else { None };
                self._actor_ewa = if let Some(actor) = game_scene_manager.get_actor("ewa") { Some(actor.clone()) } else { None };
                self._actor_koa = if let Some(actor) = game_scene_manager.get_actor("koa") { Some(actor.clone()) } else { None };
                self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
            }
            ScenarioIntroPhase::End => {
                //self._actor_aru.as_ref().unwrap().borrow_mut()._character_stats.set_hunger(0.8);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
            }
            _ => (),
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, game_ui_manager: &mut GameUIManager, any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::None => {
                self.set_scenario_phase(ScenarioIntroPhase::Start.to_string().as_str(), 0.0);
            }
            ScenarioIntroPhase::Start => {
                self.set_scenario_phase(ScenarioIntroPhase::StoryBoard.to_string().as_str(), 0.0);
            }
            ScenarioIntroPhase::StoryBoard => {
                let story_board_phase = self.get_story_board_phase();
                if game_ui_manager.is_done_game_image_progress() && any_key_pressed {
                    if STORY_BOARDS.len() <= story_board_phase {
                        game_ui_manager.set_image_manual_fade_inout(STORY_IMAGE_NONE, STORY_BOARD_FADE_TIME);
                        game_ui_manager.set_auto_fade_inout(true);
                        self.set_scenario_phase(ScenarioIntroPhase::Zoomin.to_string().as_str(), SLEEP_PHASE_TIME);
                    } else {
                        game_ui_manager.set_image_auto_fade_inout(&STORY_BOARDS[story_board_phase], STORY_BOARD_FADE_TIME);
                        self.next_story_board_phase();
                    }
                }
            }
            ScenarioIntroPhase::Zoomin => {
                let game_scene_manager = ptr_as_mut(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                let progress = 1.0 - (phase_ratio * -5.0).exp2();
                let position = self._around_start_position.lerp(&self._around_end_position, progress);
                let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                main_camera._transform_object.set_position(&position);
                main_camera._transform_object.set_rotation(&rotation);

                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioIntroPhase::End.to_string().as_str(), 0.0);
                }
            }
            ScenarioIntroPhase::End => {
            }
        }

        self._scenario_track.update_scenario_track(delta_time as f32 * if any_key_hold { 5.0 } else { 1.0 });
    }
}
