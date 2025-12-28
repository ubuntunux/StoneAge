use crate::application::application::Application;
use crate::game_module::actors::character::InteractionObject;
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::actors::props::{
    Prop, PropCreateInfo, PropData, PropDataType, PropID, PropManager, PropMap, PropStats,
};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_HIT, EFFECT_HIT, GAME_MODE_2D, NPC_ATTACK_HIT_RANGE};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::Vector3;
use rand;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::bounding_box::BoundingBox;
use rust_engine_3d::scene::collision::{CollisionData, CollisionType};
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use std::collections::HashMap;
use std::ffi::c_void;
use rust_engine_3d::utilities::math;
use crate::game_module::actors::character_data::ActionAnimationState;

impl Default for PropData {
    fn default() -> Self {
        PropData {
            _prop_type: PropDataType::None,
            _model_data_name: String::new(),
            _name: String::new(),
            _max_hp: 0,
            _item_data_name: String::new(),
            _item_drop_count_max: 1,
            _item_drop_count_min: 1,
            _item_regenerate_time: 0.0,
        }
    }
}

impl PropData {
    pub fn get_item_drop_count(&self) -> i32 {
        rand::random_range(self._item_drop_count_min..=self._item_drop_count_max)
    }
}

impl<'a> Prop<'a> {
    pub fn create_prop(
        prop_manager: *const PropManager<'a>,
        prop_id: PropID,
        prop_name: &str,
        prop_data: &RcRefCell<PropData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        item_render_objects: Vec<RcRefCell<RenderObjectData<'a>>>,
        prop_create_info: &PropCreateInfo,
    ) -> Prop<'a> {
        let prop_data_ref = prop_data.borrow();
        let item_count = prop_data_ref.get_item_drop_count();
        let mut prop = Prop {
            _prop_name: String::from(prop_name),
            _prop_id: prop_id,
            _prop_radius: render_object.borrow()._collision._bounding_box.get_mag_xz(),
            _prop_manager: prop_manager,
            _render_object: render_object.clone(),
            _item_render_objects: item_render_objects,
            _prop_data: prop_data.clone(),
            _prop_stats: Box::from(PropStats {
                _is_alive: false,
                _prop_hp: prop_data_ref._max_hp,
                _item_regenerate_time: 0.0,
                _item_count_max: item_count,
                _item_count: item_count,
                _position: prop_create_info._position.clone(),
                _rotation: prop_create_info._rotation.clone(),
                _scale: prop_create_info._scale.clone(),
                _is_in_player_range: false,
            }),
            _instance_parameters: prop_create_info._instance_parameters.clone(),
        };

        if prop_data_ref._prop_type == PropDataType::Ceiling || prop_data_ref._prop_type == PropDataType::Gate {
            render_object.borrow_mut().set_collision_type(CollisionType::NONE);
        }

