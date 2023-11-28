use std::collections::HashMap;

use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};

use crate::application::application::Application;
use crate::game_module::character::animation_blend_mask::AnimationBlendMasks;
use crate::game_module::character::character::{ActionAnimationState, Character, CharacterCreateInfo};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type CharacterMap = HashMap<u64, RcRefCell<Character>>;

pub struct CharacterManager {
    pub _game_client: *const GameClient,
    pub _game_scene_manager: *const GameSceneManager,
    pub _game_resources: *const GameResources,
    pub _animation_blend_masks: Box<AnimationBlendMasks>,
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
            _animation_blend_masks: Box::new(AnimationBlendMasks::create_animation_blend_maks()),
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
            character_name,
            &render_object_create_info
        );
        let idle_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._idle_animation_mesh);
        let walk_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._walk_animation_mesh);
        let jump_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._jump_animation_mesh);
        let attack_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._attack_animation_mesh);
        let id = self.generate_id();
        let character = newRcRefCell(Character::create_character_instance(
            id,
            is_player,
            character_name,
            character_data,
            &render_object_data,
            idle_animation,
            walk_animation,
            jump_animation,
            attack_animation,
            self._animation_blend_masks.as_ref(),
            &character_create_info._position,
            &character_create_info._rotation,
            &character_create_info._scale
        ));
        if is_player {
            self._player = Some(character.clone());
        }
        self._characters.insert(id, character.clone());
        character
    }
    pub fn remove_character(&mut self, character: &RcRefCell<Character>) {
        self._characters.remove(&character.borrow().get_character_id());
        self.get_game_scene_manager().get_scene_manager_mut().remove_skeletal_render_object(&character.borrow()._character_name);
    }
    pub fn get_player(&self) -> &RcRefCell<Character> {
        self._player.as_ref().unwrap()
    }
    pub fn update_character_manager(&mut self, _engine_core: &EngineCore, delta_time: f64) {
        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        let player = ptr_as_ref(self._player.as_ref().unwrap().as_ptr());

        for character in self._characters.values() {
            let mut character_mut = character.borrow_mut();
            character_mut.update_character(delta_time as f32);
            if character_mut._character_id != player._character_id && player.is_action(ActionAnimationState::ATTACK) {
                dead_characters.push(character.clone());
            }
        }

        for character in dead_characters.iter_mut() {
            self.remove_character(character);
        }
    }
}