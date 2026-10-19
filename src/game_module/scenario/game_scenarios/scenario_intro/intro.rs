use crate::game_module::actors::character::ActionAnimationState;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::props::Prop;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_constants::*;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_game_resources, get_game_scene_manager, get_game_scene_manager_mut, get_game_ui_manager,
    get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::{GameUIManager, QuestItem};
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{
    POINTER_ICON_SIZE, add_prop_pointer_text_box, complete_quest_item, get_quest_item_save_data,
    is_quest_item_completed, load_quest_item_save_data, remove_prop_text_box, set_actors_behavior_none,
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
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_void;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const USE_STORY_BOARDS: bool = false;
const PHASE_TIME_SLEEP: f32 = 5.0;
const PHASE_TIME_HUNGRY: f32 = 3.0;
const WAKEUP_DELAY_ARU_DEFAULT: f32 = 2.0;
const WAKEUP_DELAY_EWA_DEFAULT: f32 = 3.5;
const WAKEUP_DELAY_KOA_DEFAULT: f32 = 4.0;
const AROUND_START_ROT: Vector3<f32> = Vector3::new(0.4, 0.0, 0.0);
const AROUND_END_ROT: Vector3<f32> = Vector3::new(0.35, 0.0, 0.0);
const AROUND_CAMERA_EXTRA_DIST: f32 = 6.0;
const ASSEMBLE_RADIUS_PADDING: f32 = 0.5;
const TREE_FRUIT_SEARCH_RADIUS: f32 = 8.0;
const FRUIT_POINTER_OFFSET: Vector2<f32> = Vector2::new(0.0, -50.0);
const WAKEUP_TIME_OF_DAY_SPEED_RATIO: f32 = 0.2;

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
    GoToTheForest,
    GatheringFood,
    ReturnHome,
    WrapUpTheDay,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ScenarioIntroQuestSaveData {
    pub _has_quest: bool,
    pub _quest_title: Option<String>,
    pub _sub_quest_go_to_the_forest: Option<QuestItemSaveData>,
    pub _sub_quest_hit_the_tree: Option<QuestItemSaveData>,
    pub _sub_quest_gather_food: Option<QuestItemSaveData>,
    pub _sub_quest_return_home: Option<QuestItemSaveData>,
    pub _sub_quest_wrap_up_the_day: Option<QuestItemSaveData>,
    #[serde(alias = "_sub_quest_eat_food")]
    pub _sub_quest_cooking: Option<QuestItemSaveData>,
    pub _sub_quest_sleep: Option<QuestItemSaveData>,
}

impl ScenarioIntroQuestSaveData {
    pub fn has_any_quest(&self) -> bool {
        self._has_quest
            || self._sub_quest_go_to_the_forest.is_some()
            || self._sub_quest_hit_the_tree.is_some()
            || self._sub_quest_gather_food.is_some()
            || self._sub_quest_return_home.is_some()
            || self._sub_quest_wrap_up_the_day.is_some()
            || self._sub_quest_cooking.is_some()
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
    _prop_table: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_aru: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_ewa: Option<RcRefCell<Prop<'a>>>,
    _prop_bed_for_koa: Option<RcRefCell<Prop<'a>>>,
    _quest: Option<RcRefCell<QuestTitle<'a>>>,
    _sub_quest_go_to_the_forest: Option<QuestItem<'a>>,
    _sub_quest_hit_the_tree: Option<QuestItem<'a>>,
    _sub_quest_gather_food: Option<QuestItem<'a>>,
    _sub_quest_return_home: Option<QuestItem<'a>>,
    _sub_quest_wrap_up_the_day: Option<QuestItem<'a>>,
    _sub_quest_cooking: Option<QuestItem<'a>>,
    _sub_quest_sleep: Option<QuestItem<'a>>,
    _tree_fruit_items: HashMap<*const c_void, ActorWrapper<'a>>,
    _wakeup_delay_aru: f32,
    _wakeup_delay_ewa: f32,
    _wakeup_delay_koa: f32,
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
            _prop_table: None,
            _prop_bed_for_aru: None,
            _prop_bed_for_ewa: None,
            _prop_bed_for_koa: None,
            _quest: None,
            _sub_quest_go_to_the_forest: None,
            _sub_quest_hit_the_tree: None,
            _sub_quest_gather_food: None,
            _sub_quest_return_home: None,
            _sub_quest_wrap_up_the_day: None,
            _sub_quest_cooking: None,
            _sub_quest_sleep: None,
            _tree_fruit_items: HashMap::new(),
            _wakeup_delay_aru: WAKEUP_DELAY_ARU_DEFAULT,
            _wakeup_delay_ewa: WAKEUP_DELAY_EWA_DEFAULT,
            _wakeup_delay_koa: WAKEUP_DELAY_KOA_DEFAULT,
            _around_start_position: Vector3::zeros(),
            _around_end_position: Vector3::zeros(),
            _around_start_rotation: AROUND_START_ROT,
            _around_end_rotation: AROUND_END_ROT,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
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
        let radius = target.borrow().get_collision()._bounding_box._mag_xz + ASSEMBLE_RADIUS_PADDING;
        let (direction, dist) =
            math::make_normalize_xz_with_norm(&(target.borrow().get_position() - actor.borrow().get_position()));
        if radius < dist {
            actor.borrow_mut().set_move(&direction);
            return false;
        }
        true
    }

    pub fn emoji_hungry(&self, game_ui_manager: &mut GameUIManager<'a>, actor: &RcRefCell<Character<'a>>) {
        let contents = vec![TextBoxContent::MaterialInstance(
            String::from(MATERIAL_EMOJI_HUNGRY),
            None,
        )];
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

    pub fn create_move_to_forest_text_box(&self) {
        add_prop_pointer_text_box(&self._prop_gate, false);
    }

    pub fn remove_move_to_forest_text_box(&self) {
        remove_prop_text_box(&self._prop_gate);
    }

    pub fn create_return_home_text_box(&self) {
        add_prop_pointer_text_box(&self._prop_gate, false);
    }

    pub fn remove_return_home_text_box(&self) {
        remove_prop_text_box(&self._prop_gate);
    }

    pub fn remove_move_to_tutorial_stage_text_box(&self) {
        self.remove_move_to_forest_text_box();
    }

    pub fn create_hit_this_tree_text_box(&self) {
        add_prop_pointer_text_box(&self._prop_tree, false);
    }

    pub fn remove_hit_this_tree_text_box(&self) {
        remove_prop_text_box(&self._prop_tree);
    }

    pub fn create_prop_table_text_box(&self) {
        add_prop_pointer_text_box(&self._prop_table, false);
    }

    pub fn remove_prop_table_text_box(&self) {
        remove_prop_text_box(&self._prop_table);
    }

    pub fn create_prop_bed_for_aru_text_box(&self) {
        add_prop_pointer_text_box(&self._prop_bed_for_aru, true);
    }

    pub fn remove_prop_bed_for_aru_text_box(&self) {
        remove_prop_text_box(&self._prop_bed_for_aru);
    }

    pub fn create_actor_ewa_text_box(&self) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let wrapper = ActorWrapper::Character(actor.clone());
            let contents = vec![TextBoxContent::MaterialInstance(
                MATERIAL_UI_POINTER.to_string(),
                Some(POINTER_ICON_SIZE),
            )];
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

    pub fn remove_actor_ewa_text_box(&self) {
        if let Some(actor) = self._actor_ewa.as_ref() {
            let wrapper = ActorWrapper::Character(actor.clone());
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
        self.remove_move_to_forest_text_box();
        self.remove_return_home_text_box();
        self.remove_hit_this_tree_text_box();
        self.remove_prop_table_text_box();
        self.remove_prop_bed_for_aru_text_box();
        self.remove_actor_ewa_text_box();
        self.remove_all_tree_fruit_text_boxes();

        self._player = None;
        self._actor_ewa = None;
        self._actor_koa = None;
        self._prop_gate = None;
        self._prop_gate_stage01 = None;
        self._prop_tree = None;
        self._prop_table = None;
        self._prop_bed_for_aru = None;
        self._prop_bed_for_ewa = None;
        self._prop_bed_for_koa = None;
    }

    pub fn complete_sub_quest_go_to_the_forest(&mut self) {
        if complete_quest_item(&self._sub_quest_go_to_the_forest) {
            self.remove_move_to_forest_text_box();
        }
    }

    pub fn complete_sub_quest_hit_the_tree(&mut self) {
        if complete_quest_item(&self._sub_quest_hit_the_tree) {
            self.remove_hit_this_tree_text_box();
        }
    }

    pub fn complete_sub_quest_gather_food(&mut self) {
        if complete_quest_item(&self._sub_quest_gather_food) {
            self.remove_all_tree_fruit_text_boxes();
        }
    }

    pub fn complete_sub_quest_return_home(&mut self) {
        if complete_quest_item(&self._sub_quest_return_home) {
            self.remove_return_home_text_box();
        }
    }

    pub fn remove_sub_quest_go_to_the_forest(&mut self) {
        if let Some(quest_item) = self._sub_quest_go_to_the_forest.take() {
            if let Some(quest) = self._quest.as_ref() {
                quest.borrow_mut().remove_quest_item(&quest_item);
            }
            self.remove_move_to_forest_text_box();
        }
    }

    pub fn remove_sub_quest_hit_the_tree(&mut self) {
        if let Some(quest_item) = self._sub_quest_hit_the_tree.take() {
            if let Some(quest) = self._quest.as_ref() {
                quest.borrow_mut().remove_quest_item(&quest_item);
            }
            self.remove_hit_this_tree_text_box();
        }
    }

    pub fn remove_sub_quest_gather_food(&mut self) {
        if let Some(quest_item) = self._sub_quest_gather_food.take() {
            if let Some(quest) = self._quest.as_ref() {
                quest.borrow_mut().remove_quest_item(&quest_item);
            }
            self.remove_all_tree_fruit_text_boxes();
        }
    }

    pub fn remove_sub_quest_return_home(&mut self) {
        if let Some(quest_item) = self._sub_quest_return_home.take() {
            if let Some(quest) = self._quest.as_ref() {
                quest.borrow_mut().remove_quest_item(&quest_item);
            }
            self.remove_return_home_text_box();
        }
    }

    pub fn remove_uncompleted_sub_quests_before_wrap_up(&mut self) {
        if self._sub_quest_go_to_the_forest.is_some() && !is_quest_item_completed(&self._sub_quest_go_to_the_forest) {
            self.remove_sub_quest_go_to_the_forest();
        }
        if self._sub_quest_hit_the_tree.is_some() && !is_quest_item_completed(&self._sub_quest_hit_the_tree) {
            self.remove_sub_quest_hit_the_tree();
        }
        if self._sub_quest_gather_food.is_some() && !is_quest_item_completed(&self._sub_quest_gather_food) {
            self.remove_sub_quest_gather_food();
        }
        if self._sub_quest_return_home.is_some() && !is_quest_item_completed(&self._sub_quest_return_home) {
            self.remove_sub_quest_return_home();
        }
    }

    pub fn has_any_food(&self) -> bool {
        let player_has_food =
            self._player.as_ref().is_some_and(|p| p.borrow().get_attached_item_data_type().is_eatable());
        if player_has_food {
            return true;
        }

        let inventory_has_food = get_game_ui_manager()._item_bar_widget.as_ref().is_some_and(|bar| {
            bar._inventory_slots.iter().any(|slot| slot._item_count > 0 && slot._item_data_type.is_eatable())
        });
        if inventory_has_food {
            return true;
        }

        let table_storage_has_food = get_game_ui_manager()
            .get_table_storage_widget()
            .is_some_and(|storage| storage.has_eatable_table_storage_item());
        if table_storage_has_food {
            return true;
        }

        let table_has_food = self
            ._prop_table
            .as_ref()
            .is_some_and(|table| get_game_scene_manager().get_item_manager().has_item_on_table(table));
        if table_has_food {
            return true;
        }

        false
    }

    pub fn remove_sub_quest_cooking(&mut self) {
        if let Some(cooking_quest) = self._sub_quest_cooking.take() {
            if let Some(quest) = self._quest.as_ref() {
                quest.borrow_mut().remove_quest_item(&cooking_quest);
            }
            self.remove_prop_table_text_box();
        }
    }

    pub fn is_sub_quest_cooking_active(&self) -> bool {
        self._sub_quest_cooking.is_some()
            && is_quest_item_completed(&self._sub_quest_wrap_up_the_day)
            && !is_quest_item_completed(&self._sub_quest_cooking)
    }

    pub fn is_sub_quest_sleep_active(&self) -> bool {
        self._sub_quest_sleep.is_some()
            && !is_quest_item_completed(&self._sub_quest_sleep)
            && is_quest_item_completed(&self._sub_quest_wrap_up_the_day)
            && (self._sub_quest_cooking.is_none() || is_quest_item_completed(&self._sub_quest_cooking))
    }

    pub fn complete_sub_quest_wrap_up_the_day(&mut self) {
        complete_quest_item(&self._sub_quest_wrap_up_the_day);
        self.remove_actor_ewa_text_box();
        if !self.has_any_food() {
            self.remove_sub_quest_cooking();
        }
        self.remove_uncompleted_sub_quests_before_wrap_up();

        if self._scenario_track._scenario_phase != ScenarioPhase::WrapUpTheDay {
            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
        }

        if self.is_sub_quest_cooking_active() {
            self.create_prop_table_text_box();
        } else {
            self.remove_prop_table_text_box();
        }

        if self.is_sub_quest_sleep_active() {
            self.create_prop_bed_for_aru_text_box();
        } else {
            self.remove_prop_bed_for_aru_text_box();
        }
    }

    pub fn complete_sub_quest_sleep(&mut self) {
        if complete_quest_item(&self._sub_quest_sleep) {
            self.remove_prop_bed_for_aru_text_box();
        }
        self.complete_sub_quest_cooking();
    }

    pub fn check_hit_the_tree_complete(&mut self) {
        if let Some(prop_tree) = &self._prop_tree
            && prop_tree.borrow()._prop_stats._hit_blink_time > 0.0
        {
            self.complete_sub_quest_hit_the_tree();
        }
    }

    pub fn update_tree_fruit_text_boxes(&mut self) {
        if is_quest_item_completed(&self._sub_quest_gather_food) {
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
                    if (diff.x * diff.x + diff.z * diff.z).sqrt() <= TREE_FRUIT_SEARCH_RADIUS {
                        let actor_wrapper = ActorWrapper::RenderObject(item_borrow._render_object.clone());
                        let key = actor_wrapper.get_key();
                        current_fruit_keys.insert(key, actor_wrapper);
                    }
                }
            }

            let game_ui_manager = get_game_ui_manager_mut();
            for (key, actor_wrapper) in current_fruit_keys.iter() {
                if !self._tree_fruit_items.contains_key(key) {
                    let contents = vec![TextBoxContent::MaterialInstance(
                        MATERIAL_UI_POINTER.to_string(),
                        Some(POINTER_ICON_SIZE),
                    )];
                    game_ui_manager.add_text_box_item(
                        actor_wrapper.clone(),
                        &contents,
                        &TextBoxItemOption {
                            _layer_type: TextBoxLayerType::GamePlayLayer,
                            _bounce: true,
                            _visible_background: false,
                            _offset: FRUIT_POINTER_OFFSET,
                            ..Default::default()
                        },
                    );
                    self._tree_fruit_items.insert(*key, actor_wrapper.clone());
                }
            }

            self._tree_fruit_items.retain(|key, _| {
                let keep = current_fruit_keys.contains_key(key);
                if !keep {
                    game_ui_manager.remove_text_box_item(*key);
                }
                keep
            });
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
                self._sub_quest_go_to_the_forest = Some(quest.borrow_mut().add_quest_item(
                    QuestCreateInfo::DefaultQuest(DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Go to the forest.")),
                    }),
                ));
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
                        _gather_item_count: 1,
                    },
                )));
                self._sub_quest_return_home = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Return home.")),
                    },
                )));
                self._sub_quest_wrap_up_the_day = Some(quest.borrow_mut().add_quest_item(
                    QuestCreateInfo::DefaultQuest(DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Wrap up the day.")),
                    }),
                ));
                self._sub_quest_cooking = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Have dinner.")),
                    },
                )));
                self._sub_quest_sleep = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                    DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Sleep.")),
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
        self._sub_quest_go_to_the_forest = None;
        self._sub_quest_hit_the_tree = None;
        self._sub_quest_gather_food = None;
        self._sub_quest_return_home = None;
        self._sub_quest_wrap_up_the_day = None;
        self._sub_quest_cooking = None;
        self._sub_quest_sleep = None;
    }

    pub fn reset_actors_to_bed_positions(&mut self) {
        for (actor_opt, bed_opt) in [
            (&self._player, &self._prop_bed_for_aru),
            (&self._actor_ewa, &self._prop_bed_for_ewa),
            (&self._actor_koa, &self._prop_bed_for_koa),
        ] {
            if let (Some(actor), Some(prop)) = (actor_opt.as_ref(), bed_opt.as_ref()) {
                actor.borrow_mut().set_position(prop.borrow().get_position());
            }
        }
    }

    pub fn get_quest_save_data(&self) -> ScenarioIntroQuestSaveData {
        ScenarioIntroQuestSaveData {
            _has_quest: self._quest.is_some(),
            _quest_title: self._quest.as_ref().and_then(|q| q.borrow()._quest_title.clone()),
            _sub_quest_go_to_the_forest: get_quest_item_save_data(&self._sub_quest_go_to_the_forest),
            _sub_quest_hit_the_tree: get_quest_item_save_data(&self._sub_quest_hit_the_tree),
            _sub_quest_gather_food: get_quest_item_save_data(&self._sub_quest_gather_food),
            _sub_quest_return_home: get_quest_item_save_data(&self._sub_quest_return_home),
            _sub_quest_wrap_up_the_day: get_quest_item_save_data(&self._sub_quest_wrap_up_the_day),
            _sub_quest_cooking: get_quest_item_save_data(&self._sub_quest_cooking),
            _sub_quest_sleep: get_quest_item_save_data(&self._sub_quest_sleep),
        }
    }

    pub fn load_quest_save_data(&mut self, quest_save_data: &ScenarioIntroQuestSaveData) {
        if quest_save_data.has_any_quest() {
            self.destroy_quest();
            self.create_quests();

            load_quest_item_save_data(
                &self._sub_quest_go_to_the_forest,
                &quest_save_data._sub_quest_go_to_the_forest,
            );
            load_quest_item_save_data(&self._sub_quest_hit_the_tree, &quest_save_data._sub_quest_hit_the_tree);
            load_quest_item_save_data(&self._sub_quest_gather_food, &quest_save_data._sub_quest_gather_food);
            load_quest_item_save_data(&self._sub_quest_return_home, &quest_save_data._sub_quest_return_home);
            load_quest_item_save_data(
                &self._sub_quest_wrap_up_the_day,
                &quest_save_data._sub_quest_wrap_up_the_day,
            );
            load_quest_item_save_data(&self._sub_quest_cooking, &quest_save_data._sub_quest_cooking);
            load_quest_item_save_data(&self._sub_quest_sleep, &quest_save_data._sub_quest_sleep);

            if is_quest_item_completed(&self._sub_quest_wrap_up_the_day) {
                if quest_save_data._sub_quest_go_to_the_forest.is_none()
                    || !is_quest_item_completed(&self._sub_quest_go_to_the_forest)
                {
                    self.remove_sub_quest_go_to_the_forest();
                }
                if quest_save_data._sub_quest_hit_the_tree.is_none()
                    || !is_quest_item_completed(&self._sub_quest_hit_the_tree)
                {
                    self.remove_sub_quest_hit_the_tree();
                }
                if quest_save_data._sub_quest_gather_food.is_none()
                    || !is_quest_item_completed(&self._sub_quest_gather_food)
                {
                    self.remove_sub_quest_gather_food();
                }
                if quest_save_data._sub_quest_return_home.is_none()
                    || !is_quest_item_completed(&self._sub_quest_return_home)
                {
                    self.remove_sub_quest_return_home();
                }
                if quest_save_data._sub_quest_cooking.is_none()
                    || (!self.has_any_food() && !is_quest_item_completed(&self._sub_quest_cooking))
                {
                    self.remove_sub_quest_cooking();
                }
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

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &str) {
        self._scenario_track.set_phase_from_str(scenario_phase, ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
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
        !matches!(
            self._scenario_track._scenario_phase,
            ScenarioPhase::GoToTheForest
                | ScenarioPhase::GatheringFood
                | ScenarioPhase::ReturnHome
                | ScenarioPhase::WrapUpTheDay
        )
    }

    fn complete_sub_quest_wrap_up_the_day(&mut self) {
        self.complete_sub_quest_wrap_up_the_day();
    }

    fn complete_sub_quest_cooking(&mut self) {
        if !is_quest_item_completed(&self._sub_quest_wrap_up_the_day) {
            return;
        }

        if complete_quest_item(&self._sub_quest_cooking) {
            self.remove_prop_table_text_box();
        }
        self.complete_sub_quest_wrap_up_the_day();
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
        self._scenario_create_info.spawn_and_reset_if_matching(game_scene_data_name);

        let game_scene_manager = get_game_scene_manager_mut();
        self._player = game_scene_manager.get_maybe_player().clone();
        self._actor_ewa = game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_EWA).cloned();
        self._actor_koa = game_scene_manager.get_actor_by_name(ACTOR_NAME_MONKEY_KOA).cloned();
        self._prop_gate = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();
        self._prop_table = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_TABLE).cloned();
        self._prop_bed_for_aru = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_ARU).cloned();
        self._prop_bed_for_ewa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_EWA).cloned();
        self._prop_bed_for_koa = game_scene_manager.get_prop_manager().get_prop_by_name(BED_FOR_KOA).cloned();
        self._prop_tree = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_INTRO_TREE).cloned();
        self._prop_gate_stage01 = game_scene_manager.get_prop_manager().get_prop_by_name(DEFAULT_GATE_NAME).cloned();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin => {
                self.reset_actors_to_bed_positions();
                let mut pivot = Vector3::new(0.0, CAMERA_OFFSET_Y, 0.0);
                if let Some(actor) = self._player.as_ref() {
                    pivot += *actor.borrow().get_center();
                }
                let start_rotation_matrix = math::make_rotation_matrix(
                    self._around_start_rotation.x,
                    self._around_start_rotation.y,
                    self._around_start_rotation.z,
                );
                self._around_start_position =
                    pivot - start_rotation_matrix.column(2).xyz() * (CAMERA_DISTANCE_MAX + AROUND_CAMERA_EXTRA_DIST);

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
            ScenarioPhase::GoToTheForest => {
                if game_scene_data_name == Stages::Cave.get_stage_data_name()
                    && !is_quest_item_completed(&self._sub_quest_go_to_the_forest)
                {
                    self.create_move_to_forest_text_box();
                }
            }
            ScenarioPhase::GatheringFood => {
                if game_scene_data_name == Stages::Forest.get_stage_data_name()
                    && !is_quest_item_completed(&self._sub_quest_hit_the_tree)
                {
                    self.create_hit_this_tree_text_box();
                }
            }
            ScenarioPhase::ReturnHome => {
                if game_scene_data_name == Stages::Forest.get_stage_data_name()
                    && !is_quest_item_completed(&self._sub_quest_return_home)
                {
                    self.create_return_home_text_box();
                }
            }
            ScenarioPhase::WrapUpTheDay => {
                if game_scene_data_name == Stages::Cave.get_stage_data_name() {
                    if !is_quest_item_completed(&self._sub_quest_wrap_up_the_day) {
                        self.create_actor_ewa_text_box();
                    }
                    if self.is_sub_quest_cooking_active() {
                        self.create_prop_table_text_box();
                    }
                    if self.is_sub_quest_sleep_active() {
                        self.create_prop_bed_for_aru_text_box();
                    }
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
            let Some(update_scenario_phase) = self._scenario_track.resolve_state_phase(
                state,
                prev_scenario_phase,
                next_scenario_phase,
                next_phase_duration,
            ) else {
                continue;
            };

            let phase_time = self._scenario_track.get_phase_time();
            let phase_ratio = self._scenario_track.get_phase_ratio();

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    self.reset_actors_to_bed_positions();
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::StoryBoard, None);
                }
                ScenarioPhase::StoryBoard => match state {
                    State::Begin => {
                        self.reset_actors_to_bed_positions();
                        game_scene_manager.set_time(TIME_OF_DAWN, 0.0);
                        set_actors_behavior_none(&[&self._actor_ewa, &self._actor_koa], false);
                        let forward_x = Vector3::new(1.0, 0.0, 0.0);
                        for actor in [&self._player, &self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                            actor.borrow_mut().set_move_direction(&forward_x, true);
                        }
                    }
                    State::Update => {
                        let story_board_phase = self.get_story_board_phase();
                        if !USE_STORY_BOARDS || STORY_BOARDS.len() < story_board_phase {
                            self._scenario_track
                                .set_next_scenario_phase(ScenarioPhase::Morning, Some(PHASE_TIME_SLEEP));
                        } else if story_board_phase == 0
                            || (game_ui_manager.is_done_game_image_progress() && any_key_pressed)
                        {
                            let material = if story_board_phase < STORY_BOARDS.len() {
                                STORY_BOARDS[story_board_phase]
                            } else {
                                MATERIAL_UI_NONE
                            };
                            game_ui_manager.set_image_auto_fade_inout(material, DEFAULT_FADE_TIME);
                            self.next_story_board_phase();
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::Morning => match state {
                    State::Begin => {
                        for actor in [&self._player, &self._actor_ewa, &self._actor_koa].into_iter().flatten() {
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
                        let time_of_day_ratio = phase_time * WAKEUP_TIME_OF_DAY_SPEED_RATIO;
                        if time_of_day_ratio < 1.0 {
                            game_scene_manager
                                .set_time(math::lerp(TIME_OF_DAWN, TIME_OF_EARLY_MORNING, time_of_day_ratio), 0.0);
                        }

                        let dt = delta_time as f32;
                        for (delay, actor_opt) in [
                            (&mut self._wakeup_delay_aru, &self._player),
                            (&mut self._wakeup_delay_ewa, &self._actor_ewa),
                            (&mut self._wakeup_delay_koa, &self._actor_koa),
                        ] {
                            let prev_delay = *delay;
                            *delay -= dt;
                            if 0.0 <= prev_delay
                                && *delay < 0.0
                                && let Some(actor) = actor_opt
                            {
                                actor.borrow_mut().set_action_wake_up();
                            }
                        }

                        let all_awake = [&self._player, &self._actor_ewa, &self._actor_koa].iter().all(|actor_opt| {
                            actor_opt.as_ref().is_none_or(|a| a.borrow().is_action(ActionAnimationState::None))
                        });

                        if self._wakeup_delay_koa < 0.0
                            && self._wakeup_delay_ewa < 0.0
                            && self._wakeup_delay_aru < 0.0
                            && all_awake
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
                            let aru_pos = *actor_aru.borrow().get_position();
                            actor_aru.borrow_mut().look_at(actor_koa.borrow().get_position());
                            for family_member in [actor_ewa, actor_koa] {
                                let mut member_mut = family_member.borrow_mut();
                                member_mut.look_at(&aru_pos);
                                member_mut.set_hunger(HUNGER_WARNING_THRESHOLD);
                                member_mut.set_action_hungry();
                            }
                        }
                    }
                    State::Update => {
                        if 1.0 <= phase_ratio {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GoToTheForest, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::GoToTheForest => match state {
                    State::Begin => {
                        for actor in [&self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }

                        self.create_quests();
                        self.create_move_to_forest_text_box();
                    }
                    State::Update => {
                        if game_scene_manager.is_teleport_mode() {
                            self.complete_sub_quest_go_to_the_forest();
                        }

                        if game_scene_manager.get_current_game_scene_data_name() == Stages::Forest.get_stage_data_name()
                            && !game_scene_manager.is_teleport_mode()
                        {
                            self.complete_sub_quest_go_to_the_forest();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::GatheringFood, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::GatheringFood => match state {
                    State::Begin => {
                        for actor in [&self._actor_ewa, &self._actor_koa].into_iter().flatten() {
                            actor.borrow_mut().set_next_behavior(BehaviorState::Idle, true);
                        }

                        self.create_quests();
                        self.create_hit_this_tree_text_box();
                    }
                    State::Update => {
                        self.update_tree_fruit_text_boxes();

                        let wrap_up_the_day_not_completed =
                            self._sub_quest_wrap_up_the_day.as_ref().is_some_and(|q| !q.borrow().is_completed_quest());

                        if wrap_up_the_day_not_completed
                            && let Some(scenario_wrap_up_the_day) =
                                game_scene_manager.get_game_scenario(ScenarioType::ScenarioWrapUpTheDay).as_ref()
                        {
                            scenario_wrap_up_the_day.borrow_mut().set_skip_wakeup(true);
                            self.complete_sub_quest_wrap_up_the_day();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
                        } else {
                            if is_quest_item_completed(&self._sub_quest_gather_food) {
                                self.complete_sub_quest_gather_food();
                                self._scenario_track.set_next_scenario_phase(ScenarioPhase::ReturnHome, None);
                            }

                            self.check_hit_the_tree_complete();
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::ReturnHome => match state {
                    State::Begin => {
                        self.create_return_home_text_box();
                    }
                    State::Update => {
                        if game_scene_manager.is_teleport_mode() {
                            self.complete_sub_quest_return_home();
                        }

                        if game_scene_manager.get_current_game_scene_data_name() == Stages::Cave.get_stage_data_name()
                            && !game_scene_manager.is_teleport_mode()
                        {
                            self.complete_sub_quest_return_home();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::WrapUpTheDay, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::WrapUpTheDay => match state {
                    State::Begin => {
                        if !is_quest_item_completed(&self._sub_quest_wrap_up_the_day) {
                            self.create_actor_ewa_text_box();
                        }

                        if self.is_sub_quest_cooking_active() {
                            self.create_prop_table_text_box();
                        } else {
                            self.remove_prop_table_text_box();
                        }

                        if self.is_sub_quest_sleep_active() {
                            self.create_prop_bed_for_aru_text_box();
                        } else {
                            self.remove_prop_bed_for_aru_text_box();
                        }
                    }
                    State::Update => {
                        let wrap_up_the_day_not_completed =
                            self._sub_quest_wrap_up_the_day.as_ref().is_some_and(|q| !q.borrow().is_completed_quest());

                        if wrap_up_the_day_not_completed
                            && let Some(scenario_wrap_up_the_day) =
                                game_scene_manager.get_game_scenario(ScenarioType::ScenarioWrapUpTheDay).as_ref()
                        {
                            scenario_wrap_up_the_day.borrow_mut().set_skip_wakeup(true);
                            self.complete_sub_quest_wrap_up_the_day();
                        }

                        if self.is_sub_quest_sleep_active() {
                            self.create_prop_bed_for_aru_text_box();
                        } else {
                            self.remove_prop_bed_for_aru_text_box();
                        }

                        if let Some(scenario_wrap_up_the_day) =
                            game_scene_manager.get_game_scenario(ScenarioType::ScenarioWrapUpTheDay).as_ref()
                        {
                            if scenario_wrap_up_the_day.borrow().is_sleeping() {
                                self.complete_sub_quest_sleep();
                            }
                        } else if is_quest_item_completed(&self._sub_quest_sleep) {
                            self.clear_all();
                            game_scene_manager.request_open_game_scenario(ScenarioType::ScenarioIntro_Ufo);
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, None);
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {}
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