        prop.initialize_prop();
        prop
    }
    pub fn initialize_prop(&mut self) {
        self._prop_stats._is_alive = true;
        self._prop_stats._item_regenerate_time = 0.0;
        self.update_item_visible();
        self.update_transform();
    }
    pub fn get_prop_id(&self) -> PropID {
        self._prop_id
    }
    pub fn get_prop_manager(&self) -> &PropManager<'a> {
        ptr_as_ref(self._prop_manager)
    }
    pub fn get_instance_parameters(&self, key: &str) -> Option<serde_json::Value> {
        self._instance_parameters.get(key).cloned()
    }
    pub fn can_drop_item(&self) -> bool {
        0 < self._prop_stats._item_count
    }
    pub fn get_position(&self) -> &Vector3<f32> {
        &ptr_as_ref(self._render_object.as_ptr())._transform_object._position
    }
    pub fn get_bounding_box(&self) -> &BoundingBox {
        &ptr_as_ref(self._render_object.as_ptr())._bounding_box
    }
    pub fn get_collision(&self) -> &CollisionData {
        &ptr_as_ref(self._render_object.as_ptr())._collision
    }
    pub fn is_alive(&self) -> bool {
        self._prop_stats._is_alive
    }
    pub fn set_dead(&mut self) {
        self._prop_stats._is_alive = false;
    }
    pub fn set_hit_damage(&mut self, damage: i32) {
        if PropDataType::Harvestable == self._prop_data.borrow()._prop_type {
            // nothing to do
        } else {
            self._prop_stats._prop_hp -= damage;
            if self._prop_stats._prop_hp <= 0 {
                self.set_dead();
            }
        }

        let effect_create_info = EffectCreateInfo {
            _effect_position: self.get_bounding_box().get_center().clone(),
            _effect_data_name: String::from(EFFECT_HIT),
            ..Default::default()
        };
        self.get_prop_manager().get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
        self.get_prop_manager().get_audio_manager_mut().play_audio_bank(AUDIO_HIT, AudioLoop::ONCE, None);
    }
    pub fn update_generate_item(&mut self, delta_time: f64) {
        if self._prop_data.borrow()._prop_type == PropDataType::Harvestable && self._prop_stats._item_count < self._prop_stats._item_count_max {
            if self._prop_data.borrow()._item_regenerate_time <= self._prop_stats._item_regenerate_time {
                self._prop_stats._item_count += 1;
                self._prop_stats._item_regenerate_time = 0.0;
                self.update_item_visible();
            } else {
                self._prop_stats._item_regenerate_time += delta_time as f32;
            }
        }
    }
    pub fn drop_items(&mut self, mut drop_count: i32, player_position: &Vector3<f32>) -> Vec<ItemCreateInfo> {
        if self._prop_stats._item_count <= drop_count {
            drop_count = self._prop_stats._item_count;
            self._prop_stats._item_count = 0;
        } else {
            self._prop_stats._item_count -= drop_count;
        }
        self.update_item_visible();

        let mut item_create_infos: Vec<ItemCreateInfo> = vec![];
        for drop_index in 0..drop_count {
            let mut position = self.get_bounding_box().get_center().clone();
            let mut velocity = Vector3::new(0.0, 0.0, 0.0);
            if let Some(item_render_object) = &self._item_render_objects.get((self._prop_stats._item_count + drop_index) as usize) {
                match self._prop_data.borrow()._prop_type {
                    PropDataType::Harvestable => {
                        position = item_render_object.borrow()._transform_object._position.clone();
                        let mut to_item = item_render_object.borrow()._transform_object._position - self.get_position();
                        to_item = math::make_normalize_xz(&to_item);
                        velocity = Vector3::new(to_item.x, 1.0, to_item.z) * 2.0;
                    },
                    _ => {
                        velocity = Vector3::new(
                            (rand::random::<f32>() * 2.0 - 1.0) * 5.0,
                            rand::random::<f32>() * 10.0,
                            (rand::random::<f32>() * 2.0 - 1.0) * 5.0,
                        );
                    }
                };
            }

            if GAME_MODE_2D {
                position.z = player_position.z;
                velocity.z = 0.0;
            }

            item_create_infos.push(ItemCreateInfo {
                _item_data_name: self._prop_data.borrow()._item_data_name.clone(),
                _position: position,
                _velocity: velocity,
                ..Default::default()
            });
        }
        item_create_infos
    }
    pub fn update_item_visible(&mut self) {
        for (i, item_render_object) in self._item_render_objects.iter().enumerate() {
            item_render_object.borrow_mut().set_visible( i < self._prop_stats._item_count as usize );
        }
    }
    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_position_rotation_scale(
            &self._prop_stats._position,
            &self._prop_stats._rotation,
            &self._prop_stats._scale,
        );
    }
    pub fn update_render_object(&mut self) {
        self._render_object.borrow_mut().update_render_object_data(0.0);
    }
    pub fn update_prop(&mut self, delta_time: f64) {
        self.update_generate_item(delta_time);
        self.update_transform();
    }
}

