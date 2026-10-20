use crate::game_module::actors::character::{CharacterDataType, CharacterID};
use crate::game_module::game_service_locator::{
    get_game_client, get_game_client_mut, get_game_resources, get_game_scene_manager,
};
use nalgebra::Vector3;
use rust_engine_3d::scene::ui::{
    HorizontalAlign, Orientation, PIVOT_CENTER, UILayoutType, UIManager, UIWidgetTypes, VerticalAlign, WidgetDefault,
};
use rust_engine_3d::utilities::system::{extract_name_and_uuid, ptr_as_mut};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use std::collections::HashSet;
use std::rc::Rc;

pub const CHAR_PANEL_BG_COLOR: u32 = get_color32(240, 222, 186, 248);
pub const CHAR_PANEL_BORDER_COLOR: u32 = get_color32(135, 78, 42, 255);
pub const CHAR_PANEL_TITLE_COLOR: u32 = get_color32(75, 45, 20, 255);
pub const CHAR_LIST_BG_COLOR: u32 = get_color32(252, 245, 226, 230);
pub const CHAR_CARD_BG_COLOR: u32 = get_color32(245, 232, 205, 230);
pub const CHAR_CARD_BORDER_COLOR: u32 = get_color32(200, 165, 125, 200);
pub const CHAR_STATUS_ALIVE_COLOR: u32 = get_color32(100, 255, 100, 255);
pub const CHAR_STATUS_DEAD_COLOR: u32 = get_color32(255, 100, 100, 255);
pub const CHAR_EMPTY_TEXT_COLOR: u32 = get_color32(180, 180, 180, 255);

#[derive(Clone, Debug)]
pub struct DisplayCharacterInfo {
    pub _character_id: CharacterID,
    pub _name: String,
    pub _scene_data_name: String,
    pub _scene_display_name: String,
    pub _is_alive: bool,
    pub _is_tamed: bool,
    pub _is_civilian: bool,
    pub _hp: i32,
    pub _max_hp: i32,
    pub _intimacy: f32,
    pub _position: Vector3<f32>,
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
        "game_scenes/intro_stage" => "FOREST".to_string(),
        "game_scenes/stage_cave" => "HOME".to_string(),
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
            _character_id: *char_id,
            _name: char_borrow._character_name.clone(),
            _scene_data_name: current_scene_data_name.clone(),
            _scene_display_name: scene_display,
            _is_alive: char_borrow.is_alive(),
            _is_tamed: char_borrow.is_tamed(),
            _is_civilian: char_borrow.is_civilian(),
            _hp: char_borrow._character_stats.get_hp(),
            _max_hp: char_borrow._character_stats.get_max_hp(),
            _intimacy: char_borrow.get_intimacy(),
            _position: *char_borrow.get_position(),
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
                    CharacterDataType::Civilian | CharacterDataType::Wife | CharacterDataType::Daughter
                )
            } else {
                false
            };

            let scene_display = get_scene_display_name(scene_data_name);

            result.push(DisplayCharacterInfo {
                _character_id: char_id,
                _name: name,
                _scene_data_name: scene_data_name.clone(),
                _scene_display_name: scene_display,
                _is_alive: char_save._character_stats._is_alive,
                _is_tamed: char_save._character_stats._is_tamed,
                _is_civilian: is_civilian,
                _hp: char_save._character_stats._hp,
                _max_hp: char_save._character_stats._max_hp,
                _intimacy: char_save._character_stats._intimacy,
                _position: char_save._character_create_info._position,
            });
        }
    }

    result
}

pub fn status_str(is_alive: bool) -> &'static str {
    if is_alive { "ALIVE" } else { "DEAD" }
}

pub fn status_color(is_alive: bool) -> u32 {
    if is_alive {
        CHAR_STATUS_ALIVE_COLOR
    } else {
        CHAR_STATUS_DEAD_COLOR
    }
}

pub fn create_character_list_panel<'a>(
    panel_name: &str,
    container_name: &str,
    title_text: &str,
) -> (Rc<WidgetDefault<'a>>, Rc<WidgetDefault<'a>>) {
    let layer = UIManager::create_widget(panel_name, UIWidgetTypes::Default);
    let layer_mut = ptr_as_mut(layer.as_ref());
    let ui_component = layer_mut.get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::VERTICAL);
    ui_component.set_halign(HorizontalAlign::CENTER);
    ui_component.set_valign(VerticalAlign::TOP);
    ui_component.set_pivot_preset(PIVOT_CENTER);
    ui_component.set_expandable(false);
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_padding(10.0);
    ui_component.set_color(CHAR_PANEL_BG_COLOR);
    ui_component.set_border_color(CHAR_PANEL_BORDER_COLOR);
    ui_component.set_round(5.0);

    let title_name = format!("{}_title", panel_name);
    let title_widget = UIManager::create_widget(&title_name, UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(title_widget.as_ref()).get_ui_component_mut();
    ui_component.set_size(300.0, 30.0);
    ui_component.set_halign(HorizontalAlign::CENTER);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_text(title_text);
    ui_component.set_font_size(25.0);
    ui_component.set_font_color(CHAR_PANEL_TITLE_COLOR);
    ui_component.set_color(get_color32(0, 0, 0, 0));
    layer_mut.add_widget(&title_widget);

    let list_container = UIManager::create_widget(container_name, UIWidgetTypes::Default);
    let list_container_mut = ptr_as_mut(list_container.as_ref());
    let ui_component = list_container_mut.get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_layout_orientation(Orientation::VERTICAL);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::TOP);
    ui_component.set_expandable(false);
    ui_component.set_scroll_y(true);
    ui_component.set_enable_renderable_area(true);
    ui_component.set_size_hint_x(Some(1.0));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_color(CHAR_LIST_BG_COLOR);
    layer_mut.add_widget(&list_container);

    (layer, list_container)
}

pub fn create_character_card<'a>(card_name: &str, info_text: &str, is_alive: bool) -> Rc<WidgetDefault<'a>> {
    let card_widget = UIManager::create_widget(card_name, UIWidgetTypes::Default);
    let card_mut = ptr_as_mut(card_widget.as_ref());
    let ui_comp = card_mut.get_ui_component_mut();
    ui_comp.set_layout_type(UILayoutType::BoxLayout);
    ui_comp.set_layout_orientation(Orientation::HORIZONTAL);
    ui_comp.set_halign(HorizontalAlign::CENTER);
    ui_comp.set_valign(VerticalAlign::CENTER);
    ui_comp.set_size(620.0, 48.0);
    ui_comp.set_margin(4.0);
    ui_comp.set_padding(8.0);
    ui_comp.set_color(CHAR_CARD_BG_COLOR);
    ui_comp.set_border_color(CHAR_CARD_BORDER_COLOR);
    ui_comp.set_round(5.0);
    ui_comp.set_text(info_text);
    ui_comp.set_font_size(22.0);
    ui_comp.set_font_color(status_color(is_alive));
    card_widget
}

pub fn create_character_empty_card<'a>(card_name: &str, empty_text: &str) -> Rc<WidgetDefault<'a>> {
    let empty_widget = UIManager::create_widget(card_name, UIWidgetTypes::Default);
    let ui_comp = ptr_as_mut(empty_widget.as_ref()).get_ui_component_mut();
    ui_comp.set_size(400.0, 60.0);
    ui_comp.set_margin(20.0);
    ui_comp.set_text(empty_text);
    ui_comp.set_font_size(24.0);
    ui_comp.set_font_color(CHAR_EMPTY_TEXT_COLOR);
    ui_comp.set_color(get_color32(0, 0, 0, 0));
    empty_widget
}
