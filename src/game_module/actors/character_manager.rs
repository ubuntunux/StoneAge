use crate::application::application::Application;
use crate::game_module::actors::character::{ActorWrapper, Character};
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::actors::items::ItemManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{
    AUDIO_STOMACH_GROWLING, CHARACTER_INTERACTION_DISTANCE, CHARACTER_INTERACTION_TIME, GAME_VIEW_MODE, GameViewMode,
    ITEM_HAND, ITEM_SPIRIT_BALL, MATERIAL_EMOJI_GOOD, MATERIAL_EMOJI_HUNGRY, NPC_ATTACK_HIT_RANGE,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, GameSceneManager};
use crate::game_module::game_ui_manager::GameUIManager;
use crate::game_module::widgets::text_box_widget::TextBoxContent;
use crate::game_module::widgets::text_box_widget::TextBoxLayerType;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, SceneObjectType};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, extract_name_and_uuid, newRcRefCell, ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_void;
use uuid::Uuid;

pub type CharacterID = Uuid;
pub type CharacterMap<'a> = HashMap<CharacterID, RcRefCell<Character<'a>>>;
pub type CharacterNameMap<'a> = HashMap<String, RcRefCell<Character<'a>>>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_id: CharacterID,
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct CharacterManager<'a> {
    pub _game_client: *const GameClient<'a>,
    pub _game_scene_manager: *const GameSceneManager<'a>,
    pub _game_resources: *const GameResources<'a>,
    pub _audio_manager: *const AudioManager<'a>,
    pub _scene_manager: *const SceneManager<'a>,
    pub _game_ui_manager: *const GameUIManager<'a>,
    pub _player: Option<RcRefCell<Character<'a>>>,
    pub _target_character: Option<RcRefCell<Character<'a>>>,
    pub _target_focus_time: f64,
    pub _characters: CharacterMap<'a>,
    pub _character_name_map: CharacterNameMap<'a>,
}

