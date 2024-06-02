use std::collections::HashMap;
use nalgebra::Vector3;

use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};

use crate::application::application::Application;
use crate::game_module::actors::animation_blend_mask::AnimationBlendMasks;
use crate::game_module::actors::character::{Character, CharacterCreateInfo};
use crate::game_module::actors::foods::FoodCreateInfo;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

pub type CharacterMap<'a> = HashMap<u64, RcRefCell<Character<'a>>>;

pub struct CharacterManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _animation_blend_masks: Box<AnimationBlendMasks>,
    pub _id_generator: u64,
    pub _player: Option<RcRefCell<Character<'a>>>,
    pub _characters: CharacterMap<'a>
}

impl<'a> CharacterManager<'a> {
    pub fn create_character_manager() -> Box<CharacterManager<'a>> {
        Box::new(CharacterManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _animation_blend_masks: Box::new(AnimationBlendMasks::create_animation_blend_maks()),
            _id_generator: 0,
            _player: None,
            _characters: HashMap::new(),
        })
    }

    pub fn initialize_character_manager(&mut self, engine_core: &EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_character_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
    }
    pub fn destroy_character_manager(&mut self) {

    }
    pub fn get_game_client(&self) -> &GameClient<'a> {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient<'a> {
        ptr_as_mut(self._game_client)
    }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager<'a> {
        ptr_as_ref(self._game_scene_manager)
    }
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager<'a> {
        ptr_as_mut(self._game_scene_manager)
    }
    pub fn get_audio_manager(&self) -> &AudioManager<'a> {
        ptr_as_ref(self._audio_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_scene_manager(&self) -> &SceneManager<'a> {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }
    pub fn get_character(&self, character_id: u64) -> Option<&RcRefCell<Character<'a>>> {
        self._characters.get(&character_id)
    }
    pub fn create_character(&mut self, character_name: &str, character_create_info: &CharacterCreateInfo, is_player: bool) -> RcRefCell<Character<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let character_data = game_resources.get_character_data(character_create_info._character_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: character_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_skeletal_render_object(
            character_name,
            &render_object_create_info
        );
        let dead_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._dead_animation_mesh);
        let idle_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._idle_animation_mesh);
        let hit_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._hit_animation_mesh);
        let walk_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._walk_animation_mesh);
        let jump_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._jump_animation_mesh);
        let attack_animation = game_resources.get_engine_resources().get_mesh_data(&character_data.borrow()._attack_animation_mesh);
        let id = self.generate_id();
        let character = newRcRefCell(Character::create_character_instance(
            self,
            id,
            is_player,
            character_name,
            character_data,
            &render_object_data,
            dead_animation,
            idle_animation,
            hit_animation,
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
        self.get_scene_manager_mut().remove_skeletal_render_object(character.borrow()._render_object.borrow()._object_id);
    }
    pub fn get_player(&self) -> &RcRefCell<Character<'a>> {
        self._player.as_ref().unwrap()
    }

    pub fn play_audio(&self, audio_name_bank: &str) {
        self.get_audio_manager_mut().create_audio_instance_from_bank(audio_name_bank, AudioLoop::ONCE);
    }

    pub fn play_effect(&self, effect_name: &str, effect_create_info: &EffectCreateInfo) {
        self.get_scene_manager_mut().add_effect(effect_name, effect_create_info);
    }

    pub fn update_character_manager(&mut self, delta_time: f64) {
        for character in self._characters.values() {
            let mut character_mut = character.borrow_mut();
            character_mut.update_character(self.get_game_scene_manager(), delta_time as f32);
        }

        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        let player = ptr_as_ref(self._player.as_ref().unwrap().as_ptr());
        if player.is_attacking() {
            for character in self._characters.values() {
                let mut character_mut = character.borrow_mut();
                if character_mut._is_alive {
                    if character_mut._character_id != player._character_id {
                        if character_mut.collide_point(&player.get_attack_point()) {
                            character_mut.set_damage(player.get_attack_point(), player.get_power());
                            if false == character_mut._is_alive {
                                dead_characters.push(character.clone());

                                // TestCode: Food
                                let food_create_info = FoodCreateInfo {
                                    _food_data_name: String::from("meat"),
                                    _position: character_mut.get_position().clone() + Vector3::new(0.0, 0.5, 0.0),
                                    ..Default::default()
                                };
                                ptr_as_mut(self.get_game_scene_manager()._food_manager.clone()).create_food("food", &food_create_info);
                            }
                        }
                    }
                }
            }
        }

        for character in dead_characters.iter() {
            self.remove_character(character);
        }
    }
}