// PropManager
impl<'a> PropManager<'a> {
    pub fn create_prop_manager() -> Box<PropManager<'a>> {
        Box::new(PropManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _id_generator: PropID(0),
            _props: HashMap::new(),
        })
    }

    pub fn initialize_prop_manager(
        &mut self,
        engine_core: &EngineCore<'a>,
        application: &Application<'a>,
    ) {
        log::info!("initialize_prop_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
    }
    pub fn destroy_prop_manager(&mut self) {}
    pub fn get_game_resources(&self) -> &GameResources<'a> {
        ptr_as_ref(self._game_resources)
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
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }
    pub fn generate_id(&mut self) -> PropID {
        let id = self._id_generator.clone();
        self._id_generator = PropID(self._id_generator.0 + 1);
        id
    }
    pub fn get_prop(&self, prop_id: PropID) -> Option<&RcRefCell<Prop<'a>>> {
        self._props.get(&prop_id)
    }
    pub fn get_prop_by_name(&self, prop_name: &str) -> Option<&RcRefCell<Prop<'a>>> {
        for prop in self._props.values() {
            if prop.borrow()._prop_name == prop_name {
                return Some(prop);
            }
        }
        None
    }
    pub fn get_props(&self) -> &PropMap<'a> {
        &self._props
    }
    pub fn create_prop(
        &mut self,
        prop_name: &str,
        prop_create_info: &PropCreateInfo,
    ) -> RcRefCell<Prop<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let prop_data = game_resources.get_prop_data(prop_create_info._prop_data_name.as_str());

        // create prop render objects
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: prop_data.borrow()._model_data_name.clone(),
            _position: prop_create_info._position.clone(),
            _rotation: prop_create_info._rotation.clone(),
            _scale: prop_create_info._scale.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_dynamic_render_object(
            prop_name,
            &render_object_create_info,
            None,
        );

        // create item render objects
        let mut item_render_objects: Vec<RcRefCell<RenderObjectData<'a>>> = Vec::new();
        for (_, socket) in render_object_data.borrow()._sockets.iter() {
            let item_data = game_resources.get_item_data(prop_data.borrow()._item_data_name.as_str());
            let render_object_create_info = RenderObjectCreateInfo {
                _model_data_name: item_data.borrow()._model_data_name.clone(),
                _position: math::extract_location(&socket.borrow()._transform),
                _rotation: math::matrix_decompose_pitch_yaw_roll(&socket.borrow()._transform),
                _scale: math::extract_scale(&socket.borrow()._transform),
                ..Default::default()
            };
            let render_object_data = self.get_scene_manager_mut().add_dynamic_render_object(
                render_object_create_info._model_data_name.as_str(),
                &render_object_create_info,
                None,
            );
            item_render_objects.push(render_object_data);
        }

        let id = self.generate_id();
        let prop = newRcRefCell(Prop::create_prop(
            self,
            id,
            prop_name,
            prop_data,
            &render_object_data,
            item_render_objects,
            &prop_create_info,
        ));
        self._props.insert(id, prop.clone());
        prop
    }

    pub fn remove_prop(&mut self, prop: &RcRefCell<Prop<'a>>) {
        self._props.remove(&prop.borrow().get_prop_id());
        self.get_scene_manager_mut()
            .remove_static_render_object(prop.borrow()._render_object.borrow()._object_id);
    }

    pub fn clear_props(&mut self) {
        let props: Vec<RcRefCell<Prop<'a>>> = self._props.values().cloned().collect();
        for prop in props {
            self.remove_prop(&prop);
        }
    }

    pub fn get_teleport_point(&self, gate_prop_name: &str) -> Option<Vector3<f32>> {
        let gate = self.get_prop_by_name(gate_prop_name);
        if gate.is_some() {
            let prop = gate.unwrap().borrow();
            let prop_bounding_box = prop.get_bounding_box();
            let mut teleport_point = prop.get_position()
                + prop_bounding_box.get_orientation().column(2) * (prop_bounding_box._mag_xz + 1.0);
            teleport_point.y = teleport_point.y.max(
                self.get_game_scene_manager()
                    .get_scene_manager()
                    .get_height_map_data()
                    .get_height_bilinear(&teleport_point, 0),
            );
            return Some(teleport_point);
        }
        None
    }

    pub fn update_prop_manager(&mut self, delta_time: f64) {
        for prop in self._props.values() {
            prop.borrow_mut().update_prop(delta_time);
        }

        let player_refcell = self.get_game_scene_manager().get_character_manager().get_player().clone();
        let mut player = player_refcell.borrow_mut();

        let mut dead_props: Vec<RcRefCell<Prop>> = Vec::new();
        {
            let check_direction = true;
            if player.is_alive() {
                for prop_refcell in self._props.values() {
                    let mut prop = prop_refcell.borrow_mut();
                    let key = prop_refcell.as_ptr() as *const c_void;
                    let is_interaction_object = player._controller.is_interaction_object(key);
                    let prop_type = prop._prop_data.borrow()._prop_type;
                    let bounding_box = prop.get_bounding_box();

                    match prop_type {
                        PropDataType::Bed => {
                            prop._prop_stats._is_in_player_range = player.get_bounding_box().collide_bound_box(&bounding_box._min, &bounding_box._max);
                            if is_interaction_object == false && prop._prop_stats._is_in_player_range {
                                player._controller.add_interaction_object(InteractionObject::PropBed(prop_refcell.clone()));
                            } else if is_interaction_object && prop._prop_stats._is_in_player_range == false {
                                player._controller.remove_interaction_object(InteractionObject::PropBed(prop_refcell.clone()));
                            }
                        }
                        PropDataType::Ceiling => {
                            prop._render_object.borrow_mut().set_render_camera(!bounding_box.collide_point(&player.get_position()));
                        }
                        PropDataType::Destruction => {
                            prop._prop_stats._is_in_player_range = player.check_in_range(prop.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction);
                            if player._animation_state.is_attack_event() && prop._prop_stats._is_in_player_range {
                                prop.set_hit_damage(player.get_power(player._animation_state.get_action_event()));
                                if false == prop.is_alive() {
                                    let drop_count = prop._prop_stats._item_count;
                                    for item_create_info in prop.drop_items(drop_count, player.get_position()).iter_mut() {
                                        self.get_game_scene_manager().get_item_manager_mut().create_item(&item_create_info);
                                    }
                                    dead_props.push(prop_refcell.clone());
                                }
                            }

                            if is_interaction_object == false && prop._prop_stats._is_in_player_range && prop.is_alive() {
                                player._controller.add_interaction_object(InteractionObject::PropGathering(prop_refcell.clone()));
                            } else if is_interaction_object && prop._prop_stats._is_in_player_range == false || prop.is_alive() == false {
                                player._controller.remove_interaction_object(InteractionObject::PropGathering(prop_refcell.clone()));
                            }
                        }
                        PropDataType::Harvestable => {
                            prop._prop_stats._is_in_player_range = player.check_in_range(prop.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction);
                            let can_drop_item = prop.can_drop_item();
                            if can_drop_item && player._animation_state.is_attack_event() && prop._prop_stats._is_in_player_range {
                                prop.set_hit_damage(0);
                                let drop_count = 1;
                                for item_create_info in prop.drop_items(drop_count, player.get_position()).iter_mut() {
                                    self.get_game_scene_manager().get_item_manager_mut().create_item(&item_create_info);
                                }
                            }

                            if is_interaction_object == false && prop._prop_stats._is_in_player_range && can_drop_item {
                                player._controller.add_interaction_object(InteractionObject::PropGathering(prop_refcell.clone()));
                            } else if is_interaction_object && (prop._prop_stats._is_in_player_range == false || can_drop_item == false) {
                                player._controller.remove_interaction_object(InteractionObject::PropGathering(prop_refcell.clone()));
                            }
                        }
                        PropDataType::Gate => {
                            prop._prop_stats._is_in_player_range = bounding_box.collide_point(player.get_center());
                            if prop._prop_stats._is_in_player_range {
                                if player._animation_state.is_action_event(ActionAnimationState::EnterGate) {
                                    let linked_gate = prop.get_instance_parameters("_linked_gate");
                                    let linked_stage = prop.get_instance_parameters("_linked_stage");
                                    if linked_stage.is_some() && linked_gate.is_some() {
                                        self.get_game_scene_manager_mut().set_teleport_stage(
                                            linked_stage.unwrap().as_str().unwrap(),
                                            linked_gate.unwrap().as_str().unwrap(),
                                        );
                                    }
                                }
                            }

                            if is_interaction_object == false && prop._prop_stats._is_in_player_range {
                                player._controller.add_interaction_object(InteractionObject::PropGate(prop_refcell.clone()));
                            } else if is_interaction_object && prop._prop_stats._is_in_player_range == false {
                                player._controller.remove_interaction_object(InteractionObject::PropGate(prop_refcell.clone()));
                            }
                        }
                        PropDataType::Pickup => {
                            prop._prop_stats._is_in_player_range = player.get_bounding_box().collide_bound_box(&bounding_box._min, &bounding_box._max);
                            if prop._prop_stats._is_in_player_range {
                                if player._animation_state.is_action_event(ActionAnimationState::Pickup) {
                                    let mut pickup_items: bool = false;
                                    let drop_count = prop._prop_stats._item_count;
                                    for item_create_info in prop.drop_items(drop_count, player.get_position()).iter() {
                                        pickup_items |= self.get_game_scene_manager().get_item_manager_mut().instance_pickup_item(&item_create_info);
                                    }

                                    if pickup_items {
                                        prop._prop_stats._is_in_player_range = false;
                                        dead_props.push(prop_refcell.clone());
                                    }
                                }
                            }

                            if is_interaction_object == false && prop._prop_stats._is_in_player_range {
                                player._controller.add_interaction_object(InteractionObject::PropPickup(prop_refcell.clone()));
                            } else if is_interaction_object && prop._prop_stats._is_in_player_range == false {
                                player._controller.remove_interaction_object(InteractionObject::PropPickup(prop_refcell.clone()));
                            }
                        },
                        PropDataType::Monolith => {
                            prop._prop_stats._is_in_player_range = player.check_in_range(prop.get_collision(), NPC_ATTACK_HIT_RANGE, check_direction);
                            if prop._prop_stats._is_in_player_range {
                                if player._animation_state.is_action_event(ActionAnimationState::OpenToolbox) {
                                    self.get_game_client_mut().set_need_toolbox_mode(true);
                                }
                            }

                            if is_interaction_object == false && prop._prop_stats._is_in_player_range {
                                player._controller.add_interaction_object(InteractionObject::PropMonolith(prop_refcell.clone()));
                            } else if is_interaction_object && prop._prop_stats._is_in_player_range == false {
                                player._controller.remove_interaction_object(InteractionObject::PropMonolith(prop_refcell.clone()));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        for prop_refcell in dead_props.iter() {
            self.remove_prop(prop_refcell);
        }
    }
}
