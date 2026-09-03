use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_game_resources, get_game_scene_manager_mut, get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::{GameUIManager, QuestItem};
use crate::game_module::scenario::game_scenarios::scenario_wrap_up_the_day::ScenarioWrapUpTheDay;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::{QuestCreateInfo, QuestItemSaveData};
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxItemOption, TextBoxLayerType};
use nalgebra::{Vector2, Vector3};
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{get_audio_manager_mut, get_scene_manager};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell, ptr_as_mut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_void;
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
    GatheringFood,
    WrapUpTheDay,
    Sleeping,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ScenarioIntroQuestSaveData {
    pub _has_quest: bool,
    pub _quest_title: Option<String>,
    pub _sub_quest_hit_the_tree: Option<QuestItemSaveData>,
    pub _sub_quest_gather_food: Option<QuestItemSaveData>,
    pub _sub_quest_feed_ewa: Option<QuestItemSaveData>,
    pub _sub_quest_feed_koa: Option<QuestItemSaveData>,
    pub _sub_quest_sleep: Option<QuestItemSaveData>,
}

impl ScenarioIntroQuestSaveData {
    pub fn has_any_quest(&self) -> bool {
        self._has_quest
            || self._sub_quest_hit_the_tree.is_some()
            || self._sub_quest_gather_food.is_some()
            || self._sub_quest_feed_ewa.is_some()
            || self._sub_quest_feed_koa.is_some()
            || self._sub_quest_sleep.is_some()
    }
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
    #[serde(default)]
    pub _quest_save_data: ScenarioIntroQuestSaveData,
}

