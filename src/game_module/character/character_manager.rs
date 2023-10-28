use std::collections::HashMap;

use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};

use crate::application::application::Application;
use crate::game_module::character::character::{Character, CharacterCreateInfo};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type CharacterMap = HashMap<u64, RcRefCell<Character>>;

pub struct CharacterManager {
    pub _game_client: *const GameClient,
    pub _game_scene_manager: *const GameSceneManager,
    pub _game_resources: *const GameResources,
    pub _id_generator: u64,
    pub _player: Option<RcRefCell<Character>>,
    pub _characters: CharacterMap
}

impl CharacterManager {
    pub fn create_character_manager() -> Box<CharacterManager> {
        Box::new(CharacterManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _id_generator: 0,
            _player: None,
            _characters: HashMap::new(),
        })
    }

    pub fn initialize_character_manager(&mut self, application: &Application) {
        log::info!("initialize_character_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
    }
    pub fn destroy_character_manager(&mut self) {

    }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager { ptr_as_ref(self._game_scene_manager) }
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager { ptr_as_mut(self._game_scene_manager) }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }
    pub fn get_character(&self, character_id: u64) -> Option<&RcRefCell<Character>> {
        self._characters.get(&character_id)
    }
    pub fn create_character(&mut self, character_name: &str, character_create_info: &CharacterCreateInfo, is_player: bool) -> RcRefCell<Character> {
        let game_resources = ptr_as_ref(self._game_resources);
        let character_data = game_resources.get_character_data(character_create_info._character_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: character_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_game_scene_manager().get_scene_manager_mut().add_skeletal_render_object(
            character_create_info._character_data_name.as_str(),
            &render_object_create_info
        );
        let id = self.generate_id();
        let character = newRcRefCell(Character::create_character_instance(
            id,
            character_name,
            character_data,
            &render_object_data
        ));

        if is_player {
            self._player = Some(character.clone());
        }

        // set character transform
        {
            let mut character_mut = character.borrow_mut();
            character_mut._controller.as_mut()._position.clone_from(&character_create_info._position);
            character_mut._controller.as_mut()._rotation.clone_from(&character_create_info._rotation);
            character_mut._controller.as_mut()._scale.clone_from(&character_create_info._scale);
        }
        self._characters.insert(id, character.clone());
        character
    }
    pub fn remove_character(&mut self, character: &mut Character) {
        self._characters.remove(&character.get_character_id());
    }
    pub fn get_player(&self) -> &RcRefCell<Character> {
        self._player.as_ref().unwrap()
    }
    pub fn update_character_manager(&mut self, _delta_time: f32) {
        // let game_client = ptr_as_ref(self._game_client);
        // for character in self._characters.values() {
        //     let character_mut = ptr_as_mut(character.as_ref());
        //     if character_mut._is_player {
        //         character_mut.update_character_controller(game_client, delta_time);
        //     }
        // }
    }
}