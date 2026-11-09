use crate::game_module::actors::character::Character;
use crate::game_module::actors::items::{Item, ItemDataType};
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    ITEM_ENERGY_BALL, ITEM_NAME_ENERGY_BALL, ITEM_NAME_SPIRIT_BALL, ITEM_NAME_WOOD, ITEM_SPIRIT_BALL, ITEM_WOOD,
    PROP_NAME_MONOLITH,
};
use crate::game_module::game_controller::KeyBindingType;
use crate::game_module::game_scene_manager::Stages;
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_resources, get_game_scene_manager, get_game_scene_manager_mut, get_game_ui_manager,
    get_game_ui_manager_mut,
};
use crate::game_module::game_ui_manager::QuestItem;
use crate::game_module::scenario::scenario::{
    GameScenarioCreateInfo, ScenarioBase, ScenarioDataCreateInfo, ScenarioType,
};
use crate::game_module::scenario::scenario_common::{
    add_item_pointer_text_box, add_prop_pointer_text_box, complete_quest_item, get_quest_item_save_data,
    is_quest_item_completed, load_quest_item_save_data, remove_item_text_box, remove_prop_text_box,
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

const PHASE_END_DURATION: f32 = 2.0;

const QUEST_TITLE_GATHER_GIFTS: &str = "Gather Alien's Gifts";
const QUEST_TITLE_UPGRADE_ABILITY: &str = "Upgrade Weapon Ability";
const QUEST_TITLE_LEARN_AND_CRAFT: &str = "Learn Knowledge & Craft Wooden Club";
const QUEST_TITLE_TELEPORT: &str = "Teleport via OUMUAMUA";

const QUEST_DESC_UPGRADE_ABILITY: &str = "Upgrade Wooden Weapon Damage I from OUMUAMUA";
const QUEST_DESC_UNLOCK_WOODEN_CLUB: &str = "Learn Wooden Weapon Knowledge from OUMUAMUA";
const QUEST_DESC_CRAFT_WOODEN_CLUB: &str = "Craft a Wooden Club (";
const QUEST_HINT_CRAFT_WOODEN_CLUB: &str = "Open craft menu )";
const QUEST_DESC_TELEPORT: &str = "Teleport to Cave Home via OUMUAMUA";

#[derive(Clone, PartialEq, Eq, Hash, Display, Debug, Copy, EnumIter, EnumString, EnumCount)]
pub enum ScenarioPhase {
    None,
    Begin,
    GatherMaterials,
    UsingOumuamua,
    UpgradeWoodenWeaponDamage,
    UnlockWoodenClub,
    CraftWoodenClub,
    Teleport,
    End,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ScenarioUsingOumuamuaQuestSaveData {
    pub _has_quest: bool,
    pub _quest_title: Option<String>,
    pub _sub_quest_gather_wood: Option<QuestItemSaveData>,
    pub _sub_quest_gather_energy_ball: Option<QuestItemSaveData>,
    pub _sub_quest_gather_spirit_ball: Option<QuestItemSaveData>,
    pub _sub_quest_upgrade_wooden_weapon_damage: Option<QuestItemSaveData>,
    pub _sub_quest_unlock_wooden_club: Option<QuestItemSaveData>,
    pub _sub_quest_craft_wooden_club: Option<QuestItemSaveData>,
    pub _sub_quest_teleport: Option<QuestItemSaveData>,
}

impl ScenarioUsingOumuamuaQuestSaveData {
    pub fn has_any_quest(&self) -> bool {
        self._has_quest
            || self._sub_quest_gather_wood.is_some()
            || self._sub_quest_gather_energy_ball.is_some()
            || self._sub_quest_gather_spirit_ball.is_some()
            || self._sub_quest_upgrade_wooden_weapon_damage.is_some()
            || self._sub_quest_unlock_wooden_club.is_some()
            || self._sub_quest_craft_wooden_club.is_some()
            || self._sub_quest_teleport.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ScenarioUsingOumuamuaSaveData {
    #[serde(default, skip_serializing_if = "ScenarioDataCreateInfo::is_empty")]
    pub _scenario_create_info: ScenarioDataCreateInfo,
    pub _scenario_phase: String,
    pub _next_scenario_phase: String,
    pub _phase_duration: Option<f32>,
    pub _next_phase_duration: Option<f32>,
    pub _phase_time: f32,
    #[serde(default)]
    pub _quest_save_data: ScenarioUsingOumuamuaQuestSaveData,
}

pub struct ScenarioUsingOumuamua<'a> {
    _scenario_type: ScenarioType,
    _scenario_create_info: ScenarioDataCreateInfo,
    _player: Option<RcRefCell<Character<'a>>>,
    _prop_monolith: Option<RcRefCell<Prop<'a>>>,
    _item_wood: Option<RcRefCell<Item<'a>>>,
    _item_energy_ball: Option<RcRefCell<Item<'a>>>,
    _item_spirit_ball: Option<RcRefCell<Item<'a>>>,
    _quest_gather_materials: Option<RcRefCell<QuestTitle<'a>>>,
    _quest_upgrade_ability: Option<RcRefCell<QuestTitle<'a>>>,
    _quest_learn_and_craft: Option<RcRefCell<QuestTitle<'a>>>,
    _quest_teleport: Option<RcRefCell<QuestTitle<'a>>>,
    _sub_quest_gather_wood: Option<QuestItem<'a>>,
    _sub_quest_gather_energy_ball: Option<QuestItem<'a>>,
    _sub_quest_gather_spirit_ball: Option<QuestItem<'a>>,
    _sub_quest_upgrade_wooden_weapon_damage: Option<QuestItem<'a>>,
    _sub_quest_unlock_wooden_club: Option<QuestItem<'a>>,
    _sub_quest_craft_wooden_club: Option<QuestItem<'a>>,
    _sub_quest_teleport: Option<QuestItem<'a>>,
    _scenario_track: ScenarioTrack<ScenarioPhase>,
}

impl<'a> ScenarioUsingOumuamua<'a> {
    pub fn create_game_scenario(
        scenario_type: ScenarioType,
        scenario_create_info: &ScenarioDataCreateInfo,
    ) -> RcRefCell<ScenarioUsingOumuamua<'a>> {
        newRcRefCell(ScenarioUsingOumuamua {
            _scenario_type: scenario_type,
            _scenario_create_info: scenario_create_info.clone(),
            _player: None,
            _prop_monolith: None,
            _item_wood: None,
            _item_energy_ball: None,
            _item_spirit_ball: None,
            _quest_gather_materials: None,
            _quest_upgrade_ability: None,
            _quest_learn_and_craft: None,
            _quest_teleport: None,
            _sub_quest_gather_wood: None,
            _sub_quest_gather_energy_ball: None,
            _sub_quest_gather_spirit_ball: None,
            _sub_quest_upgrade_wooden_weapon_damage: None,
            _sub_quest_unlock_wooden_club: None,
            _sub_quest_craft_wooden_club: None,
            _sub_quest_teleport: None,
            _scenario_track: ScenarioTrack::new(ScenarioPhase::None, ScenarioPhase::Begin),
        })
    }

    pub fn create_quest_gather_materials(&mut self) {
        if self._quest_gather_materials.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let game_resources = get_game_resources();
            let item_wood = game_resources.get_item_data(ITEM_WOOD);
            let item_energy_ball = game_resources.get_item_data(ITEM_ENERGY_BALL);
            let item_spirit_ball = game_resources.get_item_data(ITEM_SPIRIT_BALL);

            let quest = game_ui_manager.add_quest(Some(String::from(QUEST_TITLE_GATHER_GIFTS)));
            self._sub_quest_gather_wood = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(
                GatherItemData {
                    _item_data_name: String::from(ITEM_WOOD),
                    _item_data: item_wood.clone(),
                    _gather_item_count: 1,
                },
            )));

            self._sub_quest_gather_energy_ball = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(
                GatherItemData {
                    _item_data_name: String::from(ITEM_ENERGY_BALL),
                    _item_data: item_energy_ball.clone(),
                    _gather_item_count: 1,
                },
            )));

            self._sub_quest_gather_spirit_ball = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::GatherItem(
                GatherItemData {
                    _item_data_name: String::from(ITEM_SPIRIT_BALL),
                    _item_data: item_spirit_ball.clone(),
                    _gather_item_count: 1,
                },
            )));

            self._quest_gather_materials = Some(quest);
        }
    }

    pub fn create_quest_upgrade_ability(&mut self) {
        if self._quest_upgrade_ability.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let quest = game_ui_manager.add_quest(Some(String::from(QUEST_TITLE_UPGRADE_ABILITY)));
            self._sub_quest_upgrade_wooden_weapon_damage = Some(quest.borrow_mut().add_quest_item(
                QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from(QUEST_DESC_UPGRADE_ABILITY)),
                    _key_binding_type: None,
                    _key_binding_text: None,
                }),
            ));
            self._quest_upgrade_ability = Some(quest);
        }
    }

    pub fn create_quest_learn_and_craft(&mut self) {
        if self._quest_learn_and_craft.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let quest = game_ui_manager.add_quest(Some(String::from(QUEST_TITLE_LEARN_AND_CRAFT)));
            self._sub_quest_unlock_wooden_club = Some(quest.borrow_mut().add_quest_item(
                QuestCreateInfo::DefaultQuest(DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from(QUEST_DESC_UNLOCK_WOODEN_CLUB)),
                    _key_binding_type: None,
                    _key_binding_text: None,
                }),
            ));

            self._sub_quest_craft_wooden_club = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from(QUEST_DESC_CRAFT_WOODEN_CLUB)),
                    _key_binding_type: Some(KeyBindingType::Craft),
                    _key_binding_text: Some(String::from(QUEST_HINT_CRAFT_WOODEN_CLUB)),
                },
            )));
            self._quest_learn_and_craft = Some(quest);
        }
    }

    pub fn create_quest_teleport(&mut self) {
        if self._quest_teleport.is_none() {
            let game_ui_manager = get_game_ui_manager_mut();
            let quest = game_ui_manager.add_quest(Some(String::from(QUEST_TITLE_TELEPORT)));
            self._sub_quest_teleport = Some(quest.borrow_mut().add_quest_item(QuestCreateInfo::DefaultQuest(
                DefaultQuestData {
                    _quest_icon_name: None,
                    _quest_description: Some(String::from(QUEST_DESC_TELEPORT)),
                    _key_binding_type: None,
                    _key_binding_text: None,
                },
            )));
            self._quest_teleport = Some(quest);
        }
    }

    pub fn create_parallel_quests(&mut self) {
        self.create_quest_upgrade_ability();
        self.create_quest_learn_and_craft();
        self.create_quest_teleport();
    }

    pub fn create_quests(&mut self) {
        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin | ScenarioPhase::GatherMaterials => {
                self.create_quest_gather_materials();
            }
            ScenarioPhase::UsingOumuamua
            | ScenarioPhase::UpgradeWoodenWeaponDamage
            | ScenarioPhase::UnlockWoodenClub
            | ScenarioPhase::CraftWoodenClub
            | ScenarioPhase::Teleport => {
                self.create_parallel_quests();
            }
            ScenarioPhase::End => {}
        }
    }

    pub fn destroy_quest_gather_materials(&mut self) {
        if let Some(quest) = &self._quest_gather_materials {
            quest.borrow_mut().destroy_quest();
        }
        self._quest_gather_materials = None;
        self._sub_quest_gather_wood = None;
        self._sub_quest_gather_energy_ball = None;
        self._sub_quest_gather_spirit_ball = None;
    }

    pub fn destroy_quest_upgrade_ability(&mut self) {
        if let Some(quest) = &self._quest_upgrade_ability {
            quest.borrow_mut().destroy_quest();
        }
        self._quest_upgrade_ability = None;
        self._sub_quest_upgrade_wooden_weapon_damage = None;
    }

    pub fn destroy_quest_learn_and_craft(&mut self) {
        if let Some(quest) = &self._quest_learn_and_craft {
            quest.borrow_mut().destroy_quest();
        }
        self._quest_learn_and_craft = None;
        self._sub_quest_unlock_wooden_club = None;
        self._sub_quest_craft_wooden_club = None;
    }

    pub fn destroy_quest_teleport(&mut self) {
        if let Some(quest) = &self._quest_teleport {
            quest.borrow_mut().destroy_quest();
        }
        self._quest_teleport = None;
        self._sub_quest_teleport = None;
    }

    pub fn destroy_all_quests(&mut self) {
        self.destroy_quest_gather_materials();
        self.destroy_quest_upgrade_ability();
        self.destroy_quest_learn_and_craft();
        self.destroy_quest_teleport();
    }

    fn ensure_player(&mut self) {
        if self._player.is_none() {
            self._player = get_character_manager().get_maybe_player().clone();
        }
    }

    fn ensure_monolith_prop(&mut self) {
        if self._prop_monolith.is_none() {
            self._prop_monolith =
                get_game_scene_manager_mut().get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();
        }
    }

    fn ensure_items(&mut self) {
        let item_manager = get_game_scene_manager().get_item_manager();
        if self._item_wood.is_none() {
            self._item_wood = item_manager.get_item_by_name(ITEM_NAME_WOOD).cloned();
        }
        if self._item_energy_ball.is_none() {
            self._item_energy_ball = item_manager.get_item_by_name(ITEM_NAME_ENERGY_BALL).cloned();
        }
        if self._item_spirit_ball.is_none() {
            self._item_spirit_ball = item_manager.get_item_by_name(ITEM_NAME_SPIRIT_BALL).cloned();
        }
    }

    pub fn create_item_text_boxes(&mut self) {
        self.ensure_items();
        if !is_quest_item_completed(&self._sub_quest_gather_wood) {
            add_item_pointer_text_box(&self._item_wood, true);
        }
        if !is_quest_item_completed(&self._sub_quest_gather_energy_ball) {
            add_item_pointer_text_box(&self._item_energy_ball, true);
        }
        if !is_quest_item_completed(&self._sub_quest_gather_spirit_ball) {
            add_item_pointer_text_box(&self._item_spirit_ball, true);
        }
    }

    pub fn remove_item_text_boxes(&mut self) {
        self.ensure_items();
        remove_item_text_box(&self._item_wood);
        remove_item_text_box(&self._item_energy_ball);
        remove_item_text_box(&self._item_spirit_ball);
    }

    pub fn update_item_text_boxes(&mut self) {
        self.ensure_items();
        if is_quest_item_completed(&self._sub_quest_gather_wood) {
            remove_item_text_box(&self._item_wood);
        } else {
            add_item_pointer_text_box(&self._item_wood, true);
        }
        if is_quest_item_completed(&self._sub_quest_gather_energy_ball) {
            remove_item_text_box(&self._item_energy_ball);
        } else {
            add_item_pointer_text_box(&self._item_energy_ball, true);
        }
        if is_quest_item_completed(&self._sub_quest_gather_spirit_ball) {
            remove_item_text_box(&self._item_spirit_ball);
        } else {
            add_item_pointer_text_box(&self._item_spirit_ball, true);
        }
    }

    pub fn is_all_gather_quests_completed(&self) -> bool {
        is_quest_item_completed(&self._sub_quest_gather_wood)
            && is_quest_item_completed(&self._sub_quest_gather_energy_ball)
            && is_quest_item_completed(&self._sub_quest_gather_spirit_ball)
    }

    pub fn is_all_parallel_quests_completed(&self) -> bool {
        is_quest_item_completed(&self._sub_quest_upgrade_wooden_weapon_damage)
            && is_quest_item_completed(&self._sub_quest_unlock_wooden_club)
            && is_quest_item_completed(&self._sub_quest_craft_wooden_club)
            && is_quest_item_completed(&self._sub_quest_teleport)
    }

    pub fn is_monolith_interaction_needed(&self) -> bool {
        !is_quest_item_completed(&self._sub_quest_upgrade_wooden_weapon_damage)
            || !is_quest_item_completed(&self._sub_quest_unlock_wooden_club)
            || !is_quest_item_completed(&self._sub_quest_teleport)
    }

    pub fn create_toolbox_text_box(&mut self) {
        self.ensure_monolith_prop();
        add_prop_pointer_text_box(&self._prop_monolith, true);
    }

    pub fn remove_toolbox_text_box(&mut self) {
        self.ensure_monolith_prop();
        remove_prop_text_box(&self._prop_monolith);
    }

    pub fn update_toolbox_text_box(&mut self) {
        if self.is_monolith_interaction_needed() {
            self.create_toolbox_text_box();
        } else {
            self.remove_toolbox_text_box();
        }
    }

    pub fn get_quest_save_data(&self) -> ScenarioUsingOumuamuaQuestSaveData {
        let has_quest = self._quest_gather_materials.is_some()
            || self._quest_upgrade_ability.is_some()
            || self._quest_learn_and_craft.is_some()
            || self._quest_teleport.is_some();

        ScenarioUsingOumuamuaQuestSaveData {
            _has_quest: has_quest,
            _quest_title: None,
            _sub_quest_gather_wood: get_quest_item_save_data(&self._sub_quest_gather_wood),
            _sub_quest_gather_energy_ball: get_quest_item_save_data(&self._sub_quest_gather_energy_ball),
            _sub_quest_gather_spirit_ball: get_quest_item_save_data(&self._sub_quest_gather_spirit_ball),
            _sub_quest_upgrade_wooden_weapon_damage: get_quest_item_save_data(
                &self._sub_quest_upgrade_wooden_weapon_damage,
            ),
            _sub_quest_unlock_wooden_club: get_quest_item_save_data(&self._sub_quest_unlock_wooden_club),
            _sub_quest_craft_wooden_club: get_quest_item_save_data(&self._sub_quest_craft_wooden_club),
            _sub_quest_teleport: get_quest_item_save_data(&self._sub_quest_teleport),
        }
    }

    pub fn load_quest_save_data(&mut self, quest_save_data: &ScenarioUsingOumuamuaQuestSaveData) {
        if quest_save_data.has_any_quest() {
            self.destroy_all_quests();
            self.create_quests();

            load_quest_item_save_data(&self._sub_quest_gather_wood, &quest_save_data._sub_quest_gather_wood);
            load_quest_item_save_data(
                &self._sub_quest_gather_energy_ball,
                &quest_save_data._sub_quest_gather_energy_ball,
            );
            load_quest_item_save_data(
                &self._sub_quest_gather_spirit_ball,
                &quest_save_data._sub_quest_gather_spirit_ball,
            );
            load_quest_item_save_data(
                &self._sub_quest_upgrade_wooden_weapon_damage,
                &quest_save_data._sub_quest_upgrade_wooden_weapon_damage,
            );
            load_quest_item_save_data(
                &self._sub_quest_unlock_wooden_club,
                &quest_save_data._sub_quest_unlock_wooden_club,
            );
            load_quest_item_save_data(
                &self._sub_quest_craft_wooden_club,
                &quest_save_data._sub_quest_craft_wooden_club,
            );
            load_quest_item_save_data(&self._sub_quest_teleport, &quest_save_data._sub_quest_teleport);
        }
    }
}

