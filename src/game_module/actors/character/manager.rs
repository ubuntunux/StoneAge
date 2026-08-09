use crate::game_module::actors::character::character::Character;
use crate::game_module::actors::character::controller::CharacterControllerSaveData;

use crate::game_module::actors::character::stats::*;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::{ItemCreateInfo, ItemID};
use crate::game_module::behavior::behavior_base::BehaviorSaveData;
use crate::game_module::game_constants::{
    AUDIO_STOMACH_GROWLING, CHARACTER_INTERACTION_DISTANCE, CHARACTER_INTERACTION_TIME, CORPSE_AUTO_REMOVE_TIME,
    FARM_MEAT_COUNT, GAME_VIEW_MODE, GameViewMode, ITEM_HAND, ITEM_MEAT, ITEM_SPIRIT_BALL, MATERIAL_EMOJI_GOOD,
    MATERIAL_EMOJI_HUNGRY, NPC_ATTACK_HIT_RANGE, NPC_TRACKING_RANGE,
};
use crate::game_module::game_scene_manager::{CharacterCreateInfoMap, CharacterSaveDataMap};
use crate::game_module::widgets::text_box_widget::TextBoxContent;
use crate::game_module::widgets::text_box_widget::TextBoxLayerType;
use nalgebra::Vector3;

use crate::game_module::game_service_locator::{get_game_resources, get_game_scene_manager, get_game_ui_manager_mut};
use rust_engine_3d::core::engine_service_locator::{get_scene_manager, get_scene_manager_mut};
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectSaveData, SceneObjectType};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, extract_name_and_uuid, newRcRefCell, ptr_as_mut, ptr_as_ref};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_void;
use uuid::Uuid;

pub type CharacterID = Uuid;
pub type CharacterMap<'a> = HashMap<CharacterID, RcRefCell<Character<'a>>>;
pub type CharacterNameMap<'a> = HashMap<String, RcRefCell<Character<'a>>>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_id: CharacterID,
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

