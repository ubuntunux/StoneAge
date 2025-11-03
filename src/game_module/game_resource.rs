use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::game_module::actors::character_data::{CharacterData, CharacterDataCreateInfo};
use crate::game_module::actors::items::ItemData;
use crate::game_module::actors::props::PropData;
use crate::game_module::actors::weapons::WeaponData;
use crate::game_module::actors::weapons::WeaponDataCreateInfo;
use crate::game_module::game_scene_manager::GameSceneDataCreateInfo;
use crate::game_module::scenario::scenario::ScenarioDataCreateInfo;
use rust_engine_3d::resource::resource::{
    get_resource_data_must, get_unique_resource_name, EngineResources, ResourceDataContainer,
    APPLICATION_RESOURCE_PATH,
};
use rust_engine_3d::utilities::system::{self, newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde_json::{self};

pub const GAME_DATA_DIRECTORY: &str = "game_data";
pub const CHARACTER_DATA_FILE_PATH: &str = "game_data/characters";
pub const ITEM_DATA_FILE_PATH: &str = "game_data/items";
pub const GAME_SCENE_FILE_PATH: &str = "game_data/game_scenes";
pub const PROP_DATA_FILE_PATH: &str = "game_data/props";
pub const SCENARIO_FILE_PATH: &str = "game_data/scenario";
pub const WEAPON_DATA_FILE_PATH: &str = "game_data/weapons";

pub const EXT_GAME_DATA: &str = "data";

pub const DEFAULT_GAME_DATA_NAME: &str = "default";

pub type CharacterDataMap = ResourceDataContainer<CharacterData>;
pub type ItemDataMap = ResourceDataContainer<ItemData>;
pub type GameSceneDataCreateInfoMap = ResourceDataContainer<GameSceneDataCreateInfo>;
pub type PropDataMap = ResourceDataContainer<PropData>;
pub type ScenarioDataCreateInfoMap = ResourceDataContainer<ScenarioDataCreateInfo>;
pub type WeaponDataMap<'a> = ResourceDataContainer<WeaponData<'a>>;

#[derive(Clone)]
pub struct GameResources<'a> {
    _engine_resources: *const EngineResources<'a>,
    _scenario_data_create_info_map: ScenarioDataCreateInfoMap,
    _game_scene_data_create_info_map: GameSceneDataCreateInfoMap,
    _character_data_map: CharacterDataMap,
    _item_data_map: ItemDataMap,
    _prop_data_map: PropDataMap,
    _weapon_data_map: WeaponDataMap<'a>,
}

