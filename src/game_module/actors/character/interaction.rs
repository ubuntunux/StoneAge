use crate::game_module::actors::character::character::Character;
use crate::game_module::actors::character::data::{ActionAnimationState, MoveAnimationState, RequestType};
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::behavior::behavior_base::BehaviorState;
use crate::game_module::game_client::GamePhase;
use crate::game_module::game_constants::{
    AUDIO_PICKUP_ITEM, INTIMACY_FEEDING_MULTIPLIER, INTIMACY_INTERACTION_ADD, ITEM_ENERGY_BALL,
    TAME_ENERGY_BALL_PICKUP_DELAY, TAME_ENERGY_BALL_VELOCITY_Y,
};
use crate::game_module::game_service_locator::{
    get_character_manager_mut, get_game_client, get_game_client_mut, get_game_scene_manager, get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::get_audio_manager_mut;
use rust_engine_3d::utilities::system::RcRefCell;

const PICKUP_ANIMATION_SPEED: f32 = 2.0;

impl<'a> Character<'a> {
    pub fn set_action_interaction(&mut self) {
        if !get_game_client().is_available_interaction() {
            return;
        }
        if self._controller.is_on_ground() && self.is_available_move() && self.is_idle_action() {
            let item_manager = get_game_scene_manager().get_item_manager_mut();
            let target_interaction = self
                ._controller
                ._interaction_objects
                .values()
                .find(|obj| matches!(obj, InteractionObject::Taming(_)))
                .cloned()
                .unwrap_or_else(|| self._controller._nearest_interaction_object.clone());

            match target_interaction {
                InteractionObject::PropBed(prop) => {
                    if get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay) {
                        self.set_move_idle();
                        get_game_client_mut().set_next_game_phase(GamePhase::Interaction);
                        get_game_ui_manager_mut().open_bed_interaction_menu(prop);
                    }
                }
                InteractionObject::PropPickup(_) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, PICKUP_ANIMATION_SPEED);
                }
                InteractionObject::PropMonolith(_) => {
                    get_game_client_mut().set_next_game_phase(GamePhase::OpenToolbox);
                    self.set_move_idle();
                }
                InteractionObject::PropTable(prop) => {
                    self.look_at(prop.borrow().get_position());
                    self.set_move_idle();
                    if get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay) {
                        if item_manager.has_item_on_table(&prop) {
                            if let Some(item_data_name) = item_manager.consume_item_on_table(&prop) {
                                self.set_action_eating();
                                self.apply_food_item_stats(item_data_name.as_str());
                                get_game_ui_manager_mut().get_player_records_mut().add_food_eaten_count(1);
                            }
                        } else {
                            get_game_client_mut().set_next_game_phase(GamePhase::Interaction);
                            get_game_ui_manager_mut().open_table_interaction_menu(prop);
                        }
                    }
                }
                InteractionObject::Npc(character) => {
                    if get_game_client().is_available_interaction() {
                        self.set_move_idle();
                        get_game_client_mut().set_next_game_phase(GamePhase::Interaction);
                        get_game_ui_manager_mut().open_npc_interaction_menu(character);
                    }
                }
                InteractionObject::Taming(character) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, PICKUP_ANIMATION_SPEED);
                    let target_position = *character.borrow().get_position();

                    let item_create_info = ItemCreateInfo {
                        _item_data_name: String::from(ITEM_ENERGY_BALL),
                        _position: target_position,
                        _velocity: Vector3::new(0.0, TAME_ENERGY_BALL_VELOCITY_Y, 0.0),
                        _pickup_delay: TAME_ENERGY_BALL_PICKUP_DELAY,
                        ..Default::default()
                    };
                    item_manager.create_item(item_create_info._item_data_name.as_str(), &item_create_info, None);

                    character.borrow_mut().tame();
                    get_game_ui_manager_mut().notify_tamed();

                    self._controller.remove_character_interaction_objects(&character);
                }
                InteractionObject::Farming(character) => {
                    self.set_next_action_animation(ActionAnimationState::Pickup, PICKUP_ANIMATION_SPEED);
                    let is_destroyed = {
                        let mut corpse = character.borrow_mut();
                        corpse.set_hit_damage(1, Some(self.get_position()), true);
                        corpse.get_corpse_hit_count() <= 0
                    };
                    if is_destroyed {
                        get_character_manager_mut().farm_character(&character);
                    }
                }
                _ => {}
            }
        }
    }

    fn prepare_npc_for_interaction(&mut self, npc: &mut Character<'a>, next_behavior: BehaviorState) {
        self.look_at(npc.get_position());
        npc.look_at(self.get_position());
        npc.set_is_interacting(true);
        if !npc.is_move_state(MoveAnimationState::SitDownLoop) {
            npc.set_move_idle();
        }
        npc.set_next_behavior(next_behavior, true);
    }

    pub fn execute_npc_talk(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        self.look_at(target_npc.borrow().get_position());
        let mut npc = target_npc.borrow_mut();
        npc.set_is_interacting(true);
        if !npc.is_move_state(MoveAnimationState::SitDownLoop) {
            npc.set_move_idle();
        }
        npc.set_next_behavior(BehaviorState::Interaction, true);
        npc.set_is_stat_displayed(true);
        npc.add_intimacy(INTIMACY_INTERACTION_ADD);
    }

    pub fn execute_npc_request(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        let mut npc = target_npc.borrow_mut();
        if !npc.is_action(ActionAnimationState::Eating) && npc._behavior.get_behavior_state() != BehaviorState::Eating {
            let next_phase = match npc.get_request_type() {
                RequestType::Cooking => Some(GamePhase::OpenCooking),
                RequestType::Craft => Some(GamePhase::OpenCraft),
                _ => None,
            };

            if let Some(phase) = next_phase {
                get_game_client_mut().set_next_game_phase(phase);
                self.prepare_npc_for_interaction(&mut npc, BehaviorState::Idle);
            }
        }
    }

    pub fn execute_npc_dance(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        self.set_action_dance();
        let mut npc = target_npc.borrow_mut();
        self.prepare_npc_for_interaction(&mut npc, BehaviorState::Dance);
        npc.add_intimacy(INTIMACY_INTERACTION_ADD);
    }

    pub fn execute_npc_give_item(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        let item_manager = get_game_scene_manager().get_item_manager_mut();
        if target_npc.borrow().get_attached_item().is_none() {
            let eatable_item_name = if let Some(attached_item) = self.get_attached_item()
                && attached_item.borrow().get_item_data_type().is_eatable()
            {
                Some(attached_item.borrow()._item_data_name.clone())
            } else {
                None
            };

            if let Some(item_data_name) = eatable_item_name {
                item_manager.remove_inventory_item(item_data_name.as_str(), 1);
                let mut npc = target_npc.borrow_mut();
                npc.apply_food_item_stats(item_data_name.as_str());
                npc.look_at(self.get_position());
                npc.set_next_behavior(BehaviorState::Eating, true);
                npc.add_intimacy(INTIMACY_INTERACTION_ADD * INTIMACY_FEEDING_MULTIPLIER);
            }
        }
    }

    pub fn execute_npc_table_storage(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        self.look_at(target_npc.borrow().get_position());
        get_game_client_mut().set_next_game_phase(GamePhase::OpenTableStorage);
        self.set_move_idle();
    }

    pub fn execute_npc_give_all_materials(&mut self, target_npc: &RcRefCell<Character<'a>>) {
        let mut npc = target_npc.borrow_mut();
        self.prepare_npc_for_interaction(&mut npc, BehaviorState::Interaction);

        let transferred_count = get_game_ui_manager_mut().transfer_all_materials_to_table_storage();
        if transferred_count > 0 {
            npc.add_intimacy(INTIMACY_INTERACTION_ADD * INTIMACY_FEEDING_MULTIPLIER);
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
    }

    pub fn set_action_request(&mut self) {
        if !get_game_client().is_available_interaction() {
            return;
        }
        if self._controller.is_on_ground() && self.is_available_move() && self.is_idle_action() {
            let target_interaction = self._controller._nearest_interaction_object.clone();
            if let InteractionObject::Npc(character) = target_interaction {
                self.execute_npc_request(&character);
            }
        }
    }

    pub fn callback_changed_interaction_object(&mut self) {
        if !get_game_client().is_available_interaction() {
            return;
        }
        if let InteractionObject::PropGate(_) = self._controller._nearest_interaction_object.clone() {
            get_game_client_mut().set_next_game_phase(GamePhase::WorldMapOpen);
            self.set_move_idle();
        }
    }
}
