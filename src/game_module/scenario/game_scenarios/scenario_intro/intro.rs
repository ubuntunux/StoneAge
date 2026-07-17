use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::*;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_ui_manager::{GameUIManager, QuestItem};
use crate::game_module::scenario::game_scenarios::scenario_wrap_up_the_day::ScenarioWrapUpTheDay;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::QuestCreateInfo;
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxLayerType};
use nalgebra::Vector3;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const USE_STORY_BOARDS: bool = false;
const PHASE_TIME_SLEEP: f32 = 5.0;
const PHASE_TIME_HUNGRY: f32 = 3.0;

pub const STORY_BOARDS: [&str; 2] = [
    "ui/story_board/story_board_intro_00",
    "ui/story_board/story_board_intro_01",
];

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
enum ScenarioPhase {
    None,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ScenarioIntroSaveData {
    pub _scenario_create_info: ScenarioDataCreateInfo,
    pub _scenario_phase: String,
    pub _next_scenario_phase: String,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
    pub _story_board_phase: usize,
    pub _wakeup_delay_aru: f32,
    pub _wakeup_delay_ewa: f32,
    pub _wakeup_delay_koa: f32,
}

pub struct ScenarioIntro<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
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
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioIntro<'a>> {
        newRcRefCell(ScenarioIntro {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
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
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_time: 0.0,
                _phase_duration: None,
                _next_phase_duration: None,
            },
            _story_board_phase: 0,
        })
    }

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
        let (direction, dist) =
            math::make_normalize_xz_with_norm(&(target.borrow().get_position() - actor.borrow().get_position()));
        if radius < dist {
            actor.borrow_mut().set_move(&direction);
            return false;
        }
        true
    }

    pub fn emoji_hungry(&self, game_ui_manager: &mut GameUIManager<'a>, actor: &RcRefCell<Character<'a>>) {
        let contents = vec![TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY))];
        game_ui_manager.add_text_box_item(
            TextBoxLayerType::InteractionLayer,
            ActorWrapper::Character(actor.clone()),
            &contents,
            Some(CHARACTER_INTERACTION_TIME),
        );
        actor.borrow_mut().set_move_idle();
    }

    pub fn create_move_to_tutorial_stage_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_gate.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::Text(String::from(
                "\"Move to the Forest to find Food!\"",
            ))];
            game_scene_manager.get_game_ui_manager_mut().add_text_box_item(
                TextBoxLayerType::GamePlayLayer,
                wrapper,
                &contents,
                None,
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
                TextBoxLayerType::GamePlayLayer,
                actor_wrapper,
                &contents,
                None,
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
                TextBoxLayerType::GamePlayLayer,
                wrapper,
                &contents,
                None,
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
                TextBoxLayerType::GamePlayLayer,
                wrapper,
                &contents,
                None,
            );
        }
    }

    pub fn remove_wrap_up_the_day_text_box(&self, game_scene_manager: &GameSceneManager<'a>) {
        if let Some(prop) = self._prop_table.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            game_scene_manager.get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn clear_all(&mut self) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
        self.remove_hit_this_tree_text_box(game_scene_manager);
        self.remove_return_home_text_box(game_scene_manager);
        self.remove_give_food_to_ewa_text_box(game_scene_manager);
        self.remove_give_food_to_koa_text_box(game_scene_manager);
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
    }
}

