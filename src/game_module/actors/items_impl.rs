use crate::application::application::Application;
use crate::game_module::actors::item_updater::create_item_updater;
use crate::game_module::actors::items::{
    Item, ItemCreateInfo, ItemData, ItemDataType, ItemID, ItemManager, ItemProperties,
};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{AUDIO_ITEM_INVENTORY, AUDIO_PICKUP_ITEM, EAT_ITEM_DISTANCE, WEAPON_SOCKET_NAME};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::{Vector3};
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::collision::CollisionType;
use rust_engine_3d::scene::height_map::HeightMapData;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use std::collections::HashMap;
use rust_engine_3d::scene::socket::Socket;

impl Default for ItemCreateInfo {
    fn default() -> Self {
        ItemCreateInfo {
            _item_data_name: "".to_string(),
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _pickup_delay: 0.5,
        }
    }
}

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            _item_type: ItemDataType::None,
            _model_data_name: String::new(),
            _name: String::new(),
            _ui_material_instance: String::new(),
            _weapon_damage: 10.0,
            _weapon_range: 0.0,
        }
    }
}

impl<'a> Item<'a> {
    pub fn create_item(
        item_id: ItemID,
        item_data_name: &str,
        item_data: &RcRefCell<ItemData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        attach_socket: Option<RcRefCell<Socket>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
        velocity: &Vector3<f32>,
        pickup_delay: f32,
    ) -> Item<'a> {
        let mut item = Item {
            _item_data_name: String::from(item_data_name),
            _item_id: item_id,
            _item_data: item_data.clone(),
            _render_object: render_object.clone(),
            _attach_socket: attach_socket,
            _item_properties: Box::new(ItemProperties {
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
                _velocity: velocity.clone(),
                _is_on_ground: false,
                _pickup_delay: pickup_delay,
            }),
            _item_updater: create_item_updater(item_data.borrow()._item_type),
        };