impl<'a> ScenarioBase<'a> for ScenarioUsingOumuamua<'a> {
    fn get_scenario_type(&self) -> ScenarioType {
        ScenarioType::ScenarioUsingOumuamua
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

        match serde_json::from_str::<ScenarioUsingOumuamuaSaveData>(&scenario_save_data._scenario_data) {
            Ok(save_data) => {
                if !save_data._scenario_create_info.is_empty() {
                    self._scenario_create_info = save_data._scenario_create_info;
                }
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
            Err(error) => {
                log::error!("Failed to deserialize ScenarioUsingOumuamuaSaveData: {error:?}");
            }
        }
        self._scenario_create_info.restore_static_scene_from_template(self.get_scenario_type());
    }

    fn get_scenario_save_data(&self) -> GameScenarioCreateInfo {
        let scenario_save_data = ScenarioUsingOumuamuaSaveData {
            _scenario_create_info: ScenarioDataCreateInfo::default(),
            _scenario_phase: self._scenario_track._scenario_phase.to_string(),
            _next_scenario_phase: self._scenario_track._next_scenario_phase.to_string(),
            _phase_duration: self._scenario_track._phase_duration,
            _next_phase_duration: self._scenario_track._next_phase_duration,
            _phase_time: self._scenario_track._phase_time,
            _quest_save_data: self.get_quest_save_data(),
        };
        GameScenarioCreateInfo {
            _scenario_type: self.get_scenario_type(),
            _scenario_create_info: self._scenario_create_info.get_save_data(),
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
        self.remove_item_text_boxes();
        self.remove_toolbox_text_box();
        self.destroy_all_quests();
    }

    fn on_close_game_scene(&mut self, _game_scene_data_name: &str) {
        self.remove_item_text_boxes();
        self.remove_toolbox_text_box();
    }

    fn on_open_game_scene(&mut self, _game_scene_data_name: &str) {
        let game_scene_manager = get_game_scene_manager_mut();
        game_scene_manager.spawn_game_scenario_objects(&self._scenario_create_info);
        self._scenario_create_info.reset();
        self._player = get_character_manager().get_maybe_player().clone();
        self._prop_monolith = game_scene_manager.get_prop_manager().get_prop_by_name(PROP_NAME_MONOLITH).cloned();

        self.create_quests();

        match self._scenario_track._scenario_phase {
            ScenarioPhase::None | ScenarioPhase::Begin | ScenarioPhase::GatherMaterials => {
                if !self.is_all_gather_quests_completed() {
                    self.create_item_text_boxes();
                }
            }
            ScenarioPhase::UsingOumuamua
            | ScenarioPhase::UpgradeWoodenWeaponDamage
            | ScenarioPhase::UnlockWoodenClub
            | ScenarioPhase::CraftWoodenClub
            | ScenarioPhase::Teleport => {
                if self.is_monolith_interaction_needed() {
                    self.create_toolbox_text_box();
                }
                if is_quest_item_completed(&self._sub_quest_unlock_wooden_club) {
                    get_game_ui_manager_mut().set_controls_visibility(true);
                }
            }
            ScenarioPhase::End => {}
        }
    }

    fn update_game_scenario(&mut self, _any_key_hold: bool, _any_key_pressed: bool, delta_time: f64) {
        self.ensure_player();

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
                    self.create_quest_gather_materials();
                    self.create_item_text_boxes();
                    self._scenario_track.set_next_scenario_phase(ScenarioPhase::GatherMaterials, None);
                }
                ScenarioPhase::GatherMaterials => match state {
                    State::Begin => {
                        self.create_quest_gather_materials();
                        self.create_item_text_boxes();
                    }
                    State::Update => {
                        self.update_item_text_boxes();
                        if self.is_all_gather_quests_completed() {
                            self.remove_item_text_boxes();
                            self.destroy_quest_gather_materials();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::UsingOumuamua, None);
                        }
                    }
                    State::End => {
                        self.remove_item_text_boxes();
                    }
                },
                ScenarioPhase::UsingOumuamua
                | ScenarioPhase::UpgradeWoodenWeaponDamage
                | ScenarioPhase::UnlockWoodenClub
                | ScenarioPhase::CraftWoodenClub
                | ScenarioPhase::Teleport => match state {
                    State::Begin => {
                        self.create_parallel_quests();
                        self.create_toolbox_text_box();
                    }
                    State::Update => {
                        // Quest 2: 무기 공격력 강화 체크
                        let is_ability_upgraded = get_game_ui_manager()
                            .get_unlocked_toolbox_items()
                            .contains(&ToolboxIconType::WoodenWeaponDamage1);
                        if is_ability_upgraded {
                            complete_quest_item(&self._sub_quest_upgrade_wooden_weapon_damage);
                        }

                        // Quest 3-1: 무기 제작 지식 습득 체크
                        let is_unlocked =
                            get_game_ui_manager().get_unlocked_toolbox_items().contains(&ToolboxIconType::WoodenWeapon)
                                || get_game_ui_manager()
                                    .get_unlocked_toolbox_items()
                                    .contains(&ToolboxIconType::WoodenClub);
                        if is_unlocked {
                            complete_quest_item(&self._sub_quest_unlock_wooden_club);
                            get_game_ui_manager_mut().set_controls_visibility(true);
                        }

                        // Quest 3-2: 무기 제작 체크
                        let has_wooden_club =
                            get_game_ui_manager().get_item_count(ItemDataType::WoodenClub.item_code()) >= 1;
                        if has_wooden_club {
                            complete_quest_item(&self._sub_quest_craft_wooden_club);
                        }

                        // Quest 4: 텔레포트 체크
                        let game_scene_manager = get_game_scene_manager();
                        let is_teleporting = game_scene_manager.is_teleport_mode();
                        let reached_jungle = game_scene_manager.get_current_game_scene_data_name()
                            == Stages::Jungle.get_stage_data_name();
                        if is_teleporting || reached_jungle {
                            complete_quest_item(&self._sub_quest_teleport);
                        }

                        // 모놀리스 포인터 갱신
                        self.update_toolbox_text_box();

                        // 3개 퀘스트 모두 완료 시 종료 단계로 전환
                        if self.is_all_parallel_quests_completed() {
                            self.remove_toolbox_text_box();
                            self._scenario_track.set_next_scenario_phase(ScenarioPhase::End, Some(PHASE_END_DURATION));
                        }
                    }
                    State::End => {
                        self.remove_toolbox_text_box();
                    }
                },
                ScenarioPhase::End => {
                    if state == State::Begin {
                        self.remove_item_text_boxes();
                        self.remove_toolbox_text_box();
                        self.destroy_all_quests();
                    }
                }
            }

            if state == State::Update {
                self._scenario_track.update_scenario_phase_time(delta_time as f32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_using_oumuamua_save_data_deserialization_without_scenario_create_info() {
        let json_data = r#"{
            "_scenario_phase": "None",
            "_next_scenario_phase": "None",
            "_phase_duration": null,
            "_next_phase_duration": null,
            "_phase_time": 0.0,
            "_quest_save_data": {
                "_has_quest": true,
                "_quest_title": "Gather materials",
                "_sub_quest_gather_wood": {"_is_completed": true},
                "_sub_quest_gather_energy_ball": {"_is_completed": false}
            }
        }"#;

        let save_data: ScenarioUsingOumuamuaSaveData =
            serde_json::from_str(json_data).expect("ScenarioUsingOumuamuaSaveData deserialization should succeed");
        assert_eq!(save_data._scenario_phase, "None");
        assert!(save_data._scenario_create_info.is_empty());
        assert!(save_data._quest_save_data._has_quest);

        let wood_quest =
            save_data._quest_save_data._sub_quest_gather_wood.expect("_sub_quest_gather_wood must be present");
        assert!(wood_quest._is_completed);

        let energy_quest = save_data
            ._quest_save_data
            ._sub_quest_gather_energy_ball
            .expect("_sub_quest_gather_energy_ball must be present");
        assert!(!energy_quest._is_completed);
    }

    #[test]
    fn test_scenario_using_oumuamua_save_data_roundtrip_omits_empty_create_info() {
        let save_data = ScenarioUsingOumuamuaSaveData {
            _scenario_create_info: ScenarioDataCreateInfo::default(),
            _scenario_phase: "Teleport".to_string(),
            _next_scenario_phase: "Teleport".to_string(),
            _phase_duration: None,
            _next_phase_duration: None,
            _phase_time: 5.0,
            _quest_save_data: ScenarioUsingOumuamuaQuestSaveData::default(),
        };

        let serialized = serde_json::to_string(&save_data).expect("Serialization failed");
        assert!(!serialized.contains("_scenario_create_info"));

        let deserialized: ScenarioUsingOumuamuaSaveData =
            serde_json::from_str(&serialized).expect("Deserialization failed");
        assert_eq!(deserialized._scenario_phase, "Teleport");
        assert!(deserialized._scenario_create_info.is_empty());
    }

    #[test]
    fn test_teleport_stage_discovery_and_icon_type_mapping() {
        assert_eq!(
            ToolboxIconType::from_stage_data_name("game_scenes/jungle"),
            Some(ToolboxIconType::MapJungle)
        );
        assert_eq!(
            ToolboxIconType::from_stage_data_name("game_scenes/stage_cave"),
            Some(ToolboxIconType::MapHome)
        );
        assert_eq!(
            ToolboxIconType::from_stage_data_name("game_scenes/forest"),
            Some(ToolboxIconType::MapForest)
        );
        assert_eq!(
            ToolboxIconType::from_stage_data_name("game_scenes/desert"),
            Some(ToolboxIconType::MapDesert)
        );
        assert_eq!(ToolboxIconType::MapJungle.stage_data_name(), Some("game_scenes/jungle"));
        assert_eq!(ToolboxIconType::MapJungle.as_str(), "Jungle");
        assert_eq!(ToolboxIconType::MapDesert.stage_data_name(), Some("game_scenes/desert"));
        assert_eq!(ToolboxIconType::MapDesert.as_str(), "Desert");
    }
}
