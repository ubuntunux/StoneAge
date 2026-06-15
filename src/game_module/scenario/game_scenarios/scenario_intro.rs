use std::str::FromStr;
use nalgebra::Vector3;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::utilities::math;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::{GameUIManager, QuestItem};
use crate::game_module::scenario::scenario::{ScenarioBase, ScenarioDataCreateInfo, ScenarioTrack, ScenarioType};
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_resource::GameResources;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::QuestCreateInfo;
use crate::game_module::widgets::text_box_widget::TextBoxContent;

const SKIP_SCENARIO: bool = false;
const USE_STORY_BOARDS: bool = false;
const PHASE_TIME_SLEEP: f32 = 5.0;
const PHASE_TIME_HUNGRY: f32 = 3.0;

pub const STORY_BOARDS: [&str; 2] = ["ui/story_board/story_board_intro_00", "ui/story_board/story_board_intro_01"];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    Begin,
    StoryBoard,
    Morning,
    WakeUp,
    AssembleFamily,
    IamHungry,
    MoveToTutorialStage,
    GatheringFood,
    BackHome,
    WrapUpTheDay,
    Sleeping,
    End,
}

pub struct ScenarioIntro<'a> {
    _is_load_completed: bool,
    _scenario_type: ScenarioType,
    _game_scene_manager: *const GameSceneManager<'a>,
    _actor_aru: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_gate: Option<RcRefCell<Prop<'a>>>,
    _prop_gate_stage01: Option<RcRefCell<Prop<'a>>>,
    _prop_tree: Option<RcRefCell<Prop<'a>>>,
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _quest: Option<RcRefCell<QuestTitle<'a>>>,
    _sub_quest_move_to_tutorial_stage: Option<QuestItem<'a>>,
    _sub_quest_gather_food: Option<QuestItem<'a>>,
    _sub_quest_back_home: Option<QuestItem<'a>>,
    _sub_quest_sleep: Option<QuestItem<'a>>,
    _was_completed_sub_quest_gather_food: bool,
    _wakeup_delay_aru: f32,
    _wakeup_delay_ewa: f32,
    _wakeup_delay_koa: f32,
    _camera_direction: Vector3<f32>,
    _camera_distance: f32,
    _around_start_position: Vector3<f32>,
    _around_end_position: Vector3<f32>,
    _around_start_rotation: Vector3<f32>,
    _around_end_rotation: Vector3<f32>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
    _story_board_phase: usize,
}

impl<'a> ScenarioIntro<'a> {
    pub fn create_game_scenario(
        game_scene_manager: *const GameSceneManager<'a>,
        _game_resources: *const GameResources<'a>,
        scenario_type: ScenarioType,
        _scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioIntro<'a>> {
        newRcRefCell(ScenarioIntro {
            _is_load_completed: false,
            _scenario_type: scenario_type,
            _game_scene_manager: game_scene_manager,
            _actor_aru: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_gate: None,
            _prop_gate_stage01: None,
            _prop_tree: None,
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _quest: None,
            _sub_quest_move_to_tutorial_stage: None,
            _sub_quest_gather_food: None,
            _sub_quest_back_home: None,
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
                _scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
            },
            _story_board_phase: 0,
        })
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
        let (direction, dist) = math::make_normalize_xz_with_norm(&(target.borrow().get_position() - actor.borrow().get_position()));
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
            Some(CHARACTER_INTERACTION_TIME)
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

