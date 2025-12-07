use std::collections::HashMap;

use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};

use crate::application::application::Application;
use crate::game_module::actors::character::{Character, CharacterCreateInfo};
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::actors::weapons::{Weapon, WeaponCreateInfo};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{ITEM_SPIRIT_BALL, NPC_ATTACK_HIT_RANGE};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct CharacterID(u64);

pub type CharacterMap<'a> = HashMap<String, RcRefCell<Character<'a>>>;

pub struct CharacterManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _id_generator: CharacterID,
    pub _player: Option<RcRefCell<Character<'a>>>,
    pub _target_character: Option<RcRefCell<Character<'a>>>,
    pub _target_focus_time: f64,
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
            _id_generator: CharacterID(0),
            _player: None,
            _target_character: None,
            _target_focus_time: 0.0,
            _characters: HashMap::new(),
        })
    }

    pub fn initialize_character_manager(
        &mut self,
        engine_core: &EngineCore<'a>,
        application: &Application<'a>,
    ) {
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
    pub fn generate_id(&mut self) -> CharacterID {
        let id = self._id_generator.clone();
        self._id_generator = CharacterID(self._id_generator.0 + 1);
        id
    }
    pub fn get_characters(&self) -> &CharacterMap<'a> {
        &self._characters
    }
    pub fn get_character(&self, character_name: &str) -> Option<&RcRefCell<Character<'a>>> {
        self._characters.get(character_name)
    }
    pub fn add_character_weapon(
        &self,
        character: &mut Character<'a>,
        weapon_create_info: &WeaponCreateInfo,
    ) {
        // remove previous weapon
        if let Some(weapon) = character.get_weapon() {
            self.get_scene_manager_mut()
                .remove_skeletal_render_object(weapon._render_object.borrow()._object_id);
            character.remove_weapon();
        }

        // create a new weapon
        let game_resources = ptr_as_ref(self._game_resources);
        let weapon_data = game_resources.get_weapon_data(&weapon_create_info._weapon_data_name);
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: weapon_data
                .borrow()
                ._model_data
                .borrow()
                ._model_data_name
                .clone(),
            _position: weapon_create_info._position.clone(),
            _rotation: weapon_create_info._rotation.clone(),
            _scale: weapon_create_info._scale.clone(),
        };

        let render_object_data = self.get_scene_manager_mut().add_skeletal_render_object(
            weapon_create_info._weapon_data_name.as_str(),
            &render_object_create_info,
        );

        let mut weapon: Option<Box<Weapon<'a>>> = None;
        if let Some(weapon_socket) = character
            ._render_object
            .borrow()
            ._sockets
            .get(&weapon_create_info._weapon_socket_name)
        {
            weapon = Some(Box::new(Weapon::create_weapon(
                weapon_socket,
                weapon_create_info,
                weapon_data,
                &render_object_data,
            )));
        }

        if weapon.is_some() {
            character.add_weapon(weapon.unwrap());
        }
    }
    pub fn create_character(
        &mut self,
        character_name: &str,
        character_create_info: &CharacterCreateInfo,
        is_player: bool,
    ) -> RcRefCell<Character<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let mut spawn_point = character_create_info._position.clone();
        spawn_point.y = spawn_point.y.max(
            self.get_scene_manager()
                .get_height_map_data()
                .get_height_bilinear(&spawn_point, 0),
        );

        let character_data_name = character_create_info._character_data_name.as_str();
        let character_data = game_resources.get_character_data(character_data_name);
        let character_data_ref = character_data.borrow();
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: character_data_ref._model_data_name.clone(),
            _position: spawn_point.clone(),
            _rotation: character_create_info._rotation.clone(),
            _scale: character_create_info._scale.clone(),
        };

        let render_object_data = self
            .get_scene_manager_mut()
            .add_skeletal_render_object(character_name, &render_object_create_info);

        let id = self.generate_id();
        let character = newRcRefCell(Character::create_character_instance(
            self,
            character_name,
            id,
            is_player,
            character_data_name,
            character_data,
            &render_object_data,
            &spawn_point,
            &character_create_info._rotation,
            &character_create_info._scale,
        ));

        // add weapon
        if !character_data
            .borrow()
            ._weapon_create_info
            ._weapon_data_name
            .is_empty()
        {
            self.add_character_weapon(
                &mut *character.borrow_mut(),
                &character_data.borrow()._weapon_create_info,
            )
        }

        if is_player {
            self._player = Some(character.clone());
        }

        self._characters
            .insert(String::from(character_name), character.clone());

        character
    }

    pub fn remove_character(&mut self, character: &RcRefCell<Character>) {
        self._characters
            .remove(character.borrow().get_character_name().as_str());
        self.get_scene_manager_mut()
            .remove_skeletal_render_object(character.borrow()._render_object.borrow()._object_id);
    }
    pub fn clear_characters(&mut self, clear_player: bool) {
        let characters = self
            ._characters
            .values()
            .cloned()
            .collect::<Vec<RcRefCell<Character>>>();
        for character in characters.iter() {
            if clear_player || character.borrow().is_player() == false {
                self.remove_character(character);
            }
        }

        if clear_player {
            self._player = None;
        }
    }
    pub fn is_valid_player(&self) -> bool {
        self._player.is_some()
    }
    pub fn get_player(&self) -> &RcRefCell<Character<'a>> {
        self._player.as_ref().unwrap()
    }
    pub fn is_valid_target_character(&self) -> bool {
        self._target_character.is_some()
    }
    pub fn get_target_character(&self) -> &RcRefCell<Character<'a>> {
        self._target_character.as_ref().unwrap()
    }
    pub fn set_target_character(&mut self, target_character: Option<RcRefCell<Character<'a>>>) {
        self._target_character = target_character;
    }
    pub fn update_character_manager(&mut self, delta_time: f64) {
        let player = ptr_as_mut(self._player.as_ref().unwrap().as_ptr());
        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        let mut register_target_character: Option<RcRefCell<Character<'a>>> = None;
        for character in self._characters.values() {
            let character_mut = ptr_as_mut(character.as_ptr());

            // update character
            character_mut.update_character(self.get_scene_manager(), player, delta_time as f32);

            if character_mut.is_alive() == false {
                continue;
            }

            // check attack
            let check_direction = true;
            if character_mut._animation_state.is_attack_event() {
                if character_mut._is_player {
                    // player attack to npc
                    for target_character in self._characters.values() {
                        let target_character_mut = ptr_as_mut(target_character.as_ptr());
                        if false == target_character_mut._is_player
                            && target_character_mut.is_alive()
                            && false == target_character_mut._character_stats._invincibility
                            && character_mut.check_in_range(target_character_mut.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction
                        ) {
                            register_target_character = Some(target_character.clone());
                            let target_position = ptr_as_ref(target_character_mut.get_position());
                            target_character_mut.set_hit_damage(
                                character_mut.get_power(character_mut._animation_state.get_action_event()),
                                Some(character_mut.get_front()),
                            );

                            if false == target_character_mut.is_alive() {
                                dead_characters.push(target_character.clone());

                                // TestCode: Item
                                let item_create_info = ItemCreateInfo {
                                    _item_data_name: String::from(ITEM_SPIRIT_BALL),
                                    _position: target_position + Vector3::new(0.0, 0.5, 0.0),
                                    ..Default::default()
                                };
                                self.get_game_scene_manager().get_item_manager_mut().create_item(&item_create_info, true);
                            }
                        }
                    }
                } else {
                    // npc attack to player
                    if player.is_alive()
                        && false == player._character_stats._invincibility
                        && character_mut.check_in_range(
                            player.get_collision(),
                            NPC_ATTACK_HIT_RANGE,
                            check_direction,
                        )
                    {
                        player.set_hit_damage(
                            character_mut
                                .get_power(character_mut._animation_state.get_action_event()),
                            Some(character_mut.get_front()),
                        );
                    }
                }
            }
        }

        // remove characters
        // for character in dead_characters.iter() {
        //     self.remove_character(character);
        // }

        // target character for ui
        if register_target_character.is_some() {
            self.set_target_character(register_target_character);
            self._target_focus_time = 0.0;
        } else {
            const TARGET_FOCUS_TIME: f64 = 2.0;
            if self._target_focus_time < TARGET_FOCUS_TIME {
                self._target_focus_time += delta_time;
                if TARGET_FOCUS_TIME <= self._target_focus_time {
                    self.set_target_character(None);
                }
            }
        }
    }
}
