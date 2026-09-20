use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{ITEM_WOOD, MATERIAL_UI_POINTER};
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_resources, get_game_scene_manager_mut, get_game_ui_manager,
    get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::{QuestCreateInfo, QuestItemSaveData};
use crate::game_module::widgets::text_box_widget::{TextBoxContent, TextBoxItemOption, TextBoxLayerType};
use nalgebra::Vector2;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioPhase {
    None,
    Begin,
    CraftWoodenClub,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ScenarioWoodenClubQuestSaveData {
    pub _has_quest: bool,
    pub _quest_title: Option<String>,
    pub _sub_quest_unlock_wooden_club: Option<QuestItemSaveData>,
    pub _sub_quest_gather_wood: Option<QuestItemSaveData>,
    pub _sub_quest_craft_wooden_club: Option<QuestItemSaveData>,
}

impl ScenarioWoodenClubQuestSaveData {
    pub fn has_any_quest(&self) -> bool {
        self._has_quest
            || self._sub_quest_unlock_wooden_club.is_some()
            || self._sub_quest_gather_wood.is_some()
            || self._sub_quest_craft_wooden_club.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ScenarioWoodenClubSaveData {
    pub _scenario_create_info: ScenarioDataCreateInfo,
    pub _scenario_phase: String,
    pub _next_scenario_phase: String,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
    #[serde(default)]
    pub _quest_save_data: ScenarioWoodenClubQuestSaveData,
}

pub struct ScenarioWoodenClub<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _player: Option<RcRefCell<Character<'a>>>,
    _prop_monolith: Option<RcRefCell<Prop<'a>>>,
    _quest: Option<RcRefCell<QuestTitle<'a>>>,
    _sub_quest_unlock_wooden_club: Option<QuestItem<'a>>,
    _sub_quest_gather_wood: Option<QuestItem<'a>>,
    _sub_quest_craft_wooden_club: Option<QuestItem<'a>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioWoodenClub<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioWoodenClub<'a>> {
        newRcRefCell(ScenarioWoodenClub {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _player: None,
            _prop_monolith: None,
            _quest: None,
            _sub_quest_unlock_wooden_club: None,
            _sub_quest_gather_wood: None,
            _sub_quest_craft_wooden_club: None,
            _scenario_track: ScenarioTrack {
                _scenario_phase: ScenarioPhase::None,
                _next_scenario_phase: ScenarioPhase::Begin,
                _phase_duration: None,
                _next_phase_duration: None,
                _phase_time: 0.0,
            },
        })
    }

    pub fn create_quests(&mut self) {
        if self._quest.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let item_wood = get_game_resources().get_item_data(ITEM_WOOD);

            self._quest = Some(game_ui_manager.add_quest(Some(String::from("Crafting a Wooden Club"))));
            if let Some(quest) = &self._quest {
                self._sub_quest_unlock_wooden_club = Some(quest.borrow_mut().add_quest_item(
                    QuestCreateInfo::DefaultQuest(DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Unlock Wooden Club in Toolbox")),
                    }),
                ));

                self._sub_quest_gather_wood = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(
                    GatherItemData {
                        _item_data_name: String::from(ITEM_WOOD),
                        _item_data: item_wood.clone(),
                        _gather_item_count: 1,
                    },
                )));

                self._sub_quest_craft_wooden_club = Some(quest.borrow_mut().add_quest_item(
                    QuestCreateInfo::DefaultQuest(DefaultQuestData {
                        _quest_icon_name: None,
                        _quest_description: Some(String::from("Open Inventory & Craft Wooden Club")),
                    }),
                ));
            }
        }
    }

    pub fn destroy_quest(&mut self) {
        if let Some(quest) = &self._quest {
            quest.borrow_mut().destroy_quest();
        }
        self._quest = None;
        self._sub_quest_unlock_wooden_club = None;
        self._sub_quest_gather_wood = None;
        self._sub_quest_craft_wooden_club = None;
    }

    pub fn create_toolbox_text_box(&mut self) {
        if self._prop_monolith.is_none() {
            self._prop_monolith =
                get_game_scene_manager_mut().get_prop_manager().get_prop_by_name("monolith").cloned();
        }
        if let Some(prop_monolith) = self._prop_monolith.as_ref() {
            let wrapper = ActorWrapper::Prop(prop_monolith.clone());
            if !get_game_ui_manager().has_text_box_item(wrapper.get_key()) {
                let contents = vec![TextBoxContent::MaterialInstance(
                    MATERIAL_UI_POINTER.to_string(),
                    Some(Vector2::new(50.0, 50.0)),
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
    }

    pub fn remove_toolbox_text_box(&mut self) {
        if self._prop_monolith.is_none() {
            self._prop_monolith =
                get_game_scene_manager_mut().get_prop_manager().get_prop_by_name("monolith").cloned();
        }
        if let Some(prop_monolith) = self._prop_monolith.as_ref() {
            let wrapper = ActorWrapper::Prop(prop_monolith.clone());
            get_game_ui_manager_mut().remove_text_box_item(wrapper.get_key());
        }
    }

    pub fn complete_sub_quest_unlock_wooden_club(&mut self) {
        if let Some(q) = &self._sub_quest_unlock_wooden_club {
            if !q.borrow().is_completed_quest() {
                q.borrow_mut().set_completed_quest();
                self.remove_toolbox_text_box();
            }
        }
    }

    pub fn get_quest_save_data(&self) -> ScenarioWoodenClubQuestSaveData {
        ScenarioWoodenClubQuestSaveData {
            _has_quest: self._quest.is_some(),
            _quest_title: self._quest.as_ref().and_then(|q| q.borrow()._quest_title.clone()),
            _sub_quest_unlock_wooden_club: self
                ._sub_quest_unlock_wooden_club
                .as_ref()
                .map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_gather_wood: self
                ._sub_quest_gather_wood
                .as_ref()
                .map(|q| q.borrow().get_quest_item_save_data()),
            _sub_quest_craft_wooden_club: self
                ._sub_quest_craft_wooden_club
                .as_ref()
                .map(|q| q.borrow().get_quest_item_save_data()),
        }
    }

    pub fn load_quest_save_data(&mut self, quest_save_data: &ScenarioWoodenClubQuestSaveData) {
        if quest_save_data.has_any_quest() {
            self.destroy_quest();
            self.create_quests();

            if let Some(save_data) = &quest_save_data._sub_quest_unlock_wooden_club
                && let Some(q) = &self._sub_quest_unlock_wooden_club
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_gather_wood
                && let Some(q) = &self._sub_quest_gather_wood
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
            if let Some(save_data) = &quest_save_data._sub_quest_craft_wooden_club
                && let Some(q) = &self._sub_quest_craft_wooden_club
            {
                q.borrow_mut().load_quest_item_save_data(save_data);
            }
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioWoodenClub<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        ScenarioType::ScenarioWoodenClub
    }

    fn get_scenario_phase_as_string(&self) -> String {
        self._scenario_track._scenario_phase.to_string()
    }

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &String) {
        if let Ok(phase) = ScenarioPhase::from_str(scenario_phase.as_str()) {
            self._scenario_track.set_scenario_phase(phase, None);
        }
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        let scenario_save_data: ScenarioWoodenClubSaveData =
            serde_json::from_str(&scenario_save_data._scenario_data).unwrap();
        self._scenario_create_info = scenario_save_data._scenario_create_info;
        let phase = ScenarioPhase::from_str(&scenario_save_data._scenario_phase).unwrap();
        let next_phase = ScenarioPhase::from_str(&scenario_save_data._next_scenario_phase).unwrap();
        self._scenario_track._scenario_phase = phase;
        self._scenario_track._next_scenario_phase = next_phase;
        self._scenario_track._phase_duration = scenario_save_data._phase_duration;
        self._scenario_track._next_phase_duration = scenario_save_data._next_phase_duration;
        self._scenario_track._phase_time = scenario_save_data._phase_time;
        self.load_quest_save_data(&scenario_save_data._quest_save_data);
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let scenario_save_data = ScenarioWoodenClubSaveData {
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_phase: self._scenario_track._scenario_phase.to_string(),
            _next_scenario_phase: self._scenario_track._next_scenario_phase.to_string(),
            _phase_duration: self._scenario_track._phase_duration,
            _next_phase_duration: self._scenario_track._next_phase_duration,
            _phase_time: self._scenario_track._phase_time,
            _quest_save_data: self.get_quest_save_data(),
        };
        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.clone(),
            _scenario_track_create_info: self._scenario_track.save_scenario_track_data(),
            _scenario_data: serde_json::to_string(&scenario_save_data).unwrap(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        false
    }

    fn is_allow_player_control(&self) -> bool {
        true
    }

    fn is_end_of_scenario(&self) -> bool {
        self._scenario_track._scenario_phase == ScenarioPhase::End
    }

    fn destroy_game_scenario(&mut self) {
        self.remove_toolbox_text_box();
        self.destroy_quest();
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.remove_toolbox_text_box();
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager_mut();
        game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
        self._player = get_character_manager().get_maybe_player().clone();
        self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name("monolith").cloned();

        self.create_quests();

        let unlock_not_completed = self
            ._sub_quest_unlock_wooden_club
            .as_ref()
            .is_none_or(|q| !q.borrow().is_completed_quest());
        if unlock_not_completed {
            self.create_toolbox_text_box();
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
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

            match update_scenario_phase {
                ScenarioPhase::None => {
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::Begin, None);
                }
                ScenarioPhase::Begin => {
                    self.create_quests();
                    self.create_toolbox_text_box();
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::CraftWoodenClub, None);
                }
                ScenarioPhase::CraftWoodenClub => match state {
                    State::Begin => {
                        self.create_quests();
                        self.create_toolbox_text_box();
                    }
                    State::Update => {
                        // Check Step 1: Unlock Wooden Club in Toolbox
                        let is_unlocked = get_game_ui_manager()
                            .get_unlocked_toolbox_items()
                            .contains("wooden_club");
                        if is_unlocked {
                            self.complete_sub_quest_unlock_wooden_club();
                        } else {
                            self.create_toolbox_text_box();
                        }

                        // Check Step 3: Craft Wooden Club in Inventory
                        let has_wooden_club =
                            get_game_ui_manager().get_item_count(ItemDataType::WoodenClub.item_code()) >= 1;
                        if has_wooden_club {
                            if let Some(q) = &self._sub_quest_craft_wooden_club {
                                if !q.borrow().is_completed_quest() {
                                    q.borrow_mut().set_completed_quest();
                                }
                            }
                        }

                        let sub3_complete = self
                            ._sub_quest_craft_wooden_club
                            .as_ref()
                            .is_some_and(|q| q.borrow().is_completed_quest());

                        if sub3_complete {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, Some(1.0));
                        }
                    }
                    _ => {}
                },
                ScenarioPhase::End => {
                    if state == State::Begin {
                        self.remove_toolbox_text_box();
                        self.destroy_quest();
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}
