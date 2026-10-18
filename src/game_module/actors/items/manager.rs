use crate::game_module::actors::character::Character;
use crate::game_module::actors::common::resolve_actor_id;
use crate::game_module::actors::items::api::{
    Item, ItemCreateInfo, ItemDataType, ItemID, ItemManager, ItemMap, ItemSaveData,
};
use crate::game_module::actors::items::item::get_default_food_hp;
use crate::game_module::actors::props::Prop;
use crate::game_module::game_constants::{
    AUDIO_ITEM_INVENTORY, AUDIO_PICKUP_ITEM, EAT_ITEM_DISTANCE, PROP_NAME_TABLE, TABLE_ITEM_PICKUP_DELAY_INFINITE,
    TABLE_ITEM_SEARCH_MAX_Y_OFFSET, TABLE_ITEM_SEARCH_MIN_Y_OFFSET, TABLE_ITEM_SEARCH_RADIUS_XZ,
    TABLE_ITEM_SPAWN_RANDOM_OFFSET_RANGE, WEAPON_SOCKET_NAME,
};
use crate::game_module::game_scene_manager::{ItemCreateInfoMap, ItemSaveDataMap};
use crate::game_module::game_service_locator::{
    get_character_manager, get_character_manager_mut, get_game_resources, get_game_scene_manager,
    get_game_scene_manager_mut, get_game_ui_manager, get_game_ui_manager_mut,
};
use crate::game_module::scenario::scenario::ScenarioType;
use nalgebra::Vector3;
use rust_engine_3d::audio::audio_manager::AudioLoop;
use rust_engine_3d::core::engine_service_locator::{
    get_audio_manager_mut, get_engine_resources, get_scene_manager, get_scene_manager_mut,
};
use rust_engine_3d::scene::collision::CollisionType;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::socket::Socket;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, extract_name_and_uuid, newRcRefCell};
use std::collections::HashMap;
use uuid::Uuid;

const DROP_INVENTORY_ITEM_BASE_SPEED: f32 = 2.0;
const DROP_INVENTORY_ITEM_RANDOM_SPEED: f32 = 2.0;
const DROP_INVENTORY_ITEM_PICKUP_DELAY: f32 = 1.0;

