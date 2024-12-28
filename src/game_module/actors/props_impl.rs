use std::collections::HashMap;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::application::application::Application;
use crate::game_module::actors::props::{Prop, PropCreateInfo, PropData, PropDataType, PropProperties, PropManager};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_CRUNCH, EAT_ITEM_DISTANCE};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

impl Default for PropData {
    fn default() -> Self {
        PropData {
            _prop_type: PropDataType::Rock,
            _model_data_name: String::new(),
            _max_hp: 0,
            _item_data_name: String::new(),
        }
    }
}

impl<'a> Prop<'a> {
    pub fn create_prop(
        prop_id: u64,
        prop_name: &str,
        prop_data: &RcRefCell<PropData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        prop_create_info: &PropCreateInfo,
    ) -> Prop<'a> {
        let mut prop = Prop {
            _prop_name: String::from(prop_name),
            _prop_id: prop_id,
            _render_object: render_object.clone(),
            _prop_data: prop_data.clone(),
            _prop_properties: Box::from(PropProperties {
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
        self.update_transform();
    }

    pub fn get_prop_id(&self) -> u64 {
        self._prop_id
    }

    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_transform(
            &self._prop_properties._position,
            &self._prop_properties._rotation,
            &self._prop_properties._scale,
        );
    }

    pub fn update_render_object(&mut self) {
        self._render_object.borrow_mut().update_render_object_data(0.0);
    }

    pub fn update_prop(&mut self, _delta_time: f64) {
    }
}

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

        let mut eaten_props: Vec<RcRefCell<Prop>> = Vec::new();
        {
            let game_scene_manager = self.get_game_scene_manager();
            let player = game_scene_manager.get_character_manager().get_player();
            let player_mut = player.borrow_mut();
            let player_position = player_mut.get_position();
            for prop in self._props.values() {
                let prop_ref = prop.borrow();
                let dist = (prop_ref._prop_properties._position - player_position).norm();
                if dist <= EAT_ITEM_DISTANCE {
                    eaten_props.push(prop.clone());
                    log::info!("add to eaten_props");
                }
            }
        }

        for prop in eaten_props.iter() {
            self.get_audio_manager_mut().play_audio_bank(AUDIO_CRUNCH, AudioLoop::ONCE, None);
            log::info!("Remove prop");
            self.remove_prop(prop);
        }
    }
}