impl<'a> GameResources<'a> {
    pub fn create_game_resources() -> Box<GameResources<'a>> {
        Box::new(GameResources {
            _engine_resources: std::ptr::null(),
            _scenario_data_create_info_map: ScenarioDataCreateInfoMap::new(),
            _game_scene_data_create_info_map: GameSceneDataCreateInfoMap::new(),
            _character_data_map: CharacterDataMap::new(),
            _item_data_map: ItemDataMap::new(),
            _prop_data_map: PropDataMap::new(),
            _weapon_data_map: WeaponDataMap::new(),
        })
    }
    pub fn get_engine_resources(&self) -> &EngineResources<'a> {
        ptr_as_ref(self._engine_resources)
    }
    pub fn get_engine_resources_mut(&self) -> &mut EngineResources<'a> {
        ptr_as_mut(self._engine_resources)
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources()
            .collect_resources(dir, extensions)
    }
    pub fn initialize_game_resources(&mut self, engine_resources: &EngineResources<'a>) {
        self._engine_resources = engine_resources;
    }

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
        let scenario_data_files: Vec<PathBuf> =
            self.collect_resources(&scenario_directory, &[EXT_GAME_DATA]);
        for scenario_data_file in scenario_data_files {
            let scenario_data_name = get_unique_resource_name(
                &self._scenario_data_create_info_map,
                &game_data_directory,
                &scenario_data_file,
            );
            let loaded_contents = system::load(&scenario_data_file);
            let scenario_data_create_info: ScenarioDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._scenario_data_create_info_map.insert(
                scenario_data_name.clone(),
                newRcRefCell(scenario_data_create_info),
            );
        }
    }

    pub fn unload_scenario_data(&mut self) {
        self._scenario_data_create_info_map.clear();
    }

    pub fn has_scenario_data(&self, resource_name: &str) -> bool {
        self._scenario_data_create_info_map
            .get(resource_name)
            .is_some()
    }

    pub fn get_scenario_data(&self, resource_name: &str) -> &RcRefCell<ScenarioDataCreateInfo> {
        get_resource_data_must(
            "scenario_data",
            &self._scenario_data_create_info_map,
            resource_name,
        )
    }

    // game scene data
    pub fn load_game_scene_data(&mut self) {
        log::info!("    load_game_scene_data");
        let game_data_directory = PathBuf::from(GAME_DATA_DIRECTORY);
        let game_scene_directory = PathBuf::from(GAME_SCENE_FILE_PATH);
        let game_scene_data_files: Vec<PathBuf> =
            self.collect_resources(&game_scene_directory, &[EXT_GAME_DATA]);
        for game_scene_data_file in game_scene_data_files {
            let game_scene_data_name = get_unique_resource_name(
                &self._game_scene_data_create_info_map,
                &game_data_directory,
                &game_scene_data_file,
            );
            let loaded_contents = system::load(&game_scene_data_file);
            let game_scene_data_create_info: GameSceneDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._game_scene_data_create_info_map.insert(
                game_scene_data_name.clone(),
                newRcRefCell(game_scene_data_create_info),
            );
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
        let mut write_file =
            File::create(&game_scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String =
            serde_json::to_string(&game_scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file
            .write(write_contents.as_bytes())
            .expect("Failed to write");

        self._game_scene_data_create_info_map.insert(
            String::from(game_scene_data_name),
            newRcRefCell(game_scene_data_create_info.clone()),
        );
    }

    pub fn has_game_scene_data(&self, resource_name: &str) -> bool {
        self._game_scene_data_create_info_map
            .get(resource_name)
            .is_some()
    }

    pub fn get_game_scene_data(&self, resource_name: &str) -> &RcRefCell<GameSceneDataCreateInfo> {
        get_resource_data_must(
            "game_scene_data",
            &self._game_scene_data_create_info_map,
            resource_name,
        )
    }

    // Game Data
    fn load_game_data(&mut self) {
        log::info!("    load_game_data");
        self.load_character_data();
        self.load_item_data();
        self.load_prop_data();
        self.load_weapon_data();
    }

    fn unload_game_data(&mut self) {
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
        let game_data_files: Vec<PathBuf> =
            self.collect_resources(&prop_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let prop_data_name = get_unique_resource_name(
                &self._prop_data_map,
                &game_data_directory,
                &game_data_file,
            );
            let loaded_contents = system::load(&game_data_file);
            let prop_data: PropData =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._prop_data_map
                .insert(prop_data_name.clone(), newRcRefCell(prop_data));
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
        let game_data_files: Vec<PathBuf> =
            self.collect_resources(&weapon_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let weapon_data_name = get_unique_resource_name(
                &self._weapon_data_map,
                &game_data_directory,
                &game_data_file,
            );
            let loaded_contents = system::load(&game_data_file);
            let weapon_data_create_info: WeaponDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let weapon_model_data = self
                .get_engine_resources()
                .get_model_data(&weapon_data_create_info._model_data_name);
            let weapon_data =
                WeaponData::create_weapon_data(&weapon_data_create_info, weapon_model_data);
            self._weapon_data_map
                .insert(weapon_data_name.clone(), newRcRefCell(weapon_data));
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
        let game_data_files: Vec<PathBuf> =
            self.collect_resources(&character_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let character_data_name = get_unique_resource_name(
                &self._character_data_map,
                &game_data_directory,
                &game_data_file,
            );
            let loaded_contents = system::load(&game_data_file);
            let character_data_create_info: CharacterDataCreateInfo =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let character_data =
                CharacterData::create_character_data(&character_data_create_info, self);
            self._character_data_map
                .insert(character_data_name.clone(), newRcRefCell(character_data));
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
        let game_data_files: Vec<PathBuf> =
            self.collect_resources(&item_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let item_data_name = get_unique_resource_name(
                &self._item_data_map,
                &game_data_directory,
                &game_data_file,
            );
            let loaded_contents = system::load(&game_data_file);
            let item_data: ItemData =
                serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._item_data_map
                .insert(item_data_name.clone(), newRcRefCell(item_data));
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
}
