use crate::game_module::actors::character::{CharacterData, CharacterDataCreateInfo};
use crate::game_module::actors::items::ItemData;
use crate::game_module::actors::props::PropData;
use crate::game_module::actors::weapons::WeaponData;
use crate::game_module::actors::weapons::WeaponDataCreateInfo;
use crate::game_module::game_option::GameOptionData;
use crate::game_module::game_scene_manager::GameSceneDataCreateInfo;
use crate::game_module::save_data::save_data::GameSaveData;
use crate::game_module::scenario::scenario::ScenarioDataCreateInfo;
use crate::game_module::widgets::toolbox_widget::item_tab_widget::ToolboxItemData;
use rust_engine_3d::core::engine_service_locator::get_engine_resources;
use rust_engine_3d::resource::resource::{
    APPLICATION_RESOURCE_PATH, ResourceDataContainer, get_resource_data_must, get_unique_resource_name,
};
use rust_engine_3d::utilities::system::{self, RcRefCell, newRcRefCell};
use serde_json::{self};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub const GAME_DATA_DIRECTORY: &str = "game_data";
pub const CHARACTER_DATA_FILE_PATH: &str = "game_data/characters";
pub const ITEM_DATA_FILE_PATH: &str = "game_data/items";
pub const GAME_SCENE_FILE_PATH: &str = "game_data/game_scenes";
pub const PROP_DATA_FILE_PATH: &str = "game_data/props";
pub const SCENARIO_FILE_PATH: &str = "game_data/scenario";
pub const WEAPON_DATA_FILE_PATH: &str = "game_data/weapons";
pub const GAME_SAVE_DATA_FILE_PATH: &str = "game_data/save_data";
pub const TOOLBOX_ITEM_DATA_FILE_PATH: &str = "game_data/toolbox_items";
pub const GAME_OPTION_DATA_FILE_PATH: &str = "game_data/game_option";
pub const GAME_OPTION_DATA_FILE_NAME: &str = "game_option";

pub const APP_QUALIFIER: &str = "";
pub const APP_ORGANIZATION: &str = "";
pub const APP_NAME: &str = "StoneAge";
pub const SAVE_DATA_DIRECTORY_NAME: &str = "save_data";
pub const GAME_OPTION_DIRECTORY_NAME: &str = "game_option";

pub const EXT_GAME_DATA: &str = "data";

pub const DEFAULT_GAME_DATA_NAME: &str = "default";

pub type CharacterDataMap = ResourceDataContainer<CharacterData>;
pub type ItemDataMap = ResourceDataContainer<ItemData>;
pub type GameSceneDataCreateInfoMap = ResourceDataContainer<GameSceneDataCreateInfo>;
pub type GameSaveDataMap = ResourceDataContainer<GameSaveData>;
pub type PropDataMap = ResourceDataContainer<PropData>;
pub type ScenarioDataCreateInfoMap = ResourceDataContainer<ScenarioDataCreateInfo>;
pub type WeaponDataMap<'a> = ResourceDataContainer<WeaponData<'a>>;
pub type ToolboxItemDataMap = ResourceDataContainer<ToolboxItemData>;

#[derive(Clone)]
pub struct GameResources<'a> {
    _scenario_data_create_info_map: ScenarioDataCreateInfoMap,
    _game_scene_data_create_info_map: GameSceneDataCreateInfoMap,
    _game_save_data_map: GameSaveDataMap,
    _character_data_map: CharacterDataMap,
    _item_data_map: ItemDataMap,
    _prop_data_map: PropDataMap,
    _weapon_data_map: WeaponDataMap<'a>,
    _toolbox_item_data_map: ToolboxItemDataMap,
    pub _game_option_data: RcRefCell<GameOptionData>,
}

