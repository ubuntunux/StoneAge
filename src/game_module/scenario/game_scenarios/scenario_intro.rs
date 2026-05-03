use std::ffi::c_void;
use std::str::FromStr;
use nalgebra::Vector3;
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
    Morning,
    WakeUp,
    AssembleFamily,
    IamHungry,
    MoveToTutorialStage,
    GatheringFood,
    BackHome,
    GiveFood,
    Sleep,
    End,
}

pub struct ScenarioIntro<'a> {
    pub _scenario_name: String,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _actor_aru: Option<RcRefCell<Character<'a>>>,
    pub _actor_ewa: Option<RcRefCell<Character<'a>>>,
    pub _actor_koa: Option<RcRefCell<Character<'a>>>,
    pub _prop_gate: Option<RcRefCell<Prop<'a>>>,
    pub _prop_gate_stage01: Option<RcRefCell<Prop<'a>>>,
    pub _prop_tree: Option<RcRefCell<Prop<'a>>>,
    pub _prop_bed: Option<RcRefCell<Prop<'a>>>,
    pub _quest: Option<RcRefCell<QuestTitle<'a>>>,
    pub _sub_quest_move_to_tutorial_stage: Option<QuestItem<'a>>,
    pub _sub_quest_gather_food: Option<QuestItem<'a>>,
    pub _sub_quest_back_home: Option<QuestItem<'a>>,
    pub _sub_quest_give_food_to_ewa: Option<QuestItem<'a>>,
    pub _sub_quest_give_food_to_koa: Option<QuestItem<'a>>,
    pub _sub_quest_sleep: Option<QuestItem<'a>>,
    pub _was_completed_sub_quest_gather_food: bool,
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
            _prop_gate: None,
            _prop_gate_stage01: None,
            _prop_tree: None,
            _prop_bed: None,
            _quest: None,
            _sub_quest_move_to_tutorial_stage: None,
            _sub_quest_gather_food: None,
            _sub_quest_back_home: None,
            _sub_quest_give_food_to_ewa: None,
            _sub_quest_give_food_to_koa: None,
            _sub_quest_sleep: None,
            _was_completed_sub_quest_gather_food: false,
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

    pub fn create_move_to_tutorial_stage_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_gate.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Move to the Forest to find Food!\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_move_to_tutorial_stage_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_gate.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn create_hit_this_tree_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop_tree) = self._prop_tree.as_ref() {
            let actor_wrapper = ActorWrapper::Prop(prop_tree.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Hit this tree to get food.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                actor_wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_hit_this_tree_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop_tree) = self._prop_tree.as_ref() {
            let actor_wrapper = ActorWrapper::Prop(prop_tree.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_return_home_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_gate_stage01.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Return home.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_return_home_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_gate_stage01.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn create_give_food_to_ewa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Give food to ewa.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                actor_wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_give_food_to_ewa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_give_food_to_koa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_koa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Give food to koa.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                actor_wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_give_food_to_koa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_koa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_take_a_sleep_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_bed.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Take a sleep.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_take_a_sleep_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_bed.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::MoveToTutorialStage |
            ScenarioIntroPhase::GatheringFood |
            ScenarioIntroPhase::BackHome |
            ScenarioIntroPhase::GiveFood |
            ScenarioIntroPhase::Sleep |
            ScenarioIntroPhase::End => {
                false
            }
            _ => {
                true
            }
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioIntroPhase::End
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);

        self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
        self.remove_hit_this_tree_text_box(game_scene_manager);
        self.remove_return_home_text_box(game_scene_manager);
        self.remove_give_food_to_ewa_text_box(game_scene_manager);
        self.remove_give_food_to_koa_text_box(game_scene_manager);
        self.remove_take_a_sleep_text_box(game_scene_manager);

        self._actor_aru = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_gate = None;
        self._prop_gate_stage01 = None;
        self._prop_tree = None;
        self._prop_bed = None;
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if game_scene_data_name == STAGE_INTRO_STAGE {
            self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
            self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
            self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
            self._prop_gate = Some(game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).unwrap().clone());
            self._prop_bed = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed").unwrap().clone());
        } else if game_scene_data_name == STAGE_01 {
            self._prop_tree = Some(game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00").unwrap().clone());
            self._prop_gate_stage01 = Some(game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).unwrap().clone());
        }

        // update quest & text box
        match self._scenario_track._scenario_phase {
            ScenarioIntroPhase::MoveToTutorialStage => {
                if game_scene_data_name == STAGE_INTRO_STAGE {
                    self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                }
            }
            ScenarioIntroPhase::GatheringFood => {
                if game_scene_data_name == STAGE_INTRO_STAGE {
                    if self._sub_quest_gather_food.as_ref().unwrap().borrow_mut().is_completed_quest() == false {
                        self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                    }
                } else if game_scene_data_name == STAGE_01 {
                    self.create_hit_this_tree_text_box(game_scene_manager);
                }
            }
            ScenarioIntroPhase::BackHome => {
                if game_scene_data_name == STAGE_INTRO_STAGE {
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                } else if game_scene_data_name == STAGE_01 {
                    if self._sub_quest_back_home.as_ref().unwrap().borrow().is_completed_quest() == false {
                        self.create_return_home_text_box(game_scene_manager);
                    }
                }
            }
            _ => ()
        }
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
            ScenarioIntroPhase::None => {}
            ScenarioIntroPhase::StoryBoard => {
                game_scene_manager.set_time_of_day(TIME_OF_MORNING, 0.0);

                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::None, None, true);
                self._actor_aru.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);

                game_scene_manager.get_game_ui_manager_mut().add_item(ITEM_HAND, 1);
            },
            ScenarioIntroPhase::Morning => {
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
            ScenarioIntroPhase::AssembleFamily => {
            }
            ScenarioIntroPhase::IamHungry => {
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
            ScenarioIntroPhase::MoveToTutorialStage => {
                // set idle
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);

                // start quest
                let item_coconut = game_scene_manager.get_game_resources().get_item_data(ITEM_COCONUT);
                self._quest = Some(game_scene_manager.get_game_ui_manager_mut().add_quest(Some(String::from("Gather food for the hungry family."))));
                self._sub_quest_move_to_tutorial_stage = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Move to tutorial stage.")),
                })));
                self._sub_quest_gather_food = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(GatherItemData {
                    _item_data_name: String::from(ITEM_COCONUT),
                    _item_data: item_coconut.clone(),
                    _gather_item_count: 3,
                })));
                self._sub_quest_back_home = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Return home.")),
                })));
                self._sub_quest_give_food_to_ewa = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Give food to ewa.")),
                })));
                self._sub_quest_give_food_to_koa = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Give food to koa.")),
                })));
                self._sub_quest_sleep = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Take a sleep.")),
                })));

                //
                self.create_move_to_tutorial_stage_text_box(game_scene_manager);
            }
            ScenarioIntroPhase::GatheringFood => {
                self.create_hit_this_tree_text_box(game_scene_manager);
            }
            ScenarioIntroPhase::BackHome => {
            }
            ScenarioIntroPhase::GiveFood => {
                self.create_give_food_to_ewa_text_box(game_scene_manager);
                self.create_give_food_to_koa_text_box(game_scene_manager);
            }
            ScenarioIntroPhase::Sleep => {
                self.create_take_a_sleep_text_box(game_scene_manager);
            }
            ScenarioIntroPhase::End => {}
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
                    self.set_scenario_phase(ScenarioIntroPhase::IamHungry.to_string().as_str(), Some(1.0));
                } else {
                    let story_board_phase = self.get_story_board_phase();
                    if game_ui_manager.is_done_game_image_progress() && any_key_pressed {
                        if USE_STORY_BOARDS == false || STORY_BOARDS.len() <= story_board_phase {
                            game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, INTRO_FADE_TIME);
                            game_ui_manager.set_auto_fade_inout(true);
                            self.set_scenario_phase(ScenarioIntroPhase::Morning.to_string().as_str(), Some(PHASE_TIME_SLEEP));
                        } else {
                            game_ui_manager.set_image_auto_fade_inout(&STORY_BOARDS[story_board_phase], STORY_BOARD_FADE_TIME);
                            self.next_story_board_phase();
                        }
                    }
                }
            }
            ScenarioIntroPhase::Morning => {
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
                    self.set_scenario_phase(ScenarioIntroPhase::AssembleFamily.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::AssembleFamily => {
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
                    self.set_scenario_phase(ScenarioIntroPhase::IamHungry.to_string().as_str(), Some(PHASE_TIME_HUNGRY));
                }
            }
            ScenarioIntroPhase::IamHungry => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioIntroPhase::MoveToTutorialStage.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::MoveToTutorialStage => {
                if game_scene_manager.get_current_game_scene_data_name() == STAGE_01 {
                    self._sub_quest_move_to_tutorial_stage.as_ref().unwrap().borrow_mut().set_completed_quest();
                    self.set_scenario_phase(ScenarioIntroPhase::GatheringFood.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::GatheringFood => {
                if game_scene_manager.get_current_game_scene_data_name() == STAGE_01 {
                    if self._sub_quest_gather_food.as_ref().unwrap().borrow().is_completed_quest() {
                        self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
                        self.remove_hit_this_tree_text_box(game_scene_manager);
                        self.create_return_home_text_box(game_scene_manager);
                        self.set_scenario_phase(ScenarioIntroPhase::BackHome.to_string().as_str(), None);
                    }
                }
            }
            ScenarioIntroPhase::BackHome => {
                if game_scene_manager.get_current_game_scene_data_name() == STAGE_INTRO_STAGE {
                    self._sub_quest_back_home.as_ref().unwrap().borrow_mut().set_completed_quest();
                    self.remove_return_home_text_box(game_scene_manager);
                    self.set_scenario_phase(ScenarioIntroPhase::GiveFood.to_string().as_str(), None);
                }
            }
            ScenarioIntroPhase::GiveFood => {
                if game_scene_manager.get_current_game_scene_data_name() == STAGE_INTRO_STAGE {
                    let mut sub_quest_give_food_to_ewa = self._sub_quest_give_food_to_ewa.as_ref().unwrap().borrow_mut().is_completed_quest();
                    if self._sub_quest_give_food_to_ewa.as_ref().unwrap().borrow().is_completed_quest() == false {
                        if self._actor_ewa.as_ref().unwrap().borrow().get_stats().is_hungry() == false {
                            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(self._actor_ewa.as_ref().unwrap().as_ptr() as *const c_void);
                            self._sub_quest_give_food_to_ewa.as_ref().unwrap().borrow_mut().set_completed_quest();
                            self.remove_give_food_to_ewa_text_box(game_scene_manager);
                            sub_quest_give_food_to_ewa = true;
                        };
                    }

                    let mut sub_quest_give_food_to_koa = self._sub_quest_give_food_to_koa.as_ref().unwrap().borrow_mut().is_completed_quest();
                    if self._sub_quest_give_food_to_koa.as_ref().unwrap().borrow().is_completed_quest() == false {
                        if self._actor_koa.as_ref().unwrap().borrow().get_stats().is_hungry() == false {
                            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(self._actor_koa.as_ref().unwrap().as_ptr() as *const c_void);
                            self._sub_quest_give_food_to_koa.as_ref().unwrap().borrow_mut().set_completed_quest();
                            self.remove_give_food_to_koa_text_box(game_scene_manager);
                            sub_quest_give_food_to_koa = true;
                        };
                    }

                    if sub_quest_give_food_to_ewa && sub_quest_give_food_to_koa {
                        self.set_scenario_phase(ScenarioIntroPhase::Sleep.to_string().as_str(), None);
                    }
                }
            }
            ScenarioIntroPhase::Sleep => {
                if game_scene_manager.get_current_game_scene_data_name() == STAGE_INTRO_STAGE {
                    if self._sub_quest_sleep.as_ref().unwrap().borrow().is_completed_quest() == false {
                        if self._actor_aru.as_ref().unwrap().borrow().is_action(ActionAnimationState::LayingDown) ||
                            self._actor_aru.as_ref().unwrap().borrow().is_action(ActionAnimationState::Sleep) {
                            self._sub_quest_sleep.as_ref().unwrap().borrow_mut().set_completed_quest();
                            self.remove_take_a_sleep_text_box(game_scene_manager);
                        }
                    }

                    if let Some(quest) = &self._quest {
                        if quest.borrow().is_completed_quest() {
                            self.set_scenario_phase(ScenarioIntroPhase::End.to_string().as_str(), None);
                        }
                    };
                }
            }
            ScenarioIntroPhase::End => {
            }
        }

        self._scenario_track.update_scenario_track(delta_time as f32);
    }
}
