use crate::game_module::actors::character::{CharacterDataType, CharacterID};
use crate::game_module::game_service_locator::{
    get_game_client, get_game_client_mut, get_game_resources, get_game_scene_manager,
};
use nalgebra::Vector3;
use rust_engine_3d::utilities::system::extract_name_and_uuid;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DisplayCharacterInfo {
    pub character_id: CharacterID,
    pub name: String,
    pub scene_data_name: String,
    pub scene_display_name: String,
    pub is_alive: bool,
    pub is_tamed: bool,
    pub is_civilian: bool,
    pub hp: i32,
    pub max_hp: i32,
    pub intimacy: f32,
    pub position: Vector3<f32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AffinityTier {
    Neutral,
    Acquaintance,
    Friend,
    CloseFriend,
    BestFriend,
}

impl AffinityTier {
    pub fn get_display_name(&self) -> &'static str {
        match self {
            AffinityTier::BestFriend => "Best Friend",
            AffinityTier::CloseFriend => "Close Friend",
            AffinityTier::Friend => "Friend",
            AffinityTier::Acquaintance => "Acquaintance",
            AffinityTier::Neutral => "Neutral",
        }
    }
}

pub fn get_affinity_tier(intimacy: f32) -> AffinityTier {
    if intimacy >= 100.0 {
        AffinityTier::BestFriend
    } else if intimacy >= 50.0 {
        AffinityTier::CloseFriend
    } else if intimacy >= 20.0 {
        AffinityTier::Friend
    } else if intimacy > 0.0 {
        AffinityTier::Acquaintance
    } else {
        AffinityTier::Neutral
    }
}

pub fn get_scene_display_name(scene_data_name: &str) -> String {
    match scene_data_name {
        "game_scenes/intro_stage" => "HOME".to_string(),
        "game_scenes/stage_01" => "FOREST".to_string(),
        "game_scenes/stage_cave" => "CAVE".to_string(),
        "game_scenes/world_map" => "WORLD MAP".to_string(),
        "game_scenes/stage_ufo" => "UFO".to_string(),
        "" => "UNKNOWN".to_string(),
        other => {
            let name = other.rsplit('/').next().unwrap_or(other);
            if name.is_empty() {
                "UNKNOWN".to_string()
            } else {
                name.to_uppercase()
            }
        }
    }
}

pub fn collect_all_characters() -> Vec<DisplayCharacterInfo> {
    // 1. Sync current scene state into game_save_data
    {
        let game_scene_manager = get_game_scene_manager();
        game_scene_manager.update_game_scene_save_data(&mut get_game_client_mut().get_game_save_data().borrow_mut());
    }

    let game_scene_manager = get_game_scene_manager();
    let game_save_data = get_game_client().get_game_save_data().borrow();
    let current_scene_data_name = game_scene_manager.get_current_game_scene_data_name().clone();
    let game_resources = get_game_resources();

    let mut result = Vec::new();
    let mut seen_ids = HashSet::new();

    // 2. Process active characters in the current loaded scene from CharacterManager
    let character_manager = game_scene_manager.get_character_manager();
    let active_characters = character_manager.get_characters();

    for (char_id, char_ref) in active_characters.iter() {
        let char_borrow = char_ref.borrow();
        if char_borrow.is_player() {
            continue;
        }

        seen_ids.insert(*char_id);

        let scene_display = get_scene_display_name(&current_scene_data_name);
        result.push(DisplayCharacterInfo {
            character_id: *char_id,
            name: char_borrow._character_name.clone(),
            scene_data_name: current_scene_data_name.clone(),
            scene_display_name: scene_display,
            is_alive: char_borrow.is_alive(),
            is_tamed: char_borrow.is_tamed(),
            is_civilian: char_borrow.is_civilian(),
            hp: char_borrow._character_stats.get_hp(),
            max_hp: char_borrow._character_stats.get_max_hp(),
            intimacy: char_borrow.get_intimacy(),
            position: *char_borrow.get_position(),
        });
    }

    // 3. Process characters from all saved scenes in GameSaveData
    for (scene_data_name, scene_save_data) in game_save_data._game_scenes.iter() {
        for (map_key, char_save) in scene_save_data._characters.iter() {
            let char_id = char_save._character_create_info._character_id;
            if seen_ids.contains(&char_id) {
                continue;
            }
            seen_ids.insert(char_id);

            let (extracted_name, _uuid) = extract_name_and_uuid(map_key);
            let name = if !extracted_name.is_empty() {
                extracted_name
            } else {
                char_save._character_create_info._character_data_name.clone()
            };

            let data_name = &char_save._character_create_info._character_data_name;
            let is_civilian = if game_resources.has_character_data(data_name.as_str()) {
                matches!(
                    game_resources.get_character_data(data_name.as_str()).borrow()._character_type,
                    CharacterDataType::Civilian | CharacterDataType::Chef | CharacterDataType::Crafter
                )
            } else {
                false
            };

            let scene_display = get_scene_display_name(scene_data_name);

            result.push(DisplayCharacterInfo {
                character_id: char_id,
                name,
                scene_data_name: scene_data_name.clone(),
                scene_display_name: scene_display,
                is_alive: char_save._character_stats._is_alive,
                is_tamed: char_save._character_stats._is_tamed,
                is_civilian,
                hp: char_save._character_stats._hp,
                max_hp: char_save._character_stats._max_hp,
                intimacy: char_save._character_stats._intimacy,
                position: char_save._character_create_info._position,
            });
        }
    }

    result
}
