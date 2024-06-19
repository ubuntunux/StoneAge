use std::collections::HashMap;

use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};

use crate::application::application::Application;
use crate::game_module::actors::character::{Character, CharacterCreateInfo};
use crate::game_module::actors::character_data::ActionAnimationState;
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
    pub _id_generator: u64,
    pub _player: Option<RcRefCell<Character<'a>>>,
    pub _characters: CharacterMap<'a>,
}

impl<'a> CharacterManager<'a> {
    pub fn create_character_manager() -> Box<CharacterManager<'a>> {
        Box::new(CharacterManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
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
    pub fn destroy_character_manager(&mut self) {}
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
        let character_data_ref = character_data.borrow();
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: character_data_ref._model_data_name.clone(),
            _position: character_create_info._position.clone(),
            _rotation: character_create_info._rotation.clone(),
            _scale: character_create_info._scale.clone(),
        };

        let render_object_data = self.get_scene_manager_mut().add_skeletal_render_object(
            character_name,
            &render_object_create_info,
        );
        let attack_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._attack_animation_mesh);
        let dead_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._dead_animation_mesh);
        let hit_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._hit_animation_mesh);
        let idle_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._idle_animation_mesh);
        let jump_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._jump_animation_mesh);
        let power_attack_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._power_attack_animation_mesh);
        let roll_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._roll_animation_mesh);
        let run_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._run_animation_mesh);
        let running_jump_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._running_jump_animation_mesh);
        let upper_animation_layer = game_resources.get_engine_resources().get_animation_layer_data(&character_data_ref._upper_animation_layer);
        let walk_animation = game_resources.get_engine_resources().get_mesh_data(&character_data_ref._walk_animation_mesh);
        let id = self.generate_id();
        let character = newRcRefCell(Character::create_character_instance(
            self,
            id,
            is_player,
            character_name,
            character_data,
            &render_object_data,
            attack_animation,
            dead_animation,
            hit_animation,
            idle_animation,
            jump_animation,
            power_attack_animation,
            roll_animation,
            run_animation,
            running_jump_animation,
            walk_animation,
            upper_animation_layer,
            &character_create_info._position,
            &character_create_info._rotation,
            &character_create_info._scale,
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
        let player = ptr_as_mut(self._player.as_ref().unwrap().as_ptr());
        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        for character in self._characters.values() {
            let character_mut = ptr_as_mut(character.as_ptr());

            // update animation key frames
            character_mut.update_move_keyframe_event();
            character_mut.update_action_keyframe_event();

            // update character
            character_mut.update_character(self.get_game_scene_manager(), delta_time as f32, player);

            // check attack
            if character_mut._attack_event != ActionAnimationState::None {
                if character_mut._is_player {
                    // player attack to npc
                    for target_character in self._characters.values() {
                        let target_character_mut = ptr_as_mut(target_character.as_ptr());
                        if false == target_character_mut._is_player &&
                            target_character_mut._is_alive &&
                            false == target_character_mut._character_property._invincibility &&
                            character_mut.check_attack_range(character_mut._attack_event, target_character_mut.get_bounding_box()) {
                            target_character_mut.set_damage(target_character_mut.get_position().clone(), character_mut.get_power(character_mut._attack_event));
                            if false == target_character_mut._is_alive {
                                dead_characters.push(target_character.clone());

                                // TestCode: Food
                                let food_create_info = FoodCreateInfo {
                                    _food_data_name: String::from("meat"),
                                    _position: target_character_mut.get_position().clone() + Vector3::new(0.0, 0.5, 0.0),
                                    ..Default::default()
                                };
                                ptr_as_mut(self.get_game_scene_manager()._food_manager.clone()).create_food("food", &food_create_info);
                            }
                        }
                    }
                } else {
                    // npc attack to player
                    if player._is_alive &&
                        false == player._character_property._invincibility &&
                        character_mut.check_attack_range(character_mut._attack_event, &player.get_bounding_box()) {
                        player.set_damage(player.get_position().clone(), character_mut.get_power(character_mut._attack_event));
                    }
                }
            }
        }

        // remove characters
        // for character in dead_characters.iter() {
        //     self.remove_character(character);
        // }
    }
}