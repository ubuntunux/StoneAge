use crate::application::application::Application;
use crate::game_module::actors::item_updater::create_item_updater;
use crate::game_module::actors::items::{
    Item, ItemCreateInfo, ItemData, ItemDataType, ItemID, ItemManager, ItemProperties,
};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{
    AUDIO_ITEM_INVENTORY, AUDIO_PICKUP_ITEM, EAT_ITEM_DISTANCE,
};
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::{AudioLoop, AudioManager};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::scene::collision::CollisionType;
use rust_engine_3d::scene::height_map::HeightMapData;
use rust_engine_3d::scene::render_object::{RenderObjectCreateInfo, RenderObjectData};
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use std::collections::HashMap;

impl ItemDataType {
    pub fn get_item_material_instance_name(item_data_type: &ItemDataType) -> &str {
        match item_data_type {
            ItemDataType::None => "ui/items/item_none",
            ItemDataType::Coconut => "ui/items/item_coconut",
            ItemDataType::Meat => "ui/items/item_meat",
            ItemDataType::Rock => "ui/items/item_rock",
            ItemDataType::Wood => "ui/items/item_wood",
            ItemDataType::SpiritBall => "ui/items/item_spirit_ball",
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
            _velocity: Vector3::zeros(),
        }
    }
}

impl Default for ItemData {
    fn default() -> Self {
        ItemData {
            _item_type: ItemDataType::None,
            _model_data_name: String::new(),
            _name: String::new(),
        }
    }
}

impl<'a> Item<'a> {
    pub fn create_item(
        item_id: ItemID,
        item_name: &str,
        item_data: &RcRefCell<ItemData>,
        render_object: &RcRefCell<RenderObjectData<'a>>,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
        velocity: &Vector3<f32>,
    ) -> Item<'a> {
        let mut item = Item {
            _item_name: String::from(item_name),
            _item_id: item_id,
            _item_data: item_data.clone(),
            _render_object: render_object.clone(),
            _item_properties: Box::new(ItemProperties {
                _position: position.clone(),
                _rotation: rotation.clone(),
                _scale: scale.clone(),
                _velocity: velocity.clone(),
                _is_on_ground: false,
            }),
            _item_updater: create_item_updater(item_data.borrow()._item_type),
        };
        item.initialize_item();
        item
    }

    pub fn initialize_item(&mut self) {
        self.update_transform();
    }

    pub fn get_item_id(&self) -> ItemID {
        self._item_id
    }

    pub fn collide_point(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._collision.collide_point(pos)
    }

    pub fn update_transform(&mut self) {
        self._render_object
            .borrow_mut()
            ._transform_object
            .set_position_rotation_scale(
                &self._item_properties._position,
                &self._item_properties._rotation,
                &self._item_properties._scale,
            );
    }

    pub fn update_item(&mut self, height_map_data: &HeightMapData, delta_time: f64) {
        let owner = ptr_as_mut(self);
        self._item_updater
            .update_item_updater(owner, height_map_data, delta_time);
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
    pub fn create_item(&mut self, item_create_info: &ItemCreateInfo) -> RcRefCell<Item<'a>> {
        let game_resources = ptr_as_ref(self._game_resources);
        let mut spawn_point = item_create_info._position.clone();
        spawn_point.y = spawn_point.y.max(self.get_scene_manager().get_height_map_data().get_height_bilinear(&spawn_point, 0));

        let item_data = game_resources.get_item_data(item_create_info._item_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: item_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_dynamic_render_object(
            item_create_info._item_data_name.as_str(),
            &render_object_create_info,
            Some(CollisionType::NONE),
        );
        let id = self.generate_id();
        let item = newRcRefCell(Item::create_item(
            id,
            item_create_info._item_data_name.as_str(),
            item_data,
            &render_object_data,
            &spawn_point,
            &item_create_info._rotation,
            &item_create_info._scale,
            &item_create_info._velocity,
        ));
        self._items.insert(id, item.clone());
        item
    }

    pub fn instance_pickup_item(&mut self, item_create_info: &ItemCreateInfo) -> bool {
        let game_resources = ptr_as_ref(self._game_resources);
        let item_data = game_resources.get_item_data(item_create_info._item_data_name.as_str());
        let item_count = 1;
        self.pick_item(&item_data.borrow()._item_type, item_count)
    }

    pub fn remove_item(&mut self, item: &RcRefCell<Item>) {
        self._items.remove(&item.borrow().get_item_id());
        self.get_scene_manager_mut()
            .remove_static_render_object(item.borrow()._render_object.borrow().get_object_id());
    }

    pub fn clear_items(&mut self) {
        let items = self
            ._items
            .values()
            .cloned()
            .collect::<Vec<RcRefCell<Item>>>();
        for item in items {
            self.remove_item(&item);
        }
    }

    pub fn pick_item(&self, item_data_type: &ItemDataType, item_count: usize) -> bool {
        let success = self
            .get_game_client()
            .get_game_ui_manager_mut()
            .add_item(item_data_type, item_count);
        if success {
            self.get_audio_manager_mut()
                .play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
        success
    }

    pub fn use_inventory_item(&self, item_data_type: &ItemDataType, item_count: usize) -> bool {
        let success = self.get_game_client().get_game_ui_manager_mut().remove_item(item_data_type, item_count);
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

    pub fn use_inventory_item_by_index(&self, item_index: usize) {
        self.select_item_by_index(item_index);
        let item_data_type = self.get_selected_inventory_item_data_type();
        let item_count = 1;
        self.use_inventory_item(&item_data_type, item_count);
    }

    pub fn get_selected_inventory_item_data_type(&self) -> ItemDataType {
        self.get_game_client()
            .get_game_ui_manager()
            .get_selected_inventory_item_data_type()
    }

    pub fn select_next_item(&self) {
        self.get_game_client()
            .get_game_ui_manager_mut()
            .select_next_item()
    }

    pub fn select_previous_item(&self) {
        self.get_game_client()
            .get_game_ui_manager_mut()
            .select_previous_item()
    }

    pub fn select_item_by_index(&self, item_index: usize) {
        self.get_game_client()
            .get_game_ui_manager_mut()
            .select_item_by_index(item_index)
    }

    pub fn update_item_manager(&mut self, delta_time: f64) {
        let game_scene_manager = ptr_as_ref(self._game_scene_manager);
        let scene_manager = ptr_as_ref(self._scene_manager);

        for item in self._items.values() {
            item.borrow_mut()
                .update_item(scene_manager.get_height_map_data(), delta_time);
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
                let check_height = item_ref._render_object.borrow()._bounding_box._min.y
                    <= player_bound_box._max.y
                    && player_bound_box._min.y
                        <= item_ref._render_object.borrow()._bounding_box._max.y;
                if math::get_norm_xz(&diff) <= EAT_ITEM_DISTANCE && check_height {
                    // pick item
                    let item_count = 1;
                    let success =
                        self.pick_item(&item_ref._item_data.borrow()._item_type, item_count);
                    if success {
                        pick_items.push(item.clone());
                    }
                } else if item_ref._item_properties._position.y
                    < scene_manager.get_dead_zone_height()
                {
                    pick_items.push(item.clone());
                }
            }
        }

        for item in pick_items.iter() {
            self.remove_item(item);
        }
    }
}