        item.initialize_item();
        item
    }

    pub fn initialize_item(&mut self) {
        if self.is_attachment() {
            self.update_item_attach_transform();
        } else {
            self.update_item_transform();
        }
    }

    pub fn get_item_id(&self) -> ItemID {
        self._item_id
    }

    pub fn get_item_data_name(&self) -> &String {
        &self._item_data_name
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._collision.collide_point(pos)
    }

    pub fn is_attachment(&self) -> bool {
        self._attach_socket.is_some()
    }

    pub fn pickable_item(&self) -> bool {
        self._item_properties._pickup_delay <= 0.0 && self.is_attachment() == false
    }

    pub fn update_pickup_delay_time(&mut self, delta_time: f64) {
        if 0.0 < self._item_properties._pickup_delay {
            self._item_properties._pickup_delay -= delta_time as f32;
        }
    }

    pub fn update_item_attach_transform(&mut self) {
        let mut render_object_mut = self._render_object.borrow_mut();
        if render_object_mut.has_animation() {
            let skeleton_transform = render_object_mut._mesh_data.borrow()._skeleton_data_list[0]._transform.clone();
            let final_transform = self._attach_socket.as_ref().unwrap().borrow()._transform * skeleton_transform;
            render_object_mut._transform_object.set_transform(&final_transform);
        } else {
            render_object_mut._transform_object.set_transform(&self._attach_socket.as_ref().unwrap().borrow()._transform);
        }
    }

    pub fn update_item_transform(&mut self) {
        self._render_object.borrow_mut()._transform_object.set_position_rotation_scale(
            &self._item_properties._position,
            &self._item_properties._rotation,
            &self._item_properties._scale,
        );
    }

    pub fn update_item(&mut self, height_map_data: &HeightMapData, delta_time: f64) {
        let owner = ptr_as_mut(self);
        self._item_updater.update_item_updater(owner, height_map_data, delta_time);
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
            _id_generator: ItemID(0),
            _items: HashMap::new(),
        })
    }

    pub fn initialize_item_manager(
        &mut self,
        engine_core: &EngineCore<'a>,
        application: &Application<'a>,
    ) {
        log::info!("initialize_item_manager");
        self._audio_manager = application.get_audio_manager();
        self._scene_manager = engine_core.get_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_client = application.get_game_client();
    }
    pub fn destroy_item_manager(&mut self) {}
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
    pub fn get_scene_manager(&self) -> &SceneManager<'a> {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager<'a> {
        ptr_as_mut(self._scene_manager)
    }
    pub fn generate_id(&mut self) -> ItemID {
        let id = self._id_generator.clone();
        self._id_generator = ItemID(self._id_generator.0 + 1);
        id
    }
    pub fn get_item(&self, item_id: ItemID) -> Option<&RcRefCell<Item<'a>>> {
        self._items.get(&item_id)
    }
    pub fn create_item(&mut self, item_create_info: &ItemCreateInfo, attach_socket: Option<RcRefCell<Socket>>) -> RcRefCell<Item<'a>> {
        let mut spawn_point = item_create_info._position.clone();
        spawn_point.y = spawn_point.y.max(self.get_scene_manager().get_height_map_data().get_height_bilinear(&spawn_point, 0));

        let game_resource = ptr_as_ref(self._game_resources);
        let item_data = game_resource.get_item_data(item_create_info._item_data_name.as_str());
        let item_model_data = game_resource.get_engine_resources().get_model_data(&item_data.borrow()._model_data_name);
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: item_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };

        let render_object_data = if item_model_data.borrow()._mesh_data.borrow().has_animation_data() {
            self.get_scene_manager_mut().add_skeletal_render_object(
                item_create_info._item_data_name.as_str(),
                &render_object_create_info
            )
        } else {
            self.get_scene_manager_mut().add_dynamic_render_object(
                item_create_info._item_data_name.as_str(),
                &render_object_create_info,
                Some(CollisionType::NONE),
            )
        };

        let id = self.generate_id();
        let item = newRcRefCell(Item::create_item(
            id,
            item_create_info._item_data_name.as_str(),
            item_data,
            &render_object_data,
            attach_socket,
            &spawn_point,
            &item_create_info._rotation,
            &item_create_info._scale,
            &item_create_info._velocity,
            item_create_info._pickup_delay,
        ));
        self._items.insert(id, item.clone());
        item
    }

    pub fn instance_pickup_item(&mut self, item_create_info: &ItemCreateInfo) -> bool {
        let item_count = 1;
        self.pick_item(item_create_info._item_data_name.as_str(), item_count)
    }

    pub fn remove_item(&mut self, item: &RcRefCell<Item>) {
        self._items.remove(&item.borrow().get_item_id());
        if item.borrow()._render_object.borrow().has_animation() {
            self.get_scene_manager_mut().remove_skeletal_render_object(item.borrow()._render_object.borrow().get_object_id());
        } else {
            self.get_scene_manager_mut().remove_static_render_object(item.borrow()._render_object.borrow().get_object_id());
        }
    }

    pub fn clear_items(&mut self) {
        let items = self._items.values().cloned().collect::<Vec<RcRefCell<Item>>>();
        for item in items {
            self.remove_item(&item);
        }
    }

    pub fn pick_item(&self, item_data_name: &str, item_count: usize) -> bool {
        let success = self.get_game_client().get_game_ui_manager_mut().add_item(item_data_name, item_count);
        if success {
            self.get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
        success
    }

    pub fn use_inventory_item(&self, item_data_name: &str, item_count: usize) -> bool {
        let success = self.get_game_client().get_game_ui_manager_mut().remove_item(item_data_name, item_count);
        if success {
            self.get_audio_manager_mut().play_audio_bank(
                AUDIO_ITEM_INVENTORY,
                AudioLoop::ONCE,
                None,
            );

            let mut player = self.get_game_scene_manager().get_character_manager().get_player().borrow_mut();
            player.get_stats_mut().add_hunger(-0.2);
            player.get_stats_mut().add_hp(10);
            player.get_stats_mut().add_stamina(10.0);
        }
        success
    }

    pub fn drop_inventory_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        let success = self.get_game_client().get_game_ui_manager_mut().remove_item(item_data_name, item_count);
        if success {
            self.get_audio_manager_mut().play_audio_bank(
                AUDIO_ITEM_INVENTORY,
                AudioLoop::ONCE,
                None,
            );

            let player = ptr_as_ref(self.get_game_scene_manager_mut().get_character_manager_mut().get_player().as_ptr());
            let yaw = player.get_rotation().y + (rand::random::<f32>() - 0.5) * std::f32::consts::PI * 0.5;
            let velocity = Vector3::new(-yaw.sin(), 1.0, -yaw.cos()) * (2.0 + rand::random::<f32>() * 2.0);
            let item_create_info = ItemCreateInfo {
                _item_data_name: String::from(item_data_name),
                _position: player.get_center().clone() + player.get_face_direction() * player.get_collision()._bounding_box._mag_xz,
                _velocity: velocity,
                _pickup_delay: 1.0,
                ..Default::default()
            };

            self.create_item(&item_create_info, None);
        }
        success
    }

    pub fn get_selected_inventory_item_data_name(&self) -> &str {
        self.get_game_client().get_game_ui_manager().get_selected_inventory_item_data_name()
    }

    pub fn select_next_item(&mut self) {
        self.get_game_client().get_game_ui_manager_mut().select_next_item();
    }

    pub fn select_previous_item(&mut self) {
        self.get_game_client().get_game_ui_manager_mut().select_previous_item();
    }

    pub fn select_item(&mut self, item_index: usize) {
        self.get_game_client().get_game_ui_manager_mut().select_item(item_index);
    }

    pub fn attach_item(&mut self, item_data_name: &str) {
        let player = ptr_as_mut(self.get_game_scene_manager_mut().get_character_manager_mut().get_player().as_ptr());
        if let Some(attached_item) = player.get_attached_item() {
            if *attached_item.borrow().get_item_data_name() == item_data_name {
                return;
            } else {
                self.remove_item(attached_item);
                player.detach_item();
            }
        }

        let item_create_info = ItemCreateInfo {
            _item_data_name: String::from(item_data_name),
            ..Default::default()
        };
        let attach_socket = Some(player._render_object.borrow()._sockets.get(&String::from(WEAPON_SOCKET_NAME)).unwrap().clone());
        let attached_item = self.create_item(&item_create_info, attach_socket);
        player.attach_item(attached_item);
    }

    pub fn detach_item(&mut self) {
        let player = ptr_as_mut(self.get_game_scene_manager_mut().get_character_manager_mut().get_player().as_ptr());
        if let Some(attached_item) = player.get_attached_item() {
            self.remove_item(attached_item);
            player.detach_item();
        }
    }

    pub fn update_item_manager(&mut self, delta_time: f64) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        let scene_manager = ptr_as_ref(self._scene_manager);

        if game_scene_manager.get_character_manager().is_valid_player() == false {
            return;
        }

        for item in self._items.values() {
            item.borrow_mut().update_item(scene_manager.get_height_map_data(), delta_time);
        }

        let mut pick_items: Vec<RcRefCell<Item>> = Vec::new();
        {
            let player = game_scene_manager.get_character_manager().get_player();
            let player_mut = player.borrow_mut();
            let player_position = player_mut.get_position();
            let player_bound_box = player_mut.get_bounding_box();
            for (_key, item) in self._items.iter() {
                let item_ref = item.borrow();
                let diff = item_ref._item_properties._position - player_position;
                let check_height =
                    item_ref._render_object.borrow()._bounding_box._min.y <= player_bound_box._max.y &&
                    player_bound_box._min.y <= item_ref._render_object.borrow()._bounding_box._max.y;
                if check_height && math::get_norm_xz(&diff) <= EAT_ITEM_DISTANCE && item_ref.pickable_item(){
                    // pick item
                    let item_count = 1;
                    let success = self.pick_item(item_ref._item_data_name.as_str(), item_count);
                    if success {
                        pick_items.push(item.clone());
                    }
                } else if item_ref._item_properties._position.y < scene_manager.get_dead_zone_height() {
                    pick_items.push(item.clone());
                }
            }
        }

        for item in pick_items.iter() {
            self.remove_item(item);
        }
    }
}
