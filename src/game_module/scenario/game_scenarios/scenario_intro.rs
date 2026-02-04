use crate::game_module::actors::character::Character;
use crate::game_module::game_constants::{AUDIO_ROOSTER, CAMERA_DISTANCE_MAX, CAMERA_OFFSET_Y, STORY_BOARD_FADE_TIME, STORY_IMAGE_NONE, TIME_OF_DAWN};
use crate::game_module::game_scene_manager::GameSceneManager;
use  crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::math;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_widget::QuestContent;
use crate::game_module::widgets::text_box_widget::TextBoxContent;

const INTRO_FADE_TIME: f32 = 2.0;
const SLEEP_PHASE_TIME: f32 = 5.0;
const WAKE_UP_PHASE_TIME: f32 = 6.0;

pub const STORY_BOARDS: [&str; 2] = ["ui/story_board/story_board_intro_00", "ui/story_board/story_board_intro_01"];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioIntroPhase {
    None,
    StoryBoard,
    Sleep,
    WakeUp,
    End
}

pub struct ScenarioIntro<'a> {
    pub _scenario_name: String,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,
    pub _wakeup_delay_aru: f32,
    pub _wakeup_delay_ewa: f32,
    pub _wakeup_delay_koa: f32,
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
            _wakeup_delay_aru: 2.0,
            _wakeup_delay_ewa: 3.5,
            _wakeup_delay_koa: 4.0,
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: Vector3::new(0.4, 0.0, 0.0),
            _around_end_rotation: Vector3::new(0.35, 0.0, 0.0),
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioIntroPhase::None,
                _phase_time: 0.0,
                _phase_duration: None,
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

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioIntroPhase::End
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>) {
        let next_scenario_phase = ScenarioIntroPhase::from_str(next_scenario_phase).unwrap();
        if next_scenario_phase != self._scenario_track._scenario_phase {
            self.update_game_scenario_end();
            self._scenario_track.set_scenario_phase(next_scenario_phase, phase_duration);
            self.update_game_scenario_begin();
        }
    }

    fn update_game_scenario_begin(&mut self) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);

        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::StoryBoard => {
                game_scene_manager.set_time_of_day(0.0, 0.0);
                self._actor_aru = if let Some(actor) = game_scene_manager.get_actor("aru") { Some(actor.clone()) } else { None };
                self._actor_ewa = if let Some(actor) = game_scene_manager.get_actor("ewa") { Some(actor.clone()) } else { None };
                self._actor_koa = if let Some(actor) = game_scene_manager.get_actor("koa") { Some(actor.clone()) } else { None };
                self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
            },
            ScenarioIntroPhase::Sleep => {
                self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Sleep);

                let pivot = self._actor_aru.as_ref().unwrap().borrow().get_center().clone() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                let start_rotation_matrix = math::make_rotation_matrix(self._around_start_rotation.x, self._around_start_rotation.y, self._around_start_rotation.z);
                self._around_start_position = pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

                let end_rotation_matrix = math::make_rotation_matrix(self._around_end_rotation.x, self._around_end_rotation.y, self._around_end_rotation.z);
                self._around_end_position = pivot - end_rotation_matrix.column(2).xyz() * CAMERA_DISTANCE_MAX;

                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);

                game_scene_manager.set_time_of_day(TIME_OF_DAWN, 0.0);
            },
            ScenarioIntroPhase::WakeUp => {
                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_ROOSTER);
            }
            _ => (),
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, game_ui_manager: &mut GameUIManager<'a>, any_key_hold: bool, any_key_pressed: bool, mut delta_time: f64) {
        if any_key_hold {
            delta_time *= 5.0;
        }

        let phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::None => {
                self.set_scenario_phase(ScenarioIntroPhase::StoryBoard.to_string().as_str(), None);
            }
            ScenarioIntroPhase::StoryBoard => {
                let story_board_phase = self.get_story_board_phase();
                if game_ui_manager.is_done_game_image_progress() && any_key_pressed {
                    const USE_STORY_BOARDS: bool = false;
                    if USE_STORY_BOARDS == false || STORY_BOARDS.len() <= story_board_phase {
                        game_ui_manager.set_image_manual_fade_inout(STORY_IMAGE_NONE, INTRO_FADE_TIME);
                        game_ui_manager.set_auto_fade_inout(true);
                        self.set_scenario_phase(ScenarioIntroPhase::Sleep.to_string().as_str(), Some(SLEEP_PHASE_TIME));
                    } else {
                        game_ui_manager.set_image_auto_fade_inout(&STORY_BOARDS[story_board_phase], STORY_BOARD_FADE_TIME);
                        self.next_story_board_phase();
                    }
                }
            }
            ScenarioIntroPhase::Sleep => {
                let game_scene_manager = ptr_as_ref(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                let progress = 1.0 - (phase_ratio * -5.0).exp2();
                let position = self._around_start_position.lerp(&self._around_end_position, progress);
                let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                main_camera._transform_object.set_position(&position);
                main_camera._transform_object.set_rotation(&rotation);

                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioIntroPhase::WakeUp.to_string().as_str(), Some(WAKE_UP_PHASE_TIME));
                }
            }
            ScenarioIntroPhase::WakeUp => {
                let prev_wakeup_delay_aru = self._wakeup_delay_aru;
                let prev_wakeup_delay_ewa = self._wakeup_delay_ewa;
                let prev_wakeup_delay_koa = self._wakeup_delay_koa;
                self._wakeup_delay_aru -= delta_time as f32;
                self._wakeup_delay_ewa -= delta_time as f32;
                self._wakeup_delay_koa -= delta_time as f32;

                if 0.0 <= prev_wakeup_delay_aru && self._wakeup_delay_aru < 0.0 {
                    //self._actor_aru.as_ref().unwrap().borrow_mut()._character_stats.set_hunger(0.8);
                    self._actor_aru.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
                }

                if 0.0 <= prev_wakeup_delay_ewa && self._wakeup_delay_ewa < 0.0 {
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
                }

                if 0.0 <= prev_wakeup_delay_koa && self._wakeup_delay_koa < 0.0 {
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::StandUp);
                }

                if self._actor_aru.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) {
                    self._actor_aru.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(-1.0, 0.0, 0.0), true);
                }

                if 20.0 < phase_time || self._actor_aru.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) && self._actor_ewa.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) && self._actor_koa.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) {
                    let material_instance = ptr_as_ref(self._game_scene_manager).get_game_resources().get_engine_resources().get_material_instance_data(
                        ItemDataType::get_item_material_instance_name(ItemDataType::Coconut)
                    ).clone();
                    game_ui_manager.add_text_box_item(
                        self._actor_ewa.as_ref().unwrap().borrow().get_character_name(),
                        &vec![TextBoxContent::MaterialInstance(material_instance)],
                        10.0
                    );

                    let material_instance = ptr_as_ref(self._game_scene_manager).get_game_resources().get_engine_resources().get_material_instance_data(
                        ItemDataType::get_item_material_instance_name(ItemDataType::Meat)
                    ).clone();

                    game_ui_manager.add_text_box_item(
                        self._actor_koa.as_ref().unwrap().borrow().get_character_name(),
                        &vec![TextBoxContent::MaterialInstance(material_instance)],
                        10.0
                    );

                    game_ui_manager.add_quest_item(QuestContent::GatherItem(GatherItemData {
                        _item_data_type: ItemDataType::Coconut,
                        _gather_item_count: 5,
                    }));

                    game_ui_manager.add_quest_item(QuestContent::GatherItem(GatherItemData {
                        _item_data_type: ItemDataType::Meat,
                        _gather_item_count: 5,
                    }));

                    self.set_scenario_phase(ScenarioIntroPhase::End.to_string().as_str(), None);
                }
            }
            _ => ()
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
