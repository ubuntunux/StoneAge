use crate::game_module::actors::character::{ActionEvent, Character};
use crate::game_module::actors::common::resolve_actor_id;
use crate::game_module::actors::interaction_object::InteractionObject;
use crate::game_module::actors::props::api::{
    Prop, PropCreateInfo, PropDataType, PropID, PropManager, PropMap, PropSaveData,
};
use crate::game_module::game_constants::{
    CHARACTER_INTERACTION_DISTANCE, GAME_VIEW_MODE, GameViewMode, NPC_ATTACK_HIT_RANGE,
};
use crate::game_module::game_scene_manager::{PropCreateInfoMap, PropSaveDataMap};
use crate::game_module::game_service_locator::{
    get_character_manager, get_game_client, get_game_resources, get_game_scene_manager, get_item_manager_mut,
};
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::Vector3;
use rust_engine_3d::core::engine_service_locator::{get_scene_manager, get_scene_manager_mut};
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, extract_name_and_uuid, newRcRefCell};
use std::collections::HashMap;
use std::ffi::c_void;
use uuid::Uuid;

impl<'a> PropManager<'a> {
    pub fn create_prop_manager() -> Box<PropManager<'a>> {
        Box::new(PropManager {
            _props: HashMap::new(),
            _prop_name_map: HashMap::new(),
            _placement_preview: crate::game_module::actors::props::placement::PlacementPreview::new(),
            _marker: std::marker::PhantomData,
        })
    }

    pub fn get_placement_preview(&self) -> &crate::game_module::actors::props::placement::PlacementPreview<'a> {
        &self._placement_preview
    }

    pub fn get_placement_preview_mut(
        &mut self,
    ) -> &mut crate::game_module::actors::props::placement::PlacementPreview<'a> {
        &mut self._placement_preview
    }

    pub fn initialize_prop_manager(&mut self) {
        log::info!("initialize_prop_manager");
    }

    pub fn destroy_prop_manager(&mut self) {}

    pub fn generate_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    pub fn get_prop(&self, prop_id: PropID) -> Option<&RcRefCell<Prop<'a>>> {
        self._props.get(&prop_id)
    }

    pub fn get_prop_by_name(&self, prop_name: &str) -> Option<&RcRefCell<Prop<'a>>> {
        self._prop_name_map.get(prop_name)
    }

    pub fn get_props(&self) -> &PropMap<'a> {
        &self._props
    }

    pub fn create_prop(&mut self, prop_name: &str, prop_create_info: &PropCreateInfo) -> RcRefCell<Prop<'a>> {
        let (prop_name, uuid) = extract_name_and_uuid(prop_name);
        let game_resources = get_game_resources();
        let prop_data = game_resources.get_prop_data(prop_create_info._prop_data_name.as_str());

        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: prop_data.borrow()._model_data_name.clone(),
            _position: prop_create_info._position,
            _rotation: prop_create_info._rotation,
            _scale: prop_create_info._scale,
            ..Default::default()
        };
        let render_object_data =
            get_scene_manager_mut().add_dynamic_render_object(prop_name.as_str(), &render_object_create_info, None);

        let mut item_render_objects: Vec<RcRefCell<RenderObjectData<'a>>> = Vec::new();
        for (_, socket) in render_object_data.borrow()._sockets.iter() {
            let item_data = game_resources.get_item_data(prop_data.borrow()._item_data_name.as_str());
            let socket_transform = &socket.borrow()._transform;
            let socket_render_create_info = RenderObjectCreateInfo {
                _model_data_name: item_data.borrow()._model_data_name.clone(),
                _position: math::extract_location(socket_transform),
                _rotation: math::matrix_decompose_pitch_yaw_roll(socket_transform),
                _scale: math::extract_scale(socket_transform),
                ..Default::default()
            };
            let socket_render_object = get_scene_manager_mut().add_dynamic_render_object(
                socket_render_create_info._model_data_name.as_str(),
                &socket_render_create_info,
                None,
            );
            item_render_objects.push(socket_render_object);
        }

        let prop_id = resolve_actor_id(prop_create_info._prop_id, uuid);
        let prop = newRcRefCell(Prop::create_prop(
            self,
            prop_id,
            prop_name.as_str(),
            prop_data,
            &render_object_data,
            item_render_objects,
            prop_create_info,
        ));

        self._props.insert(prop_id, prop.clone());
        if !self._prop_name_map.contains_key(prop_name.as_str()) {
            self._prop_name_map.insert(prop_name, prop.clone());
        }
        prop
    }

    pub fn remove_prop(&mut self, prop_ref: &RcRefCell<Prop<'a>>) {
        let prop = prop_ref.borrow();
        self._props.remove(&prop.get_prop_id());
        if let Some(target) = self._prop_name_map.get(prop._prop_name.as_str())
            && target.as_ptr() == prop_ref.as_ptr()
        {
            self._prop_name_map.remove(prop._prop_name.as_str());
        }

        let scene_manager = get_scene_manager_mut();
        for item_render_object in &prop._item_render_objects {
            scene_manager.remove_static_render_object(item_render_object.borrow()._object_id);
        }
        scene_manager.remove_static_render_object(prop._render_object.borrow()._object_id);
    }

    pub fn clear_props(&mut self) {
        let props: Vec<RcRefCell<Prop<'a>>> = self._props.values().cloned().collect();
        for prop in props {
            self.remove_prop(&prop);
        }
    }

    pub fn create_props(&mut self, prop_create_infos: &PropCreateInfoMap) {
        for (prop_name, prop_create_info) in prop_create_infos.iter() {
            self.create_prop(prop_name.as_str(), prop_create_info);
        }
    }

    pub fn load_prop_save_data(&mut self, prop_name: &str, prop_save_data: &PropSaveData) {
        let prop = self.create_prop(prop_name, &prop_save_data._prop_create_info);
        prop.borrow_mut().load_prop_save_data(prop_save_data);
    }

    pub fn load_props_save_data(&mut self, props_save_data: &PropSaveDataMap) {
        for (prop_name, prop_save_data) in props_save_data.iter() {
            self.load_prop_save_data(prop_name.as_str(), prop_save_data);
        }
    }

    pub fn get_props_save_data(&self) -> PropSaveDataMap {
        self._props.values().map(|prop| prop.borrow().get_prop_save_data()).collect()
    }

    pub fn post_process_after_prop_loading(&mut self) {
        for prop in self._props.values() {
            prop.borrow_mut().post_process_after_prop_loading();
        }
    }

    pub fn get_teleport_point(&self, gate_prop_name: &str) -> Option<Vector3<f32>> {
        if let Some(gate) = self.get_prop_by_name(gate_prop_name) {
            let prop = gate.borrow();
            let prop_bounding_box = prop.get_bounding_box();
            let mut teleport_point =
                prop.get_position() + prop_bounding_box.get_orientation().column(2) * (prop_bounding_box._mag_xz + 1.0);
            teleport_point.y =
                teleport_point.y.max(get_scene_manager().get_height_map_data().get_height_bilinear(&teleport_point, 0));
            return Some(teleport_point);
        }
        None
    }

    fn check_player_in_prop_range(player: &Character<'a>, prop: &Prop<'a>, range: f32) -> bool {
        if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
            player.check_in_range_xy(prop.get_collision(), range, false)
        } else {
            player.check_in_range(prop.get_collision(), range, false)
        }
    }

    fn sync_interaction_object(
        player: &mut Character<'a>,
        was_registered: bool,
        should_register: bool,
        interaction_object: InteractionObject<'a>,
    ) {
        if !was_registered && should_register {
            player._controller.add_interaction_object(interaction_object);
        } else if was_registered && !should_register {
            player._controller.remove_interaction_object(interaction_object);
        }
    }

    fn spawn_dropped_items(prop: &mut Prop<'a>, drop_count: i32, player_pos: &Vector3<f32>) {
        let mut dropped_items = prop.drop_items(drop_count, player_pos);
        let item_manager = get_item_manager_mut();
        for item_create_info in dropped_items.iter_mut() {
            item_manager.create_item(item_create_info._item_data_name.as_str(), item_create_info, None);
        }
    }

    pub fn update_prop_manager(&mut self, delta_time: f64) {
        if !get_character_manager().is_valid_player() {
            return;
        }

        for prop in self._props.values() {
            prop.borrow_mut().update_prop(delta_time);
        }

        let player_refcell = get_character_manager().get_player().clone();
        let mut player = player_refcell.borrow_mut();

        let mut dead_props: Vec<RcRefCell<Prop>> = Vec::new();
        if player.is_alive() {
            let is_available_interaction = get_game_client().is_available_interaction();
            for prop_refcell in self._props.values() {
                let mut prop = prop_refcell.borrow_mut();
                let key = prop_refcell.as_ptr() as *const c_void;
                let is_interaction_object = player._controller.is_interaction_object(key);
                let prop_type = prop._prop_data.borrow()._prop_type;
                let bounding_box = prop.get_bounding_box();

                match prop_type {
                    PropDataType::Bed => {
                        let to_player = player.get_position() - prop.get_position();
                        let (_to_player_dir, mut to_player_distance) = if GAME_VIEW_MODE == GameViewMode::GameViewMode2D
                        {
                            math::make_normalize_xy_with_norm(&to_player)
                        } else {
                            math::make_normalize_with_norm(&to_player)
                        };
                        to_player_distance = 0f32.max(
                            to_player_distance
                                - (player.get_collision()._bounding_box._mag_xz + bounding_box._mag_xz) * 0.5,
                        );
                        let is_wrap_up = get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
                        let is_in_player_range = is_available_interaction
                            && (is_wrap_up || prop.is_deployed_prop())
                            && to_player_distance <= CHARACTER_INTERACTION_DISTANCE;

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range,
                            InteractionObject::PropBed(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Ceiling => {
                        prop._render_object
                            .borrow_mut()
                            .set_render_camera(!bounding_box.collide_point(player.get_position()));
                    }
                    PropDataType::Destruction => {
                        let is_in_player_range = is_available_interaction
                            && Self::check_player_in_prop_range(&player, &prop, NPC_ATTACK_HIT_RANGE);

                        if player._animation_state.is_attack_event() && is_in_player_range {
                            prop.set_hit_damage(player.get_power(player._animation_state.get_action_event()));
                            if !prop.is_alive() {
                                let drop_count = prop._prop_stats._item_count;
                                Self::spawn_dropped_items(&mut prop, drop_count, player.get_position());
                                dead_props.push(prop_refcell.clone());
                            }
                        }

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range && prop.is_alive(),
                            InteractionObject::PropGathering(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Harvestable => {
                        let is_in_player_range = is_available_interaction
                            && Self::check_player_in_prop_range(&player, &prop, NPC_ATTACK_HIT_RANGE);

                        let can_drop_item = prop.can_drop_item();
                        if can_drop_item && player._animation_state.is_attack_event() && is_in_player_range {
                            prop.set_hit_damage(0);
                            Self::spawn_dropped_items(&mut prop, 1, player.get_position());
                        }

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range && prop.can_drop_item(),
                            InteractionObject::PropGathering(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Gate => {
                        let is_in_player_range = is_available_interaction
                            && if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                                bounding_box.collide_point_xy(player.get_center())
                            } else {
                                bounding_box.collide_point(player.get_center())
                            };

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range,
                            InteractionObject::PropGate(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Pickup => {
                        let mut is_in_player_range = is_available_interaction
                            && if GAME_VIEW_MODE == GameViewMode::GameViewMode2D {
                                player.get_bounding_box().collide_bound_box_xy(&bounding_box._min, &bounding_box._max)
                            } else {
                                player.get_bounding_box().collide_bound_box(&bounding_box._min, &bounding_box._max)
                            };

                        if is_in_player_range && player._animation_state.is_action_event(ActionEvent::Pickup) {
                            let mut pickup_items = false;
                            let drop_count = prop._prop_stats._item_count;
                            for item_create_info in prop.drop_items(drop_count, player.get_position()).iter() {
                                pickup_items |= get_item_manager_mut().instance_pickup_item(item_create_info);
                            }

                            if pickup_items {
                                is_in_player_range = false;
                                dead_props.push(prop_refcell.clone());
                            }
                        }

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range,
                            InteractionObject::PropPickup(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Monolith => {
                        let is_in_player_range = is_available_interaction
                            && Self::check_player_in_prop_range(&player, &prop, CHARACTER_INTERACTION_DISTANCE);

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range,
                            InteractionObject::PropMonolith(prop_refcell.clone()),
                        );
                    }
                    PropDataType::Table => {
                        let is_wrap_up = get_game_scene_manager().has_game_scenario(ScenarioType::ScenarioWrapUpTheDay);
                        let is_in_player_range = is_available_interaction
                            && is_wrap_up
                            && Self::check_player_in_prop_range(&player, &prop, CHARACTER_INTERACTION_DISTANCE);

                        Self::sync_interaction_object(
                            &mut player,
                            is_interaction_object,
                            is_in_player_range,
                            InteractionObject::PropTable(prop_refcell.clone()),
                        );
                    }
                    _ => {
                        if prop.is_deployed_prop() {
                            let is_in_player_range = is_available_interaction
                                && Self::check_player_in_prop_range(&player, &prop, CHARACTER_INTERACTION_DISTANCE);

                            Self::sync_interaction_object(
                                &mut player,
                                is_interaction_object,
                                is_in_player_range,
                                InteractionObject::PropDeployed(prop_refcell.clone()),
                            );
                        }
                    }
                }
            }
        }

        for prop_refcell in dead_props.iter() {
            self.remove_prop(prop_refcell);
        }
    }
}
