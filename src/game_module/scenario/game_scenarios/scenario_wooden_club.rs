use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::ItemDataType;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{ITEM_WOOD, PROP_NAME_MONOLITH};
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_resources, get_game_scene_manager_mut, get_game_ui_manager, get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{
    add_prop_pointer_text_box, complete_quest_item, get_quest_item_save_data, is_quest_item_completed,
    load_quest_item_save_data, remove_prop_text_box,
};
use crate::game_module::scenario::scenario_track::ScenarioTrack;
use crate::game_module::widgets::quest_widgets::quest_item_default::DefaultQuestData;
use crate::game_module::widgets::quest_widgets::quest_item_gather_item::GatherItemData;
use crate::game_module::widgets::quest_widgets::quest_title::QuestTitle;
use crate::game_module::widgets::quest_widgets::quest_widget::{QuestCreateInfo, QuestItemSaveData};
use crate::game_module::widgets::toolbox_widget::item_tab_widget::ToolboxIconType;
use rust_engine_3d::utilities::system::{RcRefCell, State, newRcRefCell};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const PHASE_END_DURATION: f32 = 1.0;

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
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
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
                        _quest_description: Some(String::from("Open Craft Menu & Craft Wooden Club")),
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

    fn ensure_monolith_prop(&mut self) {
        if self._prop_monolith.is_none() {
            self._prop_monolith =
                get_game_scene_manager_mut().get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();
        }
    }

    pub fn create_toolbox_text_box(&mut self) {
        self.ensure_monolith_prop();
        add_prop_pointer_text_box(&self._prop_monolith, true);
    }

    pub fn remove_toolbox_text_box(&mut self) {
        self.ensure_monolith_prop();
        remove_prop_text_box(&self._prop_monolith);
    }

    pub fn complete_sub_quest_unlock_wooden_club(&mut self) {
        if complete_quest_item(&self._sub_quest_unlock_wooden_club) {
            self.remove_toolbox_text_box();
        }
    }

    pub fn get_quest_save_data(&self) -> ScenarioWoodenClubQuestSaveData {
        ScenarioWoodenClubQuestSaveData {
            _has_quest: self._quest.is_some(),
            _quest_title: self._quest.as_ref().and_then(|q| q.borrow()._quest_title.clone()),
            _sub_quest_unlock_wooden_club: get_quest_item_save_data(&self._sub_quest_unlock_wooden_club),
            _sub_quest_gather_wood: get_quest_item_save_data(&self._sub_quest_gather_wood),
            _sub_quest_craft_wooden_club: get_quest_item_save_data(&self._sub_quest_craft_wooden_club),
        }
    }

    pub fn load_quest_save_data(&mut self, quest_save_data: &ScenarioWoodenClubQuestSaveData) {
        if quest_save_data.has_any_quest() {
            self.destroy_quest();
            self.create_quests();

            load_quest_item_save_data(
                &self._sub_quest_unlock_wooden_club,
                &quest_save_data._sub_quest_unlock_wooden_club,
            );
            load_quest_item_save_data(&self._sub_quest_gather_wood, &quest_save_data._sub_quest_gather_wood);
            load_quest_item_save_data(
                &self._sub_quest_craft_wooden_club,
                &quest_save_data._sub_quest_craft_wooden_club,
            );
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

    fn set_scenario_phase_as_string(&mut self, scenario_phase: &str) {
        self._scenario_track.set_phase_from_str(scenario_phase, ScenarioPhase::None);
    }

    fn load_scenario_save_data(&mut self, scenario_save_data: &GameScenarioCreateInfo) {
        self._scenario_create_info = scenario_save_data._scenario_create_info.clone();
        self._scenario_track.load_scenario_track_data(&scenario_save_data._scenario_track_create_info);

        if let Ok(save_data) = serde_json::from_str::<ScenarioWoodenClubSaveData>(&scenario_save_data._scenario_data) {
            self._scenario_create_info = save_data._scenario_create_info;
            if let Ok(phase) = save_data._scenario_phase.parse::<ScenarioPhase>() {
                self._scenario_track._scenario_phase = phase;
            }
            if let Ok(next_phase) = save_data._next_scenario_phase.parse::<ScenarioPhase>() {
                self._scenario_track._next_scenario_phase = next_phase;
            }
            self._scenario_track._phase_duration = save_data._phase_duration;
            self._scenario_track._next_phase_duration = save_data._next_phase_duration;
            self._scenario_track._phase_time = save_data._phase_time;
            self.load_quest_save_data(&save_data._quest_save_data);
        }
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
            _scenario_data: serde_json::to_string(&scenario_save_data).unwrap_or_default(),
        }
    }

    fn is_play_scenario_mode(&self) -> bool {
        false
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
        self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();

        self.create_quests();

        if !is_quest_item_completed(&self._sub_quest_unlock_wooden_club) {
            self.create_toolbox_text_box();
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
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
                        let is_unlocked =
                            get_game_ui_manager().get_unlocked_toolbox_items().contains(&ToolboxIconType::WoodenClub);
                        if is_unlocked {
                            self.complete_sub_quest_unlock_wooden_club();
                        } else {
                            self.create_toolbox_text_box();
                        }

                        let has_wooden_club =
                            get_game_ui_manager().get_item_count(ItemDataType::WoodenClub.item_code()) >= 1;
                        if has_wooden_club {
                            complete_quest_item(&self._sub_quest_craft_wooden_club);
                        }

                        if is_quest_item_completed(&self._sub_quest_craft_wooden_club) {
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, Some(PHASE_END_DURATION));
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