    pub fn remove_give_food_to_ewa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn remove_give_food_to_koa_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(actor) = self._actor_koa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_wrap_up_the_day_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_table.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from("\"Wrap up the day.\""))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                None
            );
        }
    }

    pub fn remove_wrap_up_the_day_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_table.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn clear_all(&mut self, game_scene_manager: &GameSceneManager<'a>) {
        self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
        self.remove_hit_this_tree_text_box(game_scene_manager);
        self.remove_return_home_text_box(game_scene_manager);
        self.remove_wrap_up_the_day_text_box(game_scene_manager);

        self._actor_aru = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_gate = None;
        self._prop_gate_stage01 = None;
        self._prop_tree = None;
        self._prop_bed_for_aru = None;
        self._prop_bed_for_ewa = None;
        self._prop_bed_for_koa = None;

        self._is_load_completed = false;
    }

    pub fn continue_scenario_phase(&mut self) {
        self.set_scenario_phase(ScenarioPhase::End.to_string().as_str(), None);
    }
}

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn is_load_completed(&self) -> bool {
        self._is_load_completed
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::MoveToTutorialStage |
            ScenarioPhase::GatheringFood |
            ScenarioPhase::BackHome |
            ScenarioPhase::WrapUpTheDay => false,
            _ => true
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        if let Some(quest) = &self._quest {
            quest.borrow_mut().destroy();
        }
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        self.clear_all(game_scene_manager);
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        if game_scene_data_name == Stages::Home.get_stage_data_name() {
            self._actor_aru = Some(game_scene_manager.get_actor("monkey_aru").unwrap().clone());
            self._actor_ewa = Some(game_scene_manager.get_actor("monkey_ewa").unwrap().clone());
            self._actor_koa = Some(game_scene_manager.get_actor("monkey_koa").unwrap().clone());
            self._prop_gate = Some(game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).unwrap().clone());
            self._prop_table = Some(game_scene_manager.get_prop_manager().get_prop_by_name("table").unwrap().clone());
            self._prop_bed_for_aru = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").unwrap().clone());
            self._prop_bed_for_ewa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").unwrap().clone());
            self._prop_bed_for_koa = Some(game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").unwrap().clone());
        } else if game_scene_data_name == Stages::Forest.get_stage_data_name() {
            self._prop_tree = Some(game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00").unwrap().clone());
            self._prop_gate_stage01 = Some(game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).unwrap().clone());
        }

        // update quest & text box
        match self._scenario_track._scenario_phase {
            ScenarioPhase::Begin => {
                let pivot = self._actor_aru.as_ref().unwrap().borrow().get_center().clone() + Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                let start_rotation_matrix = math::make_rotation_matrix(self._around_start_rotation.x, self._around_start_rotation.y, self._around_start_rotation.z);
                self._around_start_position = pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

                let end_rotation_matrix = math::make_rotation_matrix(self._around_end_rotation.x, self._around_end_rotation.y, self._around_end_rotation.z);
                self._around_end_position = pivot - end_rotation_matrix.column(2).xyz() * CAMERA_DISTANCE_MIN;

                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);

                self.set_scenario_phase(ScenarioPhase::StoryBoard.to_string().as_str(), None);
            }
            ScenarioPhase::MoveToTutorialStage => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                }
            }
            ScenarioPhase::GatheringFood => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    if self._sub_quest_gather_food.as_ref().unwrap().borrow_mut().is_completed_quest() == false {
                        self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                    }
                } else if game_scene_data_name == Stages::Forest.get_stage_data_name() {
                    self.create_hit_this_tree_text_box(game_scene_manager);
                }
            }
            ScenarioPhase::BackHome => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    self._actor_ewa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                    self._actor_koa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                } else if game_scene_data_name == Stages::Forest.get_stage_data_name() {
                    if self._sub_quest_back_home.as_ref().unwrap().borrow().is_completed_quest() == false {
                        self.create_return_home_text_box(game_scene_manager);
                    }
                }
            }
            _ => ()
        }

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
        let game_ui_manager = ptr_as_mut(game_scene_manager._game_ui_manager);

        match self._scenario_track._scenario_phase {
            ScenarioPhase::StoryBoard => {
                game_scene_manager.set_time_of_day(TIME_OF_DAWN, 0.0);

                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior_none();
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior_none();
                self._actor_aru.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);

                game_ui_manager.add_item(ITEM_HAND, 1);
            },
            ScenarioPhase::Morning => {
                self._actor_aru.as_ref().unwrap().borrow_mut().set_action_sleep();
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_sleep();
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_sleep();
            },
            ScenarioPhase::WakeUp => {
                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_ROOSTER);
            }
            ScenarioPhase::AssembleFamily => {
            }
            ScenarioPhase::IamHungry => {
                game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_STOMACH_GROWLING);

                self._actor_aru.as_ref().unwrap().borrow_mut().look_at(self._actor_koa.as_ref().unwrap().borrow().get_position());

                self._actor_ewa.as_ref().unwrap().borrow_mut().look_at(self._actor_aru.as_ref().unwrap().borrow().get_position());
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_action_hungry();

                self._actor_koa.as_ref().unwrap().borrow_mut().look_at(self._actor_aru.as_ref().unwrap().borrow().get_position());
                self._actor_koa.as_ref().unwrap().borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_action_hungry();
            }
            ScenarioPhase::MoveToTutorialStage => {
                // set idle
                self._actor_ewa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);
                self._actor_koa.as_ref().unwrap().borrow_mut().set_behavior(BehaviorState::Idle, None, true);

                // start quest
                let item_coconut = game_scene_manager.get_game_resources().get_item_data(ITEM_COCONUT);
                self._quest = Some(game_ui_manager.add_quest(Some(String::from("Gather food for the hungry family."))));
                self._sub_quest_move_to_tutorial_stage = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Move to the FOREST to find food.")),
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
                self._sub_quest_sleep = Some(self._quest.as_ref().unwrap().borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from("Wrap up the day.")),
                })));

                //
                if SKIP_SCENARIO {
                    self.set_scenario_phase(ScenarioPhase::WrapUpTheDay.to_string().as_str(), None);
                } else {
                    self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                }
            }
            ScenarioPhase::GatheringFood => {
                self.create_hit_this_tree_text_box(game_scene_manager);
            }
            ScenarioPhase::WrapUpTheDay => {
                self.create_wrap_up_the_day_text_box(game_scene_manager);
            }
            ScenarioPhase::End => {
                self.clear_all(game_scene_manager);

                game_scene_manager.open_game_scenario(ScenarioType::ScenarioUfo);
            }
            _ => ()
        }
    }

    fn update_game_scenario_end(&mut self) {
        match self._scenario_track._scenario_phase {
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let game_ui_manager = ptr_as_mut(game_scene_manager._game_ui_manager);

        if TIME_OF_MORNING <= game_scene_manager.get_time_of_day() {
            game_scene_manager.set_time_of_day_speed(1.0);
        }

        let phase_time = self._scenario_track.get_phase_time();
        let phase_ratio = self._scenario_track.get_phase_ratio();
        let current_scenario_phase = self._scenario_track._scenario_phase;
        match current_scenario_phase {
            ScenarioPhase::Begin => {
            }
            ScenarioPhase::StoryBoard => {
                if SKIP_SCENARIO {
                    game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                    game_ui_manager.set_auto_fade_inout(true);
                    self.set_scenario_phase(ScenarioPhase::MoveToTutorialStage.to_string().as_str(), None);
                } else {
                    let story_board_phase = self.get_story_board_phase();
                    if game_ui_manager.is_done_game_image_progress() && any_key_pressed {
                        if USE_STORY_BOARDS == false || STORY_BOARDS.len() <= story_board_phase {
                            game_ui_manager.set_image_manual_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                            game_ui_manager.set_auto_fade_inout(true);
                            self.set_scenario_phase(ScenarioPhase::Morning.to_string().as_str(), Some(PHASE_TIME_SLEEP));
                        } else {
                            game_ui_manager.set_image_auto_fade_inout(&STORY_BOARDS[story_board_phase], DEFAULT_FADE_TIME);
                            self.next_story_board_phase();
                        }
                    }
                }
            }
            ScenarioPhase::Morning => {
                let game_scene_manager = ptr_as_ref(self._game_scene_manager);
                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                let progress = 1.0 - (phase_ratio * -5.0).exp2();
                let position = self._around_start_position.lerp(&self._around_end_position, progress);
                let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                main_camera._transform_object.set_position(&position);
                main_camera._transform_object.set_rotation(&rotation);

                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::WakeUp.to_string().as_str(), None);
                }
            }
            ScenarioPhase::WakeUp => {
                let time_of_day_ratio = phase_time * 0.2;
                if time_of_day_ratio < 1.0 {
                    game_scene_manager.set_time_of_day(math::lerp(TIME_OF_DAWN, TIME_OF_EARLY_MORNING, time_of_day_ratio), 0.0);
                }

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
                    self.set_scenario_phase(ScenarioPhase::AssembleFamily.to_string().as_str(), None);
                }
            }
            ScenarioPhase::AssembleFamily => {
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
                    self.set_scenario_phase(ScenarioPhase::IamHungry.to_string().as_str(), Some(PHASE_TIME_HUNGRY));
                }
            }
            ScenarioPhase::IamHungry => {
                if 1.0 <= phase_ratio {
                    self.set_scenario_phase(ScenarioPhase::MoveToTutorialStage.to_string().as_str(), None);
                }
            }
            ScenarioPhase::MoveToTutorialStage => {
                if game_scene_manager.get_current_game_scene_data_name() == Stages::Forest.get_stage_data_name() {
                    self._sub_quest_move_to_tutorial_stage.as_ref().unwrap().borrow_mut().set_completed_quest();
                    self.set_scenario_phase(ScenarioPhase::GatheringFood.to_string().as_str(), None);
                }
            }
            ScenarioPhase::GatheringFood => {
                if game_scene_manager.get_current_game_scene_data_name() == Stages::Forest.get_stage_data_name() {
                    if self._sub_quest_gather_food.as_ref().unwrap().borrow().is_completed_quest() {
                        self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
                        self.remove_hit_this_tree_text_box(game_scene_manager);
                        self.create_return_home_text_box(game_scene_manager);
                        self.set_scenario_phase(ScenarioPhase::BackHome.to_string().as_str(), None);
                    }
                }
            }
            ScenarioPhase::BackHome => {
                if game_scene_manager.get_current_game_scene_data_name() == Stages::Home.get_stage_data_name() {
                    self._sub_quest_back_home.as_ref().unwrap().borrow_mut().set_completed_quest();
                    self.remove_return_home_text_box(game_scene_manager);
                    self.set_scenario_phase(ScenarioPhase::WrapUpTheDay.to_string().as_str(), None);
                }
            }
            ScenarioPhase::WrapUpTheDay => {
                // wait...
            }
            _ => ()
        }

        let mut completed_scenario = false;
        if game_scene_manager.get_current_game_scene_data_name() == Stages::Home.get_stage_data_name() {
            if SKIP_SCENARIO || self._sub_quest_sleep.is_some() && self._sub_quest_sleep.as_ref().unwrap().borrow().is_completed_quest() == false {
                if game_scene_manager.has_game_scenario(ScenarioType::ScenarioWrapUpTheDay) {
                    self._sub_quest_sleep.as_ref().unwrap().borrow_mut().set_completed_quest();
                    self.clear_all(game_scene_manager);
                    completed_scenario = true;
                }
            }

            if let Some(quest) = &self._quest {
                if quest.borrow().is_completed_quest() || completed_scenario {
                    self.set_scenario_phase(ScenarioPhase::Sleeping.to_string().as_str(), None);
                }
            };
        }

        self._scenario_track.update_scenario_track(current_scenario_phase, delta_time as f32);
    }
}