impl<'a> ScenarioBase<'a> for ScenarioIntro<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        self._scenario_type
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        self._scenario_track._scenario_phase =
            ScenarioPhase::from_str(scenario_phase.as_str()).unwrap_or(ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = self._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);
        if let Ok(data) = serde_json::from_str::<ScenarioIntroSaveData>(&scenario_save_data._scenario_data) {
            self._story_board_phase = data._story_board_phase;
            self._wakeup_delay_aru = data._wakeup_delay_aru;
            self._wakeup_delay_ewa = data._wakeup_delay_ewa;
            self._wakeup_delay_koa = data._wakeup_delay_koa;
        }
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let save_data = ScenarioIntroSaveData {
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_phase: self._scenario_track._scenario_phase.to_string(),
            _next_scenario_phase: self._scenario_track._next_scenario_phase.to_string(),
            _phase_duration: self._scenario_track._phase_duration,
            _next_phase_duration: self._scenario_track._next_phase_duration,
            _phase_time: self._scenario_track._phase_time,
            _story_board_phase: self._story_board_phase,
            _wakeup_delay_aru: self._wakeup_delay_aru,
            _wakeup_delay_ewa: self._wakeup_delay_ewa,
            _wakeup_delay_koa: self._wakeup_delay_koa,
        };

        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: self._scenario_track.save_scenario_track_data(),
            _scenario_data: serde_json::to_string(&save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::MoveToTutorialStage
            | ScenarioPhase::GatheringFood
            | ScenarioPhase::BackHome
            | ScenarioPhase::WrapUpTheDay => false,
            _ => true,
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        self.clear_all();

        if let Some(quest) = &self._quest {
            quest.borrow_mut().destroy();
        }
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.clear_all();
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }

        self._actor_aru = game_scene_manager.get_actor_by_name("monkey_aru").cloned();
        self._actor_ewa = game_scene_manager.get_actor_by_name("monkey_ewa").cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name("monkey_koa").cloned();
        self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name("table").cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_aru").cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
        self._prop_tree = game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00").cloned();
        self._prop_gate_stage01 = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

        // update quest & text box
        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin => {
                let mut pivot = Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                if let Some(actor) = self._actor_aru.as_ref() {
                    pivot += *actor.borrow().get_center();
                };
                let start_rotation_matrix = math::make_rotation_matrix(
                    self._around_start_rotation.x,
                    self._around_start_rotation.y,
                    self._around_start_rotation.z,
                );
                self._around_start_position =
                    pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + 6.0);

                let end_rotation_matrix = math::make_rotation_matrix(
                    self._around_end_rotation.x,
                    self._around_end_rotation.y,
                    self._around_end_rotation.z,
                );
                self._around_end_position = pivot - end_rotation_matrix.column(2).xyz() * CAMERA_DISTANCE_MIN;

                let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);

                self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
            }
            ScenarioPhase::MoveToTutorialStage => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                }
            }
            ScenarioPhase::GatheringFood => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    if let Some(quest) = &self._sub_quest_gather_food {
                        if !quest.borrow_mut().is_completed_quest() {
                            self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                        }
                    }
                } else if game_scene_data_name == Stages::Forest.get_stage_data_name() {
                    self.create_hit_this_tree_text_box(game_scene_manager);
                }
            }
            ScenarioPhase::BackHome => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    if let Some(actor) = &self._actor_ewa {
                        actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                    }
                    if let Some(actor) = &self._actor_koa {
                        actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                    }
                } else if game_scene_data_name == Stages::Forest.get_stage_data_name() {
                    let back_home_not_completed =
                        self._sub_quest_back_home.as_ref().map_or(true, |q| !q.borrow().is_completed_quest());
                    if back_home_not_completed {
                        self.create_return_home_text_box(game_scene_manager);
                    }
                }
            }
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = ptr_as_mut(self._game_scene_manager);
        let game_ui_manager = ptr_as_mut(game_scene_manager._game_ui_manager);

        if TIME_OF_MORNING <= game_scene_manager.get_time_of_day() {
            game_scene_manager.set_time_of_day_speed(1.0);
        }

        let prev_scenario_phase = self._scenario_track._scenario_phase;
        let next_scenario_phase = self._scenario_track._next_scenario_phase;
        let next_phase_duration = self._scenario_track._next_phase_duration;

        for state in State::iter() {
            if prev_scenario_phase == next_scenario_phase && (state == State::End || state == State::Begin) {
                continue;
            }

            let update_scenario_phase: ScenarioPhase = match state {
                State::End => prev_scenario_phase,
                State::Begin => {
                    self._scenario_track.set_scenario_phase(next_scenario_phase, next_phase_duration);
                    next_scenario_phase
                }
                State::Update => next_scenario_phase,
            };

            let phase_time = self._scenario_track.get_phase_time();
            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
                }
                ScenarioPhase::StoryBoard => match state {
                    State::Begin => {
                        game_scene_manager.set_time(TIME_OF_DAWN, 0.0);
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_behavior_none();
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_behavior_none();
                        }
                        if let Some(actor) = &self._actor_aru {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_move_direction(&Vector3::new(1.0, 0.0, 0.0), true);
                        }
                    }
                    State::Update => {
                        let story_board_phase = self.get_story_board_phase();
                        if !USE_STORY_BOARDS || STORY_BOARDS.len() < story_board_phase {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Morning, Some(PHASE_TIME_SLEEP));
                        } else {
                            if story_board_phase == 0
                                || game_ui_manager.is_done_game_image_progress() && any_key_pressed
                            {
                                if story_board_phase < STORY_BOARDS.len() {
                                    game_ui_manager
                                        .set_image_auto_fade_inout(STORY_BOARDS[story_board_phase], DEFAULT_FADE_TIME);
                                } else {
                                    game_ui_manager.set_image_auto_fade_inout(MATERIAL_UI_NONE, DEFAULT_FADE_TIME);
                                }
                                self.next_story_board_phase();
                            }
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Morning => match state {
                    State::Begin => {
                        if let Some(actor) = &self._actor_aru {
                            actor.borrow_mut().set_action_sleep();
                        }
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_action_sleep();
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_action_sleep();
                        }
                    }
                    State::Update => {
                        let main_camera = game_scene_manager.get_scene_manager().get_main_camera_mut();
                        let progress = 1.0 - (phase_ratio * -5.0).exp2();
                        let position = self._around_start_position.lerp(&self._around_end_position, progress);
                        let rotation = self._around_start_rotation.lerp(&self._around_end_rotation, progress);
                        main_camera._transform_object.set_position(&position);
                        main_camera._transform_object.set_rotation(&rotation);

                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WakeUp, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WakeUp => match state {
                    State::Begin => {
                        game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_ROOSTER);
                    }
                    State::Update => {
                        let time_of_day_ratio = phase_time * 0.2;
                        if time_of_day_ratio < 1.0 {
                            game_scene_manager
                                .set_time(math::lerp(TIME_OF_DAWN, TIME_OF_EARLY_MORNING, time_of_day_ratio), 0.0);
                        }

                        let prev_wakeup_delay_aru = self._wakeup_delay_aru;
                        let prev_wakeup_delay_ewa = self._wakeup_delay_ewa;
                        let prev_wakeup_delay_koa = self._wakeup_delay_koa;
                        self._wakeup_delay_aru -= delta_time as f32;
                        self._wakeup_delay_ewa -= delta_time as f32;
                        self._wakeup_delay_koa -= delta_time as f32;

                        if 0.0 <= prev_wakeup_delay_aru && self._wakeup_delay_aru < 0.0 {
                            if let Some(actor) = &self._actor_aru {
                                actor.borrow_mut().set_action_wake_up();
                            }
                        }

                        if 0.0 <= prev_wakeup_delay_ewa && self._wakeup_delay_ewa < 0.0 {
                            if let Some(actor) = &self._actor_ewa {
                                actor.borrow_mut().set_action_wake_up();
                            }
                        }

                        if 0.0 <= prev_wakeup_delay_koa && self._wakeup_delay_koa < 0.0 {
                            if let Some(actor) = &self._actor_koa {
                                actor.borrow_mut().set_action_wake_up();
                            }
                        }

                        let aru_none = self
                            ._actor_aru
                            .as_ref()
                            .map_or(true, |actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let ewa_none = self
                            ._actor_ewa
                            .as_ref()
                            .map_or(true, |actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let koa_none = self
                            ._actor_koa
                            .as_ref()
                            .map_or(true, |actor| actor.borrow_mut().is_action(ActionAnimationState::None));

                        if self._wakeup_delay_koa < 0.0
                            && self._wakeup_delay_ewa < 0.0
                            && self._wakeup_delay_aru < 0.0
                            && aru_none
                            && ewa_none
                            && koa_none
                        {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::AssembleFamily, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::AssembleFamily => {
                    if state == State::Update {
                        let mut done = true;

                        if let (Some(actor_aru), Some(actor_ewa), Some(actor_koa)) =
                            (&self._actor_aru, &self._actor_ewa, &self._actor_koa)
                        {
                            if self.update_assemble(actor_ewa, actor_aru) {
                                self.emoji_hungry(game_ui_manager, actor_ewa);
                            } else {
                                done = false;
                            }

                            if self.update_assemble(actor_koa, actor_aru) {
                                self.emoji_hungry(game_ui_manager, actor_koa);
                            } else {
                                done = false;
                            }
                        } else {
                            done = false;
                        }

                        if done {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::IamHungry, Some(PHASE_TIME_HUNGRY));
                        }
                    }
                }
                ScenarioPhase::IamHungry => match state {
                    State::Begin => {
                        game_scene_manager.get_scene_manager().play_audio_bank(AUDIO_STOMACH_GROWLING);

                        if let (Some(actor_aru), Some(actor_ewa), Some(actor_koa)) =
                            (&self._actor_aru, &self._actor_ewa, &self._actor_koa)
                        {
                            actor_aru.borrow_mut().look_at(actor_koa.borrow().get_position());
                            actor_ewa.borrow_mut().look_at(actor_aru.borrow().get_position());
                            actor_ewa.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                            actor_ewa.borrow_mut().set_action_hungry();
                            actor_koa.borrow_mut().look_at(actor_aru.borrow().get_position());
                            actor_koa.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                            actor_koa.borrow_mut().set_action_hungry();
                        }
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::MoveToTutorialStage, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::MoveToTutorialStage => match state {
                    State::Begin => {
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }

                        let item_coconut = game_scene_manager.get_game_resources().get_item_data(ITEM_COCONUT);
                        self._quest =
                            Some(game_ui_manager.add_quest(Some(String::from("Gather food for the hungry family."))));
                        if let Some(quest) = &self._quest {
                            self._sub_quest_move_to_tutorial_stage = Some(quest.borrow_mut().add_quest_item(
                                QuestCreateInfo::DefaultQuest(DefaultQuestData {
                                    _quest_icon_name: None,
                                    _quest_description: Some(String::from("Move to the FOREST to find food.")),
                                }),
                            ));
                            self._sub_quest_gather_food = Some(quest.borrow_mut().add_quest_item(
                                QuestCreateInfo::GatherItem(GatherItemData {
                                    _item_data_name: String::from(ITEM_COCONUT),
                                    _item_data: item_coconut.clone(),
                                    _gather_item_count: 3,
                                }),
                            ));
                            self._sub_quest_back_home = Some(quest.borrow_mut().add_quest_item(
                                QuestCreateInfo::DefaultQuest(DefaultQuestData {
                                    _quest_icon_name: None,
                                    _quest_description: Some(String::from("Return home.")),
                                }),
                            ));
                            self._sub_quest_sleep = Some(quest.borrow_mut().add_quest_item(
                                QuestCreateInfo::DefaultQuest(DefaultQuestData {
                                    _quest_icon_name: None,
                                    _quest_description: Some(String::from("Wrap up the day.")),
                                }),
                            ));
                        }

                        self.create_move_to_tutorial_stage_text_box(game_scene_manager);
                    }
                    State::Update => {
                        if game_scene_manager.get_current_game_scene_data_name() == Stages::Forest.get_stage_data_name()
                        {
                            if let Some(q) = &self._sub_quest_move_to_tutorial_stage {
                                q.borrow_mut().set_completed_quest();
                            }
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GatheringFood, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::GatheringFood => match state {
                    State::Begin => {
                        self.create_hit_this_tree_text_box(game_scene_manager);
                    }
                    State::Update => {
                        let completed =
                            self._sub_quest_gather_food.as_ref().map_or(false, |q| q.borrow().is_completed_quest());
                        if game_scene_manager.get_current_game_scene_data_name() == Stages::Forest.get_stage_data_name()
                            && completed
                        {
                            self.remove_move_to_tutorial_stage_text_box(game_scene_manager);
                            self.remove_hit_this_tree_text_box(game_scene_manager);
                            self.create_return_home_text_box(game_scene_manager);
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::BackHome, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::BackHome => {
                    if state == State::Update
                        && game_scene_manager.get_current_game_scene_data_name() == Stages::Home.get_stage_data_name()
                    {
                        if let Some(q) = &self._sub_quest_back_home {
                            q.borrow_mut().set_completed_quest();
                        }
                        self.remove_return_home_text_box(game_scene_manager);
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
                    }
                }
                ScenarioPhase::WrapUpTheDay => {
                    if state == State::Begin {
                        self.create_wrap_up_the_day_text_box(game_scene_manager);
                    }
                }
                ScenarioPhase::Sleeping => {
                    if state == State::Update
                        && !game_scene_manager.has_game_scenario(ScenarioType::ScenarioWrapUpTheDay)
                    {
                        self.clear_all();
                        game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioIntro_Ufo);
                        self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                    }
                }
                ScenarioPhase::End => {}
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }

        let sleep_not_completed = self._sub_quest_sleep.as_ref().map_or(false, |q| !q.borrow().is_completed_quest());
        if self._sub_quest_sleep.is_some()
            && sleep_not_completed
            && let Some(scenario_wrap_up_the_day) =
                game_scene_manager.get_game_scenario(ScenarioType::ScenarioWrapUpTheDay).as_ref()
        {
            ptr_as_mut(scenario_wrap_up_the_day.as_ptr() as *const ScenarioWrapUpTheDay).set_skip_wakeup(true);
            if let Some(q) = &self._sub_quest_sleep {
                q.borrow_mut().set_completed_quest();
            }
            self._scenario_track.set_next_scenario_phase(ScenarioPhase::Sleeping, None);
        }
    }
}