impl<'a> CharacterManager<'a> {
    pub fn create_character_manager() -> Box<CharacterManager<'a>> {
        Box::new(CharacterManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _player: None,
            _target_character: None,
            _target_focus_time: 0.0,
            _characters: HashMap::new(),
            _character_name_map: HashMap::new(),
        })
    }

    pub fn initialize_character_manager(&mut self, engine_core: &EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_character_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_ui_manager = application.get_game_ui_manager();
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
    pub fn generate_id(&self) -> CharacterID {
        Uuid::new_v4()
    }
    pub fn get_characters(&self) -> &CharacterMap<'a> {
        &self._characters
    }
    pub fn get_character(&self, character_id: CharacterID) -> Option<&RcRefCell<Character<'a>>> {
        self._characters.get(&character_id)
    }
    pub fn get_character_by_name(&self, character_name: &str) -> Option<&RcRefCell<Character<'a>>> {
        self._character_name_map.get(character_name)
    }
    pub fn create_character(
        &mut self,
        character_name: &str,
        character_create_info: &CharacterCreateInfo,
        is_player: bool,
    ) -> RcRefCell<Character<'a>> {
        let (character_name, uuid) = extract_name_and_uuid(character_name);
        let game_resources = ptr_as_ref(self._game_resources);

        // check height map
        let mut spawn_point = character_create_info._position;
        spawn_point.y =
            spawn_point.y.max(self.get_scene_manager().get_height_map_data().get_height_bilinear(&spawn_point, 0));

        // check collision objects
        let collision_objects = self.get_scene_manager().collect_collision_objects(&spawn_point, &spawn_point);
        for collision_object in collision_objects.values() {
            let block_render_object = ptr_as_ref(collision_object.as_ptr());
            let block_bound_box = &block_render_object._collision._bounding_box;
            if block_render_object._collision.collide_point(&spawn_point) && block_bound_box._max.y < spawn_point.y {
                spawn_point.y = block_bound_box._max.y;
            }
        }

        let character_data_name = character_create_info._character_data_name.as_str();
        let character_data = game_resources.get_character_data(character_data_name);
        let character_data_ref = character_data.borrow();
        let render_object_create_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Default,
            _model_data_name: character_data_ref._model_data_name.clone(),
            _position: spawn_point,
            _rotation: character_create_info._rotation,
            _scale: character_create_info._scale,
        };

        let item_manager: *const ItemManager<'a> = ptr_as_ref(self._game_scene_manager)._item_manager.as_ref();
        let render_object_data = self
            .get_scene_manager_mut()
            .add_skeletal_render_object(character_name.as_str(), &render_object_create_info);

        let character_id = if character_create_info._character_id.is_nil() {
            self.generate_id()
        } else {
            assert_eq!(character_create_info._character_id, uuid.unwrap());
            character_create_info._character_id
        };

        let character = newRcRefCell(Character::create_character_instance(
            self,
            item_manager,
            character_name.as_str(),
            character_id,
            is_player,
            character_data_name,
            character_data,
            &render_object_data,
            &spawn_point,
            &character_create_info._rotation,
            &character_create_info._scale,
        ));

        if is_player {
            self.get_game_scene_manager().get_game_ui_manager_mut().add_item(ITEM_HAND, 1);
            self._player = Some(character.clone());
        }

        self._characters.insert(character_id, character.clone());
        if !self._character_name_map.contains_key(character_name.as_str()) {
            self._character_name_map.insert(character_name.clone(), character.clone());
        }
        character
    }

    pub fn remove_character(&mut self, character_ref: &RcRefCell<Character<'a>>) {
        let mut character = character_ref.borrow_mut();
        character.destroy_character();
        self._characters.remove(&character._character_id);
        if let Some(target) = self._character_name_map.get(character._character_name.as_str())
            && target.as_ptr() == character_ref.as_ptr()
        {
            self._character_name_map.remove(character._character_name.as_str());
        }

        self.get_scene_manager_mut().remove_skeletal_render_object(character._render_object.borrow()._object_id);
    }
    pub fn clear_characters(&mut self, clear_player: bool) {
        let characters = self._characters.values().cloned().collect::<Vec<RcRefCell<Character>>>();
        for character in characters.iter() {
            if clear_player || !character.borrow().is_player() {
                self.remove_character(character);
            }
        }

        if clear_player {
            self._player = None;
        }
    }

    pub fn create_characters(&mut self, character_create_infos: &CharacterCreateInfoMap) {
        for (character_name, character_create_info) in character_create_infos.iter() {
            self.create_character(character_name.as_str(), character_create_info, false);
        }
    }

    pub fn get_characters_save_data(&self) -> CharacterCreateInfoMap {
        self._characters
            .values()
            .filter(|character| !character.borrow().is_player())
            .map(|character| character.borrow().get_character_save_data())
            .collect()
    }
    pub fn post_process_after_characters_loading(&mut self) {
        for character in self._characters.values() {
            character.borrow_mut().post_process_after_character_loading();
        }
    }
    pub fn is_valid_player(&self) -> bool {
        self._player.is_some()
    }
    pub fn get_maybe_player(&self) -> &Option<RcRefCell<Character<'a>>> {
        &self._player
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

    pub fn update_character_text_box(
        &self,
        game_ui_manager: &mut GameUIManager<'a>,
        refcell_character: &RcRefCell<Character<'a>>,
    ) {
        let mut character = refcell_character.borrow_mut();
        if character._character_stats.get_is_stat_displayed() {
            let mut contents = vec![];
            if character.get_stats().is_hungry() {
                contents.push(TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY)));
                contents.push(TextBoxContent::Audio(String::from(AUDIO_STOMACH_GROWLING)));
            } else {
                contents.push(TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_GOOD)));
            }

            game_ui_manager.add_text_box_item(
                TextBoxLayerType::InteractionLayer,
                ActorWrapper::Character(refcell_character.clone()),
                &contents,
                Some(CHARACTER_INTERACTION_TIME),
            );

            character._character_stats.set_is_stat_displayed(false);
        }
    }

    pub fn update_interaction_ui(
        &self,
        player: &mut Character<'a>,
        character: &RcRefCell<Character<'a>>,
        to_player_distance: f32,
    ) {
        let key = character.as_ptr() as *const c_void;
        let was_interaction_object = player._controller.is_interaction_object(key);
        let is_in_player_range = to_player_distance <= CHARACTER_INTERACTION_DISTANCE;
        if !was_interaction_object && is_in_player_range {
            player._controller.add_interaction_object(InteractionObject::Npc(character.clone()));
        } else if was_interaction_object && !is_in_player_range {
            player._controller.remove_interaction_object(InteractionObject::Npc(character.clone()));
        }
    }

    pub fn update_character_manager(&mut self, delta_time: f64) {
        if self._player.is_none() {
            return;
        }

        let game_ui_manager = ptr_as_mut(self._game_ui_manager);
        let player = ptr_as_mut(self._player.as_ref().unwrap().as_ptr());
        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        let mut register_target_character: Option<RcRefCell<Character<'a>>> = None;
        for character in self._characters.values() {
            // update character
            let character_mut = ptr_as_mut(character.as_ptr());
            character_mut.update_character(self.get_scene_manager(), player, delta_time as f32);

            if !character_mut.is_alive() {
                continue;
            }

            // get distance to player
            let to_player = player.get_position() - character_mut.get_position();
            let (_to_player_dir, mut to_player_distance) = if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                math::make_normalize_xy_with_norm(&to_player)
            } else {
                math::make_normalize_with_norm(&to_player)
            };
            to_player_distance = 0f32.max(
                to_player_distance
                    - (player.get_collision()._bounding_box._mag_xz
                        + character_mut.get_collision()._bounding_box._mag_xz)
                        * 0.5,
            );

            // update interaction ui
            if !character_mut.is_player() {
                self.update_character_text_box(game_ui_manager, character);
                self.update_interaction_ui(player, character, to_player_distance);
            }

            // check attack
            let check_direction = true;
            if character_mut._animation_state.is_attack_event() {
                if character_mut._is_player {
                    // player attack to npc
                    for target_character in self._characters.values() {
                        let target_character_mut = ptr_as_mut(target_character.as_ptr());
                        if !target_character_mut._is_player
                            && target_character_mut.is_alive()
                            && !target_character_mut._character_stats._invincibility
                            && character_mut.check_in_range(
                                target_character_mut.get_collision(),
                                NPC_ATTACK_HIT_RANGE,
                                check_direction,
                            )
                        {
                            register_target_character = Some(target_character.clone());

                            let target_position = ptr_as_ref(target_character_mut.get_position());

                            // hit..
                            target_character_mut.set_hit_damage(
                                character_mut.get_power(character_mut._animation_state.get_action_event()),
                                Some(character_mut.get_face_direction()),
                            );

                            // character dead..
                            if !target_character_mut.is_alive() {
                                dead_characters.push(target_character.clone());

                                // TestCode: Item
                                let item_create_info = ItemCreateInfo {
                                    _item_data_name: String::from(ITEM_SPIRIT_BALL),
                                    _position: *target_position,
                                    ..Default::default()
                                };
                                self.get_game_scene_manager().get_item_manager_mut().create_item(
                                    item_create_info._item_data_name.as_str(),
                                    &item_create_info,
                                    None,
                                );
                            }
                        }
                    }
                } else {
                    // npc attack to player
                    if player.is_alive()
                        && !player._character_stats._invincibility
                        && character_mut.check_in_range(player.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction)
                    {
                        player.set_hit_damage(
                            character_mut.get_power(character_mut._animation_state.get_action_event()),
                            Some(character_mut.get_face_direction()),
                        );
                    }
                }
            }
        }

        // remove characters
        for character in dead_characters.iter() {
            character.borrow_mut()._character_stats.set_is_stat_displayed(false);
            player._controller.remove_interaction_object(InteractionObject::Npc(character.clone()));
            game_ui_manager.remove_text_box_item(character.as_ptr() as *const c_void);
            //self.remove_character(character);
        }

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
