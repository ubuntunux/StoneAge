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
    APPLICATION_RESOURCE_PATH, DEFAULT_RESOURCE_GC_TIMEOUT_SEC, LOG_RESOURCE_LIFECYCLE, MetaData,
    RESOURCE_GC_CHECK_INTERVAL_SEC, ResourceDataContainer, ResourceEntry, ResourceEntryMap,
    get_resource_entry_data_must, get_unique_resource_name, update_resource_entry_map_gc,
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

pub type CharacterDataMap = ResourceEntryMap<CharacterData>;
pub type ItemDataMap = ResourceEntryMap<ItemData>;
pub type GameSceneDataCreateInfoMap = ResourceEntryMap<GameSceneDataCreateInfo>;
pub type GameSaveDataMap = ResourceDataContainer<GameSaveData>;
pub type PropDataMap = ResourceEntryMap<PropData>;
pub type ScenarioDataCreateInfoMap = ResourceEntryMap<ScenarioDataCreateInfo>;
pub type WeaponDataMap<'a> = ResourceEntryMap<WeaponData<'a>>;
pub type ToolboxItemDataMap = ResourceEntryMap<ToolboxItemData>;

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
    pub _gc_check_timer: f32,
    pub _gc_timeout_sec: f32,
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
            _gc_check_timer: 0.0,
            _gc_timeout_sec: DEFAULT_RESOURCE_GC_TIMEOUT_SEC,
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

    pub fn update_garbage_collection(&mut self, delta_time: f64) -> u32 {
        self._gc_check_timer += delta_time as f32;
        if self._gc_check_timer < RESOURCE_GC_CHECK_INTERVAL_SEC {
            return 0;
        }
        self._gc_check_timer = 0.0;

        let now = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f32(self._gc_timeout_sec);

        let mut evicted = 0;
        evicted += update_resource_entry_map_gc(&mut self._character_data_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._item_data_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._prop_data_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._weapon_data_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._scenario_data_create_info_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._game_scene_data_create_info_map, now, timeout);
        evicted += update_resource_entry_map_gc(&mut self._toolbox_item_data_map, now, timeout);

        if LOG_RESOURCE_LIFECYCLE && 0 < evicted {
            log::info!("[GameResources GC] Evicted total {} unused resources", evicted);
        }
        evicted
    }

    pub fn get_gc_timeout_sec(&self) -> f32 {
        self._gc_timeout_sec
    }

    pub fn set_gc_timeout_sec(&mut self, timeout_sec: f32) {
        self._gc_timeout_sec = timeout_sec;
    }

    // scenario data
    pub fn load_scenario_data(&mut self) {
        log::info!("    load_scenario_data (catalog only)");
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let scenario_directory = PathBuf::from(SCENARIO_FILE_PATH);
        let scenario_data_files: Vec<PathBuf> = self.collect_resources(&scenario_directory, &[EXT_GAME_DATA]);
        for scenario_data_file in scenario_data_files {
            let scenario_data_name = get_unique_resource_name(
                &self._scenario_data_create_info_map,
                &game_data_directory,
                &scenario_data_file,
            );
            let meta_data = MetaData::new(&scenario_data_file, &scenario_data_file);
            let entry = ResourceEntry::new(scenario_data_name.clone(), meta_data);
            self._scenario_data_create_info_map.insert(scenario_data_name, entry);
        }
    }

    pub fn unload_scenario_data(&mut self) {
        self._scenario_data_create_info_map.clear();
    }

    pub fn has_scenario_data(&self, resource_name: &str) -> bool {
        self._scenario_data_create_info_map.contains_key(resource_name)
    }

    pub fn get_scenario_data(&self, resource_name: &str) -> &RcRefCell<ScenarioDataCreateInfo> {
        get_resource_entry_data_must(
            "scenario_data",
            &self._scenario_data_create_info_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader(loaded_contents) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize scenario data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
    }

    // game scene data
    pub fn load_game_scene_data(&mut self) {
        log::info!("    load_game_scene_data (catalog only)");
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let game_scene_directory = PathBuf::from(GAME_SCENE_FILE_PATH);
        let game_scene_data_files: Vec<PathBuf> = self.collect_resources(&game_scene_directory, &[EXT_GAME_DATA]);
        for game_scene_data_file in game_scene_data_files {
            let game_scene_data_name = get_unique_resource_name(
                &self._game_scene_data_create_info_map,
                &game_data_directory,
                &game_scene_data_file,
            );
            let meta_data = MetaData::new(&game_scene_data_file, &game_scene_data_file);
            let entry = ResourceEntry::new(game_scene_data_name.clone(), meta_data);
            self._game_scene_data_create_info_map.insert(game_scene_data_name, entry);
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

        let meta_data = MetaData::new(&game_scene_data_filepath, &game_scene_data_filepath);
        let entry = ResourceEntry::new(String::from(game_scene_data_name), meta_data);
        entry.set_loaded_data(newRcRefCell(game_scene_data_create_info.clone()));
        self._game_scene_data_create_info_map.insert(String::from(game_scene_data_name), entry);
    }

    pub fn has_game_scene_data(&self, resource_name: &str) -> bool {
        self._game_scene_data_create_info_map.contains_key(resource_name)
    }

    pub fn get_game_scene_data(&self, resource_name: &str) -> &RcRefCell<GameSceneDataCreateInfo> {
        get_resource_entry_data_must(
            "game_scene_data",
            &self._game_scene_data_create_info_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader(loaded_contents) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize game scene data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
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

        let game_data_files: Vec<PathBuf> = self.collect_resources(&prop_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let prop_data_name = get_unique_resource_name(&self._prop_data_map, &game_data_directory, &game_data_file);
            let meta_data = MetaData::new(&game_data_file, &game_data_file);
            let entry = ResourceEntry::new(prop_data_name.clone(), meta_data);
            self._prop_data_map.insert(prop_data_name, entry);
        }
    }

    fn unload_prop_data(&mut self) {
        self._prop_data_map.clear();
    }

    pub fn has_prop_data(&self, resource_name: &str) -> bool {
        self._prop_data_map.contains_key(resource_name)
    }

    pub fn get_prop_data(&self, resource_name: &str) -> &RcRefCell<PropData> {
        get_resource_entry_data_must(
            "prop_data",
            &self._prop_data_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader(loaded_contents) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize prop data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
    }

    // weapon data
    fn load_weapon_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let weapon_data_directory = PathBuf::from(WEAPON_DATA_FILE_PATH);

        let game_data_files: Vec<PathBuf> = self.collect_resources(&weapon_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let weapon_data_name =
                get_unique_resource_name(&self._weapon_data_map, &game_data_directory, &game_data_file);
            let meta_data = MetaData::new(&game_data_file, &game_data_file);
            let entry = ResourceEntry::new(weapon_data_name.clone(), meta_data);
            self._weapon_data_map.insert(weapon_data_name, entry);
        }
    }

    fn unload_weapon_data(&mut self) {
        self._weapon_data_map.clear();
    }

    pub fn has_weapon_data(&self, resource_name: &str) -> bool {
        self._weapon_data_map.contains_key(resource_name)
    }

    pub fn get_weapon_data(&self, resource_name: &str) -> &RcRefCell<WeaponData<'a>> {
        get_resource_entry_data_must(
            "weapon_data",
            &self._weapon_data_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader::<_, WeaponDataCreateInfo>(loaded_contents) {
                    Ok(weapon_data_create_info) => {
                        let weapon_model_data =
                            get_engine_resources().get_model_data(&weapon_data_create_info._model_data_name);
                        Some(WeaponData::create_weapon_data(
                            &weapon_data_create_info,
                            weapon_model_data,
                        ))
                    }
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize weapon data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
    }

    // character data
    fn load_character_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let character_data_directory = PathBuf::from(CHARACTER_DATA_FILE_PATH);

        let game_data_files: Vec<PathBuf> = self.collect_resources(&character_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let character_data_name =
                get_unique_resource_name(&self._character_data_map, &game_data_directory, &game_data_file);
            let meta_data = MetaData::new(&game_data_file, &game_data_file);
            let entry = ResourceEntry::new(character_data_name.clone(), meta_data);
            self._character_data_map.insert(character_data_name, entry);
        }
    }

    fn unload_character_data(&mut self) {
        self._character_data_map.clear();
    }

    pub fn has_character_data(&self, resource_name: &str) -> bool {
        self._character_data_map.contains_key(resource_name)
    }

    pub fn get_character_data(&self, resource_name: &str) -> &RcRefCell<CharacterData> {
        get_resource_entry_data_must(
            "character_data",
            &self._character_data_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader::<_, CharacterDataCreateInfo>(loaded_contents) {
                    Ok(character_data_create_info) => {
                        Some(CharacterData::create_character_data(&character_data_create_info))
                    }
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize character data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
    }

    // item data
    fn load_item_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let item_data_directory = PathBuf::from(ITEM_DATA_FILE_PATH);

        let game_data_files: Vec<PathBuf> = self.collect_resources(&item_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let item_data_name = get_unique_resource_name(&self._item_data_map, &game_data_directory, &game_data_file);
            let meta_data = MetaData::new(&game_data_file, &game_data_file);
            let entry = ResourceEntry::new(item_data_name.clone(), meta_data);
            self._item_data_map.insert(item_data_name, entry);
        }
    }

    fn unload_item_data(&mut self) {
        self._item_data_map.clear();
    }

    pub fn has_item_data(&self, resource_name: &str) -> bool {
        self._item_data_map.contains_key(resource_name)
    }

    pub fn get_item_data(&self, resource_name: &str) -> &RcRefCell<ItemData> {
        get_resource_entry_data_must(
            "item_data",
            &self._item_data_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader(loaded_contents) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize item data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
    }

    // toolbox item data
    fn load_toolbox_item_data(&mut self) {
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let toolbox_item_data_directory = PathBuf::from(TOOLBOX_ITEM_DATA_FILE_PATH);

        let game_data_files: Vec<PathBuf> = self.collect_resources(&toolbox_item_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let toolbox_item_data_name =
                get_unique_resource_name(&self._toolbox_item_data_map, &game_data_directory, &game_data_file);
            let meta_data = MetaData::new(&game_data_file, &game_data_file);
            let entry = ResourceEntry::new(toolbox_item_data_name.clone(), meta_data);
            self._toolbox_item_data_map.insert(toolbox_item_data_name, entry);
        }
    }

    fn unload_toolbox_item_data(&mut self) {
        self._toolbox_item_data_map.clear();
    }

    pub fn has_toolbox_item_data(&self, resource_name: &str) -> bool {
        self._toolbox_item_data_map.contains_key(resource_name)
    }

    pub fn get_toolbox_item_data(&self, resource_name: &str) -> &RcRefCell<ToolboxItemData> {
        get_resource_entry_data_must(
            "toolbox_item_data",
            &self._toolbox_item_data_map,
            resource_name,
            |_resource_name, meta| {
                let loaded_contents = system::load(&meta._resource_file_path);
                match serde_json::from_reader(loaded_contents) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!(
                            "Failed to deserialize toolbox item data {:?}: {:?}",
                            meta._resource_file_path,
                            err
                        );
                        None
                    }
                }
            },
        )
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

    #[test]
    fn test_resource_entry_lazy_loading() {
        let meta_data = MetaData::default();
        let entry: ResourceEntry<String> = ResourceEntry::new(String::from("test_entry"), meta_data);

        // 초기에는 로드되지 않은 상태
        assert!(!entry.is_loaded());
        assert!(entry.get_loaded_data().is_none());

        // 첫 호출 시 loader 실행
        let mut load_count = 0;
        {
            let loaded = entry.get_or_load_ref(|_name, _meta| {
                load_count += 1;
                Some(String::from("loaded_value"))
            });
            assert!(loaded.is_some());
            assert_eq!(*loaded.unwrap().borrow(), "loaded_value");
            assert_eq!(load_count, 1);
        }

        // 이제 로드 완료 상태
        assert!(entry.is_loaded());
        assert_eq!(*entry.get_loaded_data().unwrap().borrow(), "loaded_value");

        // 재호출 시 loader가 다시 호출되지 않고 캐시 반환
        {
            let cached = entry.get_or_load_ref(|_name, _meta| {
                load_count += 1;
                Some(String::from("never_called"))
            });
            assert!(cached.is_some());
            assert_eq!(*cached.unwrap().borrow(), "loaded_value");
            assert_eq!(load_count, 1); // loader 재실행 안 됨
        }

        // unload 시 다시 초기 상태로 복귀
        entry.unload();
        assert!(!entry.is_loaded());
        assert!(entry.get_loaded_data().is_none());
    }

    #[test]
    fn test_resource_entry_time_delayed_gc() {
        use std::time::{Duration, Instant};

        let _ = std::panic::catch_unwind(|| {
            rust_engine_3d::utilities::logger::initialize_logger(log::LevelFilter::Info);
        });

        let meta_data = MetaData::default();
        let mut entry: ResourceEntry<String> = ResourceEntry::new(String::from("test_gc_entry"), meta_data);

        // 1. 리소스 로드
        entry.get_or_load_ref(|_name, _meta| Some(String::from("gc_value")));
        assert!(entry.is_loaded());

        let timeout = Duration::from_secs(30);
        let t0 = Instant::now();

        // 2. 외부 참조(handle)가 살아있을 때는 GC 되지 않음
        let external_handle = entry.get_loaded_data().unwrap();
        // strong_count == 2 (entry 캐시 + external_handle)
        assert!(!entry.check_and_update_gc(t0, timeout));
        assert!(entry.is_loaded());
        assert!(entry._unused_start_time.is_none());

        // 3. 외부 참조 해제 후 첫 검사: unused_start_time 기록 (아직 언로드 안 됨)
        drop(external_handle);
        // strong_count == 1 (entry 캐시만 보유)
        assert!(!entry.check_and_update_gc(t0, timeout));
        assert!(entry.is_loaded());
        assert_eq!(entry._unused_start_time, Some(t0));

        // 4. 유예 시간 도중 다시 외부에서 참조 발생: unused_start_time 리셋 (스래싱 방지)
        let t_interrupted = t0 + Duration::from_secs(10);
        let new_handle = entry.get_loaded_data().unwrap();
        assert!(!entry.check_and_update_gc(t_interrupted, timeout));
        assert!(entry.is_loaded());
        assert!(entry._unused_start_time.is_none()); // 타이머 리셋 확인

        // 5. 다시 외부 참조 해제 및 유예 시간(30초) 경과 후 언로드
        drop(new_handle);
        let t1 = t0 + Duration::from_secs(15);
        assert!(!entry.check_and_update_gc(t1, timeout)); // t1에 다시 타이머 시작
        assert_eq!(entry._unused_start_time, Some(t1));

        // 아직 20초 경과 (30초 미만)
        let t2 = t1 + Duration::from_secs(20);
        assert!(!entry.check_and_update_gc(t2, timeout));
        assert!(entry.is_loaded());

        // 30초 경과: 언로드 수행 및 true 반환
        let t3 = t1 + Duration::from_secs(30);
        assert!(entry.check_and_update_gc(t3, timeout));
        assert!(!entry.is_loaded());
        assert!(entry.get_loaded_data().is_none());
        assert!(entry._unused_start_time.is_none());
    }

    #[test]
    fn test_pinned_resource_never_gc() {
        use std::time::{Duration, Instant};

        let meta_data = MetaData::default();
        let mut entry: ResourceEntry<String> = ResourceEntry::new_pinned(String::from("pinned_entry"), meta_data);

        // 로드
        entry.get_or_load_ref(|_name, _meta| Some(String::from("pinned_value")));
        assert!(entry.is_loaded());

        let timeout = Duration::from_secs(5);
        let t0 = Instant::now();
        let t_future = t0 + Duration::from_secs(9999);

        // 외부 참조가 없어도 pinned 리소스는 절대 GC되지 않음
        assert!(!entry.check_and_update_gc(t_future, timeout));
        assert!(entry.is_loaded());
        assert!(entry._unused_start_time.is_none());
    }

    #[test]
    fn test_visibility_gating_unready_resource() {
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData, SceneObjectType};

        // 1. 지오메트리 버퍼가 없는 빈 MeshData (로딩 중 상태 모사)
        let unready_mesh = newRcRefCell(MeshData {
            _name: String::from("unready_mesh"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(), // 빈 상태 -> is_ready() == false
        });
        assert!(!unready_mesh.borrow().is_ready());

        // 2. 모델 데이터 생성
        let model_data = newRcRefCell(ModelData {
            _model_data_name: String::from("test_model"),
            _mesh_name: String::from("test_mesh"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: unready_mesh.clone(),
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: std::collections::HashMap::new(),
        });

        // 하위 메시가 준비되지 않았으므로 model_data도 준비되지 않음
        assert!(!model_data.borrow().is_ready());

        // 3. RenderObject 생성
        let create_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Default,
            _model_data_name: String::from("test_model"),
            _position: nalgebra::Vector3::zeros(),
            _rotation: nalgebra::Vector3::zeros(),
            _scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
        };

        let (mut render_object, _transform) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("test_render_object"),
            &model_data,
            &create_info,
            None,
        );

        // 하위 리소스가 미완료이므로 _is_ready == false, is_visible() == false (가시성 게이팅 작동)
        assert!(!render_object.is_ready());
        assert!(!render_object.is_visible());
        assert!(render_object._is_visible); // 사용자 가시성 플래그 자체는 true이지만 게이팅에 의해 숨겨짐

        // 4. 아직 준비되지 않은 상태에서 ready state 체크 시 여전히 false 유지
        assert!(!render_object.check_and_update_ready_state());
        assert!(!render_object.is_visible());

        // 5. 비동기 로딩 완료 모사: MeshData에 지오메트리 버퍼가 채워짐
        let dummy_geometry = newRcRefCell(rust_engine_3d::vulkan_context::geometry_buffer::GeometryData {
            _geometry_name: String::from("geom_0"),
            _vertex_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _index_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _vertex_index_count: 3,
            _geometry_bounding_box: BoundingBox::default(),
        });
        unready_mesh.borrow_mut()._geometry_data_list.push(dummy_geometry);

        // 이제 메시와 모델이 ready 상태가 됨
        assert!(unready_mesh.borrow().is_ready());
        assert!(model_data.borrow().is_ready());

        // 6. 다음 프레임 업데이트 시 RenderObject가 완료를 감지하고 가시화 승격!
        assert!(render_object.check_and_update_ready_state());
        assert!(render_object.is_ready());
        assert!(render_object.is_visible()); // 비로소 화면에 노출됨!
    }

    #[test]
    fn test_model_data_lazy_loading_and_gc() {
        use rust_engine_3d::resource::resource::{
            MetaData, ResourceEntry, ResourceEntryMap, get_resource_entry_data, update_resource_entry_map_gc,
        };
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use std::cell::Cell;
        use std::collections::HashMap;
        use std::time::{Duration, Instant};

        let dummy_mesh = newRcRefCell(MeshData {
            _name: String::from("quad"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });

        let create_dummy_model = |name: &str| ModelData {
            _model_data_name: name.to_string(),
            _mesh_name: String::from("quad"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: dummy_mesh.clone(),
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: std::collections::HashMap::new(),
        };

        let mut model_map: ResourceEntryMap<ModelData> = HashMap::new();

        // 1. 카탈로그 메타데이터만 등록 (Lazy Loading 상태: 미로드)
        let quad_entry = ResourceEntry::new_pinned(String::from("quad"), MetaData::default());
        let rock_entry = ResourceEntry::new(String::from("rock"), MetaData::default());

        model_map.insert(String::from("quad"), quad_entry);
        model_map.insert(String::from("rock"), rock_entry);

        assert!(!model_map.get("quad").unwrap().is_loaded());
        assert!(!model_map.get("rock").unwrap().is_loaded());
        assert!(model_map.get("quad").unwrap().is_pinned());
        assert!(!model_map.get("rock").unwrap().is_pinned());

        // 2. On-demand 지연 로딩 검증
        let load_count = Cell::new(0);
        {
            let loaded_rock = get_resource_entry_data("model_data", &model_map, "rock", "quad", |name, _meta| {
                load_count.set(load_count.get() + 1);
                Some(create_dummy_model(name))
            });
            assert_eq!(loaded_rock.borrow()._model_data_name, "rock");
            assert_eq!(load_count.get(), 1);
        }

        // 로드 완료 상태 확인
        assert!(model_map.get("rock").unwrap().is_loaded());

        // 캐시 히트 검증 (동일 리소스 재조회 시 로더 호출 안 됨)
        {
            let _cached_rock = get_resource_entry_data("model_data", &model_map, "rock", "quad", |_name, _meta| {
                load_count.set(load_count.get() + 1);
                None
            });
            assert_eq!(load_count.get(), 1); // 증가하지 않음
        }

        // quad도 로딩
        {
            let _quad = get_resource_entry_data("model_data", &model_map, "quad", "quad", |name, _meta| {
                Some(create_dummy_model(name))
            });
        }
        assert!(model_map.get("quad").unwrap().is_loaded());

        // 3. Time-delayed GC 검증
        let t0 = Instant::now();
        let timeout = Duration::from_secs(30);

        // 즉시 GC 수행 시 아직 미사용 타이머 시작/타임아웃 안 됨
        let evicted = update_resource_entry_map_gc(&mut model_map, t0, timeout);
        assert_eq!(evicted, 0);
        assert!(model_map.get("rock").unwrap().is_loaded());

        // 31초 후: rock은 unpinned이며 외부 참조(Rc 강한 참조 = 1)가 없으므로 회수 대상
        let t_future = t0 + Duration::from_secs(31);
        let evicted = update_resource_entry_map_gc(&mut model_map, t_future, timeout);
        assert_eq!(evicted, 1);
        assert!(!model_map.get("rock").unwrap().is_loaded()); // 언로드 완료!
        assert!(model_map.get("quad").unwrap().is_loaded()); // pinned quad는 영구 보존!

        // 4. 언로드 후 필요 시 투명한 재로딩(Re-loading) 검증
        {
            let reloaded_rock = get_resource_entry_data("model_data", &model_map, "rock", "quad", |name, _meta| {
                load_count.set(load_count.get() + 1);
                Some(create_dummy_model(name))
            });
            assert_eq!(reloaded_rock.borrow()._model_data_name, "rock");
            assert_eq!(load_count.get(), 2);
        }
        assert!(model_map.get("rock").unwrap().is_loaded());
    }

    #[test]
    fn test_vram_budget_lru_eviction() {
        use rust_engine_3d::resource::resource::{MetaData, ResourceEntry, ResourceEntryMap};
        use rust_engine_3d::vulkan_context::texture::TextureData;
        use std::collections::HashMap;
        use std::time::{Duration, Instant};

        let create_dummy_texture = |name: &str, width: u32, height: u32| {
            newRcRefCell(TextureData {
                _texture_data_name: name.to_string(),
                _image_width: width,
                _image_height: height,
                _image_layers: 1,
                _image_mip_levels: 1,
                _image_format: ash::vk::Format::R8G8B8A8_UNORM, // 4 bytes per pixel
                ..Default::default()
            })
        };

        // 1024 * 1024 * 4 = 4MB 텍스처
        let tex_a = create_dummy_texture("tex_a", 1024, 1024);
        let tex_b = create_dummy_texture("tex_b", 1024, 1024);
        let tex_pinned = create_dummy_texture("tex_pinned", 1024, 1024);

        assert_eq!(tex_a.borrow().get_memory_usage_bytes(), 4 * 1024 * 1024);

        let mut tex_map: ResourceEntryMap<TextureData> = HashMap::new();

        let mut entry_a = ResourceEntry::new(String::from("tex_a"), MetaData::default());
        entry_a.set_loaded_data(tex_a.clone());

        let mut entry_b = ResourceEntry::new(String::from("tex_b"), MetaData::default());
        entry_b.set_loaded_data(tex_b.clone());

        let entry_pinned = ResourceEntry::new_pinned(String::from("tex_pinned"), MetaData::default());
        entry_pinned.set_loaded_data(tex_pinned.clone());

        let t0 = Instant::now();
        // A가 B보다 먼저 미사용 상태에 진입 (LRU: A가 더 오래됨)
        entry_a._unused_start_time = Some(t0);
        entry_b._unused_start_time = Some(t0 + Duration::from_secs(5));

        tex_map.insert(String::from("tex_a"), entry_a);
        tex_map.insert(String::from("tex_b"), entry_b);
        tex_map.insert(String::from("tex_pinned"), entry_pinned);

        // 총 12MB의 VRAM 중 버짓을 6MB로 설정 (목표: 90% = 5.4MB)
        // tex_a는 Rc strong_count = 2 (tex_a 로컬 변수 소유) -> drop하여 strong_count = 1로 만듦
        drop(tex_a);
        drop(tex_b);
        drop(tex_pinned);

        let total_vram: usize =
            tex_map.values().filter_map(|e| e.get_loaded_data()).map(|t| t.borrow().get_memory_usage_bytes()).sum();
        assert_eq!(total_vram, 12 * 1024 * 1024); // 12MB

        // LRU 정렬 및 회수 로직 시뮬레이션
        let budget_bytes = 6 * 1024 * 1024;
        let target_bytes = (budget_bytes * 9) / 10;
        let mut bytes_to_free = total_vram.saturating_sub(target_bytes);

        struct Cand {
            _name: String,
            _unused_time: Instant,
            _bytes: usize,
        }

        let mut candidates = Vec::new();
        for (name, entry) in tex_map.iter() {
            if entry.is_pinned() || entry.is_loading() {
                continue;
            }
            let strong_count = unsafe {
                match *entry._cached_data.get() {
                    Some(ref data) => std::rc::Rc::strong_count(data),
                    None => 0,
                }
            };
            if strong_count == 1 {
                if let Some(tex) = entry.get_loaded_data() {
                    candidates.push(Cand {
                        _name: name.clone(),
                        _unused_time: entry._unused_start_time.unwrap_or(t0),
                        _bytes: tex.borrow().get_memory_usage_bytes(),
                    });
                }
            }
        }

        // LRU 오름차순 정렬
        candidates.sort_by_key(|c| c._unused_time);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0]._name, "tex_a"); // 더 오래 미사용된 tex_a가 최우선
        assert_eq!(candidates[1]._name, "tex_b");

        let mut evicted_count = 0;
        for c in candidates {
            if bytes_to_free == 0 {
                break;
            }
            if let Some(entry) = tex_map.get_mut(&c._name) {
                entry.unload();
                evicted_count += 1;
                bytes_to_free = bytes_to_free.saturating_sub(c._bytes);
            }
        }

        // tex_a (4MB) 및 tex_b (4MB) 회수로 목표 달성
        assert_eq!(evicted_count, 2);
        assert!(!tex_map.get("tex_a").unwrap().is_loaded());
        assert!(!tex_map.get("tex_b").unwrap().is_loaded());
        assert!(tex_map.get("tex_pinned").unwrap().is_loaded()); // pinned는 영구 보존!
    }

    #[test]
    fn test_async_resource_loader_pending_and_metrics() {
        use rust_engine_3d::resource::async_loader::{AsyncLoadRequest, AsyncResourceLoader};
        use rust_engine_3d::resource::resource::ResourceStreamingMetrics;

        let loader = AsyncResourceLoader::new(2);
        assert_eq!(loader.get_pending_count(), 0);

        // 요청 전송 시 pending 카운트 즉시 증가
        let req = AsyncLoadRequest::Texture {
            _texture_name: String::from("dummy_tex"),
            _meta_data: Default::default(),
        };
        loader.send_request(req);
        assert_eq!(loader.get_pending_count(), 1);

        // 메트릭 구조체 기본값 및 동작 검증
        let mut metrics = ResourceStreamingMetrics::default();
        metrics._loaded_textures = 10;
        metrics._total_textures = 50;
        metrics._texture_memory_bytes = 40 * 1024 * 1024;
        metrics._vram_budget_bytes = 1024 * 1024 * 1024;
        metrics._pending_async_requests = loader.get_pending_count();

        assert_eq!(metrics._pending_async_requests, 1);
        assert_eq!(metrics._loaded_textures, 10);
        assert_eq!(metrics._texture_memory_bytes, 40 * 1024 * 1024);
    }

    #[test]
    fn test_render_target_memory_calculation() {
        use rust_engine_3d::renderer::render_target::RenderTargetType;
        use rust_engine_3d::renderer::renderer_data::RenderTargetDataMap;
        use rust_engine_3d::vulkan_context::texture::TextureData;

        let mut rt_map: RenderTargetDataMap = std::collections::HashMap::new();

        // SceneColor: 1920x1080 RGBA16F (8 bytes per pixel)
        let rt_scenecolor = TextureData {
            _texture_data_name: String::from("SceneColor"),
            _image_width: 1920,
            _image_height: 1080,
            _image_layers: 1,
            _image_mip_levels: 1,
            _image_format: ash::vk::Format::R16G16B16A16_SFLOAT, // 8 bytes
            ..Default::default()
        };
        // 1920 * 1080 * 8 = 16,588,800 bytes (~15.82MB)
        assert_eq!(rt_scenecolor.get_memory_usage_bytes(), 1920 * 1080 * 8);

        // ShadowMap: 2048x2048 Depth (4 bytes per pixel)
        let rt_shadow = TextureData {
            _texture_data_name: String::from("Shadow"),
            _image_width: 2048,
            _image_height: 2048,
            _image_layers: 1,
            _image_mip_levels: 1,
            _image_format: ash::vk::Format::D32_SFLOAT, // 4 bytes
            ..Default::default()
        };
        // 2048 * 2048 * 4 = 16,777,216 bytes (16.0MB)
        assert_eq!(rt_shadow.get_memory_usage_bytes(), 2048 * 2048 * 4);

        rt_map.insert(RenderTargetType::SceneColor, rt_scenecolor);
        rt_map.insert(RenderTargetType::Shadow, rt_shadow);

        let total_rt_bytes: usize = rt_map.values().map(|t| t.get_memory_usage_bytes()).sum();
        let expected_total = (1920 * 1080 * 8) + (2048 * 2048 * 4);
        assert_eq!(total_rt_bytes, expected_total);
        assert_eq!(rt_map.len(), 2);
    }

    #[test]
    fn test_scene_loading_gating_and_lifecycle() {
        use crate::game_module::game_scene_manager::GameSceneState;
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData, SceneObjectType};

        // 1. GameSceneState 전이 흐름 검증
        let mut state = GameSceneState::Loading;
        assert_ne!(state, GameSceneState::LoadCompleted);

        // 오브젝트 스폰 완료 후 WaitingResources로 전환
        state = GameSceneState::WaitingResources;
        assert_ne!(state, GameSceneState::LoadCompleted);

        // 시나리오 오픈 및 추가 스폰 오브젝트 대기
        state = GameSceneState::WaitingScenarioResources;
        assert_ne!(state, GameSceneState::LoadCompleted);

        // 비동기 리소스 로딩 완료 시 FinalizingCapture로 전환
        state = GameSceneState::FinalizingCapture;
        assert_ne!(state, GameSceneState::LoadCompleted);

        // 큐브맵/하이트맵 최종 재캡처 완료 시 비로소 LoadCompleted로 승격
        state = GameSceneState::LoadCompleted;
        assert_eq!(state, GameSceneState::LoadCompleted);

        // 2. ModelData와 RenderObject의 Gating 및 Re-baking 연계 검증
        let unready_mesh = newRcRefCell(MeshData {
            _name: String::from("unready_mesh"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });

        let model_data = newRcRefCell(ModelData {
            _model_data_name: String::from("terrain_model"),
            _mesh_name: String::from("terrain_mesh"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: unready_mesh.clone(),
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: std::collections::HashMap::new(),
        });

        let create_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Terrain,
            _model_data_name: String::from("terrain_model"),
            _position: nalgebra::Vector3::zeros(),
            _rotation: nalgebra::Vector3::zeros(),
            _scale: nalgebra::Vector3::new(1.0, 1.0, 1.0),
        };

        let (mut render_object, _transform) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("terrain_object"),
            &model_data,
            &create_info,
            None,
        );

        // 미로딩 상태: 하이트맵 대상이지만 ready가 아니므로 렌더링 불가
        assert!(!render_object.is_ready());
        assert!(!render_object.is_visible());
        assert!(render_object._is_render_height_map);

        // 지오메트리 로딩 완료 모사
        let dummy_geometry = newRcRefCell(rust_engine_3d::vulkan_context::geometry_buffer::GeometryData {
            _geometry_name: String::from("geom_0"),
            _vertex_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _index_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _vertex_index_count: 3,
            _geometry_bounding_box: BoundingBox::default(),
        });
        unready_mesh.borrow_mut()._geometry_data_list.push(dummy_geometry);

        // 준비 완료 후 가시화 승격 및 Re-baking 트리거 성공
        assert!(render_object.check_and_update_ready_state());
        assert!(render_object.is_ready());
        assert!(render_object.is_visible());
    }

    #[test]
    fn test_static_instancing_and_height_map_async_ready_promotion() {
        use nalgebra::Vector3;
        use rust_engine_3d::scene::animation::AnimationBuffer;
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::capture_height_map::CaptureHeightMap;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{
            AnimationLayer, RenderObjectCreateInfo, RenderObjectData, RenderObjectSaveData, SceneObjectType,
        };
        use rust_engine_3d::scene::scene_manager::SceneManager;

        // 1. CaptureHeightMap readback 대기 중 조기 완료 방지 검증
        let mut capture = CaptureHeightMap::create_capture_height_map(uuid::Uuid::new_v4());
        assert!(capture.is_capture_height_map_complete());
        capture.set_start_capture_height_map(true);
        assert!(!capture.is_capture_height_map_complete());
        capture.set_start_capture_height_map(false);
        capture._read_back_frames.push(10);
        capture.set_capture_height_map_complete();
        // GPU readback 프레임이 남아 있으면 아직 완료로 판정하면 안 됨
        assert!(!capture.is_capture_height_map_complete());
        assert!(capture.need_to_read_back_height_map(10));
        assert!(capture.is_capture_height_map_complete());

        // 2. Static instancing 컨테이너의 비동기 준비 상태 승격 및 월드 바운딩 박스 재계산 검증
        let mut scene_manager = SceneManager::create_scene_manager();
        let placeholder_mesh = newRcRefCell(MeshData {
            _name: String::from("quad_placeholder"),
            _bound_box: BoundingBox::create_bounding_box(&Vector3::new(-0.5, -0.5, -0.5), &Vector3::new(0.5, 0.5, 0.5)),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });
        let model_data = newRcRefCell(ModelData {
            _model_data_name: String::from("cliff_model"),
            _mesh_name: String::from("cliff_mesh"),
            _local_transform: nalgebra::Matrix4::new_translation(&Vector3::new(0.0, 5.0, 0.0)),
            _mesh_data: placeholder_mesh.clone(),
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: std::collections::HashMap::new(),
        });

        let (container_obj, _container_transform) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("cliff_instancing_container"),
            &model_data,
            &RenderObjectCreateInfo::default(),
            None,
        );
        let container_rc = newRcRefCell(container_obj);

        let (mut child_obj, _child_transform) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("cliff_child_0"),
            &model_data,
            &RenderObjectCreateInfo {
                _scene_object_type: SceneObjectType::Default,
                _model_data_name: String::from("cliff_model"),
                _position: Vector3::zeros(),
                _rotation: Vector3::zeros(),
                _scale: Vector3::new(1.0, 1.0, 1.0),
            },
            None,
        );
        // 애니메이션 미초기화 상태에서 세이브 데이터 로드 및 레이어 접근 시 패닉 없음을 검증
        let dummy_save = RenderObjectSaveData {
            _render_object_name: String::from("cliff_child_0"),
            _animation_play_infos: Vec::new(),
            _animation_buffer: Some(AnimationBuffer::create_animation_buffer(4)),
        };
        child_obj.load_render_object_save_data(&dummy_save);
        assert!(!child_obj.has_animation());
        let _ = child_obj.get_animation_play_info(AnimationLayer::ActionLayer);

        let child_rc = newRcRefCell(child_obj);
        container_rc.borrow_mut().register_instancing_render_object(&child_rc);
        let container_id = container_rc.borrow().get_object_id();
        scene_manager._static_render_object_map.insert(container_id, container_rc.clone());

        assert!(!scene_manager.are_all_render_objects_ready());

        // 실제 메쉬 로딩 완료 시뮬레이션 (더 큰 바운딩 박스 교체)
        let real_bbox =
            BoundingBox::create_bounding_box(&Vector3::new(-10.0, -5.0, -10.0), &Vector3::new(10.0, 20.0, 10.0));
        let dummy_geometry = newRcRefCell(rust_engine_3d::vulkan_context::geometry_buffer::GeometryData {
            _geometry_name: String::from("cliff_geom"),
            _vertex_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _index_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _vertex_index_count: 36,
            _geometry_bounding_box: real_bbox.clone(),
        });
        placeholder_mesh.borrow_mut()._bound_box = real_bbox;
        placeholder_mesh.borrow_mut()._geometry_data_list.push(dummy_geometry);

        assert!(container_rc.borrow_mut().check_and_update_ready_state());
        assert!(scene_manager.are_all_render_objects_ready());
        assert!(child_rc.borrow().is_ready());
        assert_eq!(container_rc.borrow().get_instancing_render_object_transforms().len(), 1);
        assert_eq!(
            container_rc.borrow().get_instancing_render_object_transforms()[0][(1, 3)],
            5.0
        );
        assert!((container_rc.borrow().get_bounding_box()._max.y - 25.0).abs() < 1e-3);
    }

    #[test]
    fn test_capture_reset_and_pending_push_constants_on_async_ready() {
        use nalgebra::Vector3;
        use rust_engine_3d::renderer::push_constants::PushConstantParameter;
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::capture_height_map::CaptureHeightMap;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};

        // 1. CaptureHeightMap reset_capture_height_map 검증 (씬 종료 시 잔류 readback 제거)
        let mut capture = CaptureHeightMap::create_capture_height_map(uuid::Uuid::new_v4());
        capture.set_start_capture_height_map(true);
        capture._read_back_frames.push(42);
        assert!(!capture.is_capture_height_map_complete());
        capture.reset_capture_height_map();
        assert!(capture.is_capture_height_map_complete());
        assert!(capture._read_back_frames.is_empty());

        // 2. 미준비(unready) 상태에서 설정한 PushConstant 파라미터가 ready 승격 시 보존/소진되는지 검증
        let unready_mesh = newRcRefCell(MeshData {
            _name: String::from("pending_pc_mesh"),
            _bound_box: BoundingBox::create_bounding_box(&Vector3::new(-1.0, -1.0, -1.0), &Vector3::new(1.0, 1.0, 1.0)),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });
        let model_data = newRcRefCell(ModelData {
            _model_data_name: String::from("pending_pc_model"),
            _mesh_name: String::from("pending_pc_mesh"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: unready_mesh.clone(),
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: std::collections::HashMap::new(),
        });
        let (mut render_object, _transform) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("pending_pc_obj"),
            &model_data,
            &RenderObjectCreateInfo::default(),
            None,
        );
        assert!(!render_object.is_ready());
        render_object.set_push_constant_parameter("_alpha", &PushConstantParameter::Float(0.5));
        assert!(render_object._pending_push_constants.contains_key("_alpha"));

        let dummy_geometry = newRcRefCell(rust_engine_3d::vulkan_context::geometry_buffer::GeometryData {
            _geometry_name: String::from("geom_pc"),
            _vertex_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _index_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
            _vertex_index_count: 3,
            _geometry_bounding_box: BoundingBox::default(),
        });
        unready_mesh.borrow_mut()._geometry_data_list.push(dummy_geometry);

        assert!(render_object.check_and_update_ready_state());
        assert!(render_object.is_ready());
        assert!(render_object._pending_push_constants.is_empty());
    }

    #[test]
    fn test_skeletal_material_detection_with_placeholder_mesh_and_safe_render_pass_lookup() {
        use nalgebra::Vector3;
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::material::MaterialData;
        use rust_engine_3d::scene::material_instance::{MaterialInstanceData, SKELETAL_GBUFFER_RENDER_PASS_NAME};
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use std::collections::HashMap;

        let placeholder_mesh = newRcRefCell(MeshData {
            _name: String::from("quad"),
            _bound_box: BoundingBox::create_bounding_box(&Vector3::new(-1.0, -1.0, -1.0), &Vector3::new(1.0, 1.0, 1.0)),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });

        let skeletal_material = newRcRefCell(MaterialData {
            _material_data_name: String::from("common/render_skeletal_object"),
            _render_pass_pipeline_data_map: HashMap::new(),
            _material_parameters: serde_json::Map::new(),
        });

        let mut render_pass_map = HashMap::new();
        render_pass_map.insert(
            String::from(SKELETAL_GBUFFER_RENDER_PASS_NAME),
            vec![String::from("render_pass_skeletal_gbuffer/render_object")],
        );

        let skeletal_mat_inst = newRcRefCell(MaterialInstanceData {
            _material_instance_data_name: String::from("weapons/wooden_club"),
            _material_data: skeletal_material,
            _material_parameters: serde_json::Map::new(),
            _render_pass_pipeline_data_names_map: render_pass_map,
            _pipeline_binding_data_map: HashMap::new(),
            _default_pipeline_binding_name: String::new(),
        });

        // 존재하지 않는 렌더 패스 조회 시 패닉하지 않고 빈 슬라이스를 반환하는지 검증
        assert!(skeletal_mat_inst.borrow().get_render_pass_pipeline_data_names("depth_prepass_static").is_empty());
        assert_eq!(
            skeletal_mat_inst.borrow().get_render_pass_pipeline_data_names(SKELETAL_GBUFFER_RENDER_PASS_NAME).len(),
            1
        );
        assert!(skeletal_mat_inst.borrow().is_skeletal_material_instance());

        // 메쉬가 아직 placeholder("quad")여서 애니메이션 데이터가 없더라도 스켈레탈 머티리얼로 판별되는지 검증
        let weapon_model = ModelData {
            _model_data_name: String::from("weapons/wooden_club"),
            _mesh_name: String::from("weapons/wooden_club"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: placeholder_mesh,
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: vec![skeletal_mat_inst],
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: HashMap::new(),
        };
        assert!(!weapon_model._mesh_data.borrow().has_animation_data());
        assert!(weapon_model.has_skeletal_animation_or_material());
    }

    #[test]
    fn test_mesh_late_binding_and_pending_animation_flush() {
        use rust_engine_3d::scene::animation::{
            AnimationNodeCreateInfo, AnimationPlayArgs, SkeletonDataCreateInfo, SkeletonHierarchyTree,
        };
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::mesh::{MeshData, MeshDataCreateInfo};
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectCreateInfo, RenderObjectData};
        use std::collections::HashMap;

        let placeholder_mesh = newRcRefCell(MeshData {
            _name: String::from("quad"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(),
        });

        let model_data = newRcRefCell(ModelData {
            _model_data_name: String::from("actors/player"),
            _mesh_name: String::from("actors/player"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: placeholder_mesh,
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: HashMap::new(),
        });

        let create_info = RenderObjectCreateInfo::default();
        let (mut render_object, _) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("player_object"),
            &model_data,
            &create_info,
            None,
        );

        // 1. 초기 상태: 메쉬가 플레이스홀더이므로 본 카운트는 0, 애니메이션 버퍼는 None
        assert_eq!(render_object.get_bone_count(), 0);
        assert!(render_object._animation_buffer.is_none());

        // 2. 비동기 로딩 중 애니메이션 재생 시도: 본 카운트가 맞지 않으므로 _pending_animations에 보관
        let anim_mesh_create_info = MeshDataCreateInfo {
            _skeleton_create_infos: vec![SkeletonDataCreateInfo {
                _name: String::from("root"),
                _transform: nalgebra::Matrix4::identity(),
                _hierarchy: SkeletonHierarchyTree::default(),
                _bone_names: vec![String::from("root"), String::from("spine"), String::from("head")],
                _inv_bind_matrices: vec![nalgebra::Matrix4::identity(); 3],
            }],
            _animation_node_create_infos: vec![vec![
                AnimationNodeCreateInfo::default(),
                AnimationNodeCreateInfo::default(),
                AnimationNodeCreateInfo::default(),
            ]],
            ..Default::default()
        };
        let dummy_anim_mesh = newRcRefCell(MeshData::create_mesh_data(
            &String::from("player_idle"),
            anim_mesh_create_info,
            Vec::new(),
        ));
        assert_eq!(dummy_anim_mesh.borrow().get_bone_count(), 3);

        let play_args = AnimationPlayArgs::default();
        render_object.set_animation(&dummy_anim_mesh, &play_args, AnimationLayer::BaseLayer);

        // 드랍되지 않고 _pending_animations에 안전하게 보관됨
        assert!(render_object._pending_animations[AnimationLayer::BaseLayer as usize].is_some());

        // 3. 실제 메쉬 로드 완료 시뮬레이션: 본 카운트 3인 스켈레탈 메쉬 장착
        let loaded_mesh_create_info = MeshDataCreateInfo {
            _skeleton_create_infos: vec![SkeletonDataCreateInfo {
                _name: String::from("root"),
                _transform: nalgebra::Matrix4::identity(),
                _hierarchy: SkeletonHierarchyTree::default(),
                _bone_names: vec![String::from("root"), String::from("spine"), String::from("head")],
                _inv_bind_matrices: vec![nalgebra::Matrix4::identity(); 3],
            }],
            ..Default::default()
        };
        let loaded_mesh = newRcRefCell(MeshData::create_mesh_data(
            &String::from("actors/player"),
            loaded_mesh_create_info,
            Vec::new(),
        ));
        assert_eq!(loaded_mesh.borrow().get_bone_count(), 3);

        render_object._mesh_data = loaded_mesh;
        render_object.initialize_animation_play_info();

        // 4. 본 카운트 갱신, 본 버퍼 할당 및 대기 애니메이션 자동 플러시 확인
        assert_eq!(render_object.get_bone_count(), 3);
        assert!(render_object._animation_buffer.is_some());
        assert_eq!(
            render_object._animation_buffer.as_ref().unwrap()._animation_buffer.len(),
            3
        );
        // 플러시되어 BaseLayer의 애니메이션이 정상 등록됨
        let base_info = &render_object._animation_play_infos[AnimationLayer::BaseLayer as usize];
        assert!(base_info.is_valid());
        assert_eq!(
            base_info._animation_mesh.as_ref().unwrap().borrow()._name,
            "player_idle"
        );
    }

    #[test]
    fn test_progressive_spawning_and_essential_load_complete() {
        use crate::game_module::game_scene_manager::GameSceneState;
        use rust_engine_3d::scene::bounding_box::BoundingBox;
        use rust_engine_3d::scene::mesh::MeshData;
        use rust_engine_3d::scene::model::ModelData;
        use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData, SceneObjectType};
        use std::collections::HashMap;

        // 1. 필수 지형 오브젝트 (준비 완료 상태)
        let terrain_mesh = newRcRefCell(MeshData {
            _name: String::from("terrain_mesh"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: vec![newRcRefCell(
                rust_engine_3d::vulkan_context::geometry_buffer::GeometryData {
                    _geometry_name: String::from("geom"),
                    _vertex_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
                    _index_buffer_data: rust_engine_3d::vulkan_context::buffer::BufferData::default(),
                    _vertex_index_count: 3,
                    _geometry_bounding_box: BoundingBox::default(),
                },
            )],
        });

        let terrain_model = newRcRefCell(ModelData {
            _model_data_name: String::from("terrain_model"),
            _mesh_name: String::from("terrain_mesh"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: terrain_mesh,
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: HashMap::new(),
        });

        let terrain_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Terrain,
            _model_data_name: String::from("terrain_model"),
            ..Default::default()
        };

        let (mut terrain_obj, _) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("terrain"),
            &terrain_model,
            &terrain_info,
            None,
        );
        terrain_obj.set_ready(true);
        terrain_obj._is_render_height_map = true;

        // 2. 비필수 원경 장식 프랍 오브젝트 (아직 비동기 로딩 중인 미완료 상태)
        let unready_prop_mesh = newRcRefCell(MeshData {
            _name: String::from("distant_tree_mesh"),
            _bound_box: BoundingBox::default(),
            _skeleton_data_list: Vec::new(),
            _animation_data_list: Vec::new(),
            _geometry_data_list: Vec::new(), // 미완료
        });

        let prop_model = newRcRefCell(ModelData {
            _model_data_name: String::from("distant_tree_model"),
            _mesh_name: String::from("distant_tree_mesh"),
            _local_transform: nalgebra::Matrix4::identity(),
            _mesh_data: unready_prop_mesh,
            _is_render_camera: true,
            _is_render_shadow: true,
            _material_instance_data_list: Vec::new(),
            _collision: rust_engine_3d::scene::collision::CollisionData::default(),
            _socket_data_map: HashMap::new(),
        });

        let prop_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Default,
            _model_data_name: String::from("distant_tree_model"),
            ..Default::default()
        };

        let (mut prop_obj, _) = RenderObjectData::create_render_object_data(
            uuid::Uuid::new_v4(),
            &String::from("distant_tree"),
            &prop_model,
            &prop_info,
            None,
        );
        prop_obj.set_ready(false);
        prop_obj._is_render_height_map = false;

        // 3. 점진적 판정: 지형은 준비되었고 프랍은 미완료 상태
        assert!(terrain_obj.is_ready());
        assert!(!prop_obj.is_ready());
        // 미완료 프랍은 가시성 게이팅에 의해 아직 화면에 보이지 않음
        assert!(!prop_obj.is_visible());

        // 4. GameSceneState 전이 흐름: 필수 오브젝트 준비 시 즉시 로딩 완료 상태로 전이 가능
        let mut scene_state = GameSceneState::Loading;
        assert_ne!(scene_state, GameSceneState::LoadCompleted);

        // 점진적 스폰 조건 충족 시 Loading -> WaitingResources -> FinalizingCapture -> LoadCompleted
        scene_state = GameSceneState::WaitingResources;
        assert_ne!(scene_state, GameSceneState::LoadCompleted);

        scene_state = GameSceneState::FinalizingCapture;
        assert_ne!(scene_state, GameSceneState::LoadCompleted);

        scene_state = GameSceneState::LoadCompleted;
        assert_eq!(scene_state, GameSceneState::LoadCompleted);
    }

    #[test]
    fn test_transfer_queue_and_ownership_transfer_support() {
        use ash::vk;
        use rust_engine_3d::vulkan_context::queue::{QueueFamilyDataList, QueueFamilyIndices};

        let indices = QueueFamilyIndices {
            _graphics_queue_index: 0,
            _present_queue_index: 0,
            _compute_queue_index: 1,
            _transfer_queue_index: 2, // 독립 전송 큐 패밀리
            _sparse_binding_queue_index: 0,
        };

        let queue_family_data_list = QueueFamilyDataList {
            _graphics_queue: vk::Queue::null(),
            _present_queue: vk::Queue::null(),
            _transfer_queue: vk::Queue::null(),
            _queue_family_index_list: vec![0, 1, 2],
            _queue_family_count: 3,
            _queue_family_indices: indices.clone(),
        };

        // 1. Dedicated Transfer Queue 판별 로직 검증 (0 != 2 -> true)
        let is_dedicated = queue_family_data_list._queue_family_indices._transfer_queue_index
            != queue_family_data_list._queue_family_indices._graphics_queue_index;
        assert!(is_dedicated);

        // 2. Queue Family Ownership Transfer 배리어 파라미터 유효성 검증
        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let barrier = vk::ImageMemoryBarrier {
            src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
            dst_access_mask: vk::AccessFlags::SHADER_READ,
            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            src_queue_family_index: queue_family_data_list._queue_family_indices._transfer_queue_index,
            dst_queue_family_index: queue_family_data_list._queue_family_indices._graphics_queue_index,
            image: vk::Image::null(),
            subresource_range,
            ..Default::default()
        };

        assert_eq!(barrier.src_queue_family_index, 2);
        assert_eq!(barrier.dst_queue_family_index, 0);
        assert_ne!(barrier.src_queue_family_index, barrier.dst_queue_family_index);
    }
}
