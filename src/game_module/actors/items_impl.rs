use std::collections::HashMap;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::application::application::Application;
use crate::game_module::actors::items::{Item, ItemCreateInfo, ItemData, ItemDataType, ItemManager, ItemProperties};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{EAT_ITEM_DISTANCE, PICKUP_ITEM};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;

impl ItemDataType {
    pub fn get_item_material_instance_name(item_data_type: &ItemDataType) -> &str {
        match item_data_type {
            ItemDataType::Meat => "ui/items/item_meat",
            ItemDataType::Rock => "ui/items/item_rock",
            ItemDataType::Wood => "ui/items/item_wood"
        }
    }
}

impl Default for ItemCreateInfo {
    fn default() -> Self {
        ItemCreateInfo {
            _item_data_name: String::new(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            _item_type: ItemDataType::Meat,
            _model_data_name: String::new(),
            _name: String::new(),
        }
    }
}

impl<'a> Item<'a> {
    pub fn create_item(
        item_id: u64,
        item_name: &str,
        item_data: &RcRefCell<ItemData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>) -> Item<'a> {
        let mut item = Item {
            _item_name: String::from(item_name),
            _item_id: item_id,
            _item_data: item_data.clone(),
            _render_object: render_object.clone(),
            _item_properties: Box::from(ItemProperties {
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
            }),
        };
        item.initialize_item();
        item
    }

    pub fn initialize_item(&mut self) {
        self.update_transform();
    }

    pub fn get_item_id(&self) -> u64 {
        self._item_id
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._collision.collide_point(pos)
    }

    pub fn update_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_position_rotation_scale(
            &self._item_properties._position,
            &self._item_properties._rotation,
            &self._item_properties._scale,
        );
    }

    pub fn update_item(&mut self, _delta_time: f64) {
    }
}

impl<'a> ItemManager<'a> {
    pub fn create_item_manager() -> Box<ItemManager<'a>> {
        Box::new(ItemManager {
            _game_client: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _id_generator: 0,
            _items: HashMap::new(),
        })
    }

    pub fn initialize_item_manager(&mut self, engine_core: &EngineCore<'a>, application: &Application<'a>) {
        log::info!("initialize_item_manager");
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_client = application.get_game_client();
    }
    pub fn destroy_item_manager(&mut self) {
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
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager<'a> {
        ptr_as_mut(self._game_scene_manager)
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
    pub fn get_item(&self, item_id: u64) -> Option<&RcRefCell<Item<'a>>> {
        self._items.get(&item_id)
    }
    pub fn create_item(&mut self, item_create_info: &ItemCreateInfo) -> RcRefCell<Item<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let item_data = game_resources.get_item_data(item_create_info._item_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: item_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_static_render_object(
            item_create_info._item_data_name.as_str(),
            &render_object_create_info,
        );
        let id = self.generate_id();
        let item = newRcRefCell(Item::create_item(
            id,
            item_create_info._item_data_name.as_str(),
            item_data,
            &render_object_data,
            &item_create_info._position,
            &item_create_info._rotation,
            &item_create_info._scale,
        ));
        self._items.insert(id, item.clone());
        item
    }

    pub fn remove_item(&mut self, item: &RcRefCell<Item>) {
        self._items.remove(&item.borrow().get_item_id());
        self.get_scene_manager_mut().remove_static_render_object(item.borrow()._render_object.borrow()._object_id);
    }

    pub fn pick_item(&self, item_data_type: &ItemDataType, item_count: i32) {
        self.get_game_client().get_game_ui_manager_mut().add_item(item_data_type, item_count)
    }

    pub fn update_item_manager(&mut self, delta_time: f64) {
        for item in self._items.values() {
            item.borrow_mut().update_item(delta_time);
        }

        let mut pick_items: Vec<RcRefCell<Item>> = Vec::new();
        {
            let game_scene_manager = ptr_as_ref(self._game_scene_manager);
            let player = game_scene_manager.get_character_manager().get_player();
            let player_mut = player.borrow_mut();
            let player_position = player_mut.get_position();
            for (_key, item) in self._items.iter() {
                let dist = (item.borrow()._item_properties._position - player_position).norm();
                if dist <= EAT_ITEM_DISTANCE {
                    // pick item
                    self.pick_item(&item.borrow()._item_data.borrow()._item_type, 1);
                    self.get_audio_manager_mut().play_audio_bank(PICKUP_ITEM, AudioLoop::ONCE, None);
                    pick_items.push(item.clone());
                }
            }
        }

        for item in pick_items.iter() {
            self.remove_item(item);
        }
    }
}