pub struct ScenarioIntro<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,

    _player: Option<RcRefCell<Character<'a>>>,
    _actor_ewa: Option<RcRefCell<Character<'a>>>,
    _actor_koa: Option<RcRefCell<Character<'a>>>,
    _prop_gate: Option<RcRefCell<Prop<'a>>>,
    _prop_gate_stage01: Option<RcRefCell<Prop<'a>>>,
    _prop_tree: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _quest: Option<RcRefCell<QuestTitle<'a>>>,
    _sub_quest_hit_the_tree: Option<QuestItem<'a>>,
    _sub_quest_gather_food: Option<QuestItem<'a>>,
    _sub_quest_feed_ewa: Option<QuestItem<'a>>,
    _sub_quest_feed_koa: Option<QuestItem<'a>>,
    _sub_quest_sleep: Option<QuestItem<'a>>,
    _tree_fruit_items: HashMap<*const c_void, ActorWrapper<'a>>,
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
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioIntro<'a>> {
        newRcRefCell(ScenarioIntro {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _player: None,
            _actor_ewa: None,
            _actor_koa: None,
            _prop_gate: None,
            _prop_gate_stage01: None,
            _prop_tree: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _quest: None,
            _sub_quest_hit_the_tree: None,
            _sub_quest_gather_food: None,
            _sub_quest_feed_ewa: None,
            _sub_quest_feed_koa: None,
            _sub_quest_sleep: None,
            _tree_fruit_items: HashMap::new(),
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
        let contents = vec![TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY), None)];
        game_ui_manager.add_text_box_item(
            ActorWrapper::Character(actor.clone()),
            &contents,
            &TextBoxItemOption {
                _layer_type: TextBoxLayerType::InteractionLayer,
                _duration: Some(CHARACTER_INTERACTION_TIME),
                ..Default::default()
            },
        );
        actor.borrow_mut().set_move_idle();
    }

    pub fn remove_move_to_tutorial_stage_text_box(&self) {
        if let Some(prop) = self._prop_gate.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn create_hit_this_tree_text_box(&self) {
        if let Some(prop_tree) = self._prop_tree.as_ref() {
            let actor_wrapper = ActorWrapper::Prop(prop_tree.clone());
            let contents = vec![TextBoxContent::MaterialInstance(MATERIAL_UI_POINTER.to_string(), Some(Vector2::new(50.0, 50.0)))];
            get_game_ui_manager_mut().add_text_box_item(
                actor_wrapper,
                &contents,
                &TextBoxItemOption {
                    _layer_type: TextBoxLayerType::GamePlayLayer,
                    _bounce: true,
                    _visible_background: false,
                    ..Default::default()
                },
            );
        }
    }

    pub fn remove_hit_this_tree_text_box(&self) {
        if let Some(prop_tree) = self._prop_tree.as_ref() {
            let actor_wrapper = ActorWrapper::Prop(prop_tree.clone());
            get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_give_food_to_ewa_text_box(&self) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let wrapper = ActorWrapper::Character(actor.clone());
            let contents = vec![TextBoxContent::MaterialInstance(MATERIAL_UI_POINTER.to_string(), Some(Vector2::new(50.0, 50.0)))];
            get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                &TextBoxItemOption {
                    _layer_type: TextBoxLayerType::GamePlayLayer,
                    _bounce: true,
                    _visible_background: false,
                    ..Default::default()
                },
            );
        }
    }

    pub fn remove_give_food_to_ewa_text_box(&self) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_give_food_to_koa_text_box(&self) {
        if let Some(actor) = self._actor_koa.as_ref() {
            let wrapper = ActorWrapper::Character(actor.clone());
            let contents = vec![TextBoxContent::MaterialInstance(MATERIAL_UI_POINTER.to_string(), Some(Vector2::new(50.0, 50.0)))];
            get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                &TextBoxItemOption {
                    _layer_type: TextBoxLayerType::GamePlayLayer,
                    _bounce: true,
                    _visible_background: false,
                    ..Default::default()
                },
            );
        }
    }

    pub fn remove_give_food_to_koa_text_box(&self) {
        if let Some(actor) = self._actor_koa.as_ref() {
            let actor_wrapper = ActorWrapper::Character(actor.clone());
            get_game_ui_manager_mut().remove_text_box_item(actor_wrapper.get_key());
        }
    }

    pub fn create_wrap_up_the_day_text_box(&self) {
        if let Some(prop) = self._prop_bed_for_aru.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            let contents = vec![TextBoxContent::MaterialInstance(MATERIAL_UI_POINTER.to_string(), Some(Vector2::new(50.0, 50.0)))];
            get_game_ui_manager_mut().add_text_box_item(
                wrapper,
                &contents,
                &TextBoxItemOption {
                    _layer_type: TextBoxLayerType::GamePlayLayer,
                    _bounce: true,
                    _visible_background: false,
                    ..Default::default()
                },
            );
        }
    }

    pub fn remove_wrap_up_the_day_text_box(&self) {
        if let Some(prop) = self._prop_bed_for_aru.as_ref() {
            let wrapper = ActorWrapper::Prop(prop.clone());
            get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn remove_all_tree_fruit_text_boxes(&mut self) {
        let game_ui_manager = get_game_ui_manager_mut();
        for (key, _) in self._tree_fruit_items.drain() {
            game_ui_manager.remove_text_box_item(key);
        }
    }

    pub fn clear_all(&mut self) {
        self.remove_move_to_tutorial_stage_text_box();
        self.remove_hit_this_tree_text_box();
        self.remove_give_food_to_ewa_text_box();
        self.remove_give_food_to_koa_text_box();
        self.remove_wrap_up_the_day_text_box();

        self.remove_all_tree_fruit_text_boxes();

        self._player = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_gate = None;
        self._prop_gate_stage01 = None;
        self._prop_tree = None;
        self._prop_bed_for_aru = None;
        self._prop_bed_for_ewa = None;
        self._prop_bed_for_koa = None;
    }

    pub fn update_tree_fruit_text_boxes(&mut self) {
        let is_gather_food_completed =
            self._sub_quest_gather_food.as_ref().is_some_and(|q| q.borrow().is_completed_quest());

        if is_gather_food_completed {
            if !self._tree_fruit_items.is_empty() {
                self.remove_all_tree_fruit_text_boxes();
            }
            return;
        }

        if let Some(prop_tree) = self._prop_tree.as_ref() {
            let tree_pos = *prop_tree.borrow().get_position();
            let game_scene_manager = get_game_scene_manager_mut();
            let items = game_scene_manager.get_item_manager().get_items();
            let mut current_fruit_keys = HashMap::new();

            for item in items.values() {
                let item_borrow = item.borrow();
                if item_borrow.get_item_data_name() == ITEM_COCONUT {
                    let item_pos = item_borrow._item_properties._position;
                    let diff = item_pos - tree_pos;
                    if diff.norm() <= 8.0 {
                        let actor_wrapper = ActorWrapper::RenderObject(item_borrow._render_object.clone());
                        let key = actor_wrapper.get_key();
                        current_fruit_keys.insert(key, actor_wrapper);
                    }
                }
            }

            let game_ui_manager = get_game_ui_manager_mut();
            for (key, actor_wrapper) in current_fruit_keys.iter() {
                if !self._tree_fruit_items.contains_key(key) {
                    let contents = vec![TextBoxContent::MaterialInstance(MATERIAL_UI_POINTER.to_string(), Some(Vector2::new(50.0, 50.0)))];
                    game_ui_manager.add_text_box_item(
                        actor_wrapper.clone(),
                        &contents,
                        &TextBoxItemOption {
                            _layer_type: TextBoxLayerType::GamePlayLayer,
                            _bounce: true,
                            _visible_background: false,
                            _offset: Vector2::new(0.0, -50.0),
                            ..Default::default()
                        },
                    );
                    self._tree_fruit_items.insert(*key, actor_wrapper.clone());
                }
            }

            let mut removed_keys = Vec::new();
            for key in self._tree_fruit_items.keys() {
                if !current_fruit_keys.contains_key(key) {
                    removed_keys.push(*key);
                }
            }

            for key in removed_keys {
                game_ui_manager.remove_text_box_item(key);
                self._tree_fruit_items.remove(&key);
            }
        } else if !self._tree_fruit_items.is_empty() {
            self.remove_all_tree_fruit_text_boxes();
        }
    }

    pub fn create_quests(&mut self) {
        if self._quest.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let item_coconut = get_game_resources().get_item_data(ITEM_COCONUT);
            self._quest = Some(game_ui_manager.add_quest(Some(String::from("Gather food for the hungry family."))));
            if let Some(quest) = &self._quest {
                self._sub_quest_hit_the_tree = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Hit the tree.")),
                    },
                )));
                self._sub_quest_gather_food = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(
                    GatherItemData {
                        _item_data_name: String::from(ITEM_COCONUT),
                        _item_data: item_coconut.clone(),
                        _gather_item_count: 2,
                    },
                )));
                self._sub_quest_feed_ewa = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Feed food to Ewa.")),
                    },
                )));
                self._sub_quest_feed_koa = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Feed food to Koa.")),
                    },
                )));
                self._sub_quest_sleep = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Wrap up the day.")),
                    },
                )));
            }
        }
    }

    pub fn destroy_quest(&mut self) {
        if let Some(quest) = &self._quest {
            quest.borrow_mut().destroy_quest();
        }
        self._quest = None;
        self._sub_quest_hit_the_tree = None;
        self._sub_quest_gather_food = None;
        self._sub_quest_feed_ewa = None;
        self._sub_quest_feed_koa = None;
        self._sub_quest_sleep = None;
    }

    pub fn get_quest_save_data(&self) -> ScenarioIntroQuestSaveData {
        ScenarioIntroQuestSaveData {
            _has_quest: self._quest.is_some(),
            _quest_title: self._quest.as_ref().and_then(|q| q.borrow()._quest_title.clone()),
            _sub_quest_hit_the_tree: self._sub_quest_hit_the_tree.as_ref().map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_gather_food: self._sub_quest_gather_food.as_ref().map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_feed_ewa: self._sub_quest_feed_ewa.as_ref().map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_feed_koa: self._sub_quest_feed_koa.as_ref().map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_sleep: self._sub_quest_sleep.as_ref().map(|q| q.borrow().get_quest_item_save_data()),
        }
    }

    pub fn load_quest_save_data(&mut self, quest_save_data: &ScenarioIntroQuestSaveData) {
        if quest_save_data.has_any_quest() {
            self.destroy_quest();
            self.create_quests();

            if let Some(save_data) = &quest_save_data._sub_quest_hit_the_tree
                && let Some(q) = &self._sub_quest_hit_the_tree
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_gather_food
                && let Some(q) = &self._sub_quest_gather_food
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_feed_ewa
                && let Some(q) = &self._sub_quest_feed_ewa
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_feed_koa
                && let Some(q) = &self._sub_quest_feed_koa
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_sleep
                && let Some(q) = &self._sub_quest_sleep
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
        }
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
            self.load_quest_save_data(&data._quest_save_data);
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
            _quest_save_data: self.get_quest_save_data(),
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
            ScenarioPhase::GatheringFood
            | ScenarioPhase::WrapUpTheDay => false,
            _ => true,
        }
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        self.clear_all();
        self.destroy_quest();
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.clear_all();
    }

    fn on_open_game_scene(&mut self, game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager_mut();
        if self._scenario_create_info.get_game_scene_data_name() == game_scene_data_name {
            game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
            self._scenario_create_info.reset();
        }

        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager.get_actor_by_name("monkey_ewa").cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name("monkey_koa").cloned();
        self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_ewa").cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name("bed_for_koa").cloned();
        self._prop_tree = game_scene_manager.get_prop_manager().get_prop_by_name("birch_tree_00").cloned();
        self._prop_gate_stage01 = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

        // update quest & text box
        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin => {
                let mut pivot = Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                if let Some(actor) = self._player.as_ref() {
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

                let main_camera = get_scene_manager().get_main_camera_mut();
                main_camera._transform_object.set_position(&self._around_start_position);
                main_camera._transform_object.set_rotation(&self._around_start_rotation);

                self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
            }
            ScenarioPhase::GatheringFood => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    let hit_tree_not_completed =
                        self._sub_quest_hit_the_tree.as_ref().is_none_or(|q| !q.borrow().is_completed_quest());
                    if hit_tree_not_completed {
                        self.create_hit_this_tree_text_box();
                    }

                    let is_gather_food_completed =
                        self._sub_quest_gather_food.as_ref().is_some_and(|q| q.borrow().is_completed_quest());
                    if is_gather_food_completed {
                        let feed_ewa_not_completed =
                            self._sub_quest_feed_ewa.as_ref().is_none_or(|q| !q.borrow().is_completed_quest());
                        if feed_ewa_not_completed {
                            if let Some(actor) = &self._actor_ewa {
                                if !actor.borrow().get_stats().is_hungry() {
                                    actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                                }
                                self.create_give_food_to_ewa_text_box();
                            }
                        }

                        let feed_koa_not_completed =
                            self._sub_quest_feed_koa.as_ref().is_none_or(|q| !q.borrow().is_completed_quest());
                        if feed_koa_not_completed {
                            if let Some(actor) = &self._actor_koa {
                                if !actor.borrow().get_stats().is_hungry() {
                                    actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                                }
                                self.create_give_food_to_koa_text_box();
                            }
                        }
                    }
                }
            }
            ScenarioPhase::WrapUpTheDay => {
                if game_scene_data_name == Stages::Home.get_stage_data_name() {
                    self.create_wrap_up_the_day_text_box();
                }
            }
            _ => (),
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, any_key_pressed: bool, delta_time: f64) {
        let game_scene_manager = get_game_scene_manager_mut();
        let game_ui_manager = get_game_ui_manager_mut();

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
                        if let Some(actor) = &self._player {
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
                        if let Some(actor) = &self._player {
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
                        let main_camera = get_scene_manager().get_main_camera_mut();
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
                        get_audio_manager_mut().play_audio_bank(AUDIO_ROOSTER, AudioLoop::ONCE, None);
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

                        if 0.0 <= prev_wakeup_delay_aru
                            && self._wakeup_delay_aru < 0.0
                            && let Some(actor) = &self._player
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        if 0.0 <= prev_wakeup_delay_ewa
                            && self._wakeup_delay_ewa < 0.0
                            && let Some(actor) = &self._actor_ewa
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        if 0.0 <= prev_wakeup_delay_koa
                            && self._wakeup_delay_koa < 0.0
                            && let Some(actor) = &self._actor_koa
                        {
                            actor.borrow_mut().set_action_wake_up();
                        }

                        let aru_none = self
                            ._player
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let ewa_none = self
                            ._actor_ewa
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));
                        let koa_none = self
                            ._actor_koa
                            .as_ref()
                            .is_none_or(|actor| actor.borrow_mut().is_action(ActionAnimationState::None));

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
                            (&self._player, &self._actor_ewa, &self._actor_koa)
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
                        get_audio_manager_mut().play_audio_bank(AUDIO_STOMACH_GROWLING, AudioLoop::ONCE, None);

                        if let (Some(actor_aru), Some(actor_ewa), Some(actor_koa)) =
                            (&self._player, &self._actor_ewa, &self._actor_koa)
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
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GatheringFood, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::GatheringFood => match state {
                    State::Begin => {
                        if let Some(actor) = &self._actor_ewa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }
                        if let Some(actor) = &self._actor_koa {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }

                        self.create_quests();
                        self.create_hit_this_tree_text_box();
                    }
                    State::Update => {
                        self.update_tree_fruit_text_boxes();

                        if let Some(prop_tree) = &self._prop_tree {
                            if prop_tree.borrow()._prop_stats._hit_blink_time > 0.0 {
                                if let Some(q) = &self._sub_quest_hit_the_tree {
                                    if !q.borrow().is_completed_quest() {
                                        q.borrow_mut().set_completed_quest();
                                        self.remove_hit_this_tree_text_box();
                                    }
                                }
                            }
                        }

                        let is_gather_food_completed =
                            self._sub_quest_gather_food.as_ref().is_some_and(|q| q.borrow().is_completed_quest());
                        if is_gather_food_completed {
                            let feed_ewa_not_completed =
                                self._sub_quest_feed_ewa.as_ref().is_none_or(|q| !q.borrow().is_completed_quest());
                            if feed_ewa_not_completed {
                                if let Some(actor) = &self._actor_ewa {
                                    let key = ActorWrapper::Character(actor.clone()).get_key();
                                    if !game_ui_manager.has_text_box_item(key) {
                                        if !actor.borrow().get_stats().is_hungry() {
                                            actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                                        }
                                        self.create_give_food_to_ewa_text_box();
                                    }
                                }
                            }

                            let feed_koa_not_completed =
                                self._sub_quest_feed_koa.as_ref().is_none_or(|q| !q.borrow().is_completed_quest());
                            if feed_koa_not_completed {
                                if let Some(actor) = &self._actor_koa {
                                    let key = ActorWrapper::Character(actor.clone()).get_key();
                                    if !game_ui_manager.has_text_box_item(key) {
                                        if !actor.borrow().get_stats().is_hungry() {
                                            actor.borrow_mut().set_hunger(HUNGER_WARNING_THRESHOLD);
                                        }
                                        self.create_give_food_to_koa_text_box();
                                    }
                                }
                            }
                        }

                        if let Some(actor_ewa) = &self._actor_ewa {
                            let ewa_borrow = actor_ewa.borrow();
                            if ewa_borrow.get_hunger() < HUNGER_WARNING_THRESHOLD
                                || ewa_borrow.get_attached_item().is_some()
                                || ewa_borrow.is_action(ActionAnimationState::Eating)
                            {
                                if let Some(q) = &self._sub_quest_feed_ewa {
                                    q.borrow_mut().set_completed_quest();
                                }
                                self.remove_give_food_to_ewa_text_box();
                            }
                        }

                        if let Some(actor_koa) = &self._actor_koa {
                            let koa_borrow = actor_koa.borrow();
                            if koa_borrow.get_hunger() < HUNGER_WARNING_THRESHOLD
                                || koa_borrow.get_attached_item().is_some()
                                || koa_borrow.is_action(ActionAnimationState::Eating)
                            {
                                if let Some(q) = &self._sub_quest_feed_koa {
                                    q.borrow_mut().set_completed_quest();
                                }
                                self.remove_give_food_to_koa_text_box();
                            }
                        }

                        let ewa_completed =
                            self._sub_quest_feed_ewa.as_ref().is_some_and(|q| q.borrow().is_completed_quest());
                        let koa_completed =
                            self._sub_quest_feed_koa.as_ref().is_some_and(|q| q.borrow().is_completed_quest());

                        if ewa_completed && koa_completed {
                            self.remove_move_to_tutorial_stage_text_box();
                            self.remove_hit_this_tree_text_box();
                            self.remove_all_tree_fruit_text_boxes();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WrapUpTheDay => {
                    if state == State::Begin {
                        self.create_wrap_up_the_day_text_box();
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

        let sleep_not_completed = self._sub_quest_sleep.as_ref().is_some_and(|q| !q.borrow().is_completed_quest());
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
