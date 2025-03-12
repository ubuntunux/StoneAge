use std::collections::HashMap;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::scene::collision::CollisionData;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::application::application::Application;
use crate::game_module::actors::character_data::ActionAnimationState;
use crate::game_module::actors::items::ItemCreateInfo;
use crate::game_module::actors::props::{Prop, PropCreateInfo, PropData, PropDataType, PropManager, PropMap, PropStats};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_HIT, EFFECT_HIT, NPC_ATTACK_HIT_RANGE};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

impl Default for PropData {
    fn default() -> Self {
        PropData {
            _prop_type: PropDataType::Rock,
            _model_data_name: String::new(),
            _max_hp: 0,
            _item_data_name: String::new(),
            _enable_collision: true,
        }
    }
}

// Prop
impl<'a> Prop<'a> {
    pub fn create_prop(
        prop_manager: *const PropManager<'a>,
        prop_id: u64,
        prop_name: &str,
        prop_data: &RcRefCell<PropData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        prop_create_info: &PropCreateInfo,
    ) -> Prop<'a> {
        let mut prop = Prop {
            _prop_name: String::from(prop_name),
            _prop_id: prop_id,
            _prop_manager: prop_manager,
            _render_object: render_object.clone(),
            _prop_data: prop_data.clone(),
            _prop_stats: Box::from(PropStats {
                _is_alive: false,
                _prop_hp: prop_data.borrow()._max_hp,
                _position: prop_create_info._position.clone(),
                _rotation: prop_create_info._rotation.clone(),
                _scale: prop_create_info._scale.clone(),
            }),
        };
        prop.initialize_prop();
        prop
    }

    pub fn initialize_prop(&mut self) {
        self._prop_stats._is_alive = true;
        self.update_transform();
    }

    pub fn get_prop_id(&self) -> u64 {
        self._prop_id
    }

    pub fn get_prop_manager(&self) -> &PropManager<'a> {
        ptr_as_ref(self._prop_manager)
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &ptr_as_ref(self._render_object.as_ptr())._transform_object._position
    }

    pub fn get_collision(&self) -> &CollisionData {
        &ptr_as_ref(self._render_object.as_ptr())._collision
    }

    pub fn set_dead(&mut self) {
        if self._prop_stats._is_alive {
            //self.get_prop_manager().get_audio_manager_mut().play_audio_bank(AUDIO_CRUNCH, AudioLoop::ONCE, None);
            self._prop_stats._is_alive = false;
        }
    }

    pub fn set_damage(&mut self, attack_point: &Vector3<f32>, damage: i32) {
        self._prop_stats._prop_hp -= damage;
        if self._prop_stats._prop_hp <= 0 {
            self.set_dead();
        }

        let effect_create_info = EffectCreateInfo {
            _effect_position: attack_point.clone(),
            _effect_data_name: String::from(EFFECT_HIT),
            ..Default::default()
        };
        self.get_prop_manager().get_scene_manager_mut().add_effect(EFFECT_HIT, &effect_create_info);
        self.get_prop_manager().get_audio_manager_mut().play_audio_bank(AUDIO_HIT, AudioLoop::ONCE, None);
    }

    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_transform(
            &self._prop_stats._position,
            &self._prop_stats._rotation,
            &self._prop_stats._scale,
        );
    }

    pub fn update_render_object(&mut self) {
        self._render_object.borrow_mut().update_render_object_data(0.0);
    }

    pub fn update_prop(&mut self, _delta_time: f64) {
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
            _id_generator: 0,
            _props: HashMap::new(),
        })
    }

    pub fn initialize_prop_manager(&mut self, engine_core: &EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_prop_manager");
        self._game_client = application.get_game_client();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
    }
    pub fn destroy_prop_manager(&mut self) {
    }
    pub fn get_game_resources(&self) -> &GameResources<'a> {
        ptr_as_ref(self._game_resources)
    }
    pub fn get_game_client(&self) -> &GameClient<'a> {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager<'a> {
        ptr_as_ref(self._game_scene_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager<'a> {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }
    pub fn get_prop(&self, prop_id: u64) -> Option<&RcRefCell<Prop<'a>>> {
        self._props.get(&prop_id)
    }
    pub fn get_props(&self) -> &PropMap<'a> {
        &self._props
    }
    pub fn create_prop(&mut self, prop_name: &str, prop_create_info: &PropCreateInfo) -> RcRefCell<Prop<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let prop_data = game_resources.get_prop_data(prop_create_info._prop_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: prop_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_static_render_object(
            prop_name,
            &render_object_create_info,
        );
        let id = self.generate_id();
        let prop = newRcRefCell(Prop::create_prop(
            self,
            id,
            prop_name,
            prop_data,
            &render_object_data,
            &prop_create_info
        ));
        self._props.insert(id, prop.clone());
        prop
    }

    pub fn remove_prop(&mut self, prop: &RcRefCell<Prop>) {
        self._props.remove(&prop.borrow().get_prop_id());
        self.get_scene_manager_mut().remove_static_render_object(prop.borrow()._render_object.borrow()._object_id);
    }

    pub fn update_prop_manager(&mut self, delta_time: f64) {
        for prop in self._props.values() {
            prop.borrow_mut().update_prop(delta_time);
        }

        let mut dead_props: Vec<RcRefCell<Prop>> = Vec::new();
        {
            let game_scene_manager = self.get_game_scene_manager();
            let player_refcell = game_scene_manager.get_character_manager().get_player();
            let player = player_refcell.borrow_mut();
            if player._character_stats._is_alive {
                if player._animation_state._attack_event != ActionAnimationState::None {
                    for prop_refcell in self._props.values() {
                        let mut prop = prop_refcell.borrow_mut();
                        if player.check_in_range(prop.get_collision(), NPC_ATTACK_HIT_RANGE, true) {
                            let prop_position = ptr_as_ref(prop.get_position());
                            prop.set_damage(prop_position, player.get_power(player._animation_state._attack_event));

                            if false == prop._prop_stats._is_alive {
                                dead_props.push(prop_refcell.clone());

                                // TestCode: Item
                                let item_create_info = ItemCreateInfo {
                                    _item_data_name: prop._prop_data.borrow()._item_data_name.clone(),
                                    _position: prop_position + Vector3::new(0.0, 0.5, 0.0),
                                    ..Default::default()
                                };
                                self.get_game_scene_manager().get_item_manager_mut().create_item(&item_create_info);
                            }
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