impl<'a> ItemManager<'a> {
    pub fn create_item_manager() -> Box<ItemManager<'a>> {
        Box::new(ItemManager {
            _items: HashMap::new(),
            _item_name_map: HashMap::new(),
            _table_storage_item_ids: Vec::new(),
            _marker: std::marker::PhantomData,
        })
    }

    pub fn clear_table_storage_items_in_world(&mut self) {
        let item_ids = std::mem::take(&mut self._table_storage_item_ids);
        for item_id in item_ids {
            if let Some(item_refcell) = self._items.get(&item_id).cloned() {
                if item_refcell.borrow().is_valid_table_food() {
                    self._table_storage_item_ids.push(item_id);
                    continue;
                }

                self.remove_item(&item_refcell);
            }
        }
    }

    fn find_eatable_item_on_table(&self, table_prop: &RcRefCell<Prop<'a>>) -> Option<RcRefCell<Item<'a>>> {
        // 1. _table_storage_item_ids 중 유효하고 eatable이며 남은 HP가 있는 아이템 확인
        for id in self._table_storage_item_ids.iter().rev() {
            if let Some(item_rc) = self._items.get(id)
                && item_rc.borrow().is_valid_table_food()
            {
                return Some(item_rc.clone());
            }
        }

        // 2. 테이블 바운딩 박스 주변에 놓인 eatable 아이템 탐색
        let (table_center, table_min_y, table_max_y) = {
            let b = table_prop.borrow();
            let bbox = b.get_bounding_box();
            (bbox._center, bbox._min.y, bbox._max.y)
        };
        for item_rc in self._items.values() {
            let item = item_rc.borrow();
            if item.is_attachment() || !item.is_valid_table_food() {
                continue;
            }
            let pos = item.get_position();
            let dist_xz = ((pos.x - table_center.x).powi(2) + (pos.z - table_center.z).powi(2)).sqrt();
            if dist_xz < TABLE_ITEM_SEARCH_RADIUS_XZ
                && pos.y >= table_min_y - TABLE_ITEM_SEARCH_MIN_Y_OFFSET
                && pos.y <= table_max_y + TABLE_ITEM_SEARCH_MAX_Y_OFFSET
            {
                return Some(item_rc.clone());
            }
        }

        None
    }

    pub fn get_item_on_table(&self, table_prop: &RcRefCell<Prop<'a>>) -> Option<String> {
        self.find_eatable_item_on_table(table_prop).map(|item| item.borrow()._item_data_name.clone())
    }

    pub fn has_item_on_table(&self, table_prop: &RcRefCell<Prop<'a>>) -> bool {
        self.find_eatable_item_on_table(table_prop).is_some()
    }

    pub fn consume_item_on_table(&mut self, table_prop: &RcRefCell<Prop<'a>>) -> Option<String> {
        self.clear_table_storage_items_in_world();

        if let Some(item_refcell) = self.find_eatable_item_on_table(table_prop) {
            let (item_data_name, is_finished, item_id) = {
                let mut item = item_refcell.borrow_mut();
                let (name, finished) = item.consume_food_charge();
                (name, finished, item.get_item_id())
            };

            if is_finished {
                self._table_storage_item_ids.retain(|&id| id != item_id);
                self.remove_item(&item_refcell);
            }

            return Some(item_data_name);
        }

        None
    }

    pub fn place_table_storage_item_on_table(
        &mut self,
        target_table: Option<RcRefCell<Prop<'a>>>,
        slot_idx: usize,
    ) -> bool {
        let Some(table) = target_table
            .or_else(|| get_game_scene_manager().get_prop_manager().get_prop_by_name(PROP_NAME_TABLE).cloned())
        else {
            return false;
        };

        let item_to_spawn = get_game_ui_manager_mut()
            .get_table_storage_widget_mut()
            .and_then(|table_storage_widget| table_storage_widget.take_item_at_slot(slot_idx));

        let Some(item_data_name) = item_to_spawn else {
            return false;
        };

        let (table_center, table_top_y) = {
            let table_borrow = table.borrow();
            let bounding_box = table_borrow.get_bounding_box();
            (bounding_box._center, bounding_box._max.y)
        };

        // 테이블 상판 위에 약간의 랜덤 오프셋을 두어 아이템들이 겹치지 않게 배치
        let offset_x = (rand::random::<f32>() - 0.5) * TABLE_ITEM_SPAWN_RANDOM_OFFSET_RANGE;
        let offset_z = (rand::random::<f32>() - 0.5) * TABLE_ITEM_SPAWN_RANDOM_OFFSET_RANGE;
        let spawn_position = Vector3::new(table_center.x + offset_x, table_top_y, table_center.z + offset_z);

        let food_hp = get_default_food_hp(&item_data_name, ItemDataType::Food);
        let item_create_info = ItemCreateInfo {
            _item_data_name: item_data_name.clone(),
            _position: spawn_position,
            _rotation: Vector3::new(0.0, rand::random::<f32>() * std::f32::consts::TAU, 0.0),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _pickup_delay: TABLE_ITEM_PICKUP_DELAY_INFINITE, // 플레이어가 자동 습득하지 않도록 방지
            _current_hp: food_hp,
            _max_hp: food_hp,
            ..Default::default()
        };
        let item = self.create_item(&item_data_name, &item_create_info, None);
        item.borrow_mut()._item_properties._is_on_ground = true;
        self._table_storage_item_ids.push(item.borrow().get_item_id());

        // QuickSlot UI 갱신
        let game_ui_manager = get_game_ui_manager_mut();
        let item_bar = game_ui_manager.get_item_bar_widget_mut();
        item_bar.update_quick_slot_widgets();
        item_bar.update_item_selection_widget();

        get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);

        if let Some(scenario) = get_game_scene_manager_mut().get_game_scenario(ScenarioType::ScenarioIntro_Intro) {
            scenario.borrow_mut().complete_sub_quest_cooking();
        }

        true
    }

    pub fn initialize_item_manager(&mut self) {
        log::info!("initialize_item_manager");
    }

    pub fn destroy_item_manager(&mut self) {}

    pub fn generate_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    pub fn get_item(&self, item_id: ItemID) -> Option<&RcRefCell<Item<'a>>> {
        self._items.get(&item_id)
    }

    pub fn get_item_by_name(&self, item_name: &str) -> Option<&RcRefCell<Item<'a>>> {
        self._item_name_map.get(item_name)
    }

    pub fn create_item(
        &mut self,
        item_name: &str,
        item_create_info: &ItemCreateInfo,
        attach_socket: Option<RcRefCell<Socket>>,
    ) -> RcRefCell<Item<'a>> {
        let (item_name, uuid) = extract_name_and_uuid(item_name);
        let mut spawn_point = item_create_info._position;
        spawn_point.y =
            spawn_point.y.max(get_scene_manager().get_height_map_data().get_height_bilinear(&spawn_point, 0));

        let game_resource = get_game_resources();
        let item_data = game_resource.get_item_data(item_create_info._item_data_name.as_str());
        let item_model_data = get_engine_resources().get_model_data(&item_data.borrow()._model_data_name);
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: item_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };

        let render_object_data = if item_model_data.borrow()._mesh_data.borrow().has_animation_data() {
            get_scene_manager_mut()
                .add_skeletal_render_object(item_create_info._item_data_name.as_str(), &render_object_create_info)
        } else {
            get_scene_manager_mut().add_dynamic_render_object(
                item_create_info._item_data_name.as_str(),
                &render_object_create_info,
                Some(CollisionType::NONE),
            )
        };

        let item_id = resolve_actor_id(item_create_info._item_id, uuid);

        let item = newRcRefCell(Item::create_item(
            item_id,
            item_name.as_str(),
            item_create_info._item_data_name.as_str(),
            item_data,
            &render_object_data,
            attach_socket,
            &spawn_point,
            &item_create_info._rotation,
            &item_create_info._scale,
            &item_create_info._velocity,
            item_create_info._pickup_delay,
            item_create_info._current_hp,
            item_create_info._max_hp,
        ));
        self._items.insert(item_id, item.clone());
        if !self._item_name_map.contains_key(item_name.as_str()) {
            self._item_name_map.insert(item_name, item.clone());
        }
        item
    }

    pub fn instance_pickup_item(&mut self, item_create_info: &ItemCreateInfo) -> bool {
        self.pick_item(item_create_info._item_data_name.as_str(), 1)
    }

    pub fn remove_item(&mut self, item_refcell: &RcRefCell<Item<'a>>) {
        let item = item_refcell.borrow();
        self._items.remove(&item.get_item_id());
        if let Some(target) = self._item_name_map.get(item._item_name.as_str())
            && target.as_ptr() == item_refcell.as_ptr()
        {
            self._item_name_map.remove(item._item_name.as_str());
        }

        if item._render_object.borrow().has_animation() {
            get_scene_manager_mut().remove_skeletal_render_object(item._render_object.borrow().get_object_id());
        } else {
            get_scene_manager_mut().remove_static_render_object(item._render_object.borrow().get_object_id());
        }
    }

    pub fn get_items(&self) -> &ItemMap<'a> {
        &self._items
    }

    pub fn clear_items(&mut self) {
        let items = self._items.values().cloned().collect::<Vec<RcRefCell<Item>>>();
        for item in items {
            self.remove_item(&item);
        }
    }

    pub fn create_items(&mut self, item_create_infos: &ItemCreateInfoMap) {
        for (item_name, item_create_info) in item_create_infos.iter() {
            self.create_item(item_name, item_create_info, None);
        }
    }

    pub fn load_item_save_data(&mut self, item_name: &str, item_save_data: &ItemSaveData) {
        let item = self.create_item(item_name, &item_save_data._item_create_info, None);
        item.borrow_mut().load_item_save_data(item_save_data);
    }

    pub fn load_items_save_data(&mut self, item_save_data_map: &ItemSaveDataMap) {
        for (item_name, item_save_data) in item_save_data_map.iter() {
            self.load_item_save_data(item_name, item_save_data);
        }
    }

    pub fn get_items_save_data(&self) -> ItemSaveDataMap {
        self._items.values().map(|item| item.borrow().get_item_save_data()).collect()
    }

    pub fn post_process_after_item_loading(&mut self) {
        for item in self._items.values() {
            item.borrow_mut().post_process_after_item_loading();
        }
        if let Some(table_storage_widget) = get_game_ui_manager().get_table_storage_widget() {
            table_storage_widget.post_process_after_item_loading();
        } else {
            self.clear_table_storage_items_in_world();
        }
    }

    pub fn pick_item(&self, item_data_name: &str, item_count: usize) -> bool {
        let success = get_game_ui_manager_mut().add_item(item_data_name, item_count, true);
        if success {
            get_audio_manager_mut().play_audio_bank(AUDIO_PICKUP_ITEM, AudioLoop::ONCE, None);
        }
        success
    }

    pub fn remove_inventory_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        let success = get_game_ui_manager_mut().remove_item_with_table_storage(item_data_name, item_count);
        if success {
            get_audio_manager_mut().play_audio_bank(AUDIO_ITEM_INVENTORY, AudioLoop::ONCE, None);
        }
        success
    }

    pub fn drop_inventory_item(&mut self, item_data_name: &str, item_count: usize) -> bool {
        let item_data = get_game_resources().get_item_data(item_data_name).borrow();
        if !item_data._item_type.is_droppable() {
            return false;
        }

        let success = self.remove_inventory_item(item_data_name, item_count);
        if success {
            let player = get_character_manager_mut().get_player().borrow();
            let yaw = player.get_rotation().y + (rand::random::<f32>() - 0.5) * std::f32::consts::PI * 0.5;
            let drop_speed = DROP_INVENTORY_ITEM_BASE_SPEED + rand::random::<f32>() * DROP_INVENTORY_ITEM_RANDOM_SPEED;
            let velocity = Vector3::new(-yaw.sin(), 1.0, -yaw.cos()) * drop_speed + player.get_final_velocity();
            let item_create_info = ItemCreateInfo {
                _item_data_name: String::from(item_data_name),
                _position: *player.get_center()
                    + player.get_face_direction() * player.get_collision()._bounding_box._mag_xz,
                _velocity: velocity,
                _pickup_delay: DROP_INVENTORY_ITEM_PICKUP_DELAY,
                ..Default::default()
            };

            self.create_item(item_data_name, &item_create_info, None);
        }
        success
    }

    pub fn attach_item(&mut self, character: &mut Character<'a>, item_data_name: &str) {
        if let Some(attached_item) = character.get_attached_item() {
            if *attached_item.borrow().get_item_data_name() == item_data_name
                && attached_item.borrow()._attach_socket.is_some()
            {
                return;
            } else {
                self.remove_item(attached_item);
                character.detach_item();
            }
        }

        let item_create_info = ItemCreateInfo {
            _item_data_name: String::from(item_data_name),
            ..Default::default()
        };

        let attach_socket = character._render_object.borrow()._sockets.get(&String::from(WEAPON_SOCKET_NAME)).cloned();
        let attached_item = self.create_item(item_data_name, &item_create_info, attach_socket);
        character.attach_item(attached_item);
    }

    pub fn detach_item(&mut self, character: &mut Character<'a>) {
        if let Some(attached_item) = character.get_attached_item() {
            self.remove_item(attached_item);
            character.detach_item();
        }
    }

    pub fn update_item_manager(&mut self, delta_time: f64) {
        let character_manager = get_character_manager();
        let scene_manager = get_scene_manager();

        if !character_manager.is_valid_player() {
            return;
        }

        for item in self._items.values() {
            item.borrow_mut().update_item(scene_manager.get_height_map_data(), delta_time);
        }

        let (player_position, player_bound_box) = {
            let player = character_manager.get_player().borrow();
            (*player.get_position(), player.get_bounding_box().clone())
        };

        let mut items_to_pick: Vec<(String, RcRefCell<Item>)> = Vec::new();
        let mut items_to_remove: Vec<RcRefCell<Item>> = Vec::new();

        for item in self._items.values() {
            let (item_pos, item_min_y, item_max_y, item_data_name, is_pickable) = {
                let item_ref = item.borrow();
                let render_obj = item_ref._render_object.borrow();
                (
                    item_ref._item_properties._position,
                    render_obj._bounding_box._min.y,
                    render_obj._bounding_box._max.y,
                    item_ref._item_data_name.clone(),
                    item_ref.pickable_item(),
                )
            };

            let diff = item_pos - player_position;
            let check_height = item_min_y <= player_bound_box._max.y && player_bound_box._min.y <= item_max_y;

            if check_height && math::get_norm_xz(&diff) <= EAT_ITEM_DISTANCE && is_pickable {
                items_to_pick.push((item_data_name, item.clone()));
            } else if item_pos.y < scene_manager.get_dead_zone_height() {
                items_to_remove.push(item.clone());
            }
        }

        for (item_data_name, item) in items_to_pick {
            if self.pick_item(item_data_name.as_str(), 1) {
                items_to_remove.push(item);
            }
        }

        for item in items_to_remove.iter() {
            self.remove_item(item);
        }
    }
}
