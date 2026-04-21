use std::ffi::c_void;
use nalgebra::Vector3;
use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::utilities::math;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::{GameUIManager, QuestItem};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack};
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::QuestCreateInfo;
use crate::game_module::widgets::text_box_widget::TextBoxContent;

const SKIP_SCENARIO: bool = true;
const USE_STORY_BOARDS: bool = false;
const INTRO_FADE_TIME: f32 = 2.0;
const PHASE_TIME_SLEEP: f32 = 5.0;
const PHASE_TIME_HUNGRY: f32 = 3.0;

pub const STORY_BOARDS: [&str; 2] = ["ui/story_board/story_board_intro_00", "ui/story_board/story_board_intro_01"];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioIntroPhase {
    None,
    StoryBoard,
    Sleep,
    WakeUp,
    Assemble,
    Hungry,
    End,
    QuestGathering,
}

pub struct ScenarioIntro<'a> {
    pub _scenario_name: String,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,
    pub _prop_tree: Option<RcRefCell<Prop<'a>>>,
    pub _quest: Option<RcRefCell<QuestTitle<'a>>>,
    pub _quest_gather_food: Option<QuestItem<'a>>,
    pub _was_completed_quest_gather_food: bool,
    pub _quest_return_home: Option<QuestItem<'a>>,
    pub _wakeup_delay_aru: f32,
    pub _wakeup_delay_ewa: f32,
    pub _wakeup_delay_koa: f32,
    pub _camera_direction: Vector3<f32>,
    pub _camera_distance: f32,
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
            _game_scene_manager: game_scene_manager,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_tree: None,
            _quest: None,
            _quest_gather_food: None,
            _was_completed_quest_gather_food: false,
            _quest_return_home: None,
            _wakeup_delay_aru: 2.0,
            _wakeup_delay_ewa: 3.5,
            _wakeup_delay_koa: 4.0,
            _camera_direction: Default::default(),
            _camera_distance: 0.0,
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

    pub fn update_assemble(&self, actor: &RcRefCell<Character<'a>>, target: &RcRefCell<Character<'a>>) -> bool {
        let radius = target.borrow().get_collision()._bounding_box._mag_xz + 0.5;
        let (direction, dist) = math::make_normalize_with_norm(&(target.borrow().get_position() - actor.borrow().get_position()));
        if radius < dist {
            actor.borrow_mut().set_move(&direction);
            return false;
        }
        true
    }

    pub fn emoji_hungry(&self, game_ui_manager: &mut GameUIManager<'a>, actor: &RcRefCell<Character<'a>>) {
        let contents = vec![TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY))];
        game_ui_manager.add_text_box_item(
            ActorWrapper::Character(actor.clone()),
            &contents,
            Some( CHARACTER_INTERACTION_TIME )
        );
        actor.borrow_mut().set_move_idle();
    }
}

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn is_play_scenario_mode(&self) -> bool {
        self._scenario_track._scenario_phase != ScenarioIntroPhase::QuestGathering && self._scenario_track._scenario_phase != ScenarioIntroPhase::End
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioIntroPhase::End
    }

    fn set_scenario_phase(&mut self, next_scenario_phase: &str, phase_duration: Option<f32>) {
        let next_scenario_phase = ScenarioIntroPhase::from_str(next_scenario_phase).expect("scenario error");
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
                game_scene_manager.set_time_of_day(TIME_OF_MORNING, 0.0);
                self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
                self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
                self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
                self._prop_tree = Some(game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00.001").unwrap().clone());
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);

                game_scene_manager.get_game_ui_manager_mut().add_item(ITEM_HAND, 1);
            },
            ScenarioIntroPhase::Sleep => {
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep();
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep();
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep();

                let pivot = self._actor_aru.as_ref().unwrap().borrow().get_center().clone() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                let start_rotation_matrix = math::make_rotation_matrix(self._around_start_rotation.x, self._around_start_rotation.y, self._around_start_rotation.z);
                self._around_start_position = pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

                let end_rotation_matrix = math::make_rotation_matrix(self._around_end_rotation.x, self._around_end_rotation.y, self._around_end_rotation.z);
                self._around_end_position = pivot - end_rotation_matrix.column(2).xyz() * CAMERA_DISTANCE_MIN;

                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);
            },
            ScenarioIntroPhase::WakeUp => {
                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_ROOSTER);
            }
            ScenarioIntroPhase::Assemble => {
            }
            ScenarioIntroPhase::Hungry => {
                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_STOMACH_GROWLING);

                let direction = math::make_normalize_xz(&(self._actor_koa.as_ref().unwrap().borrow().get_position() - self._actor_aru.as_ref().unwrap().borrow().get_position()));
                self._actor_aru.as_ref().unwrap().borrow_mut().look_at(&direction);

                let direction = math::make_normalize_xz(&(self._actor_aru.as_ref().unwrap().borrow().get_position() - self._actor_ewa.as_ref().unwrap().borrow().get_position()));
                self._actor_ewa.as_ref().unwrap().borrow_mut().look_at(&direction);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_hungry();

                let direction = math::make_normalize_xz(&(self._actor_aru.as_ref().unwrap().borrow().get_position() - self._actor_koa.as_ref().unwrap().borrow().get_position()));
                self._actor_koa.as_ref().unwrap().borrow_mut().look_at(&direction);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_hungry();
            }
            ScenarioIntroPhase::QuestGathering => {
                // self._actor_ewa.as_ref().unwrap().borrow_mut().set_sit_down();
                // self._actor_koa.as_ref().unwrap().borrow_mut().set_sit_down();
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);

                // quest
                let item_coconut = game_scene_manager.get_game_resources().get_item_data(ITEM_COCONUT);
                self._quest = Some(game_scene_manager.get_game_ui_manager_mut().add_quest(Some(String::from("Gather food for the hungry family."))));
                self._quest_gather_food = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(GatherItemData {
                    _item_data_name: String::from(ITEM_COCONUT),
                    _item_data: item_coconut.clone(),
                    _gather_item_count: 3,
                })));
                self._quest_return_home = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Return home.")),
                })));

                // text box
                let contents = vec![TextBoxContent::Text(String::from("\"Hit this tree to get food.\""))];
                game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                    ActorWrapper::Prop(self._prop_tree.as_ref().unwrap().clone()),
                    &contents,
                    None
                );
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

        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        if TIME_OF_MORNING <= game_scene_manager.get_time_of_day() {
            game_scene_manager.set_time_of_day_speed(1.0);
        }

        let _phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::None => {
                self.set_scenario_phase(ScenarioIntroPhase::StoryBoard.to_string().as_str(), None);
            }
            ScenarioIntroPhase::StoryBoard => {
                if SKIP_SCENARIO {
                    game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, INTRO_FADE_TIME);
                    game_ui_manager.set_auto_fade_inout(true);
                    self.set_scenario_phase(ScenarioIntroPhase::Hungry.to_string().as_str(), Some(1.0));
                } else {
                    let story_board_phase = self.get_story_board_phase();
                    if game_ui_manager.is_done_game_image_progress() && any_key_pressed {
                        if USE_STORY_BOARDS == false || STORY_BOARDS.len() <= story_board_phase {
                            game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, INTRO_FADE_TIME);
                            game_ui_manager.set_auto_fade_inout(true);
                            self.set_scenario_phase(ScenarioIntroPhase::Sleep.to_string().as_str(), Some(PHASE_TIME_SLEEP));
                        } else {
                            game_ui_manager.set_image_auto_fade_inout(&STORY_BOARDS[story_board_phase], STORY_BOARD_FADE_TIME);
                            self.next_story_board_phase();
                        }
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
                    self.set_scenario_phase(ScenarioIntroPhase::WakeUp.to_string().as_str(), None);
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
                    self._actor_aru.as_ref().unwrap().borrow_mut().set_action_wake_up();
                }

                if 0.0 <= prev_wakeup_delay_ewa && self._wakeup_delay_ewa < 0.0 {
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_wake_up();
                }

                if 0.0 <= prev_wakeup_delay_koa && self._wakeup_delay_koa < 0.0 {
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_action_wake_up();
                }

                if self._wakeup_delay_koa < 0.0 &&
                    self._wakeup_delay_ewa < 0.0 &&
                    self._wakeup_delay_aru < 0.0 &&
                    self._actor_aru.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) &&
                    self._actor_ewa.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) &&
                    self._actor_koa.as_ref().unwrap().borrow_mut().is_action(ActionAnimationState::None) {
                    self.set_scenario_phase(ScenarioIntroPhase::Assemble.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::Assemble => {
                let mut done = true;

                if self.update_assemble(self._actor_ewa.as_ref().unwrap(), self._actor_aru.as_ref().unwrap()) {
                    self.emoji_hungry(game_ui_manager, self._actor_ewa.as_ref().unwrap())
                } else {
                    done = false;
                }

                if self.update_assemble(self._actor_koa.as_ref().unwrap(), self._actor_aru.as_ref().unwrap()) {
                    self.emoji_hungry(game_ui_manager, self._actor_koa.as_ref().unwrap())
                } else {
                    done = false;
                }

                if done {
                    self.set_scenario_phase(ScenarioIntroPhase::Hungry.to_string().as_str(), Some(PHASE_TIME_HUNGRY));
                }
            }
            ScenarioIntroPhase::Hungry => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioIntroPhase::QuestGathering.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::QuestGathering => {
                let completed_quest_gather_food = if let Some(quest) = &self._quest_gather_food {
                    quest.borrow().is_completed_quest()
                } else {
                    false
                };

                if self._was_completed_quest_gather_food == false && completed_quest_gather_food {
                    game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(self._prop_tree.as_ref().unwrap().as_ptr() as *const c_void);

                    let contents = vec![TextBoxContent::Text(String::from("\"Return home.\""))];
                    game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                        ActorWrapper::Character(self._actor_ewa.as_ref().unwrap().clone()),
                        &contents,
                        None
                    );
                    self._was_completed_quest_gather_food = true;
                }

                let mut completed_quest_return_home = false;
                if completed_quest_gather_food {
                    let to_ewa = math::get_norm_xz(&(self._actor_ewa.as_ref().unwrap().borrow().get_position() - self._actor_aru.as_ref().unwrap().borrow().get_position()));
                    if to_ewa < (self._actor_ewa.as_ref().unwrap().borrow().get_collision()._bounding_box._mag_xz + CHARACTER_INTERACTION_DISTANCE) {
                        game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(self._actor_ewa.as_ref().unwrap().as_ptr() as *const c_void);
                        self._quest_return_home.as_ref().unwrap().borrow_mut().set_completed_quest();
                        completed_quest_return_home = true;
                    };
                }

                if completed_quest_gather_food && completed_quest_return_home {
                    self.set_scenario_phase(ScenarioIntroPhase::End.to_string().as_str(), None);
                }
            }
            _ => ()
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