impl<'a> GameResources<'a> {
    pub fn create_game_resources() -> Box<GameResources<'a>> {
        let game_resources = GameResources {
            _scenario_data_create_info_map: ScenarioDataCreateInfoMap::new(),
            _game_scene_data_create_info_map: GameSceneDataCreateInfoMap::new(),
            _game_save_data_map: GameSaveDataMap::new(),
            _character_data_map: CharacterDataMap::new(),
            _item_data_map: ItemDataMap::new(),
            _prop_data_map: PropDataMap::new(),
            _weapon_data_map: WeaponDataMap::new(),
            _toolbox_item_data_map: ToolboxItemDataMap::new(),
            _game_option_data: newRcRefCell(GameOptionData::default()),
        };
        Box::new(game_resources)
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        get_engine_resources().collect_resources(dir, extensions)
    }
    pub fn initialize_game_resources(&mut self) {}

    pub fn load_game_resources(&mut self) {
        self.load_game_data();
        self.load_game_scene_data();
        self.load_scenario_data();
    }
    pub fn destroy_game_resources(&mut self) {
        self.unload_scenario_data();
        self.unload_game_scene_data();
        self.unload_game_data();
    }

    // scenario data
    pub fn load_scenario_data(&mut self) {
        log::info!("    load_scenario_data");
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let scenario_directory = PathBuf::from(SCENARIO_FILE_PATH);
        let scenario_data_files: Vec<PathBuf> = self.collect_resources(&scenario_directory, &[EXT_GAME_DATA]);
        for scenario_data_file in scenario_data_files {
            let scenario_data_name = get_unique_resource_name(
                &self._scenario_data_create_info_map,
                &game_data_directory,
                &scenario_data_file,
            );
            let loaded_contents = system::load(&scenario_data_file);
            log::info!("loaded_contents: {:?}", scenario_data_file);
            let scenario_data_create_info: ScenarioDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._scenario_data_create_info_map
                .insert(scenario_data_name.clone(), newRcRefCell(scenario_data_create_info));
        }
    }

    pub fn unload_scenario_data(&mut self) {
        self._scenario_data_create_info_map.clear();
    }

    pub fn has_scenario_data(&self, resource_name: &str) -> bool {
        self._scenario_data_create_info_map.get(resource_name).is_some()
    }

    pub fn get_scenario_data(&self, resource_name: &str) -> &RcRefCell<ScenarioDataCreateInfo> {
        get_resource_data_must("scenario_data", &self._scenario_data_create_info_map, resource_name)
    }

    // game scene data
    pub fn load_game_scene_data(&mut self) {
        log::info!("    load_game_scene_data");
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let game_scene_directory = PathBuf::from(GAME_SCENE_FILE_PATH);
        let game_scene_data_files: Vec<PathBuf> = self.collect_resources(&game_scene_directory, &[EXT_GAME_DATA]);
        for game_scene_data_file in game_scene_data_files {
            let game_scene_data_name = get_unique_resource_name(
                &self._game_scene_data_create_info_map,
                &game_data_directory,
                &game_scene_data_file,
            );
            let loaded_contents = system::load(&game_scene_data_file);
            let game_scene_data_create_info: GameSceneDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._game_scene_data_create_info_map
                .insert(game_scene_data_name.clone(), newRcRefCell(game_scene_data_create_info));
        }
    }

    pub fn unload_game_scene_data(&mut self) {
        self._game_scene_data_create_info_map.clear();
    }

    pub fn save_game_scene_data(
        &mut self,
        game_scene_data_name: &str,
        game_scene_data_create_info: &GameSceneDataCreateInfo,
    ) {
        let mut game_scene_data_filepath = PathBuf::from(APPLICATION_RESOURCE_PATH);
        game_scene_data_filepath.push(GAME_SCENE_FILE_PATH);
        game_scene_data_filepath.push(game_scene_data_name);
        game_scene_data_filepath.set_extension(EXT_GAME_DATA);
        let mut write_file = File::create(&game_scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String =
            serde_json::to_string(&game_scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file.write_all(write_contents.as_bytes()).expect("Failed to write");

        self._game_scene_data_create_info_map.insert(
            String::from(game_scene_data_name),
            newRcRefCell(game_scene_data_create_info.clone()),
        );
    }

    pub fn has_game_scene_data(&self, resource_name: &str) -> bool {
        self._game_scene_data_create_info_map.get(resource_name).is_some()
    }

    pub fn get_game_scene_data(&self, resource_name: &str) -> &RcRefCell<GameSceneDataCreateInfo> {
        get_resource_data_must("game_scene_data", &self._game_scene_data_create_info_map, resource_name)
    }

    // user directories & file paths (Steam Cloud compatible)
    pub fn get_user_data_directory() -> PathBuf {
        if let Some(project_dirs) = directories::ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME) {
            project_dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from(APPLICATION_RESOURCE_PATH).join(GAME_DATA_DIRECTORY)
        }
    }

    pub fn get_user_config_directory() -> PathBuf {
        if let Some(project_dirs) = directories::ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME) {
            project_dirs.config_dir().to_path_buf()
        } else {
            PathBuf::from(APPLICATION_RESOURCE_PATH).join(GAME_DATA_DIRECTORY)
        }
    }

    pub fn get_game_save_data_directory() -> PathBuf {
        Self::get_user_data_directory().join(SAVE_DATA_DIRECTORY_NAME)
    }

    pub fn get_game_save_data_filepath(game_save_data_name: &str) -> PathBuf {
        let save_dir = Self::get_game_save_data_directory();
        let relative_name = if let Some(stripped) = game_save_data_name.strip_prefix("save_data/") {
            stripped
        } else {
            game_save_data_name
        };
        let mut filepath = save_dir.join(relative_name);
        filepath.set_extension(EXT_GAME_DATA);
        filepath
    }

    pub fn get_legacy_game_save_data_directory() -> PathBuf {
        let mut path = PathBuf::from(APPLICATION_RESOURCE_PATH);
        path.push(GAME_SAVE_DATA_FILE_PATH);
        path
    }

    pub fn get_legacy_game_option_data_filepath() -> PathBuf {
        let mut game_option_filepath = PathBuf::from(APPLICATION_RESOURCE_PATH);
        game_option_filepath.push(GAME_OPTION_DATA_FILE_PATH);
        game_option_filepath.push(GAME_OPTION_DATA_FILE_NAME);
        game_option_filepath.set_extension(EXT_GAME_DATA);
        game_option_filepath
    }

    fn migrate_legacy_data_if_needed() {
        // 1. Migrate option data
        let new_option_filepath = Self::get_game_option_data_filepath();
        if !new_option_filepath.exists() {
            let legacy_option_filepath = Self::get_legacy_game_option_data_filepath();
            if legacy_option_filepath.exists() {
                if let Some(parent) = new_option_filepath.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if let Err(e) = std::fs::copy(&legacy_option_filepath, &new_option_filepath) {
                    log::warn!(
                        "Failed to migrate legacy game option from {:?} to {:?}: {:?}",
                        legacy_option_filepath,
                        new_option_filepath,
                        e
                    );
                } else {
                    log::info!(
                        "Successfully migrated game option from {:?} to {:?}",
                        legacy_option_filepath,
                        new_option_filepath
                    );
                }
            }
        }

        // 2. Migrate save data
        let new_save_dir = Self::get_game_save_data_directory();
        let legacy_save_dir = Self::get_legacy_game_save_data_directory();
        if legacy_save_dir.exists() {
            if !new_save_dir.exists() {
                let _ = std::fs::create_dir_all(&new_save_dir);
            }
            if let Ok(entries) = std::fs::read_dir(&legacy_save_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(EXT_GAME_DATA) {
                        if let Some(file_name) = path.file_name() {
                            let new_file_path = new_save_dir.join(file_name);
                            if !new_file_path.exists() {
                                if let Err(e) = std::fs::copy(&path, &new_file_path) {
                                    log::warn!(
                                        "Failed to migrate legacy save file {:?} to {:?}: {:?}",
                                        path,
                                        new_file_path,
                                        e
                                    );
                                } else {
                                    log::info!("Migrated save file {:?} to {:?}", path, new_file_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // game save data
    pub fn load_game_save_data(&mut self) {
        Self::migrate_legacy_data_if_needed();

        let game_save_directory = Self::get_game_save_data_directory();
        if !game_save_directory.exists() {
            let _ = std::fs::create_dir_all(&game_save_directory);
            return;
        }

        match std::fs::read_dir(&game_save_directory) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if file_path.is_file() && file_path.extension().and_then(|ext| ext.to_str()) == Some(EXT_GAME_DATA)
                    {
                        if let Some(file_stem) = file_path.file_stem().and_then(|stem| stem.to_str()) {
                            let game_save_data_name = format!("{}/{}", SAVE_DATA_DIRECTORY_NAME, file_stem);
                            match File::open(&file_path) {
                                Ok(file) => match serde_json::from_reader(file) {
                                    Ok(save_data) => {
                                        self._game_save_data_map.insert(game_save_data_name, newRcRefCell(save_data));
                                    }
                                    Err(err) => {
                                        log::warn!("Failed to deserialize save file {:?}: {:?}", file_path, err);
                                    }
                                },
                                Err(err) => {
                                    log::warn!("Failed to open save file {:?}: {:?}", file_path, err);
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                log::warn!(
                    "Failed to read game save directory {:?}: {:?}",
                    game_save_directory,
                    err
                );
            }
        }
    }

    pub fn unload_game_save_data(&mut self) {
        self._game_save_data_map.clear();
    }

    pub fn has_game_save_data(&self, resource_name: &str) -> bool {
        self._game_save_data_map.get(resource_name).is_some()
    }

    pub fn get_game_save_data(&mut self, resource_name: &str) -> &RcRefCell<GameSaveData> {
        self._game_save_data_map.get(resource_name).as_ref().unwrap()
    }

    pub fn save_game_save_data(&mut self, game_save_data_name: &str, game_save_data: &GameSaveData) {
        self._game_save_data_map.insert(game_save_data_name.to_string(), newRcRefCell(game_save_data.clone()));

        let game_save_data_filepath = Self::get_game_save_data_filepath(game_save_data_name);
        if let Some(parent) = game_save_data_filepath.parent() {
            if !parent.exists() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        match File::create(&game_save_data_filepath) {
            Ok(mut write_file) => match serde_json::to_string_pretty(game_save_data) {
                Ok(write_contents) => {
                    if let Err(e) = write_file.write_all(write_contents.as_bytes()) {
                        log::warn!("Failed to write save file {:?}: {:?}", game_save_data_filepath, e);
                    } else {
                        log::info!("Saved game data to {:?}", game_save_data_filepath);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to serialize game save data: {:?}", e);
                }
            },
            Err(e) => {
                log::warn!("Failed to create save file {:?}: {:?}", game_save_data_filepath, e);
            }
        }
    }

    pub fn remove_game_save_data(&mut self, resource_name: &str) {
        if self._game_save_data_map.remove(resource_name).is_some() {
            let game_save_data_filepath = Self::get_game_save_data_filepath(resource_name);
            if game_save_data_filepath.exists() {
                if let Err(e) = std::fs::remove_file(&game_save_data_filepath) {
                    log::warn!("Failed to remove save file {:?}: {:?}", game_save_data_filepath, e);
                } else {
                    log::info!("Removed game save file {:?}", game_save_data_filepath);
                }
            }
        }
    }

    pub fn rename_game_save_data(&mut self, old_name: &str, new_name: &str) {
        if let Some(save_data) = self._game_save_data_map.remove(old_name) {
            self._game_save_data_map.insert(new_name.to_string(), save_data);

            let old_filepath = Self::get_game_save_data_filepath(old_name);
            let new_filepath = Self::get_game_save_data_filepath(new_name);

            if old_filepath.exists() {
                if let Some(parent) = new_filepath.parent() {
                    if !parent.exists() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                }
                if let Err(e) = std::fs::rename(&old_filepath, &new_filepath) {
                    log::warn!(
                        "Failed to rename save file {:?} to {:?}: {:?}",
                        old_filepath,
                        new_filepath,
                        e
                    );
                } else {
                    log::info!("Renamed save file {:?} to {:?}", old_filepath, new_filepath);
                }
            }
        }
    }

    // game option data
    pub fn get_game_option_data_filepath() -> PathBuf {
        let mut game_option_filepath = Self::get_user_config_directory();
        game_option_filepath.push(GAME_OPTION_DIRECTORY_NAME);
        game_option_filepath.push(GAME_OPTION_DATA_FILE_NAME);
        game_option_filepath.set_extension(EXT_GAME_DATA);
        game_option_filepath
    }

    pub fn load_game_option_data(&mut self) {
        log::info!("    load_game_option_data");
        Self::migrate_legacy_data_if_needed();

        let game_option_filepath = Self::get_game_option_data_filepath();
        if game_option_filepath.exists() {
            match File::open(&game_option_filepath) {
                Ok(file) => match serde_json::from_reader(file) {
                    Ok(loaded_option) => {
                        *self._game_option_data.borrow_mut() = loaded_option;
                        log::info!("Loaded game option from {:?}", game_option_filepath);
                    }
                    Err(err) => {
                        log::warn!(
                            "Failed to deserialize game option file {:?}: {:?}",
                            game_option_filepath,
                            err
                        );
                    }
                },
                Err(err) => {
                    log::warn!("Failed to open game option file {:?}: {:?}", game_option_filepath, err);
                }
            }
        } else {
            log::info!(
                "Game option file not found at {:?}, creating with default settings.",
                game_option_filepath
            );
            let default_option = GameOptionData::default();
            self.save_game_option_data(&default_option);
        }
        self._game_option_data.borrow().apply();
    }

    pub fn unload_game_option_data(&mut self) {
        *self._game_option_data.borrow_mut() = GameOptionData::default();
    }

    pub fn get_game_option_data(&self) -> &RcRefCell<GameOptionData> {
        &self._game_option_data
    }

    pub fn save_game_option_data(&mut self, game_option_data: &GameOptionData) {
        *self._game_option_data.borrow_mut() = game_option_data.clone();

        let game_option_filepath = Self::get_game_option_data_filepath();
        if let Some(parent) = game_option_filepath.parent() {
            if !parent.exists() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        match File::create(&game_option_filepath) {
            Ok(mut write_file) => match serde_json::to_string_pretty(game_option_data) {
                Ok(write_contents) => {
                    if let Err(e) = write_file.write_all(write_contents.as_bytes()) {
                        log::warn!("Failed to write game option file {:?}: {:?}", game_option_filepath, e);
                    } else {
                        log::info!("Saved game option to {:?}", game_option_filepath);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to serialize game option data: {:?}", e);
                }
            },
            Err(e) => {
                log::warn!("Failed to create game option file {:?}: {:?}", game_option_filepath, e);
            }
        }

        game_option_data.apply();
    }

    // Game Data
    fn load_game_data(&mut self) {
        log::info!("    load_game_data");
        self.load_character_data();
        self.load_item_data();
        self.load_prop_data();
        self.load_weapon_data();
        self.load_toolbox_item_data();
        self.load_game_save_data();
        self.load_game_option_data();
    }

    fn unload_game_data(&mut self) {
        self.unload_game_option_data();
        self.unload_game_save_data();
        self.unload_toolbox_item_data();
        self.unload_weapon_data();
        self.unload_prop_data();
        self.unload_item_data();
        self.unload_character_data();
    }

    // prop data
    fn load_prop_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let prop_data_directory = PathBuf::from(PROP_DATA_FILE_PATH);

        // load_prop_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&prop_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let prop_data_name = get_unique_resource_name(&self._prop_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let prop_data: PropData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._prop_data_map.insert(prop_data_name.clone(), newRcRefCell(prop_data));
        }
    }

    fn unload_prop_data(&mut self) {
        self._prop_data_map.clear();
    }

    pub fn has_prop_data(&self, resource_name: &str) -> bool {
        self._prop_data_map.get(resource_name).is_some()
    }

    pub fn get_prop_data(&self, resource_name: &str) -> &RcRefCell<PropData> {
        get_resource_data_must("prop_data", &self._prop_data_map, resource_name)
    }

    // weapon data
    fn load_weapon_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let weapon_data_directory = PathBuf::from(WEAPON_DATA_FILE_PATH);

        // load_weapon_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&weapon_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let weapon_data_name =
                get_unique_resource_name(&self._weapon_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let weapon_data_create_info: WeaponDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let weapon_model_data = get_engine_resources().get_model_data(&weapon_data_create_info._model_data_name);
            let weapon_data = WeaponData::create_weapon_data(&weapon_data_create_info, weapon_model_data);
            self._weapon_data_map.insert(weapon_data_name.clone(), newRcRefCell(weapon_data));
        }
    }

    fn unload_weapon_data(&mut self) {
        self._weapon_data_map.clear();
    }

    pub fn has_weapon_data(&self, resource_name: &str) -> bool {
        self._weapon_data_map.get(resource_name).is_some()
    }

    pub fn get_weapon_data(&self, resource_name: &str) -> &RcRefCell<WeaponData<'a>> {
        get_resource_data_must("weapon_data", &self._weapon_data_map, resource_name)
    }

    // character data
    fn load_character_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let character_data_directory = PathBuf::from(CHARACTER_DATA_FILE_PATH);

        // load_character_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&character_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let character_data_name =
                get_unique_resource_name(&self._character_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let character_data_create_info: CharacterDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let character_data = CharacterData::create_character_data(&character_data_create_info);
            self._character_data_map.insert(character_data_name.clone(), newRcRefCell(character_data));
        }
    }

    fn unload_character_data(&mut self) {
        self._character_data_map.clear();
    }

    pub fn has_character_data(&self, resource_name: &str) -> bool {
        self._character_data_map.get(resource_name).is_some()
    }

    pub fn get_character_data(&self, resource_name: &str) -> &RcRefCell<CharacterData> {
        get_resource_data_must("character_data", &self._character_data_map, resource_name)
    }

    // item data
    fn load_item_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let item_data_directory = PathBuf::from(ITEM_DATA_FILE_PATH);

        // load_item_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&item_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let item_data_name = get_unique_resource_name(&self._item_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let item_data: ItemData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._item_data_map.insert(item_data_name.clone(), newRcRefCell(item_data));
        }
    }

    fn unload_item_data(&mut self) {
        self._item_data_map.clear();
    }

    pub fn has_item_data(&self, resource_name: &str) -> bool {
        self._item_data_map.get(resource_name).is_some()
    }

    pub fn get_item_data(&self, resource_name: &str) -> &RcRefCell<ItemData> {
        get_resource_data_must("item_data", &self._item_data_map, resource_name)
    }

    // toolbox item data
    fn load_toolbox_item_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let toolbox_item_data_directory = PathBuf::from(TOOLBOX_ITEM_DATA_FILE_PATH);

        let game_data_files: Vec<PathBuf> = self.collect_resources(&toolbox_item_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let toolbox_item_data_name =
                get_unique_resource_name(&self._toolbox_item_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let toolbox_item_data: ToolboxItemData =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._toolbox_item_data_map.insert(toolbox_item_data_name.clone(), newRcRefCell(toolbox_item_data));
        }
    }

    fn unload_toolbox_item_data(&mut self) {
        self._toolbox_item_data_map.clear();
    }

    pub fn has_toolbox_item_data(&self, resource_name: &str) -> bool {
        self._toolbox_item_data_map.get(resource_name).is_some()
    }

    pub fn get_toolbox_item_data(&self, resource_name: &str) -> &RcRefCell<ToolboxItemData> {
        get_resource_data_must("toolbox_item_data", &self._toolbox_item_data_map, resource_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_save_data_filepath() {
        let path1 = GameResources::get_game_save_data_filepath("save_data/00");
        let path2 = GameResources::get_game_save_data_filepath("00");
        assert_eq!(path1, path2);
        assert_eq!(path1.extension().and_then(|s| s.to_str()), Some(EXT_GAME_DATA));
        assert!(path1.ends_with(format!("save_data/00.{}", EXT_GAME_DATA)));
    }

    #[test]
    fn test_game_option_data_filepath() {
        let option_path = GameResources::get_game_option_data_filepath();
        assert_eq!(
            option_path.file_name().and_then(|s| s.to_str()),
            Some(format!("{}.{}", GAME_OPTION_DATA_FILE_NAME, EXT_GAME_DATA).as_str())
        );
        assert_eq!(
            option_path.parent().and_then(|p| p.file_name()).and_then(|s| s.to_str()),
            Some(GAME_OPTION_DIRECTORY_NAME)
        );
    }

    #[test]
    fn test_user_directories_valid() {
        let data_dir = GameResources::get_user_data_directory();
        let config_dir = GameResources::get_user_config_directory();
        let save_dir = GameResources::get_game_save_data_directory();

        assert!(!data_dir.as_os_str().is_empty());
        assert!(!config_dir.as_os_str().is_empty());
        assert_eq!(save_dir, data_dir.join(SAVE_DATA_DIRECTORY_NAME));
    }
}