impl Default for CharacterCreateInfo {
    fn default() -> Self {
        CharacterCreateInfo {
            _character_id: Default::default(),
            _character_data_name: "".to_string(),
            _position: Default::default(),
            _rotation: Default::default(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CharacterSaveData {
    pub _character_create_info: CharacterCreateInfo,
    pub _character_controller_save_data: CharacterControllerSaveData,
    pub _render_object_save_data: RenderObjectSaveData,
    pub _character_stats: CharacterStatsSaveData,
    pub _behavior: BehaviorSaveData,
    pub _animation_state: CharacterAnimationState,
    pub _attached_item: Option<ItemID>,
}

pub struct CharacterManager<'a> {
    pub _player: Option<RcRefCell<Character<'a>>>,
    pub _target_character: Option<RcRefCell<Character<'a>>>,
    pub _target_focus_time: f64,
    pub _characters: CharacterMap<'a>,
    pub _character_name_map: CharacterNameMap<'a>,
}

impl<'a> CharacterManager<'a> {
    pub fn create_character_manager() -> Box<CharacterManager<'a>> {
        Box::new(CharacterManager {
            _player: None,
            _target_character: None,
            _target_focus_time: 0.0,
            _characters: HashMap::new(),
            _character_name_map: HashMap::new(),
        })
    }

    pub fn initialize_character_manager(&mut self) {
        log::info!("initialize_character_manager");
    }
    pub fn destroy_character_manager(&mut self) {}

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
        let game_resources = get_game_resources();

        // check height map
        let mut spawn_point = character_create_info._position;
        spawn_point.y =
            spawn_point.y.max(get_scene_manager().get_height_map_data().get_height_bilinear(&spawn_point, 0));

        // check collision objects
        let collision_objects = get_scene_manager().collect_collision_objects(&spawn_point, &spawn_point);
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

        let render_object_data =
            get_scene_manager_mut().add_skeletal_render_object(character_name.as_str(), &render_object_create_info);
        let character_id = if character_create_info._character_id.is_nil() {
            self.generate_id()
        } else {
            assert_eq!(character_create_info._character_id, uuid.unwrap());
            character_create_info._character_id
        };

        let character = newRcRefCell(Character::create_character_instance(
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
            let game_ui_manager = get_game_ui_manager_mut();
            if game_ui_manager.get_item_count(ITEM_HAND) == 0 {
                game_ui_manager.add_item(ITEM_HAND, 1, false);
            }
            self._player = Some(character.clone());
        }

        self._characters.insert(character_id, character.clone());
        if !is_player && !self._character_name_map.contains_key(character_name.as_str()) {
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

        get_scene_manager_mut().remove_skeletal_render_object(character._render_object.borrow()._object_id);
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

    pub fn load_character_save_data(
        &mut self,
        character_name: &str,
        character_save_data: &CharacterSaveData,
        is_player: bool,
    ) -> RcRefCell<Character<'a>> {
        let character = self.create_character(character_name, &character_save_data._character_create_info, is_player);
        character.borrow_mut().load_character_save_data(character_save_data);
        character
    }

    pub fn load_characters_save_data(&mut self, character_save_data_map: &CharacterSaveDataMap) {
        for (character_name, character_save_data) in character_save_data_map.iter() {
            self.load_character_save_data(character_name, character_save_data, false);
        }
    }

    pub fn get_characters_save_data(&self) -> CharacterSaveDataMap {
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
    pub fn change_character_data(&mut self, character: &RcRefCell<Character<'a>>, character_data_name: &str) {
        let mut character_save_data = character.borrow().get_character_save_data();
        character_save_data.1._character_create_info._character_data_name = character_data_name.to_string();
        self.remove_character(character);
        self.load_character_save_data(
            character_save_data.0.as_str(),
            &character_save_data.1,
            character.borrow()._is_player,
        );
    }
    pub fn change_character_model(&self, character: &RcRefCell<Character<'a>>, model_data_name: &str) {
        let mut character = character.borrow_mut();
        get_scene_manager_mut().remove_skeletal_render_object(character._render_object.borrow()._object_id);

        let render_object_create_info = RenderObjectCreateInfo {
            _scene_object_type: SceneObjectType::Default,
            _model_data_name: model_data_name.to_string(),
            _position: *character.get_position(),
            _rotation: *character.get_rotation(),
            _scale: *character.get_scale(),
        };

        let render_object_data = get_scene_manager_mut()
            .add_skeletal_render_object(character.get_character_name(), &render_object_create_info);

        render_object_data.borrow_mut().copy_render_object_date(&character._render_object.borrow());
        character.change_character_model(&render_object_data);
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
    pub fn is_player_alive(&self) -> bool {
        if let Some(player) = self._player.as_ref() {
            return player.borrow().is_alive();
        }
        false
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

    pub fn update_character_text_box(&self, refcell_character: &RcRefCell<Character<'a>>) {
        let mut character = refcell_character.borrow_mut();
        if character.is_alive() && character._character_stats.get_is_stat_displayed() {
            let mut contents = vec![];
            if character.get_stats().is_hungry() {
                contents.push(TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_HUNGRY)));
                contents.push(TextBoxContent::Audio(String::from(AUDIO_STOMACH_GROWLING)));
            } else {
                contents.push(TextBoxContent::MaterialInstance(String::from(MATERIAL_EMOJI_GOOD)));
            }

            get_game_ui_manager_mut().add_text_box_item(
                TextBoxLayerType::InteractionLayer,
                ActorWrapper::Character(refcell_character.clone()),
                &contents,
                Some(CHARACTER_INTERACTION_TIME),
            );

            character._character_stats.set_is_stat_displayed(false);
        }
    }

    pub fn farm_character(&mut self, character: &RcRefCell<Character<'a>>) {
        let pos = *character.borrow().get_position();
        let item_manager = get_game_scene_manager().get_item_manager_mut();

        let item_create_info = ItemCreateInfo {
            _item_data_name: String::from(ITEM_SPIRIT_BALL),
            _position: pos,
            _velocity: Vector3::new(0.0, 4.0, 0.0),
            _pickup_delay: 0.3,
            ..Default::default()
        };
        item_manager.create_item(item_create_info._item_data_name.as_str(), &item_create_info, None);

        for i in 0..FARM_MEAT_COUNT {
            let angle = (i as f32) * (std::f32::consts::TAU / 3.0) + rand::random::<f32>() * 0.5;
            let speed = 1.5 + rand::random::<f32>() * 2.0;
            let velocity = Vector3::new(
                angle.cos() * speed,
                3.0 + rand::random::<f32>() * 2.0,
                angle.sin() * speed,
            );
            let meat_info = ItemCreateInfo {
                _item_data_name: String::from(ITEM_MEAT),
                _position: pos,
                _velocity: velocity,
                _pickup_delay: 0.3,
                ..Default::default()
            };
            item_manager.create_item(ITEM_MEAT, &meat_info, None);
        }

        // Remove corpse & interaction UI
        if let Some(player) = self._player.as_ref() {
            player.borrow_mut()._controller.remove_character_interaction_objects(character);
        }
        self.remove_character(character);
    }

    pub fn update_interaction_ui(
        &self,
        player: &mut Character<'a>,
        character: &RcRefCell<Character<'a>>,
        to_player_distance: f32,
    ) {
        let is_in_player_range = to_player_distance <= CHARACTER_INTERACTION_DISTANCE;
        let character_ref = character.borrow();

        if character_ref.is_corpse() {
            // Corpse: show Taming & Farming widgets
            let taming_obj = InteractionObject::Taming(character.clone());
            let farming_obj = InteractionObject::Farming(character.clone());
            let was_taming = player._controller.is_interaction_object(taming_obj.get_key());

            let npc_obj = InteractionObject::Npc(character.clone());
            if player._controller.is_interaction_object(npc_obj.get_key()) {
                player._controller.remove_interaction_object(npc_obj);
            }

            if !was_taming && is_in_player_range {
                player._controller.add_interaction_object(taming_obj);
                player._controller.add_interaction_object(farming_obj);
            } else if was_taming && !is_in_player_range {
                player._controller.remove_interaction_object(taming_obj);
                player._controller.remove_interaction_object(farming_obj);
            }
        } else {
            let taming_obj = InteractionObject::Taming(character.clone());
            let farming_obj = InteractionObject::Farming(character.clone());
            if player._controller.is_interaction_object(taming_obj.get_key()) {
                player._controller.remove_interaction_object(taming_obj);
                player._controller.remove_interaction_object(farming_obj);
            }

            if character_ref.is_alive() && (character_ref.is_civilian() || character_ref.is_tamed()) {
                let npc_obj = InteractionObject::Npc(character.clone());
                let was_npc = player._controller.is_interaction_object(npc_obj.get_key());
                if !was_npc && is_in_player_range {
                    player._controller.add_interaction_object(npc_obj);
                } else if was_npc && !is_in_player_range {
                    player._controller.remove_interaction_object(npc_obj);
                }
            } else {
                let npc_obj = InteractionObject::Npc(character.clone());
                if player._controller.is_interaction_object(npc_obj.get_key()) {
                    player._controller.remove_interaction_object(npc_obj);
                }
            }
        }
    }

    pub fn update_character_manager(&mut self, delta_time: f64) {
        if self._player.is_none() {
            return;
        }

        let player = ptr_as_mut(self._player.as_ref().unwrap().as_ptr());
        let mut dead_characters: Vec<RcRefCell<Character>> = Vec::new();
        let mut farmed_characters: Vec<RcRefCell<Character<'a>>> = Vec::new();
        let mut expired_dead_characters: Vec<RcRefCell<Character<'a>>> = Vec::new();
        let mut register_target_character: Option<RcRefCell<Character<'a>>> = None;
        for character in self._characters.values() {
            // update character
            let character_mut = ptr_as_mut(character.as_ptr());
            let ai_target: Option<&Character<'a>> = if character_mut.is_player() {
                None
            } else if character_mut.is_tamed() {
                // Tamed monster: targets nearest alive untamed monster within tracking range; fallback to player if intimate
                let mut min_dist = f32::MAX;
                let mut target_ref: Option<&Character<'a>> = None;
                for other in self._characters.values() {
                    let other_mut = ptr_as_mut(other.as_ptr());
                    if !other_mut.is_player() && !other_mut.is_tamed() && other_mut.is_alive() {
                        let dist = (other_mut.get_position() - character_mut.get_position()).norm();
                        if dist <= NPC_TRACKING_RANGE && dist < min_dist {
                            min_dist = dist;
                            target_ref = Some(other_mut);
                        }
                    }
                }
                if target_ref.is_some() {
                    target_ref
                } else if character_mut.is_following_intimacy() {
                    Some(player)
                } else {
                    None
                }
            } else if character_mut.is_following_intimacy() {
                Some(player)
            } else {
                // Wild monster: targets player OR nearest alive tamed monster within tracking range
                let mut min_dist = f32::MAX;
                let mut target_ref: Option<&Character<'a>> = None;
                if player.is_alive() {
                    let dist = (player.get_position() - character_mut.get_position()).norm();
                    if dist <= NPC_TRACKING_RANGE {
                        min_dist = dist;
                        target_ref = Some(player);
                    }
                }
                for other in self._characters.values() {
                    let other_mut = ptr_as_mut(other.as_ptr());
                    if other_mut.is_tamed() && other_mut.is_alive() {
                        let dist = (other_mut.get_position() - character_mut.get_position()).norm();
                        if dist <= NPC_TRACKING_RANGE && dist < min_dist {
                            min_dist = dist;
                            target_ref = Some(other_mut);
                        }
                    }
                }
                target_ref
            };

            character_mut.update_character(get_scene_manager(), ai_target, delta_time as f32);

            if !character_mut.is_alive() {
                character_mut._dead_time += delta_time as f32;
                if !character_mut.is_civilian() && character_mut._dead_time >= CORPSE_AUTO_REMOVE_TIME {
                    expired_dead_characters.push(character.clone());
                }
            } else {
                character_mut._dead_time = 0.0;
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
                self.update_character_text_box(character);
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
                            && !target_character_mut.is_tamed()
                            && !target_character_mut._character_stats._invincibility
                            && character_mut.check_in_range(
                                target_character_mut.get_collision(),
                                NPC_ATTACK_HIT_RANGE,
                                check_direction,
                            )
                        {
                            register_target_character = Some(target_character.clone());

                            if target_character_mut.is_alive()
                                && !target_character_mut.is_tamed()
                                && !target_character_mut.is_civilian()
                            {
                                // hit living monster..
                                target_character_mut.set_hit_damage(
                                    character_mut.get_power(character_mut._animation_state.get_action_event()),
                                    Some(character_mut.get_face_direction()),
                                );

                                if !target_character_mut.is_alive() {
                                    dead_characters.push(target_character.clone());
                                }
                            } else if target_character_mut.is_corpse() {
                                // hit dead corpse (monsters only) -> set_hit_damage plays hit sound & effect and decrements _corpse_hit_count
                                target_character_mut.set_hit_damage(
                                    character_mut.get_power(character_mut._animation_state.get_action_event()),
                                    Some(character_mut.get_face_direction()),
                                );

                                if target_character_mut.get_corpse_hit_count() <= 0 {
                                    farmed_characters.push(target_character.clone());
                                }
                            }
                        }
                    }
                } else if character_mut.is_tamed() {
                    // tamed monster attack to wild untamed monster
                    for target_character in self._characters.values() {
                        let target_character_mut = ptr_as_mut(target_character.as_ptr());
                        if !target_character_mut._is_player
                            && !target_character_mut.is_tamed()
                            && target_character_mut.is_alive()
                            && !target_character_mut._character_stats._invincibility
                            && character_mut.check_in_range(
                                target_character_mut.get_collision(),
                                NPC_ATTACK_HIT_RANGE,
                                check_direction,
                            )
                        {
                            target_character_mut.set_hit_damage(
                                character_mut.get_power(character_mut._animation_state.get_action_event()),
                                Some(character_mut.get_face_direction()),
                            );

                            if !target_character_mut.is_alive() {
                                dead_characters.push(target_character.clone());
                            }
                        }
                    }
                } else {
                    // wild monster attack to player OR tamed monster
                    if player.is_alive()
                        && !player._character_stats._invincibility
                        && character_mut.check_in_range(player.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction)
                    {
                        player.set_hit_damage(
                            character_mut.get_power(character_mut._animation_state.get_action_event()),
                            Some(character_mut.get_face_direction()),
                        );
                    }

                    for target_character in self._characters.values() {
                        let target_character_mut = ptr_as_mut(target_character.as_ptr());
                        if target_character_mut.is_tamed()
                            && target_character_mut.is_alive()
                            && !target_character_mut._character_stats._invincibility
                            && character_mut.check_in_range(
                                target_character_mut.get_collision(),
                                NPC_ATTACK_HIT_RANGE,
                                check_direction,
                            )
                        {
                            target_character_mut.set_hit_damage(
                                character_mut.get_power(character_mut._animation_state.get_action_event()),
                                Some(character_mut.get_face_direction()),
                            );

                            if !target_character_mut.is_alive() {
                                dead_characters.push(target_character.clone());
                            }
                        }
                    }
                }
            }
        }

        // process dead characters
        let game_ui_manager = get_game_ui_manager_mut();
        for character in dead_characters.iter() {
            character.borrow_mut()._character_stats.set_is_stat_displayed(false);
            player._controller.remove_interaction_object(InteractionObject::Npc(character.clone()));
            game_ui_manager.remove_text_box_item(character.as_ptr() as *const c_void);
        }

        // process farmed corpses
        for character in farmed_characters.iter() {
            self.farm_character(character);
        }

        // process expired dead characters (auto remove after 5s for non-civilians)
        for character in expired_dead_characters.iter() {
            if let Some(player) = self._player.as_ref() {
                player.borrow_mut()._controller.remove_character_interaction_objects(character);
            }
            self.remove_character(character);
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
